# CORE Local Deployment: standalone runtime state

Status: `STANDALONE_M01_M03_IMPLEMENTED / PRODUCT_DISTRIBUTION_NOT_ADMITTED`  
Authority: CORE-D-205, CORE-D-206 and CORE-D-207 upon protected-main promotion of #184.

## Verified local development boundary

CORE is an independent headless Rust engineering runtime. M01, M02 workspace/Git V2 and M03 Work Order V2 are implemented and independently tested on protected main by [FULL #36595275630](https://github.com/KayzenRoot/core/actions/runs/36595275630), including Linux/Windows, bounded fuzz, soak and supply-chain checks. Development and local source-based execution require no external project server, Docker, MCP retrieval provider, project registration, remote credentials or network.

## Explicit limitations

Successful local builds/tests do not establish an installer, packaging for end users, production deployment, cloud/remote service, published release, operator dashboard or accepted upgrade/rollback channel. Those require distinct Work Orders and real evidence. Optional generic host transports require independently admitted version, identity, trust and execution policy; none is needed for the current standalone baseline. M04 remains STALE/BLOCKED pending prior-V1 consumer/journal inventory #111; old #106/#118 must not be deployed. M05/M06 are planning only, M23 future local context/evidence only.

The original deployment assumptions are retained below as dated, non-operative source history.


## Sanitized historical archive (non-operative; original provenance retained in Git history)

# CORE Deployment

Status: `PRODUCT_DEPLOYMENT_PENDING_DISCOVERY`

## Bootstrap state
CORE currently has no product runtime to deploy. No VPS, container topology, cloud service or local daemon is selected by this bootstrap.

## LEGACY_PROVIDER dependency boundary
LEGACY_PROVIDER v1.0.0 is deployed separately using LEGACY_PROVIDER's supported Docker Compose distribution. CORE is exposed to LEGACY_PROVIDER only as a read-only project below the configured `LEGACY_PROVIDER_PROJECTS_ROOT`.

## Product deployment
Supported operating systems, packaging, local/cloud topology, persistence, upgrade/rollback and release channels will be selected only after CORE product architecture is frozen.

No deployment readiness claim is made in Phase 0.
