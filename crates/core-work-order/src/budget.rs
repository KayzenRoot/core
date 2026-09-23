use crate::errors::{error, WorkOrderErrorCategoryV1, WorkOrderErrorCodeV1, WorkOrderErrorV1};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceCalibrationStateV1 {
    Uncalibrated,
    Calibrated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M03ResourceBudgetV1 {
    pub max_request_bytes: u64,
    pub max_frozen_bytes: u64,
    pub max_string_bytes: u64,
    pub max_source_refs: u64,
    pub max_packets: u64,
    pub max_packet_edges: u64,
    pub max_scope_rules: u64,
    pub max_criteria: u64,
    pub max_evidence_requirements: u64,
    pub max_acceptance_evidence_edges: u64,
    pub max_lineage_edges: u64,
    pub max_context_refs: u64,
    pub max_correction_rules: u64,
    pub max_diff_entries: u64,
    pub max_diagnostic_entries: u64,
    pub max_parse_depth: u32,
    pub calibration_state: ResourceCalibrationStateV1,
}

impl M03ResourceBudgetV1 {
    pub const CALIBRATED_V1: Self = Self {
        max_request_bytes: 78_333,
        max_frozen_bytes: 78_898,
        max_string_bytes: 4_096,
        max_source_refs: 32,
        max_packets: 32,
        max_packet_edges: 31,
        max_scope_rules: 64,
        max_criteria: 32,
        max_evidence_requirements: 32,
        max_acceptance_evidence_edges: 32,
        max_lineage_edges: 32,
        max_context_refs: 32,
        max_correction_rules: 64,
        max_diff_entries: 3,
        max_diagnostic_entries: 4,
        max_parse_depth: 7,
        calibration_state: ResourceCalibrationStateV1::Calibrated,
    };

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_request_bytes: u64,
        max_frozen_bytes: u64,
        max_string_bytes: u64,
        max_source_refs: u64,
        max_packets: u64,
        max_packet_edges: u64,
        max_scope_rules: u64,
        max_criteria: u64,
        max_evidence_requirements: u64,
        max_acceptance_evidence_edges: u64,
        max_lineage_edges: u64,
        max_context_refs: u64,
        max_correction_rules: u64,
        max_diff_entries: u64,
        max_diagnostic_entries: u64,
        max_parse_depth: u32,
        calibration_state: ResourceCalibrationStateV1,
    ) -> Result<Self, WorkOrderErrorV1> {
        let value = Self {
            max_request_bytes,
            max_frozen_bytes,
            max_string_bytes,
            max_source_refs,
            max_packets,
            max_packet_edges,
            max_scope_rules,
            max_criteria,
            max_evidence_requirements,
            max_acceptance_evidence_edges,
            max_lineage_edges,
            max_context_refs,
            max_correction_rules,
            max_diff_entries,
            max_diagnostic_entries,
            max_parse_depth,
            calibration_state,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), WorkOrderErrorV1> {
        let limits = [
            self.max_request_bytes,
            self.max_frozen_bytes,
            self.max_string_bytes,
            self.max_source_refs,
            self.max_packets,
            self.max_packet_edges,
            self.max_scope_rules,
            self.max_criteria,
            self.max_evidence_requirements,
            self.max_acceptance_evidence_edges,
            self.max_lineage_edges,
            self.max_context_refs,
            self.max_correction_rules,
            self.max_diff_entries,
            self.max_diagnostic_entries,
        ];
        let calibrated_maxima = [
            Self::CALIBRATED_V1.max_request_bytes,
            Self::CALIBRATED_V1.max_frozen_bytes,
            Self::CALIBRATED_V1.max_string_bytes,
            Self::CALIBRATED_V1.max_source_refs,
            Self::CALIBRATED_V1.max_packets,
            Self::CALIBRATED_V1.max_packet_edges,
            Self::CALIBRATED_V1.max_scope_rules,
            Self::CALIBRATED_V1.max_criteria,
            Self::CALIBRATED_V1.max_evidence_requirements,
            Self::CALIBRATED_V1.max_acceptance_evidence_edges,
            Self::CALIBRATED_V1.max_lineage_edges,
            Self::CALIBRATED_V1.max_context_refs,
            Self::CALIBRATED_V1.max_correction_rules,
            Self::CALIBRATED_V1.max_diff_entries,
            Self::CALIBRATED_V1.max_diagnostic_entries,
        ];
        if limits
            .iter()
            .zip(calibrated_maxima)
            .any(|(value, maximum)| *value == 0 || *value == u64::MAX || *value > maximum)
            || self.max_parse_depth == 0
            || self.max_parse_depth == u32::MAX
            || self.max_parse_depth > Self::CALIBRATED_V1.max_parse_depth
            || self.calibration_state != ResourceCalibrationStateV1::Calibrated
        {
            return Err(error(
                WorkOrderErrorCategoryV1::Resource,
                WorkOrderErrorCodeV1::InvalidContextBudget,
            ));
        }
        Ok(())
    }
}

pub(crate) fn ensure_count(count: usize, limit: u64) -> Result<(), WorkOrderErrorV1> {
    if u64::try_from(count).map_or(true, |count| count > limit) {
        return Err(error(
            WorkOrderErrorCategoryV1::Resource,
            WorkOrderErrorCodeV1::CardinalityLimitExceeded,
        ));
    }
    Ok(())
}

pub(crate) fn ensure_serialized_size(size: usize, limit: u64) -> Result<(), WorkOrderErrorV1> {
    if u64::try_from(size).map_or(true, |size| size > limit) {
        return Err(error(
            WorkOrderErrorCategoryV1::Resource,
            WorkOrderErrorCodeV1::SerializedSizeExceeded,
        ));
    }
    Ok(())
}

pub(crate) fn ensure_string_size(value: &str, limit: u64) -> Result<(), WorkOrderErrorV1> {
    if u64::try_from(value.len()).map_or(true, |size| size > limit) {
        return Err(error(
            WorkOrderErrorCategoryV1::Resource,
            WorkOrderErrorCodeV1::CardinalityLimitExceeded,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_or_unbounded_sentinel_budgets_are_rejected() {
        assert!(M03ResourceBudgetV1::new(
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            ResourceCalibrationStateV1::Uncalibrated
        )
        .is_err());
        assert!(M03ResourceBudgetV1::new(
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            1,
            0,
            ResourceCalibrationStateV1::Calibrated
        )
        .is_err());
    }

    #[test]
    fn calibrated_profile_accepts_its_ceiling_and_rejects_every_larger_dimension() {
        let profile = M03ResourceBudgetV1::CALIBRATED_V1;
        assert!(profile.validate().is_ok());

        let mut over = profile.clone();
        over.max_request_bytes += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_frozen_bytes += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_string_bytes += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_source_refs += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_packets += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_packet_edges += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_scope_rules += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_criteria += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_evidence_requirements += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_acceptance_evidence_edges += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_lineage_edges += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_context_refs += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_correction_rules += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_diff_entries += 1;
        assert!(over.validate().is_err());
        let mut over = profile.clone();
        over.max_diagnostic_entries += 1;
        assert!(over.validate().is_err());
        let mut over = profile;
        over.max_parse_depth += 1;
        assert!(over.validate().is_err());
    }
}
