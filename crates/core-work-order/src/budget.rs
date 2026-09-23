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
        let dimensions = [
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
        if dimensions
            .iter()
            .any(|value| *value == 0 || *value == u64::MAX)
            || self.max_parse_depth == 0
            || self.max_parse_depth == u32::MAX
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
        .is_ok());
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
}
