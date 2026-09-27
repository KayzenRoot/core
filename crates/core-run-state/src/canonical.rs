use crate::{identity::CanonicalFingerprint, M04ErrorV1, M04_SCHEMA, M04_VERSION};
use core_identity::fingerprint_bytes;
use serde::{Deserialize, Serialize};

const FRAME_MAGIC: &[u8; 8] = b"CORE-M04";

/// Stable domain tags for M04 semantic fingerprints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u16)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FingerprintDomainV1 {
    RunId = 1,
    AttemptId = 2,
    StepId = 3,
    EventId = 4,
    IdempotencyKey = 5,
    Request = 6,
    Event = 7,
    JournalLink = 8,
    BoundaryRevalidation = 9,
    Continuation = 10,
    Snapshot = 11,
    PreparedCommit = 12,
    RunGeneration = 13,
    ExecutionEpoch = 14,
}

/// Stable field tags used by canonical M04 frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub(crate) enum CanonicalFieldTagV1 {
    Schema = 1,
    Version = 2,
    Kind = 3,
    Identity = 4,
    Ordinal = 5,
    Generation = 6,
    Epoch = 7,
    EventSequence = 8,
    Payload = 9,
    PreviousJournalRoot = 10,
    ResultingJournalRoot = 11,
    OperationFingerprint = 12,
    CallerKey = 13,
    Source = 14,
    Target = 15,
}

impl CanonicalFieldTagV1 {
    const fn code(self) -> u16 {
        self as u16
    }
}

/// Incremental versioned M04 frame builder. Fields must be added once in
/// ascending tag order so call order and map iteration cannot change identity.
#[derive(Debug, Clone)]
pub(crate) struct CanonicalFrameV1 {
    bytes: Vec<u8>,
    last_field_tag: Option<u16>,
}

impl CanonicalFrameV1 {
    pub(crate) fn new(domain: FingerprintDomainV1) -> Self {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(FRAME_MAGIC);
        bytes.extend_from_slice(&M04_VERSION.to_be_bytes());
        bytes.extend_from_slice(&(M04_SCHEMA.len() as u16).to_be_bytes());
        bytes.extend_from_slice(M04_SCHEMA.as_bytes());
        bytes.extend_from_slice(&(domain as u16).to_be_bytes());
        Self {
            bytes,
            last_field_tag: None,
        }
    }

    fn begin_field(&mut self, tag: CanonicalFieldTagV1) -> Result<(), M04ErrorV1> {
        let code = tag.code();
        if self.last_field_tag.is_some_and(|last| code <= last) {
            return Err(M04ErrorV1::invalid_canonical_field_order());
        }
        self.bytes.extend_from_slice(&code.to_be_bytes());
        self.last_field_tag = Some(code);
        Ok(())
    }

    pub(crate) fn push_bytes(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: &[u8],
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        let length = u64::try_from(value.len()).map_err(|_| {
            M04ErrorV1::new(
                crate::M04ErrorClassV1::InvalidInput,
                crate::M04ErrorCodeV1::InvalidInput,
                crate::M04RetryabilityV1::Never,
            )
        })?;
        self.bytes.extend_from_slice(&length.to_be_bytes());
        self.bytes.extend_from_slice(value);
        Ok(self)
    }

    pub(crate) fn push_string(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: &str,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.push_bytes(tag, value.as_bytes())
    }

    pub(crate) fn push_u8(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: u8,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        self.bytes.push(value);
        Ok(self)
    }

    pub(crate) fn push_u16(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: u16,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        self.bytes.extend_from_slice(&value.to_be_bytes());
        Ok(self)
    }

    pub(crate) fn push_u32(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: u32,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        self.bytes.extend_from_slice(&value.to_be_bytes());
        Ok(self)
    }

    pub(crate) fn push_u64(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: u64,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        self.bytes.extend_from_slice(&value.to_be_bytes());
        Ok(self)
    }

    pub(crate) fn push_bool(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: bool,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.push_u8(tag, u8::from(value))
    }

    pub(crate) fn push_enum(
        &mut self,
        tag: CanonicalFieldTagV1,
        discriminant: u16,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.push_u16(tag, discriminant)
    }

