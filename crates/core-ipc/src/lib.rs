//! Bounded, versioned local IPC framing. Transport adapters remain OS-local.

use core_contracts::ErrorCode;
use std::io;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAGIC: &[u8; 3] = b"CR1";
const HEADER_LEN: usize = 3 + 2 + 2 + 4 + 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

impl ProtocolVersion {
    pub const CURRENT: Self = Self { major: 1, minor: 0 };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub version: ProtocolVersion,
    pub epoch: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalTransport {
    UnixDomainSocket,
    WindowsNamedPipe,
}

impl LocalTransport {
    pub const fn current() -> Self {
        if cfg!(windows) {
            Self::WindowsNamedPipe
        } else {
            Self::UnixDomainSocket
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handshake {
    pub version: ProtocolVersion,
    pub epoch: u64,
    pub max_frame_size: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IpcError {
    #[error("frame is shorter than the protocol header")]
    Truncated,
    #[error("invalid frame magic")]
    BadMagic,
    #[error("protocol major version mismatch")]
    ProtocolMismatch,
    #[error("frame exceeds configured limit")]
    FrameTooLarge,
    #[error("frame has stale execution epoch")]
    StaleEpoch,
    #[error("frame has trailing bytes")]
    TrailingBytes,
}

pub fn encode(frame: &Frame, max_frame_size: usize) -> Result<Vec<u8>, IpcError> {
    if frame.payload.len() > max_frame_size {
        return Err(IpcError::FrameTooLarge);
    }
    let length = u32::try_from(frame.payload.len()).map_err(|_| IpcError::FrameTooLarge)?;
    let mut bytes = Vec::with_capacity(HEADER_LEN + frame.payload.len());
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&frame.version.major.to_be_bytes());
    bytes.extend_from_slice(&frame.version.minor.to_be_bytes());
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(&frame.epoch.to_be_bytes());
    bytes.extend_from_slice(&frame.payload);
    Ok(bytes)
}

pub fn decode(bytes: &[u8], max_frame_size: usize, expected_epoch: u64) -> Result<Frame, IpcError> {
    if bytes.len() < HEADER_LEN {
        return Err(IpcError::Truncated);
    }
    if &bytes[..3] != MAGIC {
        return Err(IpcError::BadMagic);
    }
    let major = u16::from_be_bytes([bytes[3], bytes[4]]);
    let minor = u16::from_be_bytes([bytes[5], bytes[6]]);
    if major != ProtocolVersion::CURRENT.major {
        return Err(IpcError::ProtocolMismatch);
    }
    let length = u32::from_be_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]) as usize;
    if length > max_frame_size {
        return Err(IpcError::FrameTooLarge);
    }
    let epoch = u64::from_be_bytes(bytes[11..19].try_into().expect("header length checked"));
    if epoch != expected_epoch {
        return Err(IpcError::StaleEpoch);
    }
    if bytes.len() < HEADER_LEN + length {
        return Err(IpcError::Truncated);
    }
    if bytes.len() != HEADER_LEN + length {
        return Err(IpcError::TrailingBytes);
    }
    Ok(Frame {
        version: ProtocolVersion { major, minor },
        epoch,
        payload: bytes[HEADER_LEN..].to_vec(),
    })
}

pub fn negotiate(local: Handshake, remote: Handshake) -> Result<Handshake, IpcError> {
    if local.version.major != remote.version.major {
        return Err(IpcError::ProtocolMismatch);
    }
    if local.epoch != remote.epoch {
        return Err(IpcError::StaleEpoch);
    }
    Ok(Handshake {
        version: ProtocolVersion {
            major: local.version.major,
            minor: local.version.minor.min(remote.version.minor),
        },
        epoch: local.epoch,
        max_frame_size: local.max_frame_size.min(remote.max_frame_size),
    })
}

pub fn error_code(error: &IpcError) -> ErrorCode {
    match error {
        IpcError::FrameTooLarge => ErrorCode::FrameTooLarge,
        IpcError::ProtocolMismatch => ErrorCode::ProtocolMismatch,
        IpcError::StaleEpoch => ErrorCode::StaleEpoch,
        _ => ErrorCode::ProtocolMismatch,
    }
}

#[derive(Debug, Error)]
pub enum LocalIpcError {
    #[error("local IPC I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("local IPC protocol failed: {0}")]
    Protocol(#[from] IpcError),
}

/// Reads one bounded frame from an already-connected OS-local stream.
/// The declared payload length is checked before allocating the payload buffer.
pub async fn read_frame<R: AsyncRead + Unpin>(
    reader: &mut R,
    max_frame_size: usize,
    expected_epoch: u64,
) -> Result<Frame, LocalIpcError> {
    let mut header = [0_u8; HEADER_LEN];
    reader.read_exact(&mut header).await?;
    let length = u32::from_be_bytes(header[7..11].try_into().expect("header length")) as usize;
    if length > max_frame_size {
        return Err(IpcError::FrameTooLarge.into());
    }
    let mut encoded = Vec::with_capacity(HEADER_LEN + length);
    encoded.extend_from_slice(&header);
    let mut payload = vec![0_u8; length];
    reader.read_exact(&mut payload).await?;
    encoded.extend_from_slice(&payload);
    Ok(decode(&encoded, max_frame_size, expected_epoch)?)
}

/// Writes one bounded frame to an already-connected OS-local stream.
pub async fn write_frame<W: AsyncWrite + Unpin>(
    writer: &mut W,
    frame: &Frame,
    max_frame_size: usize,
) -> Result<(), LocalIpcError> {
    let encoded = encode(frame, max_frame_size)?;
    writer.write_all(&encoded).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(unix)]
pub mod unix {
    use super::{read_frame, write_frame, Frame, LocalIpcError};
    use std::io;
    use std::path::Path;
    use tokio::net::{UnixListener, UnixStream};

