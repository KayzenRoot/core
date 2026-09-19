# CORE Deployment

Status: `PRODUCT_DEPLOYMENT_PENDING_DISCOVERY`

## Bootstrap state
CORE currently has no product runtime to deploy. No VPS, container topology, cloud service or local daemon is selected by this bootstrap.

## HIVE dependency boundary
HIVE v1.0.0 is deployed separately using HIVE's supported Docker Compose distribution. CORE is exposed to HIVE only as a read-only project below the configured `HIVE_PROJECTS_ROOT`.

## Product deployment
Supported operating systems, packaging, local/cloud topology, persistence, upgrade/rollback and release channels will be selected only after CORE product architecture is frozen.

No deployment readiness claim is made in Phase 0.
