//! One Rust authority for setup and runtime broker invocations and qualification identity.
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
    process::Stdio,
};

use enrichment_core::{canonical, capsule_protocol, config::Execution};
use serde::Serialize;
use tokio::process::Command;

pub const SUBDIRECTORIES: &[&str] = &[
    "s", "r", "l", "v", "n", "h", "t", "c", "f", "d", "x", "home", "owned",
];

#[derive(Debug, Clone, Serialize)]
pub struct Broker {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub directory: PathBuf,
}
impl Broker {
    pub fn new(root: &Path, program: &Path) -> Self {
        let mut env = BTreeMap::from([
            ("PATH".into(), "/usr/bin:/bin".into()),
            ("LANG".into(), "C.UTF-8".into()),
        ]);
        for (key, suffix) in [
            ("HOME", "home"),
            ("TMPDIR", "t"),
            ("XDG_CACHE_HOME", "c"),
            ("XDG_CONFIG_HOME", "f"),
            ("XDG_DATA_HOME", "d"),
            ("XDG_RUNTIME_DIR", "x"),
        ] {
            env.insert(key.into(), root.join(suffix).display().to_string());
        }
        for key in ["USER", "LOGNAME", "DBUS_SESSION_BUS_ADDRESS"] {
            if let Ok(value) = std::env::var(key) {
                env.insert(key.into(), value);
            }
        }
        let mut args = Vec::new();
        for (flag, suffix) in [
            ("--root", "s"),
            ("--runroot", "r"),
            ("--tmpdir", "l"),
            ("--volumepath", "v"),
            ("--network-config-dir", "n"),
            ("--hooks-dir", "h"),
        ] {
            args.extend([flag.into(), root.join(suffix).display().to_string()]);
        }
        args.extend(["--events-backend", "none", "--cgroup-manager", "systemd"].map(str::to_owned));
        Self {
            program: program.to_owned(),
            args,
            env,
            directory: root.to_owned(),
        }
    }
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.program);
        command
            .env_clear()
            .envs(&self.env)
            .args(&self.args)
            .current_dir(&self.directory)
            .stdin(Stdio::null())
            .kill_on_drop(true);
        command
    }
}

/// Bind the actual helper, controller source, protocol and effective limits; no clock/TTL.
pub fn containment_identity(config: &Execution) -> io::Result<String> {
    let (helper, _) =
        canonical::sha256_reader(std::fs::File::open(config.executor()?)?, 128 * 1024 * 1024)?;
    let (broker, _) =
        canonical::sha256_reader(std::fs::File::open(config.broker())?, 128 * 1024 * 1024)?;
    Ok(canonical::digest_hex(&serde_json::json!({
        "contract": "bounded-execution/3", "protocol": capsule_protocol::VERSION,
        "helper": helper, "broker": broker,
        "controller": canonical::sha256_hex(include_bytes!("mod.rs")),
        "description": canonical::sha256_hex(include_bytes!("description.rs")),
        "resource_validator": canonical::sha256_hex(include_bytes!("resources.rs")),
        "resources": config.resources()?, "output_bytes": config.output_bytes.clamp(1024,1048576),
        "deadline_seconds": config.deadline_seconds.clamp(1,600),
        "cleanup_deadline_seconds": config.cleanup_deadline_seconds.clamp(5,3600),
    })))
}

/// Qualification owns dedicated sidecar state next to the explicitly selected engine root.
/// Setup must never initialize a user's production evidence/cache through an XDG fallback.
pub fn qualification_paths(root: &Path) -> io::Result<enrichment_store::StatePaths> {
    let parent = root
        .parent()
        .filter(|_| root.is_absolute())
        .ok_or_else(|| io::Error::other("qualification needs an absolute dedicated engine root"))?;
    let name = root
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| io::Error::other("qualification engine root has no name"))?;
    // A new evidence format gets a new qualification sidecar too. Never adopt, migrate or
    // reset the previous generation merely to requalify the same retained container images.
    let generation = enrichment_store::state::GENERATION;
    let state = parent.join(format!(".{name}-qualification-state-{generation}"));
    let paths = enrichment_store::StatePaths::explicit(state.join("cache"), state.join("data"));
    enrichment_store::state::validate_paths(&paths)?;
    Ok(paths)
}

#[derive(Debug, Serialize)]
pub struct Description {
    pub configuration: Execution,
    pub version: &'static str,
    pub broker: Broker,
    pub subdirectories: &'static [&'static str],
    pub qualification_state: PathBuf,
    pub containment_identity: Option<String>,
    pub qualification_unavailable: Option<String>,
    pub resources: enrichment_core::config::ExecutionResources,
    pub probes: BTreeMap<&'static str, Vec<Probe>>,
}
pub fn describe(config: &Execution, root: &Path) -> io::Result<Description> {
    if !root.is_absolute()
        || root
            .to_str()
            .is_none_or(|s| s.contains([':', ',', '\n', '\r']))
    {
        return Err(io::Error::other("invalid execution root"));
    }
    let binding = containment_identity(config);
    let (containment_identity, qualification_unavailable) = match binding {
        Ok(value) => (Some(value), None),
        Err(e) => (None, Some(e.to_string())),
    };
    Ok(Description {
        version: "execution-description/4",
        configuration: config.clone(),
        broker: Broker::new(root, &config.broker()),
        subdirectories: SUBDIRECTORIES,
        qualification_state: qualification_paths(root)?
            .data_root
            .parent()
            .ok_or_else(|| io::Error::other("qualification state parent missing"))?
            .to_owned(),
        containment_identity,
        qualification_unavailable,
        resources: config.resources()?,
        probes: probes(),
    })
}

