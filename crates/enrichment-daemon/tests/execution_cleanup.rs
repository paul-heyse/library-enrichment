//! F3: cleanup ownership across task drop, removal failure and cancellation.
//!
//! These oracles never trust the runner's own `cleanup_confirmed` boolean. Absence is checked by
//! a separate broker invocation built here, and liveness by a descendant heartbeat written into
//! the mounted capsule. Those are the two facts the checkpoint review found unestablished:
//! killing the Podman client is not killing the container, and a removal that returned an error
//! is not a removal that happened.
use enrichment_core::{config::Execution, execution::ProcessEnd};
use enrichment_daemon::execution::{
    Runner,
    cleanup::{Admission, Supervisor},
};
use std::os::unix::fs::PermissionsExt;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
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
        memory_mib: 256,
        pids: 32,
        ..Execution::default()
    }
}

fn python(code: &str) -> Vec<String> {
    ["/usr/local/bin/python3", "-I", "-S", "-c", code]
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// A private-root broker invocation that shares no code with `Runner`'s own path.
fn raw_broker(root: &Path) -> std::process::Command {
    let mut cmd = std::process::Command::new("/usr/bin/podman");
    cmd.env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("HOME", root.join("home"))
        .env("TMPDIR", root.join("t"))
        .env("XDG_CACHE_HOME", root.join("c"))
        .env("XDG_CONFIG_HOME", root.join("f"))
        .env("XDG_DATA_HOME", root.join("d"))
        .env("XDG_RUNTIME_DIR", root.join("x"));
    for key in ["USER", "LOGNAME", "DBUS_SESSION_BUS_ADDRESS"] {
        if let Some(value) = std::env::var_os(key) {
            cmd.env(key, value);
        }
    }
    for (flag, suffix) in [
        ("--root", "s"),
        ("--runroot", "r"),
        ("--tmpdir", "l"),
        ("--volumepath", "v"),
        ("--network-config-dir", "n"),
        ("--hooks-dir", "h"),
    ] {
        cmd.arg(flag).arg(root.join(suffix));
    }
    cmd.args(["--events-backend", "none", "--cgroup-manager", "systemd"])
        .stdout(std::process::Stdio::null());
    cmd
}

/// Independently: does this container still exist? `container exists` exits 1 when it does not.
fn container_present(root: &Path, name: &str) -> bool {
    raw_broker(root)
        .args(["container", "exists", name])
        .status()
        .expect("the broker runs")
        .code()
        != Some(1)
}

fn owned_names(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root.join("owned")) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
        .filter_map(|e| Some(e.path().file_stem()?.to_str()?.to_owned()))
        .collect()
}

/// Ownership records this test created, ignoring anything the shared private root already held.
///
/// The execution root is a service-owned directory, not a fixture: asserting it is globally
/// empty would make one test's leftovers look like another test's defect.
fn owned_since(root: &Path, before: &[String]) -> Vec<String> {
    let mut names: Vec<String> = owned_names(root)
        .into_iter()
        .filter(|n| !before.contains(n))
        .collect();
    names.sort();
    names
}

