use core_config::CoreConfig;
use core_identity::fingerprint_bytes;
use core_runtime::Supervisor;
use serde_json::json;
use std::time::{Duration, Instant};

fn main() {
    let workload = b"m01-bootstrap-v1|standalone|empty-module-graph|hive-disabled|safe-defaults";
    let wnf = fingerprint_bytes(workload);
    let runtime = tokio::runtime::Runtime::new().expect("runtime");
    let mut samples = Vec::new();
    for index in 0..25_u64 {
        let started = Instant::now();
        runtime.block_on(async {
            let mut config = CoreConfig::defaults(10_000 + index);
            config.journal_path =
                std::env::temp_dir().join(format!("core-bench-{index}-{}", std::process::id()));
            let journal_path = config.journal_path.clone();
            let _ = std::fs::remove_file(&config.journal_path);
            let mut supervisor = Supervisor::new(config).expect("supervisor");
            supervisor.bootstrap().await.expect("bootstrap");
            supervisor.shutdown(true).expect("shutdown");
            let _ = std::fs::remove_file(journal_path);
        });
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    let millis = |duration: Duration| duration.as_secs_f64() * 1000.0;
    let result = json!({
        "suite": "BOOT",
        "work_normalization_fingerprint": wnf,
        "sample_count": samples.len(),
        "p50_ms": millis(samples[samples.len() * 50 / 100]),
        "p95_ms": millis(samples[samples.len() * 95 / 100]),
        "p99_ms": millis(samples[samples.len() * 99 / 100]),
        "regression_policy": "baseline-relative; compatible WNF and hardware only",
    });
    println!("{}", serde_json::to_string_pretty(&result).expect("json"));
}
