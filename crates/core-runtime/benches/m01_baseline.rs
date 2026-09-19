use core_config::CoreConfig;
use core_identity::fingerprint;
use core_runtime::Supervisor;
use serde_json::json;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let workload = json!({
        "suite": "M01_BOOT",
        "configuration": "standalone-defaults",
        "module_graph": [],
        "capability_graph": [],
        "hive": "disabled",
        "journal": "append-only-runtime-safety-v1",
    });
    let wnf = fingerprint(&workload).expect("workload fingerprint");
    let hardware_class = json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "family": std::env::consts::FAMILY,
        "logical_cpus": thread::available_parallelism().map_or(1, |value| value.get()),
        "power_mode": std::env::var("CORE_POWER_MODE").unwrap_or_else(|_| "unknown".into()),
    });
    let hardware_class_fingerprint = fingerprint(&hardware_class).expect("hardware fingerprint");
    let toolchain = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".into());
    let baseline_key = fingerprint(&json!({
        "hardware_class_fingerprint": hardware_class_fingerprint.clone(),
        "toolchain": toolchain.clone(),
        "wnf": wnf.clone(),
    }))
    .expect("baseline key");
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
            supervisor.shutdown().expect("shutdown");
            let _ = std::fs::remove_file(journal_path);
        });
        samples.push(started.elapsed());
    }
    samples.sort_unstable();
    let millis = |duration: Duration| duration.as_secs_f64() * 1000.0;
    let result = json!({
        "suite": "BOOT",
        "workload_class": workload,
        "work_normalization_fingerprint": wnf,
        "hardware_class": hardware_class,
        "hardware_class_fingerprint": hardware_class_fingerprint,
        "toolchain": toolchain,
        "baseline_key": baseline_key,
        "runtime_metadata": {
            "power_mode": std::env::var("CORE_POWER_MODE").unwrap_or_else(|_| "unknown".into()),
            "profile": "debug",
        },
        "sample_count": samples.len(),
        "p50_ms": millis(samples[samples.len() * 50 / 100]),
        "p95_ms": millis(samples[samples.len() * 95 / 100]),
        "p99_ms": millis(samples[samples.len() * 99 / 100]),
        "regression_policy": "baseline-relative; compatible WNF and hardware only",
    });
    println!("{}", serde_json::to_string_pretty(&result).expect("json"));
}