async fn until(deadline: Duration, mut condition: impl FnMut() -> bool) -> bool {
    let start = tokio::time::Instant::now();
    while start.elapsed() < deadline {
        if condition() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    condition()
}

fn heartbeat(root: &Path, name: &str) -> Option<u64> {
    let output = raw_broker(root)
        .args([
            "exec",
            name,
            "/usr/local/bin/python3",
            "-I",
            "-S",
            "-c",
            "import os; print(os.stat('/capsule/beat').st_size)",
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout).ok()?.trim().parse().ok()
}

/// An independent broker reads this heartbeat from live tmpfs; inputs stay read-only.
const HEARTBEAT: &str = r"
import os, time, pathlib
if os.fork() == 0:
    beat = pathlib.Path('/capsule/beat')
    while True:
        with beat.open('a') as handle:
            handle.write('x')
        time.sleep(0.05)
time.sleep(600)
";

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn a_dropped_execution_task_still_removes_its_container_and_descendants() {
    let config = config();
    let root = config.storage_root.clone().unwrap();
    let before = owned_names(&root);
    let capsule = tempfile::tempdir().unwrap();
    let permits = Arc::new(tokio::sync::Semaphore::new(1));
    let supervisor = Supervisor::new(Arc::clone(&permits), 60, 1);
    let cache = tempfile::tempdir().unwrap();
    let runner = Runner::new(
        &config,
        cache.path(),
        Arc::clone(&supervisor),
        ownership(cache.path()),
    )
    .unwrap();

    let image = config.python_image.clone().unwrap();
    let mount = capsule.path().to_path_buf();
    let task = tokio::spawn(async move {
        runner
            .run(
                &image,
                &mount,
                &python(HEARTBEAT),
                Arc::new(AtomicBool::new(false)),
            )
            .await
    });

    let live = until(Duration::from_secs(15), || {
        owned_since(&root, &before)
            .first()
            .and_then(|name| heartbeat(&root, name))
            .is_some_and(|size| size > 0)
    })
    .await;
    if !live {
        task.abort();
        let _ = until(Duration::from_secs(30), || supervisor.is_idle()).await;
        panic!("target heartbeat never became observable in live scratch");
    }
    let names = owned_since(&root, &before);
    assert_eq!(names.len(), 1, "exactly one owned container: {names:?}");
    let name = names[0].clone();
    let first = heartbeat(&root, &name).unwrap();
    assert!(
        until(Duration::from_secs(3), || heartbeat(&root, &name)
            .is_some_and(|size| size > first))
        .await,
        "the descendant heartbeat must advance before cancellation"
    );
    assert!(
        !capsule.path().join("beat").exists(),
        "scratch must never modify admitted host inputs"
    );
    assert!(container_present(&root, &name), "the container must exist");

    // Drop the execution future. `kill_on_drop` reaps the Podman client, not the container.
    task.abort();
    let _ = task.await;

    assert!(
        until(Duration::from_secs(120), || supervisor.is_idle()).await,
        "the cleanup supervisor never confirmed absence after the task was dropped"
    );
    assert!(
        !container_present(&root, &name),
        "an independent broker still sees the container after cleanup claimed to finish"
    );
    assert!(
        owned_since(&root, &before).is_empty(),
        "the durable ownership record may be deleted only after confirmed absence"
    );

    assert!(
        heartbeat(&root, &name).is_none(),
        "removed scratch cannot retain a live heartbeat"
    );
    assert_eq!(
        permits.available_permits(),
        1,
        "the worker permit must be released once absence is confirmed"
    );
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn a_failed_removal_is_reported_truthfully_and_quarantines_admission() {
    let wrapper = tempfile::tempdir().unwrap();
    let counter = wrapper.path().join("rm-attempts");
    let broker = wrapper.path().join("broker.sh");
    // A pass-through broker that refuses the first two removals. Nothing else is altered, so the
    // container really is created, really runs, and really survives the failed removals.
    std::fs::write(
        &broker,
        format!(
            "#!/bin/sh\n\
             for a in \"$@\"; do\n\
             \x20 if [ \"$a\" = rm ]; then\n\
             \x20   n=$(cat {counter} 2>/dev/null || echo 0)\n\
             \x20   if [ \"$n\" -lt 2 ]; then\n\
             \x20     expr $n + 1 > {counter}\n\
             \x20     echo 'injected removal failure' >&2\n\
             \x20     exit 1\n\
             \x20   fi\n\
             \x20 fi\n\
             done\n\
             exec /usr/bin/podman \"$@\"\n",
            counter = counter.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&broker, std::fs::Permissions::from_mode(0o755)).unwrap();

    let mut config = config();
    config.broker_path = Some(broker);
    let root = config.storage_root.clone().unwrap();
    let before = owned_names(&root);
    let capsule = tempfile::tempdir().unwrap();
    let permits = Arc::new(tokio::sync::Semaphore::new(1));
    let supervisor = Supervisor::new(Arc::clone(&permits), 120, 1);
    let cache = tempfile::tempdir().unwrap();
    let runner = Runner::new(
        &config,
        cache.path(),
        Arc::clone(&supervisor),
        ownership(cache.path()),
    )
    .unwrap();

    let observation = runner
        .run(
            config.python_image.as_ref().unwrap(),
            capsule.path(),
            &python("print('done')"),
            Arc::new(AtomicBool::new(false)),
        )
        .await
        .expect("a removal failure must not discard the process evidence");

    // The probe evidence survives whole; only the cleanup claim is withheld.
    assert_eq!(observation.end, ProcessEnd::Exited);
    assert_eq!(observation.exit_code, Some(0));
    assert!(observation.stdout.contains("done"));
    assert!(
        !observation.cleanup_confirmed,
        "a failed removal must never be reported as confirmed cleanup"
    );

    assert!(
        matches!(runner.admission(), Admission::Quarantined { .. }),
        "unresolved cleanup must quarantine new execution admission"
    );
    assert_eq!(
        permits.available_permits(),
        0,
        "the retrying cleanup must hold the worker permit"
    );
    let leaked = owned_since(&root, &before);
    assert_eq!(
        leaked.len(),
        1,
        "the ownership record must survive a failed removal: {leaked:?}"
    );
    assert!(
        container_present(&root, &leaked[0]),
        "the injected failure must leave a real container behind, or this proves nothing"
    );

    assert!(
        until(Duration::from_secs(180), || supervisor.is_idle()).await,
        "the supervisor never recovered from the injected removal failures"
    );
    assert!(
        matches!(runner.admission(), Admission::Open),
        "quarantine must lift once absence is confirmed"
    );
    assert_eq!(
        std::fs::read_to_string(&counter).unwrap().trim(),
        "2",
        "exactly the two injected failures should have been consumed"
    );
    assert!(
        owned_since(&root, &before).is_empty(),
        "no ownership record may linger once absence is confirmed"
    );
    assert!(!container_present(&root, &leaked[0]));
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn cancellation_before_start_runs_no_target_code_and_restart_reconciles_an_orphan() {
    let config = config();
    let root = config.storage_root.clone().unwrap();
    let before = owned_names(&root);
    let capsule = tempfile::tempdir().unwrap();
    let supervisor = Supervisor::new(Arc::new(tokio::sync::Semaphore::new(2)), 60, 2);
    let cache = tempfile::tempdir().unwrap();
    let runner = Runner::new(
        &config,
        cache.path(),
        Arc::clone(&supervisor),
        ownership(cache.path()),
    )
    .unwrap();

    // Creation completes, then cancellation is observed: the target is never started.
    let observation = runner
        .run(
            config.python_image.as_ref().unwrap(),
            capsule.path(),
            &python("import pathlib; pathlib.Path('/capsule/ran').write_text('target code ran')"),
            Arc::new(AtomicBool::new(true)),
        )
        .await
        .unwrap();
    assert_eq!(observation.end, ProcessEnd::Cancelled);
    assert_eq!(observation.exit_code, None);
    assert!(observation.cleanup_confirmed);
    assert!(
        !capsule.path().join("ran").exists(),
        "cancellation before start must not run the target snippet"
    );
    assert!(supervisor.is_idle());
    assert!(owned_since(&root, &before).is_empty());

    // Restart reconciliation: an owned container left by a crashed daemon is removed, and its
    // absence confirmed, before the daemon will serve anything.
    let name = format!("libenr-{}", uuid::Uuid::new_v4().simple());
    assert!(
        raw_broker(&root)
            .args([
                "create",
                "--pull=never",
                "--network=none",
                "--name",
                &name,
                config.python_image.as_ref().unwrap(),
            ])
            .args(python("import time; time.sleep(600)"))
            .status()
            .unwrap()
            .success(),
        "the orphan container must be created"
    );
    assert!(
        raw_broker(&root)
            .args(["start", &name])
            .status()
            .unwrap()
            .success()
    );
    let owner = enrichment_core::canonical::sha256_hex(
        cache
            .path()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .as_bytes(),
    );
    std::fs::write(
        root.join("owned").join(format!("{name}.json")),
        serde_json::to_vec(&serde_json::json!({
            "name": name,
            "capsule": capsule.path(),
            "image": config.python_image,
            "created_at": "2026-09-14T00:00:00Z",
            "owner": owner,
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(container_present(&root, &name));

    runner
        .recover_owned()
        .await
        .expect("reconciliation succeeds");
    assert!(
        !container_present(&root, &name),
        "startup reconciliation must confirm the orphan is gone"
    );
    assert!(!owned_names(&root).contains(&name));
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn a_dropped_or_timed_out_creator_cannot_outlive_its_cleanup_ownership() {
    // Pause the real broker before creation, then let it create only after its caller leaves.
    // A single early `container exists` check would falsely confirm absence in this interval.
    for abort in [true, false] {
        let fixture = tempfile::tempdir().unwrap();
        let entered = fixture.path().join("entered");
        let release = fixture.path().join("release");
        let broker = fixture.path().join("broker.sh");
        std::fs::write(&broker, format!(
            "#!/bin/sh\nfor arg in \"$@\"; do\n if [ \"$arg\" = create ]; then\n touch {entered}\n while [ ! -e {release} ]; do sleep .02; done\n fi\ndone\nexec /usr/bin/podman \"$@\"\n",
            entered = entered.display(), release = release.display(),
        )).unwrap();
        std::fs::set_permissions(&broker, std::fs::Permissions::from_mode(0o755)).unwrap();
        let mut config = config();
        config.broker_path = Some(broker);
        let root = config.storage_root.clone().unwrap();
        let before = owned_names(&root);
        let permits = Arc::new(tokio::sync::Semaphore::new(1));
        let supervisor = Supervisor::new(permits.clone(), 60, 1);
        let runner = Runner::new(
            &config,
            fixture.path(),
            supervisor.clone(),
            ownership(fixture.path()),
        )
        .unwrap();
        let inputs = tempfile::tempdir().unwrap();
        let task = tokio::spawn({
            let runner = runner.clone();
            let image = config.python_image.clone().unwrap();
            let capsule = inputs.path().to_owned();
            async move {
                runner
                    .run(
                        &image,
                        &capsule,
                        &python("from pathlib import Path; Path('/capsule/ran').touch()"),
                        Arc::new(AtomicBool::new(false)),
                    )
                    .await
            }
        });
        let started = until(Duration::from_secs(10), || entered.exists()).await;
        if abort {
            task.abort();
        }
        let result = tokio::time::timeout(Duration::from_secs(20), task).await;
        let names = owned_since(&root, &before);
        let held = permits.available_permits() == 0 && names.len() == 1;
        let absent_before_creation = names.len() == 1 && !container_present(&root, &names[0]);
        let refuses_unsafe_recovery = runner.recover_owned().await.is_err();
        // Always release before checking the findings, so assertion failure cannot strand the
        // test's intentionally paused creator.
        std::fs::write(&release, b"resume creator").unwrap();
        let settled = until(Duration::from_secs(30), || supervisor.is_idle()).await;
        assert!(started, "the real creator never reached the barrier");
        let result = result.expect("caller returns at abort or creation deadline");
        if abort {
            assert!(result.expect_err("owner aborted").is_cancelled());
        } else {
            assert!(
                result
                    .expect("task completes")
                    .expect_err("creation deadline")
                    .to_string()
                    .contains("creation deadline")
            );
        }
        assert!(
            held && absent_before_creation,
            "ownership must survive an absent-name check during creation"
        );
        assert!(
            refuses_unsafe_recovery,
            "restart cannot discard an uncertain creator"
        );
        assert!(
            settled,
            "cleanup did not wait for the delayed creator and then remove its container"
        );
        assert!(owned_since(&root, &before).is_empty());
        assert!(!container_present(&root, &names[0]));
        assert!(
            !fixture.path().join("ran").exists(),
            "an abandoned creation must never start target code"
        );
        assert_eq!(permits.available_permits(), 1);
    }
}

#[tokio::test]
#[ignore = "requires operator-admitted rootless producer image; run just execution-qualify"]
async fn creation_past_the_operation_deadline_never_starts_the_target() {
    let fixture = tempfile::tempdir().unwrap();
    let inputs = tempfile::tempdir().unwrap();
    let broker = fixture.path().join("delayed-broker.sh");
    let started = fixture.path().join("start-was-called");
    std::fs::write(&broker, format!("#!/bin/sh\nfor arg in \"$@\"; do\n if [ \"$arg\" = create ]; then sleep 2; fi\n if [ \"$arg\" = start ]; then touch {}; fi\ndone\nexec /usr/bin/podman \"$@\"\n", started.display())).unwrap();
    std::fs::set_permissions(&broker, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut config = config();
    config.deadline_seconds = 1;
    config.broker_path = Some(broker);
    let root = config.storage_root.clone().unwrap();
    let before = owned_names(&root);
    let supervisor = Supervisor::new(Arc::new(tokio::sync::Semaphore::new(1)), 30, 1);
    let runner = Runner::new(
        &config,
        fixture.path(),
        supervisor.clone(),
        ownership(fixture.path()),
    )
    .unwrap();
    let result = runner
        .run(
            config.python_image.as_ref().unwrap(),
            inputs.path(),
            &python("raise AssertionError('must not execute')"),
            Arc::new(AtomicBool::new(false)),
        )
        .await;
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("creation deadline")
    );
    assert!(until(Duration::from_secs(15), || supervisor.is_idle()).await);
    assert!(
        !started.exists(),
        "host deadline must prevent the actual broker start call"
    );
    assert!(owned_since(&root, &before).is_empty());
}

fn ownership(cache: &std::path::Path) -> enrichment_store::physical_ownership::OwnershipStore {
    let runtime =
        enrichment_store::runtime::QueryRuntime::new(&cache.join("test-spill"), Default::default())
            .unwrap();
    let control =
        enrichment_store::control::ControlStore::open(&cache.join("test-control"), runtime.clone())
            .unwrap();
    enrichment_store::physical_ownership::OwnershipStore::new(control, runtime, cache).unwrap()
}
