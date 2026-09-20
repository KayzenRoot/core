# M02 Resource Calibration Report

Work Order: `CORE-WO-M02-001`
Calibration mode: `CALIBRATION_ONLY`
Authorized base: `bae47b2021a897396109dfcf42e8632dde13ec21`
Calibration input HEAD: `1cb2a73ac49113729d46ddd170cb8d8345672b31`
Platform: Windows x86_64
Rust: `rustc 1.98.1 (48a229cea 2026-09-01)`
Cargo: `1.98.1`
Git: `2.55.0.windows.3`
Python: `3.12.10`
Available parallelism: `8`

## Protocol

Command:

```text
stable cargo bench -p core-workspace --bench m02_workspace
```

The runner creates only synthetic fixtures under the OS temporary directory, performs one warm-up and five measured iterations per scenario, separates cold file hashing from warm derived proof reuse, and makes no network request. Semantic assertions run for every scenario. Existing integration/adversarial tests provide the no-Git, clean/dirty Git, hostile-config, alternates, linked/authority and policy coverage; this report does not claim unsupported fixture scales as measured.

## Measurements

| Scenario | Scale | Iterations | Median us | Range us |
|---|---:|---:|---:|---:|
| system graph build | 100 | 5 | 309 | 223–347 |
| lexical path validation | 1 | 5 | 2 | 1–2 |
| streaming hash | 256 KiB | 5 | 651 | 606–758 |
| event causal map | 1 | 5 | 0 | 0–0 |
| canonical graph fingerprint | 100 | 5 | 182 | 172–212 |
| canonical graph fingerprint | 1,000 synthetic nodes | 5 | 3,099 | 2,614–3,196 |
| warm hash proof reuse | 1 | 5 | 175 | 96–299 |

The complete machine-readable output is reproducible from the benchmark command above. The zero-microsecond event measurement is timer resolution, not an unlimited budget or a correctness shortcut.

## Scale disposition

- Graph scales 1, 10, 100 and 1,000 were covered by fixture or synthetic canonical-fingerprint measurements.
- Changed-path metadata 0/1/10/100 is covered by the property/delta tests; 1,000/10,000 parser-scale fixtures are recorded as `SKIPPED_RESOURCE_BOUND`.
- Untracked policy 0/10/1,000/10,000 is covered by policy tests; top content scales are `SKIPPED_RESOURCE_BOUND` and were not extrapolated.
- A 256 KiB streaming content fixture was measured. Larger content is bounded by explicit finite policy.
- Duplicate proof/hash reuse, event-hinted and zero-event paths are covered by `core-workspace` tests; zero events never create freshness.

## Selected finite budgets

The selected defaults are the finite values in `WorkspaceResourceBudget::finalized()` and were validated by the benchmark plus the security/property gates:

| Budget | Selected value | Evidence/rationale |
|---|---:|---|
| max_repository_graph_nodes | 1,024 | admits measured 1,000 synthetic node fingerprint with finite headroom |
| max_recursion_depth | 32 | finite nested/submodule traversal ceiling from frozen policy |
| max_git_process_duration | 5,000 ms | bounded process deadline; timeout is typed |
| max_stdout_bytes | 4 MiB | finite hostile-output ceiling |
| max_stderr_bytes | 1 MiB | finite diagnostic-output ceiling |
| max_parsed_records | 10,000 | finite cardinality ceiling |
| max_concurrent_git_inspectors | 2 | below available parallelism and bounded |
| max_concurrent_hash_tasks | 2 | below available parallelism and bounded |
| max_single_file_hash_bytes_before_explicit_policy | 16 MiB | explicit large-file policy boundary |
| max_aggregate_hash_bytes_per_validation | 128 MiB | finite aggregate streaming ceiling |
| max_cache_entries | 256 | finite L1 derived-cache bound |
| max_cache_bytes | 8 MiB | finite L1 derived-cache bound |
| max_event_hint_backlog | 1,024 | overflow broadens invalidation |
| max_revalidation_wall_clock | 10,000 ms | finite action-boundary ceiling |

## Rejected candidates and errors

- Zero or sentinel-unlimited candidates: rejected by the configuration validator and security policy; no such candidate was benchmarked as success.
- Graph ceilings below the 1,000-node measured synthetic case: rejected as insufficient for the admitted finite candidate scale.
- Changed-path/untracked top scales beyond the proven local bound: `SKIPPED_RESOURCE_BOUND`; no extrapolated success is claimed.
- Persistent L2 cache, watcher adapters, WMF and alternative Git providers: rejected from this calibration because they are outside the frozen M02 Work Order, not because of an unmeasured performance claim.
- No architecture, ownership, contract semantics, authority class, dependency, provider class, BVM semantics or security invariant changed during calibration.

## Calibration Delta

The only implementation delta is the evidence-backed finalization of the finite `WorkspaceResourceBudget` defaults and its calibration marker/documentation. No product authority or topology rule changed. After this delta, the complete format/lint/test/security/static/fuzz/build gates must be rerun on the exact final head.
