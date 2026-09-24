import unittest

from scripts.ci_impact import classify


class CiImpactTests(unittest.TestCase):
    def test_docs_only_is_governance_only(self):
        result = classify(["docs/project-brain/13-CHECKPOINT.md", ".engineering/evidence/x.md"])
        self.assertEqual(result.mode, "governance-only")
        self.assertFalse(result.rust_changed)
        self.assertFalse(result.run_supply_chain)

    def test_m01_runtime_is_local(self):
        result = classify(["crates/core-runtime/src/lib.rs"])
        self.assertEqual(result.mode, "m01")
        self.assertTrue(result.run_m01)
        self.assertFalse(result.run_m02)
        self.assertFalse(result.run_m03)

    def test_core_config_runs_known_reverse_dependent_m02(self):
        result = classify(["crates/core-config/src/lib.rs"])
        self.assertEqual(result.mode, "m01+m02")
        self.assertTrue(result.run_m01)
        self.assertTrue(result.run_m02)

    def test_m02_workspace_is_local(self):
        result = classify(["crates/core-workspace/src/lib.rs"])
        self.assertEqual(result.mode, "m02")
        self.assertTrue(result.run_m02)
        self.assertFalse(result.run_m01)

    def test_m03_work_order_is_local(self):
        result = classify(["crates/core-work-order/src/lib.rs"])
        self.assertEqual(result.mode, "m03")
        self.assertTrue(result.run_m03)
        self.assertFalse(result.run_m01)

    def test_shared_identity_is_full(self):
        result = classify(["crates/core-identity/src/lib.rs"])
        self.assertTrue(result.full)
        self.assertTrue(result.run_m01)
        self.assertTrue(result.run_m02)
        self.assertTrue(result.run_m03)

    def test_workspace_manifest_is_full(self):
        self.assertTrue(classify(["Cargo.lock"]).full)

    def test_workflow_change_is_full(self):
        self.assertTrue(classify([".github/workflows/governance.yml"]).full)

    def test_unknown_surface_is_fail_closed(self):
        result = classify(["crates/future-module/src/lib.rs"])
        self.assertTrue(result.full)
        self.assertIn("unclassified", result.reason)

    def test_m04_product_surface_is_full_until_dedicated_jobs_exist(self):
        self.assertTrue(classify(["crates/core-run-state/src/lib.rs"]).full)
        self.assertTrue(classify(["fuzz/fuzz_targets/m04_replay_journal.rs"]).full)

    def test_module_fuzz_target_selects_module(self):
        self.assertEqual(classify(["fuzz/fuzz_targets/m02_git_evidence.rs"]).mode, "m02")
        self.assertEqual(classify(["fuzz/fuzz_targets/m03_packet_dag.rs"]).mode, "m03")

    def test_multiple_module_changes_form_union(self):
        result = classify([
            "crates/core-runtime/src/lib.rs",
            "crates/core-work-order/src/lib.rs",
        ])
        self.assertEqual(result.mode, "m01+m03")
        self.assertTrue(result.run_m01_fuzz)
        self.assertTrue(result.run_m03_fuzz)

    def test_push_event_is_always_full(self):
        result = classify(["docs/README.md"], event_name="push")
        self.assertTrue(result.full)

    def test_empty_pr_diff_is_fail_closed(self):
        self.assertTrue(classify([]).full)


if __name__ == "__main__":
    unittest.main()