    #[derive(Debug)]
    pub struct Listener {
        inner: UnixListener,
    }

    pub async fn bind(path: impl AsRef<Path>) -> io::Result<Listener> {
        UnixListener::bind(path).map(|inner| Listener { inner })
    }

    impl Listener {
        pub async fn accept(&self) -> io::Result<UnixStream> {
            self.inner.accept().await.map(|(stream, _)| stream)
        }
    }

    pub async fn connect(path: impl AsRef<Path>) -> io::Result<UnixStream> {
        UnixStream::connect(path).await
    }

    pub async fn read(
        stream: &mut UnixStream,
        max_frame_size: usize,
        expected_epoch: u64,
    ) -> Result<Frame, LocalIpcError> {
        read_frame(stream, max_frame_size, expected_epoch).await
    }

    pub async fn write(
        stream: &mut UnixStream,
        frame: &Frame,
        max_frame_size: usize,
    ) -> Result<(), LocalIpcError> {
        write_frame(stream, frame, max_frame_size).await
    }
}

#[cfg(windows)]
pub mod windows {
    use super::{read_frame, write_frame, Frame, LocalIpcError};
    use std::ffi::OsStr;
    use std::io;
    use tokio::net::windows::named_pipe::{
        ClientOptions, NamedPipeClient, NamedPipeServer, PipeMode, ServerOptions,
    };

    #[derive(Debug)]
    pub struct Listener {
        name: String,
        server: NamedPipeServer,
    }

    pub fn bind(name: impl AsRef<OsStr>) -> io::Result<Listener> {
        let name = name.as_ref().to_string_lossy().into_owned();
        let server = ServerOptions::new()
            .pipe_mode(PipeMode::Byte)
            .reject_remote_clients(true)
            .create(&name)?;
        Ok(Listener { name, server })
    }

    impl Listener {
        pub async fn accept(&mut self) -> io::Result<NamedPipeServer> {
            self.server.connect().await?;
            let connected = std::mem::replace(
                &mut self.server,
                ServerOptions::new()
                    .pipe_mode(PipeMode::Byte)
                    .reject_remote_clients(true)
                    .create(&self.name)?,
            );
            Ok(connected)
        }
    }

