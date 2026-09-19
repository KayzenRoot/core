# CORE Security & Governance

Status: `BOOTSTRAP_BASELINE / PRODUCT_THREAT_MODEL_PENDING`

## Bootstrap security invariants
- Secrets, credentials, tokens and private user data MUST NOT be committed.
- HIVE accesses CORE through the configured `HIVE_PROJECTS_ROOT` read-only project boundary.
- HIVE-derived memory/retrieval state is noncanonical and cannot overwrite Git truth.
- Ambiguous HIVE project identity fails closed.
- Missing/stale/conflicting canonical authority does not become ALLOW or DONE.
- Destructive Git/history operations require explicit governed authorization.
- Public issues/PRs must not contain exploit-sensitive private information.

## Product security
The product threat model, trust boundaries, authentication/authorization, secret lifecycle, dependency policy, abuse controls and release security gates remain `PENDING_DISCOVERY`.

Product implementation cannot claim production security from this bootstrap baseline.
