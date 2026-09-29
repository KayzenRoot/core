# CORE-M04-INV-TOOL-001: bounded offline inventory tooling evidence

Status: TOOLING_CANDIDATE / NO_EXTERNAL_V1_FACTS_ASSERTED
Work Order: https://github.com/KayzenRoot/core/issues/130
Parent compatibility blocker: https://github.com/KayzenRoot/core/issues/111
Starting exact canonical main: 17a198d6d5d4c2058b549d97c93cc3bf0c6637c6

## Purpose and scope

Prepare a fail-closed local owner questionnaire, strict bounded JSON preflight, deterministic negative tests and a runbook. The three categories and four explicit coverage requirements come directly from the unresolved actual-use gate in issue #111 and the draft PR #118 inventory. This is tooling only, not an acceptance of the draft's proposed breaking V1 BRC/provenance amendments. No product implementation, active lock, source hierarchy, Work Order or frozen M04 source bytes are touched.

## Negative security/authority controls

- Neutral template contains UNKNOWN for every finding and every coverage category, FALSE owner attestation and FALSE evidence flags.
- No network, disk scan, external context service access, mutation of inventory data, raw file/path output or asserted external negative facts.
- JSON parser rejects duplicate keys, unexpected/missing schema fields, invalid types/enums, invalid/future dates and inputs over 32 KiB.
- YES routes toward governed V2; UNKNOWN/incomplete inventory remains blocked; structurally all-NO is only an owner/source-review candidate with fixed external_absence_proven=false and breaking_v1_approved=false.
- Unit tests exercise neutral defaults, each category YES/UNKNOWN, every scope/evidence gate, strict parsing, date cases, redaction and exit codes.
- No owner-supplied inventory or attestation was available in the GitHub environment. Actual external status remains UNKNOWN and issue #111 must stay OPEN. All 23 global M04 evidence obligations remain PENDING.

## Assurance gates

Record exact-head hosted Governance Python tests, mandatory full CI (scripts/tests changed), GEF logical owner self-audit NOT INDEPENDENT with zero unresolved HIGH/CRITICAL, protected merge and real full main-push evidence in the PR review issue. Historical CI cannot authorize a changed head. Never infer external proof or commit private owner input from successful hosted tests.