    pub fn connect(name: impl AsRef<OsStr>) -> io::Result<NamedPipeClient> {
        ClientOptions::new().open(name)
    }

    pub async fn read(
        stream: &mut NamedPipeServer,
        max_frame_size: usize,
        expected_epoch: u64,
    ) -> Result<Frame, LocalIpcError> {
        read_frame(stream, max_frame_size, expected_epoch).await
    }

    pub async fn write(
        stream: &mut NamedPipeClient,
        frame: &Frame,
        max_frame_size: usize,
    ) -> Result<(), LocalIpcError> {
        write_frame(stream, frame, max_frame_size).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_is_bounded_and_epoch_bound() {
        let frame = Frame {
            version: ProtocolVersion::CURRENT,
            epoch: 7,
            payload: b"hello".to_vec(),
        };
        let encoded = encode(&frame, 32).unwrap();
        assert_eq!(decode(&encoded, 32, 7).unwrap(), frame);
        assert_eq!(decode(&encoded, 32, 8), Err(IpcError::StaleEpoch));
    }

    #[test]
    fn malformed_and_allocation_bomb_frames_fail_before_allocation() {
        let frame = Frame {
            version: ProtocolVersion::CURRENT,
            epoch: 1,
            payload: vec![0; 8],
        };
        let mut encoded = encode(&frame, 32).unwrap();
        encoded[7..11].copy_from_slice(&(u32::MAX).to_be_bytes());
        assert_eq!(decode(&encoded, 32, 1), Err(IpcError::FrameTooLarge));
        assert_eq!(decode(&[0, 1], 32, 1), Err(IpcError::Truncated));
    }

    #[test]
    fn handshake_uses_common_minor_and_rejects_epoch_mismatch() {
        let a = Handshake {
            version: ProtocolVersion { major: 1, minor: 2 },
            epoch: 4,
            max_frame_size: 10,
        };
        let b = Handshake {
            version: ProtocolVersion { major: 1, minor: 1 },
            epoch: 4,
            max_frame_size: 20,
        };
        assert_eq!(negotiate(a, b).unwrap().version.minor, 1);
        let mut stale = b;
        stale.epoch = 5;
        assert_eq!(negotiate(a, stale), Err(IpcError::StaleEpoch));
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn unix_domain_socket_round_trip_uses_bounded_framing() {
        let path = std::env::temp_dir().join(format!(
            "core-ipc-{}-{}.sock",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let _ = std::fs::remove_file(&path);
        let listener = unix::bind(&path).await.unwrap();
        let server = tokio::spawn(async move {
            let mut stream = listener.accept().await.unwrap();
            unix::read(&mut stream, 64, 9).await.unwrap()
        });
        let mut client = unix::connect(&path).await.unwrap();
        let frame = Frame {
            version: ProtocolVersion::CURRENT,
            epoch: 9,
            payload: b"native-local-ipc".to_vec(),
        };
        unix::write(&mut client, &frame, 64).await.unwrap();
        assert_eq!(server.await.unwrap(), frame);
        let _ = std::fs::remove_file(path);
    }

    #[cfg(windows)]
    #[tokio::test]
    async fn windows_named_pipe_round_trip_uses_bounded_framing() {
        let name = format!(r"\\.\pipe\core-ipc-{}", std::process::id());
        let mut listener = windows::bind(&name).unwrap();
        let server = tokio::spawn(async move {
            let mut stream = listener.accept().await.unwrap();
            windows::read(&mut stream, 64, 9).await.unwrap()
        });
        let mut client = windows::connect(&name).unwrap();
        let frame = Frame {
            version: ProtocolVersion::CURRENT,
            epoch: 9,
            payload: b"native-local-ipc".to_vec(),
        };
        windows::write(&mut client, &frame, 64).await.unwrap();
        assert_eq!(server.await.unwrap(), frame);
    }
}
