use crate::errors::{error, WorkOrderErrorCategoryV1, WorkOrderErrorCodeV1, WorkOrderErrorV1};
use core_identity::fingerprint;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;

const MAX_TYPED_ID_BYTES: usize = 256;

fn validate_id(value: &str) -> Result<(), WorkOrderErrorV1> {
    if value.is_empty()
        || value.len() > MAX_TYPED_ID_BYTES
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.:/".contains(&b))
    {
        return Err(error(
            WorkOrderErrorCategoryV1::SchemaVersion,
            WorkOrderErrorCodeV1::InvalidId,
        ));
    }
    Ok(())
}

fn validate_fingerprint(value: &str) -> Result<(), WorkOrderErrorV1> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(error(
            WorkOrderErrorCategoryV1::SchemaVersion,
            WorkOrderErrorCodeV1::InvalidId,
        ));
    }
    Ok(())
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, WorkOrderErrorV1> {
                let value = value.into();
                validate_id(&value)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(|_| de::Error::custom("invalid typed identifier"))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

macro_rules! fingerprint_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, WorkOrderErrorV1> {
                let value = value.into();
                validate_fingerprint(&value)?;
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(|_| de::Error::custom("invalid fingerprint"))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

id_type!(WorkOrderId);
id_type!(SourceRefId);
id_type!(WorkPacketId);
id_type!(AcceptanceCriterionId);
id_type!(EvidenceRequirementId);
id_type!(ContextRefId);
id_type!(GovernanceProofId);
id_type!(LineageEdgeId);

fingerprint_type!(WorkOrderFingerprint);
fingerprint_type!(WorkOrderCompilationId);
fingerprint_type!(EvidenceFingerprintV1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct WorkOrderRevision(u32);

impl WorkOrderRevision {
    pub fn new(value: u32) -> Result<Self, WorkOrderErrorV1> {
        if value == 0 {
            return Err(error(
                WorkOrderErrorCategoryV1::SchemaVersion,
                WorkOrderErrorCodeV1::InvalidRevision,
            ));
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }

    pub fn next(self) -> Result<Self, WorkOrderErrorV1> {
        self.0
            .checked_add(1)
            .ok_or_else(|| {
                error(
                    WorkOrderErrorCategoryV1::Lineage,
                    WorkOrderErrorCodeV1::RevisionNotNext,
                )
            })
            .and_then(Self::new)
    }
}

impl<'de> Deserialize<'de> for WorkOrderRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::new(value).map_err(|_| de::Error::custom("revision must be positive"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SemanticFieldIdV1(String);

impl SemanticFieldIdV1 {
    pub const FIELDS: &'static [&'static str] = &[
        "acceptance",
        "context_budget",
        "context_lock_requirement",
        "correction_policy",
        "governance_requirement",
        "lineage",
        "objective",
        "packets",
        "risk_assurance",
        "scope",
        "source_manifest",
        "stop_condition",
        "workspace_requirement",
    ];

    pub fn new(value: impl Into<String>) -> Result<Self, WorkOrderErrorV1> {
        let value = value.into();
        if !Self::FIELDS.contains(&value.as_str()) {
            return Err(error(
                WorkOrderErrorCategoryV1::SchemaVersion,
                WorkOrderErrorCodeV1::InvalidEnumValue,
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SemanticFieldIdV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(|_| de::Error::custom("unknown semantic field"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkOrderLogicalKeyV1 {
    pub project_namespace: String,
    pub module_namespace: String,
    pub stable_key: String,
}

impl WorkOrderLogicalKeyV1 {
    pub fn validate(&self) -> Result<(), WorkOrderErrorV1> {
        validate_id(&self.project_namespace)?;
        validate_id(&self.module_namespace)?;
        validate_id(&self.stable_key)
    }
}

impl WorkOrderId {
    pub fn from_logical_key(key: &WorkOrderLogicalKeyV1) -> Result<Self, WorkOrderErrorV1> {
        key.validate()?;
        let digest =
            fingerprint(&("nexlabs.core.work-order.logical-id.v1", key)).map_err(|_| {
                error(
                    WorkOrderErrorCategoryV1::InternalInvariant,
                    WorkOrderErrorCodeV1::InternalInvariantViolation,
                )
            })?;
        Self::new(format!("woid.v1.{digest}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_ids_and_revisions_reject_invalid_values() {
        assert!(WorkOrderId::new("").is_err());
        assert!(WorkOrderId::new("bad id").is_err());
        assert!(WorkOrderRevision::new(0).is_err());
        assert!(WorkOrderFingerprint::new("A".repeat(64)).is_err());
        assert!(WorkOrderFingerprint::new("0".repeat(63)).is_err());
    }

    #[test]
    fn logical_key_identity_is_deterministic_and_domain_separated() {
        let key = WorkOrderLogicalKeyV1 {
            project_namespace: "nexlabs".into(),
            module_namespace: "m03".into(),
            stable_key: "engine".into(),
        };
        let first = WorkOrderId::from_logical_key(&key).unwrap();
        let second = WorkOrderId::from_logical_key(&key).unwrap();
        assert_eq!(first, second);
        assert!(first.as_str().starts_with("woid.v1."));
    }
}