#[derive(Debug, Serialize)]
pub struct Probe {
    pub tool: &'static str,
    pub argv: Vec<String>,
    pub expected: &'static str,
}
fn probes() -> BTreeMap<&'static str, Vec<Probe>> {
    let probe = |tool, args: &[&str], expected| Probe {
        tool,
        argv: args.iter().map(|s| (*s).into()).collect(),
        expected,
    };
    BTreeMap::from([
        (
            "python",
            vec![
                probe(
                    "python",
                    &["/usr/local/bin/python3", "-I", "-S", "--version"],
                    "Python 3.14.7",
                ),
                probe("ty", &["/opt/producers/bin/ty", "--version"], "ty 0.0.80"),
                probe("uv", &["/opt/producers/bin/uv", "--version"], "uv 0.12.13"),
            ],
        ),
        (
            "rust",
            vec![
                probe(
                    "rustc",
                    &["/usr/local/cargo/bin/rustc", "+1.98.1", "--version"],
                    "1.98.1",
                ),
                probe(
                    "cargo",
                    &["/usr/local/cargo/bin/cargo", "+1.98.1", "--version"],
                    "1.98.1",
                ),
                probe(
                    "rust-analyzer",
                    &["/usr/local/cargo/bin/rust-analyzer", "--version"],
                    "rust-analyzer 1.98.1",
                ),
                probe(
                    "rustdoc-nightly",
                    &[
                        "/usr/local/cargo/bin/rustc",
                        "+nightly-2026-09-13",
                        "--version",
                    ],
                    "1.100.0-nightly",
                ),
            ],
        ),
    ])
}

/// Exercise the production operation protocol and cleanup path, never a second `podman run`.
#[derive(Debug, Serialize)]
pub struct QualificationProbes {
    pub tools: BTreeMap<String, enrichment_core::execution::ProcessObservation>,
    pub resources: super::resources::ResourceProbe,
}

pub async fn probe(
    config: &Execution,
    cache: &Path,
    ecosystem: &str,
    image: &str,
) -> io::Result<QualificationProbes> {
    use std::sync::{Arc, atomic::AtomicBool};
    let probes = probes()
        .remove(ecosystem)
        .ok_or_else(|| io::Error::other("unknown producer ecosystem"))?;
    let _owner = enrichment_store::state::exclusive(cache, ".execution-owner.lock")?;
    let supervisor = super::cleanup::Supervisor::new(
        Arc::new(tokio::sync::Semaphore::new(1)),
        config.cleanup_deadline_seconds,
        1,
    );
    let runner = super::Runner::new(config, cache, supervisor.clone())?;
    runner.recover_owned()?;
    super::budget::recover_orphans(cache)?;
    let runner = runner.admitted().await?;
    let lease = runner
        .lease
        .as_ref()
        .ok_or_else(|| io::Error::other("probe execution lease missing"))?;
    let capsules = cache.join("capsules");
    std::fs::create_dir_all(&capsules)?;
    let root = capsules.join(format!("qualification-{}", uuid::Uuid::new_v4().simple()));
    std::fs::create_dir(&root)?;
    let operation = async {
        let mut observed = BTreeMap::new();
        for probe in probes {
            let outcome = runner
                .run(image, &root, &probe.argv, Arc::new(AtomicBool::new(false)))
                .await?;
            if !outcome.cleanup_confirmed
                || outcome.exit_code != Some(0)
                || outcome.end != enrichment_core::execution::ProcessEnd::Exited
                || !outcome.stdout.contains(probe.expected)
            {
                return Err(io::Error::other(format!(
                    "{} qualification failed: {} {}",
                    probe.tool, outcome.stdout, outcome.stderr
                )));
            }
            observed.insert(probe.tool.into(), outcome);
        }
        let process = runner
            .run(
                image,
                &root,
                &[
                    "/bin/sh".into(),
                    "-c".into(),
                    super::resources::PROBE.into(),
                ],
                Arc::new(AtomicBool::new(false)),
            )
            .await?;
        let resources = super::resources::ResourceProbe::new(config.resources()?, process)?;
        Ok(QualificationProbes {
            tools: observed,
            resources,
        })
    }
    .await;
    let cleanup = supervisor.wait_for_cleanup(lease).await;
    runner.discard_workspace(root);
    cleanup?;
    operation
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qualification_state_is_explicit_physical_and_disjoint_from_engine() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path().join("engine");
        let paths = qualification_paths(&root).unwrap();
        assert_eq!(
            paths.cache_root,
            temporary.path().join(".engine-qualification-state-6/cache")
        );
        assert_eq!(
            paths.data_root,
            temporary.path().join(".engine-qualification-state-6/data")
        );
        assert!(
            !temporary
                .path()
                .join(".engine-qualification-state-6")
                .exists()
        );
        assert!(qualification_paths(Path::new("relative")).is_err());
        assert!(qualification_paths(Path::new("/")).is_err());
        std::os::unix::fs::symlink(
            temporary.path(),
            temporary.path().join(".engine-qualification-state-6"),
        )
        .unwrap();
        assert!(qualification_paths(&root).is_err());
    }

    #[test]
    fn qualification_preserves_the_inactive_generation() {
        let temporary = tempfile::tempdir().unwrap();
        let old = temporary.path().join(".engine-qualification-state");
        std::fs::create_dir_all(&old).unwrap();
        let witness = old.join("retained-receipt");
        std::fs::write(&witness, b"previous-generation").unwrap();
        let paths = qualification_paths(&temporary.path().join("engine")).unwrap();
        enrichment_store::state::initialize(&paths).unwrap();
        enrichment_store::state::verify(&paths).unwrap();
        assert_eq!(std::fs::read(&witness).unwrap(), b"previous-generation");
        assert_eq!(std::fs::read_dir(old).unwrap().count(), 1);
    }
}