    pub(crate) fn push_optional_bytes(
        &mut self,
        tag: CanonicalFieldTagV1,
        value: Option<&[u8]>,
    ) -> Result<&mut Self, M04ErrorV1> {
        self.begin_field(tag)?;
        match value {
            None => self.bytes.push(0),
            Some(value) => {
                self.bytes.push(1);
                let length = u64::try_from(value.len()).map_err(|_| {
                    crate::M04ErrorV1::new(
                        crate::M04ErrorClassV1::InvalidInput,
                        crate::M04ErrorCodeV1::InvalidInput,
                        crate::M04RetryabilityV1::Never,
                    )
                })?;
                self.bytes.extend_from_slice(&length.to_be_bytes());
                self.bytes.extend_from_slice(value);
            }
        }
        Ok(self)
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(crate) fn finish(self) -> Vec<u8> {
        self.bytes
    }

    /// Digest this canonical frame with the shared core-identity primitive.
    pub(crate) fn fingerprint(self) -> Result<CanonicalFingerprint, M04ErrorV1> {
        CanonicalFingerprint::from_digest(fingerprint_bytes(&self.bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::{CanonicalFieldTagV1 as Tag, CanonicalFrameV1, FingerprintDomainV1 as Domain};

    fn request_frame() -> CanonicalFrameV1 {
        let mut frame = CanonicalFrameV1::new(Domain::Request);
        frame
            .push_string(Tag::Kind, "CANCEL_RUN_REQUEST")
            .expect("first field");
        frame
            .push_string(Tag::Identity, "run-01")
            .expect("second field");
        frame.push_u64(Tag::Generation, 7).expect("third field");
        frame
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn canonical_request_frame_matches_the_v1_golden_vector() {
        let frame = request_frame();
        assert_eq!(
            frame.as_bytes(),
            request_frame().as_bytes(),
            "identical semantic input must produce identical canonical bytes"
        );
        assert_eq!(
            hex(frame.as_bytes()),
            "434f52452d4d3034000100166e65786c6162732e636f72652e72756e2d737461746500060003000000000000001243414e43454c5f52554e5f524551554553540004000000000000000672756e2d303100060000000000000007"
        );
        assert_eq!(
            frame.clone().fingerprint().expect("valid digest").as_str(),
            "dcf425cbda34f08eb813cf25954f83cb7c33169f1f330719fb502c7e5d337a10"
        );
        assert_eq!(
            request_frame()
                .fingerprint()
                .expect("same semantic input has same digest"),
            request_frame()
                .fingerprint()
                .expect("same semantic input has same digest")
        );
        assert_eq!(frame.clone().finish(), frame.as_bytes());
    }

    #[test]
    fn identical_field_bytes_in_distinct_domains_have_distinct_fingerprints() {
        let mut request = CanonicalFrameV1::new(Domain::Request);
        request
            .push_string(Tag::Identity, "same-bytes")
            .expect("request identity field");

        let mut event = CanonicalFrameV1::new(Domain::Event);
        event
            .push_string(Tag::Identity, "same-bytes")
            .expect("event identity field");

        assert_ne!(
            request.fingerprint().expect("request fingerprint"),
            event.fingerprint().expect("event fingerprint")
        );
    }

    #[test]
    fn canonical_frame_rejects_duplicate_or_out_of_order_field_tags() {
        let mut frame = CanonicalFrameV1::new(Domain::Request);
        frame
            .push_string(Tag::Identity, "run-01")
            .expect("first field");

        let error = frame
            .push_string(Tag::Kind, "CANCEL_RUN_REQUEST")
            .expect_err("backwards field order must fail");
        assert_eq!(
            error.code,
            crate::M04ErrorCodeV1::InvalidCanonicalFieldOrder
        );
    }

    #[test]
    fn internal_frame_builder_keeps_all_v1_primitive_encodings_available() {
        let tags = [
            Tag::Schema,
            Tag::Version,
            Tag::Kind,
            Tag::Identity,
            Tag::Ordinal,
            Tag::Generation,
            Tag::Epoch,
            Tag::EventSequence,
            Tag::Payload,
            Tag::PreviousJournalRoot,
            Tag::ResultingJournalRoot,
            Tag::OperationFingerprint,
            Tag::CallerKey,
            Tag::Source,
            Tag::Target,
        ];
        assert_eq!(
            tags.map(|tag| tag as u16),
            std::array::from_fn(|index| index as u16 + 1)
        );

        let mut frame = CanonicalFrameV1::new(Domain::Request);
        frame.push_u8(Tag::Schema, 1).expect("u8 field");
        frame.push_u16(Tag::Version, 2).expect("u16 field");
        frame.push_enum(Tag::Kind, 3).expect("enum field");
        frame.push_u32(Tag::Identity, 4).expect("u32 field");
        frame.push_u64(Tag::Ordinal, 5).expect("u64 field");
        frame.push_bool(Tag::Generation, true).expect("bool field");
        frame.push_bytes(Tag::Epoch, b"epoch").expect("bytes field");
        frame
            .push_string(Tag::EventSequence, "sequence")
            .expect("string field");
        frame
            .push_optional_bytes(Tag::Payload, None)
            .expect("absent optional bytes");
        frame
            .push_optional_bytes(Tag::PreviousJournalRoot, Some(b"previous"))
            .expect("present optional bytes");
        frame
            .push_bytes(Tag::ResultingJournalRoot, b"result")
            .expect("result root");
        frame
            .push_bytes(Tag::OperationFingerprint, b"operation")
            .expect("operation fingerprint");
        frame
            .push_bytes(Tag::CallerKey, b"caller")
            .expect("caller key");
        frame.push_bytes(Tag::Source, b"source").expect("source");
        frame.push_bytes(Tag::Target, b"target").expect("target");

        let encoded = frame.as_bytes();
        assert!(encoded.starts_with(b"CORE-M04"));
        assert!(encoded.len() > b"CORE-M04".len());
    }

    #[test]
    fn every_v1_domain_has_a_distinct_fixed_header_code() {
        let domains = [
            Domain::RunId,
            Domain::AttemptId,
            Domain::StepId,
            Domain::EventId,
            Domain::IdempotencyKey,
            Domain::Request,
            Domain::Event,
            Domain::JournalLink,
            Domain::BoundaryRevalidation,
            Domain::Continuation,
            Domain::Snapshot,
            Domain::PreparedCommit,
            Domain::RunGeneration,
            Domain::ExecutionEpoch,
        ];
        assert_eq!(
            domains.map(|domain| domain as u16),
            std::array::from_fn(|index| index as u16 + 1)
        );
        for domain in domains {
            let bytes = CanonicalFrameV1::new(domain).finish();
            assert_eq!(
                u16::from_be_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]),
                domain as u16
            );
        }
    }
}
