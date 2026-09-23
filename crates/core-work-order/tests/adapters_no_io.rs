mod common;

use common::{compiled, fixture};
use core_work_order::*;
use std::cell::Cell;

struct CallerOwnedResolver {
    calls: Cell<usize>,
    batch: SourceResolutionBatchV1,
}

impl CanonicalSourceResolverV1 for CallerOwnedResolver {
    fn resolve(
        &self,
        _refs: &[CanonicalSourceRefV1],
        _budget: &AdapterBudgetV1,
    ) -> Result<SourceResolutionBatchV1, AdapterFailureV1> {
        self.calls.set(self.calls.get() + 1);
        Ok(self.batch.clone())
    }
}

#[test]
fn service_uses_only_explicit_values_after_outer_resolution() {
    let fixture = fixture();
    let resolver = CallerOwnedResolver {
        calls: Cell::new(0),
        batch: fixture.context.sources.clone(),
    };
    let resolved = resolver
        .resolve(
            &fixture.request.sources,
            &AdapterBudgetV1 {
                max_input_bytes: 32_000,
                max_result_entries: 16,
                max_string_bytes: 2_000,
            },
        )
        .unwrap();
    assert_eq!(resolver.calls.get(), 1);

    let mut explicit_context = fixture.context.clone();
    explicit_context.sources = resolved;
    let compilation = compile(&fixture.request, &explicit_context, &fixture.budget).unwrap();
    validate_frozen(&compilation.frozen, &fixture.budget).unwrap();
    assert_eq!(resolver.calls.get(), 1);
}

#[test]
fn external_interfaces_are_separate_from_public_service_inputs() {
    let fixture = fixture();
    let compilation = compiled(&fixture);
    let receipt = validate_frozen(&compilation.frozen, &fixture.budget).unwrap();

    assert_eq!(receipt.work_order_id, *compilation.frozen.work_order_id());
    assert_eq!(receipt.revision, compilation.frozen.revision());
}
