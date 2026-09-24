# CORE Master Module Map

Status: `M01_M02_M03_COMPLETE_M04_DISCOVERY_ACTIVE`

## Product boundary

HIVE is the intelligence plane. CORE is the action plane. GEF is the governance protocol.

CORE MUST run standalone, but when compatible HIVE is available it substitutes HIVE-owned intelligence capabilities instead of duplicating them.

CORE is headless. No dashboard, cockpit, web UI or visual control plane belongs to CORE.

## Capability ownership

- **HIVE-own:** durable organizational memory, retrieval/RAG, HUE repository intelligence, C3 context intelligence, Decision Fabric, temporal/verified learning and canonical knowledge promotion.
- **CORE-own:** work execution, agents, host/model selection, capabilities, mutation, verification, assurance, recovery and delivery.
- **CORE fallback:** only the minimum deterministic/local capability required for safe standalone operation.
- **Shared contract:** HIVE/CORE synchronization uses versioned contracts, fingerprints and events, never shared-database coupling.

## Planned modules

| ID | Module | Primary ownership |
| --- | --- | --- |
| M01 | Core Runtime & Lifecycle | CORE |
| M02 | Project / Workspace Adapter | CORE <-> HIVE |
| M03 | Work Order Engine | CORE |
| M04 | Run / Attempt / Step Engine | CORE |
| M05 | Host Adapter Fabric | CORE |
| M06 | Capability Negotiation | CORE |
| M07 | Specialist Registry | CORE |
| M08 | Sequential Agent Orchestrator | CORE |
| M09 | Model & Effort Router | CORE |
| M10 | Execution Policy Engine | CORE |
| M11 | Capability Lease & Sandbox | CORE |
| M12 | Tool / Command Execution Fabric | CORE |
| M13 | Change & Mutation Engine | CORE |
| M14 | Verification Planner | CORE + HIVE intelligence |
| M15 | Evidence & Proof Engine | CORE |
| M16 | Review & Assurance Engine | CORE |
| M17 | Defect / Correction Engine | CORE + HIVE learning |
| M18 | Recovery & Resume Engine | CORE |
| M19 | Resource / Cost / Quota Governor | CORE |
| M20 | Git / GitHub Delivery Engine | CORE |
| M21 | CI/CD & Release Engine | CORE |
| M22 | Security / Supply-Chain Engine | CORE |
| M23 | HIVE Sync & Federation Protocol | CORE <-> HIVE |
| M24 | Headless Event & Telemetry Spine | CORE |

## Reconciled UADS capabilities

UADS concepts are donor/reference material, not automatic CORE ownership. Context radius/index/impact, durable failure memory, evidence cache intelligence and organizational learning are reconciled against HIVE before admission. Model routing, specialist routing, bounded execution, assurance, recovery and delivery remain strong CORE candidates.

## Explicit non-modules

The following are not CORE modules:
- dashboard;
- cockpit;
- visual agent graph;
- admin web UI;
- analytics UI;
- HIVE-equivalent RAG;
- HIVE-equivalent durable memory;
- HIVE-equivalent semantic repository intelligence;
- shared HIVE/CORE database.

A future NexLabs Console may consume HIVE + CORE APIs/events as a separate product.

## Planning order

Foundation:
M01 -> M02 -> M03 -> M04 -> M05 -> M06

Execution intelligence:
M07 -> M08 -> M09 -> M10 -> M11 -> M12 -> M13

Proof loop:
M14 -> M15 -> M16 -> M17 -> M18

Operations/delivery:
M19 -> M20 -> M21 -> M22

Deep federation:
M23 -> M24

The order may be refined by dependency evidence, but a later module must not silently redefine an accepted earlier contract.


## Current progression
- M01 Core Runtime & Lifecycle — COMPLETE / PROMOTED.
- M02 Project / Workspace Adapter — COMPLETE / PROMOTED.
- M03 Work Order Engine — COMPLETE / PROMOTED.
- M04 Run / Attempt / Step Engine — ROUNDS 1-5 PLANNING PROMOTED / EXECUTION ADMISSION PROMOTED / IMPLEMENTATION AUTHORIZED UNDER CORE-WO-M04-001.
- M05-M24 — discovery-only until their planning freezes.
