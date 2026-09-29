# CORE Master Module Map

Status: `STANDALONE_CUTOVER / M01_M02_M03_PROMOTED / M04_COMPATIBILITY_BLOCKED / M05_M06_DISCOVERY_ONLY`

## Product boundary

CORE is a standalone, headless engineering runtime; GEF governs source, Work Orders, exact-head testing and audit. **No external project memory/index service or Docker stack is required.** CORE's current canonical Git/Project Brain source must remain available locally even offline.

## Ownership

- **CORE:** runtime, local project/workspace and bounded Git context, capability registry, Work Orders, execution, agents, models, policy, assurance, recovery, Git/CI and evidence.
- **Provisional local context:** use existing canonical Git references and source fingerprints. A new local durable context/index/memory engine is a later, separate Work Order, **not implemented or authorized by this map**.
- **Optional external tools:** generic independently admitted versioned APIs only. No provider claims establish trust or availability; no provider is needed for CORE bootstrap.

## Planned modules

| ID | Module | Ownership |
| --- | --- | --- |
| M01 | Core Runtime & Lifecycle | CORE |
| M02 | Project / Workspace Adapter | CORE local Git |
| M03 | Work Order Engine | CORE |
| M04 | Run / Attempt / Step Engine | CORE; compatibility gate open |
| M05 | Host Adapter Fabric | CORE; planning |
| M06 | Capability Negotiation | CORE; planning |
| M07 | Specialist Registry | CORE |
| M08 | Sequential Agent Orchestrator | CORE |
| M09 | Model & Effort Router | CORE |
| M10 | Execution Policy Engine | CORE |
| M11 | Capability Lease & Sandbox | CORE |
| M12 | Tool / Command Execution Fabric | CORE |
| M13 | Change & Mutation Engine | CORE |
| M14 | Verification Planner | CORE local evidence |
| M15 | Evidence & Proof Engine | CORE |
| M16 | Review & Assurance Engine | CORE |
| M17 | Defect / Correction Engine | CORE local evidence |
| M18 | Recovery & Resume Engine | CORE |
| M19 | Resource / Cost / Quota Governor | CORE |
| M20 | Git / GitHub Delivery Engine | CORE |
| M21 | CI/CD & Release Engine | CORE |
| M22 | Security / Supply-Chain Engine | CORE |
| M23 | Local Context & Evidence Registry | CORE; **future scope** |
| M24 | Headless Event & Telemetry Spine | CORE |

## Progression

M01–M03 implemented. M04 prior-V1 consumer issue #111 remains UNKNOWN. M05 and M06 discovery plans must be rebaselined to this owner-directed standalone source rule before any executable Work Order. Retired project server assumptions are not active requirements; old dated reviews are historical only.
