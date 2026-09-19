# M01 fuzz and adversarial corpus

The M01 parser surface is bounded before allocation and has deterministic mutation tests in the IPC/config/journal unit suites. A release campaign may run these same targets under `cargo-fuzz`:

- mutate length-prefixed IPC frames, including truncated and allocation-bomb lengths;
- mutate TOML safety configuration and unknown keys;
- mutate journal JSON/hash-chain records, truncation and reordering;
- mutate compatibility versions and feature sets.

The first exact-head implementation keeps the fuzz harness dependency-free. CI records the campaign command, corpus fingerprint and duration; no fuzz pass is inferred from a compile-only result.
