use crate::errors::M04ErrorV1;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt;

fn validate_identifier(value: &str) -> Result<(), M04ErrorV1> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.:/".contains(&byte))
    {
        return Err(M04ErrorV1::invalid_identifier());
    }
    Ok(())
}

fn validate_fingerprint(value: &str) -> Result<(), M04ErrorV1> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(M04ErrorV1::invalid_fingerprint());
    }
    Ok(())
}

macro_rules! identifier_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, M04ErrorV1> {
                let value = value.into();
                validate_identifier(&value)?;
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
                Self::new(value).map_err(|_| de::Error::custom("invalid typed M04 identifier"))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
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
            pub fn new(value: impl Into<String>) -> Result<Self, M04ErrorV1> {
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
                Self::new(value).map_err(|_| de::Error::custom("invalid M04 fingerprint"))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

identifier_type!(RunId);
identifier_type!(AttemptId);
identifier_type!(StepId);
identifier_type!(EventId);
identifier_type!(IdempotencyKey);

fingerprint_type!(JournalRoot);
fingerprint_type!(CanonicalFingerprint);

impl CanonicalFingerprint {
    #[allow(dead_code)] // Used by the crate-private frame primitive until semantic builders are admitted.
    pub(crate) fn from_digest(value: String) -> Result<Self, M04ErrorV1> {
        Self::new(value)
    }
}

macro_rules! unsigned_value_type {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(u64);

        impl $name {
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

unsigned_value_type!(AttemptOrdinalV1);
unsigned_value_type!(StepOrdinalV1);
unsigned_value_type!(EventSequenceV1);
unsigned_value_type!(RunGeneration);
unsigned_value_type!(ExecutionEpoch);
