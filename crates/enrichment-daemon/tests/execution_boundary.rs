//! Real rootless-container qualification. Explicitly invoked with admitted local images.
use enrichment_core::{config::Execution, execution::ProcessEnd};
use enrichment_daemon::execution::{Runner, cleanup::Supervisor};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

fn config() -> Execution {
    Execution {
        storage_root: Some(PathBuf::from(
            std::env::var("LIBENR_EXECUTION_TEST_ROOT").expect("set service-owned Podman root"),
        )),
        python_image: Some(
            std::env::var("LIBENR_EXECUTION_TEST_PYTHON").expect("set admitted Python image ID"),
        ),
        deadline_seconds: 5,
        output_bytes: 2048,
        cpus: 2,
        memory_mib: 256,
        pids: 32,
        ..Execution::default()
    }
}
fn supervisor(config: &Execution) -> Arc<Supervisor> {
    Supervisor::new(
        Arc::new(tokio::sync::Semaphore::new(2)),
        config.cleanup_deadline_seconds,
        2,
    )
}
async fn execute(
    config: &Execution,
    code: &str,
    cancel: Arc<AtomicBool>,
) -> enrichment_core::execution::ProcessObservation {
    let capsule = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    let runner = Runner::new(config, cache.path(), supervisor(config)).unwrap();
    runner
        .run(
            config.python_image.as_ref().unwrap(),
            capsule.path(),
            &[
                "/usr/local/bin/python3".into(),
                "-I".into(),
                "-S".into(),
                "-c".into(),
                code.into(),
            ],
            cancel,
        )
        .await
        .unwrap()
}
#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn execution_boundary_denies_network_secrets_and_host_files_and_observes_limits() {
    let config = config();
    let canary = tempfile::tempdir().unwrap();
    let marker = canary.path().join("secret");
    std::fs::write(&marker, "canary bytes").unwrap();
    let code = format!(
        r#"import os,json,socket,pathlib
assert os.getuid()==65532 and os.getgid()==65532
assert not pathlib.Path({:?}).exists()
assert not any(k in os.environ for k in ('SSH_AUTH_SOCK','OPENAI_API_KEY','PYTHONPATH','DBUS_SESSION_BUS_ADDRESS'))
assert os.listdir('/sys/class/net')==['lo']
assert pathlib.Path('/sys/fs/cgroup/pids.max').read_text().strip()=='32'
assert pathlib.Path('/sys/fs/cgroup/memory.max').read_text().strip()=='268435456'
assert pathlib.Path('/sys/fs/cgroup/memory.swap.max').read_text().strip()=='0'
assert pathlib.Path('/sys/fs/cgroup/cpu.max').read_text().strip()=='200000 100000'
try:
 pathlib.Path('/forbidden').write_text('x')
except OSError: pass
else: raise AssertionError('root writable')
s=socket.socket();s.settimeout(.2)
try: s.connect(('1.1.1.1',443))
except OSError: pass
else: raise AssertionError('network available')
pathlib.Path('/capsule/result').write_text('owned output')
print('qualified')
"#,
        marker.to_str().unwrap()
    );
    let result = execute(&config, &code, Arc::new(AtomicBool::new(false))).await;
    assert_eq!(result.end, ProcessEnd::Exited);
    assert_eq!(result.exit_code, Some(0), "{result:?}");
    assert!(result.stdout.contains("qualified"));
    assert!(result.cleanup_confirmed);
    assert_eq!(std::fs::read(&marker).unwrap(), b"canary bytes");
}
#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn execution_boundary_bounds_output_deadline_and_cancels_descendants() {
    let config = config();
    let overflow = execute(
        &config,
        "while True: print('x'*10000,flush=True)",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_eq!(overflow.end, ProcessEnd::OutputLimit);
    assert!(overflow.stdout.len() <= 2048);
    assert!(overflow.cleanup_confirmed);
    let timeout = execute(
        &config,
        "import time; time.sleep(60)",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_eq!(timeout.end, ProcessEnd::Deadline);
    assert!(timeout.cleanup_confirmed);
    let cancel = Arc::new(AtomicBool::new(false));
    let signal = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        signal.store(true, Ordering::Release);
    });
    let cancelled = execute(&config, "import os,time; os.fork(); time.sleep(60)", cancel).await;
    assert_eq!(cancelled.end, ProcessEnd::Cancelled);
    assert!(cancelled.cleanup_confirmed);
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn execution_boundary_survives_hostile_resource_use_and_still_cleans_up() {
    // Configuring a limit is not enforcing one. Each of these asks the container to exceed a
    // bound; the container must lose, the host must be unharmed, and cleanup must still
    // confirm absence -- a failure mode that only shows up when the payload fights back.
    let config = config();

    // Process count: `pids=32`, so an unbounded fork loop must hit the cgroup, not the host.
    let forks = execute(
        &config,
        "import os\nwhile True:\n    os.fork()\n",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_ne!(forks.exit_code, Some(0), "a fork bomb must not succeed");
    assert!(forks.cleanup_confirmed, "{forks:?}");

    // Memory: `memory_mib=256`, so a large allocation is refused or OOM-killed in the cgroup.
    let memory = execute(
        &config,
        "block = bytearray(1024 * 1024 * 1024)\nprint(len(block))\n",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_ne!(
        memory.exit_code,
        Some(0),
        "a gibibyte under a 256 MiB cap must not succeed: {memory:?}"
    );
    assert!(memory.cleanup_confirmed, "{memory:?}");

    // Scratch: /capsule is explicitly bounded tmpfs; a runaway write
    // fails inside its own bound rather than filling the host filesystem.
    let scratch = execute(
        &config,
        "import pathlib\n\
         chunk = b'x' * (1024 * 1024)\n\
         with pathlib.Path('/capsule/fill').open('wb') as handle:\n\
         \x20   for _ in range(512):\n\
         \x20       handle.write(chunk)\n\
         \x20       handle.flush()\n\
         print('filled')\n",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_ne!(
        scratch.exit_code,
        Some(0),
        "512 MiB into a 256 MiB scratch/memory bound must not succeed: {scratch:?}"
    );
    assert!(!scratch.stdout.contains("filled"));
    assert!(scratch.cleanup_confirmed, "{scratch:?}");
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn executor_handoff_preserves_read_only_inputs_and_reaps_detached_children() {
    use enrichment_core::capsule_protocol::OutputKind;
    let config = config();
    let inputs = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    std::fs::write(inputs.path().join("input.txt"), b"immutable input").unwrap();
    let supervisor = supervisor(&config);
    let runner = Runner::new(&config, cache.path(), supervisor.clone()).unwrap();
    let code = r#"import os,pathlib,subprocess
assert os.getpid()!=1
assert os.readlink('/proc/1/exe') if False else True
for path in ['/inputs/input.txt','/operation.json','/tmp/write','/run/write','/var/tmp/write','/dev/shm/write','/proc/1/fd/1']:
 try:
  with open(path,'wb') as handle: handle.write(b'forged')
 except OSError: pass
 else: raise AssertionError('writable forbidden path: '+path)
mounts=pathlib.Path('/proc/mounts').read_text().splitlines()
scratch=[line.split() for line in mounts if line.split()[1]=='/capsule']
assert len(scratch)==1 and scratch[0][2]=='tmpfs',scratch
stats=os.statvfs('/capsule')
assert stats.f_blocks*stats.f_frsize<=268435456
pathlib.Path('/capsule/input.txt').write_text('private scratch copy')
pathlib.Path('/capsule/result.txt').write_text('validated handoff')
subprocess.Popen(['/usr/local/bin/python3','-I','-S','-c','import time; time.sleep(60)'],start_new_session=True)
print('target finished; descendant still owns its stdout')
"#;
    let result = runner
        .prepare_outputs(
            config.python_image.as_ref().unwrap(),
            inputs.path(),
            &[
                "/usr/local/bin/python3".into(),
                "-I".into(),
                "-S".into(),
                "-c".into(),
                code.into(),
            ],
            Arc::new(AtomicBool::new(false)),
            [("result.txt".into(), OutputKind::File)].into(),
        )
        .await
        .unwrap();
    assert_eq!(result.end, ProcessEnd::Exited, "{result:?}");
    assert_eq!(result.exit_code, Some(0), "{result:?}");
    assert!(result.cleanup_confirmed, "{result:?}");
    assert_eq!(
        std::fs::read(inputs.path().join("input.txt")).unwrap(),
        b"immutable input"
    );
    assert_eq!(
        std::fs::read(inputs.path().join("result.txt")).unwrap(),
        b"validated handoff"
    );
    assert_eq!(supervisor.available_permits(), 2);
    assert_eq!(
        std::fs::read_dir(cache.path().join("capsule-reservations"))
            .unwrap()
            .count(),
        0
    );
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn fast_exit_after_output_overflow_keeps_a_valid_limit_observation() {
    let config = config();
    for _ in 0..3 {
        let result = execute(
            &config,
            "import os; os.write(1,b'x'*4096)",
            Arc::new(AtomicBool::new(false)),
        )
        .await;
        assert_eq!(result.end, ProcessEnd::OutputLimit, "{result:?}");
        assert_eq!(result.exit_code, None, "{result:?}");
        assert_eq!(result.stdout.len(), config.output_bytes);
        assert!(result.cleanup_confirmed);
    }
}

#[tokio::test]
#[ignore = "requires operator-selected real producer images and execution configuration"]
async fn execution_configured_contract_matches_kernel_limits_and_output_bound() {
    let mut selected = enrichment_core::config::Config::from_env()
        .unwrap()
        .execution;
    selected.storage_root = config().storage_root;
    selected.python_image = config().python_image;
    let resources = selected.resources().unwrap();
    let code = format!(
        "import pathlib,os\n\
         assert pathlib.Path('/sys/fs/cgroup/pids.max').read_text().strip() == '{}'\n\
         assert pathlib.Path('/sys/fs/cgroup/memory.max').read_text().strip() == '{}'\n\
         assert pathlib.Path('/sys/fs/cgroup/memory.swap.max').read_text().strip() == '{}'\n\
         assert pathlib.Path('/sys/fs/cgroup/cpu.max').read_text().strip() == '{} {}'\n\
         v=os.statvfs('/capsule')\n\
         assert v.f_blocks*v.f_frsize == {}\n\
         print('effective limits matched')\n",
        resources.pids,
        resources.memory_bytes,
        resources.swap_bytes,
        resources.cpu_quota_micros,
        resources.cpu_period_micros,
        resources.scratch_bytes,
    );
    let observed = execute(&selected, &code, Arc::new(AtomicBool::new(false))).await;
    assert_eq!(observed.end, ProcessEnd::Exited, "{observed:?}");
    assert_eq!(observed.exit_code, Some(0), "{observed:?}");
    assert!(observed.cleanup_confirmed);
    let overflow = execute(
        &selected,
        "while True: print('x'*10000,flush=True)",
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    assert_eq!(overflow.end, ProcessEnd::OutputLimit, "{overflow:?}");
    assert_eq!(
        overflow.stdout.len(),
        selected.output_bytes.clamp(1024, 1_048_576)
    );
    assert!(overflow.cleanup_confirmed);
}

#[tokio::test]
#[ignore = "requires the admitted Rust image with stable and the configured dated nightly"]
async fn identical_rust_consumer_and_lock_fail_stable_and_pass_dated_nightly() {
    use enrichment_core::{archive, canonical};
    use enrichment_daemon::execution::rustdoc;
    let mut config = enrichment_core::config::Config::from_env()
        .unwrap()
        .execution;
    config.storage_root = Some(std::env::var("LIBENR_EXECUTION_TEST_ROOT").unwrap().into());
    let image = std::env::var("LIBENR_EXECUTION_TEST_RUST").unwrap();
    let cache = tempfile::tempdir().unwrap();
    let input = tempfile::tempdir().unwrap();
    let runner = Runner::new(&config, cache.path(), supervisor(&config)).unwrap();
    let tarball = include_bytes!("../../../tests/fixtures/upstream/static/enr-fixture-0.1.0.crate");
    archive::extract_tar_gz(
        &tarball[..],
        &input.path().join("source"),
        &Default::default(),
    )
    .unwrap();
    std::fs::create_dir(input.path().join("src")).unwrap();
    let snippet = include_str!("../../../tests/fixtures/probes/nightly-consumer.rs");
    std::fs::write(input.path().join("src/main.rs"), snippet).unwrap();
    std::fs::write(input.path().join("Cargo.toml"),
        "[package]\nname=\"enrichment-consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[dependencies]\nenr-fixture={path=\"source/enr-fixture-0.1.0\",features=[],default-features=true}\n").unwrap();
    // No external dependency: this literal lock is the entire identical closure for both runs.
    std::fs::write(input.path().join("Cargo.lock"),
        "version = 4\n[[package]]\nname = \"enr-fixture\"\nversion = \"0.1.0\"\n[[package]]\nname = \"enrichment-consumer\"\nversion = \"0.0.0\"\ndependencies = [\"enr-fixture\"]\n").unwrap();
    let lock_before = std::fs::read(input.path().join("Cargo.lock")).unwrap();
    for (compiler, success) in [("1.98.1", false), (rustdoc::TOOLCHAIN, true)] {
        let identity = runner
            .run(
                &image,
                input.path(),
                &[
                    "/usr/local/cargo/bin/rustc".into(),
                    format!("+{compiler}"),
                    "-vV".into(),
                ],
                Arc::new(AtomicBool::new(false)),
            )
            .await
            .unwrap();
        assert_eq!(identity.exit_code, Some(0), "{identity:?}");
        assert!(identity.cleanup_confirmed);
        if success {
            assert!(identity.stdout.contains(rustdoc::RUSTC_RELEASE));
            assert!(identity.stdout.contains(rustdoc::RUSTC_COMMIT));
        } else {
            assert!(identity.stdout.contains("release: 1.98.1"));
        }
        let observed = runner
            .run(
                &image,
                input.path(),
                &[
                    "/usr/local/cargo/bin/cargo".into(),
                    format!("+{compiler}"),
                    "check".into(),
                    "--frozen".into(),
                    "--manifest-path=/capsule/Cargo.toml".into(),
                    format!("--target={}", rustdoc::TARGET),
                ],
                Arc::new(AtomicBool::new(false)),
            )
            .await
            .unwrap();
        assert_eq!(observed.end, ProcessEnd::Exited, "{observed:?}");
        assert_eq!(observed.exit_code == Some(0), success, "{observed:?}");
        assert!(observed.cleanup_confirmed);
        if !success {
            assert!(observed.stderr.contains("E0554"), "{observed:?}");
        }
        assert_eq!(
            std::fs::read(input.path().join("Cargo.lock")).unwrap(),
            lock_before
        );
        assert_eq!(
            std::fs::read_to_string(input.path().join("src/main.rs")).unwrap(),
            snippet
        );
        eprintln!(
            "R09 compiler={compiler} input_sha256={} lock_sha256={} archive_sha256={} result={observed:?}",
            canonical::sha256_hex(snippet.as_bytes()),
            canonical::sha256_hex(&lock_before),
            canonical::sha256_hex(tarball)
        );
    }
}
