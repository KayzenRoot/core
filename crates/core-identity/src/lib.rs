//! Deterministic canonical serialization and safety/cache identity primitives.

use core_contracts::{ContentHandle, RuntimeGeneration, SchemaVersion};
use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("serialization failed: {0}")]
    Serialization(String),
    #[error("canonical JSON contains a non-finite number")]
    NonFiniteNumber,
    #[error("content handle locator is not an immutable local reference")]
    InvalidLocator,
}

/// Converts a serializable value to canonical JSON bytes. Object keys are sorted,
/// arrays preserve semantic order, and diagnostic volatility is excluded by callers.
pub fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, IdentityError> {
    let value =
        serde_json::to_value(value).map_err(|e| IdentityError::Serialization(e.to_string()))?;
    let canonical = canonical_value(value)?;
    serde_json::to_vec(&canonical).map_err(|e| IdentityError::Serialization(e.to_string()))
}

pub fn canonical_value(value: Value) -> Result<Value, IdentityError> {
    match value {
        Value::Object(object) => {
            let mut sorted = BTreeMap::new();
            for (key, value) in object {
                sorted.insert(key, canonical_value(value)?);
            }
            let mut result = Map::new();
            for (key, value) in sorted {
                result.insert(key, value);
            }
            Ok(Value::Object(result))
        }
        Value::Array(values) => values
            .into_iter()
            .map(canonical_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Value::Number(number) if number.as_f64().is_some_and(|n| !n.is_finite()) => {
            Err(IdentityError::NonFiniteNumber)
        }
        other => Ok(other),
    }
}

pub fn fingerprint<T: Serialize>(value: &T) -> Result<String, IdentityError> {
    let bytes = canonical_bytes(value)?;
    Ok(hex_digest(&bytes))
}

pub fn fingerprint_bytes(bytes: &[u8]) -> String {
    hex_digest(bytes)
}

fn hex_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// RSG is intentionally secret-free and excludes volatile timestamps/diagnostics.
pub fn runtime_safety_fingerprint<T: Serialize>(
    runtime_version: &str,
    basis: &T,
) -> Result<String, IdentityError> {
    fingerprint(&(&runtime_version, basis))
}

pub fn security_aware_fingerprint<T: Serialize>(
    basis: &T,
    dependency_lock: &str,
    policy_version: &str,
) -> Result<String, IdentityError> {
    fingerprint(&(basis, dependency_lock, policy_version))
}

pub fn generation_fingerprint(generation: &RuntimeGeneration) -> Result<String, IdentityError> {
    fingerprint(generation)
}

pub fn content_handle(
    bytes: &[u8],
    locator: impl Into<String>,
) -> Result<ContentHandle, IdentityError> {
    let locator = locator.into();
    if locator.contains("..") || locator.starts_with("http:") || locator.starts_with("https:") {
        return Err(IdentityError::InvalidLocator);
    }
    Ok(ContentHandle {
        schema: SchemaVersion::CURRENT,
        digest: fingerprint_bytes(bytes),
        byte_len: bytes.len() as u64,
        locator,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Example {
        z: u8,
        a: BTreeMap<String, u8>,
    }

    #[test]
    fn map_order_is_canonical() {
        let mut first = BTreeMap::new();
        first.insert("b".to_owned(), 2);
        first.insert("a".to_owned(), 1);
        let one = Example { z: 3, a: first };
        let two = Example {
            z: 3,
            a: [("a".to_owned(), 1), ("b".to_owned(), 2)]
                .into_iter()
                .collect(),
        };
        assert_eq!(
            canonical_bytes(&one).unwrap(),
            canonical_bytes(&two).unwrap()
        );
        assert_eq!(fingerprint(&one).unwrap(), fingerprint(&two).unwrap());
    }

    #[test]
    fn golden_vector_is_stable() {
        assert_eq!(
            fingerprint_bytes(br#"{"a":1,"b":2}"#),
            "43258cff783fe7036d8a43033f830adfc60ec037382473548ac742b888292777"
        );
    }

    #[test]
    fn content_handles_are_immutable_identity() {
        let handle = content_handle(b"hello", "artifact/hello").unwrap();
        assert_eq!(handle.byte_len, 5);
        assert!(content_handle(b"hello", "../escape").is_err());
    }
}
