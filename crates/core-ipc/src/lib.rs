//! Bounded, versioned local IPC framing. Transport adapters remain OS-local.

use core_contracts::ErrorCode;
use thiserror::Error;

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
}
