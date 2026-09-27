# CORE M04: offline inventory runbook for previous V1 external use

Status: OPTIONAL_OWNER_EVIDENCE_PREPARATION. Work Order: [#130](https://github.com/KayzenRoot/core/issues/130). External compatibility gate: [#111](https://github.com/KayzenRoot/core/issues/111).

The existing M04 contract/source draft [PR #118](https://github.com/KayzenRoot/core/pull/118) and unmerged implementation [PR #106](https://github.com/KayzenRoot/core/pull/106) must remain blocked until the owner has independently audited external V1 use. GitHub has no merged M04 product crate and no listed CORE releases, but manually exported branch builds, durable records or downstream clients cannot be disproved by repository searches or Actions artifact names.

## On the owner's machine only

Use the current clean CORE checkout. The Python standard-library tool performs **no filesystem discovery**, no network calls and no modifications. Never commit the completed inventory to the repository, attach raw local paths/journals or publish owner/client identities or credentials.

~~~powershell
python scripts/m04_legacy_inventory.py --template
~~~

Copy the neutral JSON template into a private local JSON file and review these **three independent findings**:

1. exported_v1_build_or_api: inspect V1 branch exports, compiled binaries, archives/packages and any published or internal V1 public-API consumers, including forks and other accounts;
2. retained_v1_journal_or_snapshot: inspect existing records, backups, test-to-live transfers, manually exported journals and snapshots outside this Git repository;
3. downstream_consumer_or_deployment: inspect client services, test deployments, external integrations and other locations that might still read or produce M04 V1 data.

The four **coverage** items explicitly ask whether repositories/forks/branches, local builds/export locations, durable-state/backup locations, and downstream clients/deployments were actually checked. Mark CHECKED only for locations genuinely reviewed. Record an independently verifiable, dated confidential evidence reference for each category privately. The JSON contains only its boolean evidence_recorded flag; setting it does **not** validate the evidence's existence or authenticity. Set owner_attested to true only after an actual owner review and leave answers as UNKNOWN whenever coverage is incomplete. The generated as_of date is a convenience default, not proof that any audit happened.

~~~powershell
python scripts/m04_legacy_inventory.py --inventory "C:\private\m04-v1-inventory.json"
~~~

The stdout is intentionally redacted to enumerated status and counts; neither raw file paths nor any supplied free-form content are repeated. For incomplete status the tool exits 2. If **any YES**, it reports V2_DISPOSITION_REQUIRED (exit 3): separately govern an explicitly versioned V2 with prior V1 read-only archival and migration policy. If any UNKNOWN, missing coverage, absent per-category evidence flag or missing explicit attestation remains, it reports BLOCKED_INCOMPLETE (exit 2). If all three findings are explicitly NO, all four scope items CHECKED, the three evidence flags true and the owner attests, it returns NO_LEGACY_CANDIDATE_REQUIRES_REVIEW (exit 0).

**Exit 0 proves only that the questionnaire has complete fields. It does not prove no V1 consumers exist, authorize a breaking V1 contract, close issue #111, promote PR #118 or advance PR #106.** The owner must separately submit a dated, bounded redacted attestation and underlying evidence to a governed contract review; source/lock re-admission, exact-head security CI and fresh self-audit NOT INDEPENDENT still apply.

## Security and STOP

Do not run broad automatic scans of personal disks or cloud accounts, gather secrets, attach raw private input to GitHub, assume an unanswered location is empty or replace frozen V1 bytes. The classifier never assigns all-NO on its own. Any unresolved external YES/UNKNOWN leaves the breaking-V1 branch blocked and routes the design choice to the separate governed V2 track.


## Malformed local CLI arguments are redacted

The CLI also fails closed **before reading any inventory file** when flags are unknown, mutually exclusive or missing a value. Its output is a constant `{"status":"INVALID","reason":"invalid_cli_arguments"}` and exit code **4**, rather than echoing a private JSON path or a mistakenly pasted credential via argparse. This is a terminal-output safeguard only: it does **not** scan devices, validate the truth of owner statements or change the gated status in issue #111. Preserve the private original inventory and submit only bounded, redacted owner evidence for separate review.
