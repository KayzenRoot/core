# M01 fuzz and adversarial corpus

The repository now contains a real `cargo-fuzz` workspace with four concrete targets:

- `ipc_frames`: bounded length-prefixed IPC frames, truncation and allocation-bomb lengths;
- `config_toml`: TOML safety configuration and unknown keys;
- `compatibility`: module/capability contracts, versions and feature sets;
- `journal_records`: serialized journal records and malformed payloads.

Each target has a deterministic seed under `fuzz/corpus/`. A bounded campaign must run the targets, record target names, duration, iterations, corpus fingerprint, toolchain/tool version and final exact HEAD. A compile-only result is not a fuzz pass.

Example bounded campaign:

```powershell
cargo +nightly fuzz run --manifest-path fuzz/Cargo.toml ipc_frames -- -runs=1000
cargo +nightly fuzz run --manifest-path fuzz/Cargo.toml config_toml -- -runs=1000
cargo +nightly fuzz run --manifest-path fuzz/Cargo.toml compatibility -- -runs=1000
cargo +nightly fuzz run --manifest-path fuzz/Cargo.toml journal_records -- -runs=1000
```
