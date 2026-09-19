# GEF Bootstrap in CORE

CORE pins the production-accepted GEF Bootstrap `v1.0.0` release, upstream commit `866fe3af8cccc65c929aaf6a47a924401fa448b3`.

GEF v1.0.0 is distributed as a source workspace, not a published global npm CLI. Therefore CORE does not vendor the entire `gef-bootstrap` repository. It materializes the target-project governance contract required for a new GEF-native project.

## Installed surfaces

- canonical Source Pack in `docs/project-brain/`;
- source hierarchy;
- GEF adoption/profile/baseline/current records;
- Work Order and Context Lock namespaces;
- execution, review and evidence protocols;
- deterministic governance validator;
- GitHub PR/issue/workflow scaffolding;
- HIVE-first integration.

## Upstream validation

To inspect/validate GEF itself separately:

```text
git clone https://github.com/KayzenRoot/gef-bootstrap.git
cd gef-bootstrap
git checkout v1.0.0
npm ci
npm run validate
```

GEF upstream remains independent from CORE product dependencies.
