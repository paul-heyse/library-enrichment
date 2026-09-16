//! Rootless offline execution. Only concrete daemon producers construct these commands.
pub mod admission;
pub(crate) mod budget;
pub mod capsule;
pub mod cleanup;
pub mod description;
mod handoff;
mod inventory;
pub(crate) mod ownership;
mod python_closure;
pub(crate) mod readiness;
pub mod rustdoc;
use cleanup::{ContainerGuard, Supervisor};
use enrichment_core::{
    capsule_protocol::{self as protocol, Mode, Operation, OutputKind},
    clock,
    config::Execution,
    execution::{ProcessEnd, ProcessObservation},
};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;

/// A running language server in its own container, with its traffic pipes.
///
/// Holds the [`ContainerGuard`], so dropping this removes the container and its descendants
/// rather than merely detaching from them.
pub struct ServedSession {
    pub operation_id: String,
    pub authority: enrichment_core::execution::ProcessAuthority,
    operation: Operation,
    runner: Runner,
    dispatch: Dispatch,
    /// The exact immutable host generation copied into this server's private scratch.
    pub inputs_root: PathBuf,
    /// The owned container's name.
    pub name: String,
    /// The attached broker client. Reaped on shutdown.
    pub child: tokio::process::Child,
    /// The server's stdin: framed LSP messages go here. Taken once by the session that owns it.
    pub stdin: Option<tokio::process::ChildStdin>,
    /// The server's stdout: framed LSP messages come from here. Taken once.
    pub stdout: Option<tokio::process::ChildStdout>,
    /// The last 64 KiB of the server's stderr, which is where both servers log.
    pub diagnostics: Arc<std::sync::Mutex<Vec<u8>>>,
    /// Removes the container when this session is dropped.
    pub guard: ContainerGuard,
    /// When the container was created, RFC 3339.
    pub started_at: String,
    /// The image the server runs in, for the observation's provenance.
    pub image_id: String,
}

impl ServedSession {
    /// Warm resources retain physical ownership, not permission from an earlier job.
    pub(crate) async fn admit_current_command(&mut self) -> io::Result<()> {
        let dispatch = self
            .runner
            .authorize_process(&self.image_id, &self.operation, false)
            .await?;
        self.authority = dispatch.observation();
        self.dispatch = dispatch;
        Ok(())
    }
    pub(crate) async fn check_authority(&self) -> io::Result<()> {
        self.dispatch.check().await
    }
}

#[derive(Debug, Clone)]
pub struct Runner {
    root: PathBuf,
    cache: PathBuf,
    owner: String,
    limits: Execution,
    broker: PathBuf,
    supervisor: Arc<Supervisor>,
    lease: Option<Arc<cleanup::Lease>>,
    authority: Authority,
}

#[derive(Debug, Clone)]
enum Authority {
    Command,
    Qualification(Arc<description::Qualification>),
}

#[derive(Clone)]
enum Dispatch {
    Command(Arc<enrichment_store::process_grants::ProcessGrant>),
    Qualification(String),
}
impl Dispatch {
    fn operation(&self, probe: Operation) -> Operation {
        match self {
            Self::Command(grant) => grant.operation().clone(),
            Self::Qualification(_) => probe,
        }
    }
    async fn check(&self) -> io::Result<()> {
        match self {
            Self::Command(grant) => grant.check().await.map_err(io::Error::other),
            Self::Qualification(_) => Ok(()),
        }
    }
    fn observation(&self) -> enrichment_core::execution::ProcessAuthority {
        use enrichment_core::execution::ProcessAuthority;
        match self {
            Self::Command(grant) => {
                let w = grant.witness();
                ProcessAuthority::Command {
                    effect_id: w.effect_id.clone(),
                    grant_id: w.grant_id.clone(),
                    environment_id: w.environment_id.clone(),
                    snapshot_id: w.snapshot_id.clone(),
                }
            }
            Self::Qualification(definition_id) => ProcessAuthority::Qualification {
                definition_id: definition_id.clone(),
            },
        }
    }
}

/// Local ownership lasts until container registration transfers the operation to cleanup.
struct OperationRecord {
    path: PathBuf,
    registered: bool,
    ownership: Option<PathBuf>,
}
impl OperationRecord {
    fn write(root: &Path, name: &str, operation: &Operation) -> io::Result<Self> {
        let directory = root.join("operations");
        std::fs::create_dir_all(&directory)?;
        let record = Self {
            path: directory.join(format!("{name}.json")),
            registered: false,
            ownership: None,
        };
        enrichment_store::atomic::write_atomic(&record.path, &serde_json::to_vec(operation)?)?;
        Ok(record)
    }
}
impl Drop for OperationRecord {
    fn drop(&mut self) {
        if !self.registered {
            // No creator can have started before transfer. Remove this fresh ownership
            // obligation even when registration or an earlier durability barrier failed.
            if let Some(path) = &self.ownership {
                let _ = std::fs::remove_file(path);
            }
            match std::fs::remove_file(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(error) => eprintln!("library-enrichmentd: operation record cleanup: {error}"),
            }
        }
    }
}

impl Runner {
    pub fn new(config: &Execution, cache: &Path, supervisor: Arc<Supervisor>) -> io::Result<Self> {
        config.resources()?;
        let root = config
            .storage_root
            .clone()
            .unwrap_or_else(|| cache.join("podman"));
        if !root.is_absolute() || root.to_string_lossy().contains([':', ',', '\n']) {
            return Err(io::Error::other(
                "execution storage must be an absolute service-owned path without delimiters",
            ));
        }
        std::fs::create_dir_all(&root)?;
        // `s` is `--root`, the one the broker is actually pointed at. Podman would create it,
        // but leaving it out made the Python setup scripts' copy of this list disagree with
        // this one, and a storage layout that two halves of the service describe differently is
        // the kind of difference nobody notices until it matters.
        for suffix in description::SUBDIRECTORIES {
            std::fs::create_dir_all(root.join(suffix))?;
            if !std::fs::symlink_metadata(root.join(suffix))?.is_dir() {
                return Err(io::Error::other(
                    "execution storage contains a linked or non-directory component",
                ));
            }
        }
        let physical_root = root.canonicalize()?;
        ownership::register(
            cache,
            &ownership::Root {
                path: physical_root.clone(),
                broker: config.broker(),
            },
        )?;
        Ok(Self {
            cache: cache.canonicalize()?,
            owner: enrichment_core::canonical::sha256_hex(
                cache.canonicalize()?.to_string_lossy().as_bytes(),
            ),
            root: physical_root,
            broker: config.broker(),
            limits: config.clone(),
            supervisor,
            lease: None,
            authority: Authority::Command,
        })
    }

    async fn authorize_process(
        &self,
        image: &str,
        operation: &Operation,
        acquisition: bool,
    ) -> io::Result<Dispatch> {
        match &self.authority {
            Authority::Command => {
                let grant = enrichment_store::native_effect::authorize()
                    .await
                    .map_err(io::Error::other)?;
                let capture = admission::capture(
                    &self.limits,
                    &self.cache,
                    match self.supervisor.admission() {
                        cleanup::Admission::Open => None,
                        cleanup::Admission::Quarantined { detail, .. } => Some(detail),
                    },
                );
                grant
                    .process(enrichment_store::process_grants::Facts {
                        execution: &self.limits,
                        capture,
                        image_id: image,
                        operation,
                        acquisition,
                    })
                    .await
                    .map(|grant| Dispatch::Command(Arc::new(grant)))
                    .map_err(io::Error::other)
            }
            Authority::Qualification(qualification) => {
                qualification.admit(image, operation, acquisition).await?;
                Ok(Dispatch::Qualification(operation.id()))
            }
        }
    }

    /// Carry one execution lease across all preparation and execution stages.
    pub async fn admitted(mut self) -> io::Result<Self> {
        if self.lease.is_none() {
            self.lease = Some(self.supervisor.lease().await?);
        }
        Ok(self)
    }

    pub(crate) fn using_lease(mut self, lease: Arc<cleanup::Lease>) -> Self {
        self.lease = Some(lease);
        self
    }

    /// Delete regenerable scratch only once the container that can use it is confirmed absent.
    /// On retry exhaustion or runtime shutdown, retain it for explicit reconciliation/cleanup.
    pub(crate) fn discard_workspace(&self, path: PathBuf) {
        let Some(lease) = self.lease.clone() else {
            eprintln!(
                "library-enrichmentd: retaining workspace without a continuous lease: {}",
                path.display()
            );
            return;
        };
        if !lease.active_container() {
            if let Err(error) = std::fs::remove_dir_all(&path)
                && error.kind() != io::ErrorKind::NotFound
            {
                eprintln!(
                    "library-enrichmentd: scratch removal {}: {error}",
                    path.display()
                );
            }
            return;
        }
        let supervisor = self.supervisor.clone();
        let Ok(runtime) = tokio::runtime::Handle::try_current() else {
            eprintln!(
                "library-enrichmentd: retaining active workspace after runtime shutdown: {}",
                path.display()
            );
            return;
        };
        runtime.spawn(async move {
            match supervisor.wait_for_cleanup(&lease).await {
                Ok(()) => {
                    if let Err(error) = std::fs::remove_dir_all(&path)
                        && error.kind() != io::ErrorKind::NotFound
                    {
                        eprintln!(
                            "library-enrichmentd: scratch removal {}: {error}",
                            path.display()
                        );
                    }
                }
                Err(error) => eprintln!(
                    "library-enrichmentd: retaining workspace {}: {error}",
                    path.display()
                ),
            }
        });
    }

    /// Whether new execution work may be admitted, or why cleanup is holding it back.
    pub fn admission(&self) -> cleanup::Admission {
        self.supervisor.admission()
    }

    /// Creation continues under supervision if its caller is dropped or times out.
    /// Removal cannot retire ownership until the creator has finished and recorded that fact.
    fn create_supervised(
        &self,
        mut command: Command,
        ownership: PathBuf,
        dispatch: Dispatch,
    ) -> tokio::task::JoinHandle<io::Result<std::process::Output>> {
        let runner = self.clone();
        tokio::spawn(async move {
            let mut record: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&ownership)?)?;
            let created = match dispatch.check().await {
                Ok(()) => command
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn(),
                Err(error) => Err(error),
            };
            let child = match created {
                Ok(child) => child,
                Err(error) => {
                    // A failed spawn created no process. Record that certainty so ordinary
                    // cleanup/recovery can remove this empty ownership obligation.
                    record["creation_in_progress"] = serde_json::Value::Bool(false);
                    enrichment_store::atomic::write_atomic(
                        &ownership,
                        &serde_json::to_vec(&record)?,
                    )?;
                    std::fs::File::open(&ownership)?.sync_all()?;
                    std::fs::File::open(runner.root.join("owned"))?.sync_all()?;
                    return Err(error);
                }
            };
            record["creator_pid"] = serde_json::to_value(child.id())?;
            record["creator_boot_id"] = serde_json::to_value(
                std::fs::read_to_string("/proc/sys/kernel/random/boot_id")?.trim(),
            )?;
            enrichment_store::atomic::write_atomic(&ownership, &serde_json::to_vec(&record)?)?;
            std::fs::File::open(&ownership)?.sync_all()?;
            std::fs::File::open(runner.root.join("owned"))?.sync_all()?;
            let output = child.wait_with_output().await?;
            record["creation_in_progress"] = serde_json::Value::Bool(false);
            enrichment_store::atomic::write_atomic(&ownership, &serde_json::to_vec(&record)?)?;
            std::fs::File::open(&ownership)?.sync_all()?;
            std::fs::File::open(runner.root.join("owned"))?.sync_all()?;
            Ok(output)
        })
    }

    fn broker(&self) -> Command {
        description::Broker::new(&self.root, &self.broker).command()
    }

    pub fn valid_image(image: &str) -> bool {
        image.strip_prefix("sha256:").is_some_and(|s| {
            s.len() == 64
                && s.bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        })
    }

    pub fn containment_identity(&self) -> io::Result<String> {
        description::containment_identity(&self.limits)
    }

    fn operation(
        &self,
        _name: &str,
        image: &str,
        capsule: &Path,
        args: &[String],
        mode: Mode,
        outputs: std::collections::BTreeMap<String, OutputKind>,
    ) -> io::Result<Operation> {
        let operation = Operation {
            version: protocol::VERSION,
            mode,
            argv: args.to_vec(),
            inputs: inventory::capture(capsule, self.limits.scratch_bytes())?,
            outputs,
            data_bytes: self.limits.scratch_bytes(),
            output_bytes: self.limits.output_bytes.clamp(1024, 1048576),
            deadline_millis: self.limits.deadline_seconds.clamp(1, 600) * 1000,
            binding: Operation::binding(image, &self.containment_identity()?)
                .map_err(io::Error::other)?,
        };
        operation.validate()?;
        let bytes = serde_json::to_vec(&operation)?;
        if bytes.len() > protocol::HEADER_LIMIT {
            return Err(io::Error::other(
                "operation input inventory exceeds frame bound",
            ));
        }
        Ok(operation)
    }

    /// Every containment flag, in one place.
    ///
    /// `execute` and `serve` share it so the two cannot drift: a bound that a probe is held to
    /// and a warm language server is not would be a hole with no test to find it. The only
    /// differences either caller gets are the network (acquisition alone reaches a registry)
    /// and the container timeout (a warm session lives until it is idle-evicted, not until a
    /// wall clock expires).
    fn create_args(
        &self,
        name: &str,
        image: &str,
        capsule: &Path,
        acquisition: bool,
        timeout_seconds: Option<u64>,
        interactive: bool,
    ) -> io::Result<Vec<String>> {
        let mut args = capsule::strings(&[
            "create",
            "--pull=never",
            if acquisition {
                "--network=slirp4netns"
            } else {
                "--network=none"
            },
            "--http-proxy=false",
            "--userns=keep-id:uid=65532,gid=65532",
            "--user=65532:65532",
            "--read-only",
            "--read-only-tmpfs=false",
            "--image-volume=ignore",
            "--pid=private",
            "--init=false",
            "--systemd=false",
            "--entrypoint=/libenr-helper",
            "--cap-drop=all",
            "--security-opt=no-new-privileges",
            "--log-driver=none",
            "--no-hosts",
            "--workdir=/capsule",
        ]);
        let resources = self.limits.resources()?;
        args.push(format!("--cpus={}", self.limits.cpus));
        args.push(format!("--memory={}", resources.memory_bytes));
        args.push(format!("--memory-swap={}", resources.memory_bytes));
        args.push(format!("--pids-limit={}", resources.pids));
        if interactive {
            // Without this at *create* time the container has no stdin, and `start --attach
            // --interactive` then attaches to a pipe nothing is reading. A language server sees
            // immediate end-of-input and exits before answering `initialize`.
            args.push("--interactive".to_owned());
        }
        if let Some(seconds) = timeout_seconds {
            args.push(format!("--timeout={seconds}"));
        }
        args.extend(capsule::strings(&[
            "--env=HOME=/capsule/.executor/home",
            "--env=TMPDIR=/capsule/.executor/tmp",
            "--env=TMP=/capsule/.executor/tmp",
            "--env=TEMP=/capsule/.executor/tmp",
            "--env=XDG_CONFIG_HOME=/opt/libenr-empty-config",
            "--env=CARGO_HOME=/capsule/cargo-home",
            "--env=CARGO_TARGET_DIR=/capsule/target",
            "--env=PYTHONNOUSERSITE=1",
            "--env=UV_NO_CONFIG=1",
            "--env=UV_NO_PYTHON_DOWNLOADS=1",
            "--env=UV_CACHE_DIR=/capsule/.executor/uv",
        ]));
        args.push(format!(
            "--tmpfs=/capsule:rw,exec,nosuid,nodev,size={}m,mode=1777,notmpcopyup",
            self.limits.scratch_bytes() / (1024 * 1024)
        ));
        for (source, destination, permission) in [
            (capsule.to_owned(), "/inputs", "noexec"),
            (
                self.root.join("operations").join(format!("{name}.json")),
                "/operation.json",
                "noexec",
            ),
            (self.limits.executor()?, "/libenr-helper", "exec"),
        ] {
            if source.to_string_lossy().contains([',', '\n', '\r']) {
                return Err(io::Error::other("invalid mount path"));
            }
            args.push(format!("--mount=type=bind,src={},dst={destination},ro=true,bind-nonrecursive,bind-propagation=rprivate,nosuid,nodev,{permission}", source.display()));
        }
        args.push(format!("--name={name}"));
        args.push(image.to_owned());
        args.push("--operation=/operation.json".into());
        Ok(args)
    }

    /// A long-lived container whose stdin and stdout carry a language server's traffic.
    ///
    /// Same containment as a probe -- non-root, read-only root, no capabilities, no network,
    /// bounded CPU, memory and process count -- with two differences. Stdin is piped, because
    /// LSP is a conversation rather than a command. And there is no container wall clock: a warm
    /// session is supposed to outlive the request that started it, so its bound is the session
    /// manager's idle policy and the [`ContainerGuard`] returned here, not a timeout.
    ///
    /// The caller owns the returned guard. Dropping it removes the container, retrying until
    /// absence is confirmed, exactly as a probe's does.
    pub async fn serve(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
    ) -> io::Result<ServedSession> {
        self.clone()
            .admitted()
            .await?
            .serve_admitted(image, capsule, args)
            .await
    }

    async fn serve_admitted(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
    ) -> io::Result<ServedSession> {
        if !Self::valid_image(image) || args.is_empty() {
            return Err(io::Error::other(
                "an immutable sha256 image and concrete server command are required",
            ));
        }
        let capsule = capsule.canonicalize()?;
        if capsule.to_string_lossy().contains([':', ',', '\n']) {
            return Err(io::Error::other("capsule path contains a mount delimiter"));
        }
        let name = format!("libenr-{}", uuid::Uuid::new_v4().simple());
        let operation = self.operation(
            &name,
            image,
            &capsule,
            args,
            Mode::LanguageServer,
            Default::default(),
        )?;
        let dispatch = self.authorize_process(image, &operation, false).await?;
        let operation = dispatch.operation(operation);
        let create_args = self.create_args(&name, image, &capsule, false, None, true)?;
        let mut operation_record = OperationRecord::write(&self.root, &name, &operation)?;
        let started_at = clock::now_rfc3339();
        let ownership = self.root.join("owned").join(format!("{name}.json"));
        operation_record.ownership = Some(ownership.clone());
        std::fs::create_dir_all(self.root.join("owned"))?;
        let bytes = serde_json::to_vec(
            &serde_json::json!({"name":name,"capsule":capsule,"image":image,"created_at":started_at,"operation_id":operation.id(),"authority":dispatch.observation(),"owner":self.owner,"kind":"lsp","creation_in_progress":true,"creator_boot_id":std::fs::read_to_string("/proc/sys/kernel/random/boot_id")?.trim()}),
        )?;
        enrichment_store::atomic::write_atomic(&ownership, &bytes)?;
        std::fs::File::open(&ownership)?.sync_all()?;
        std::fs::File::open(self.root.join("owned"))?.sync_all()?;
        let mut guard =
            ContainerGuard::register(self.clone(), Arc::clone(&self.supervisor), name.clone())?;
        operation_record.registered = true;

        let mut command = self.broker();
        command.args(create_args);
        let creator = self.create_supervised(command, ownership, dispatch.clone());
        let created = tokio::time::timeout(Duration::from_secs(15), creator).await;
        match created {
            Ok(Ok(Ok(output))) if output.status.success() => {}
            Ok(Ok(Ok(output))) => {
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                guard.remove_now().await;
                return Err(io::Error::other(format!(
                    "language server container creation failed: {stderr}"
                )));
            }
            Ok(Ok(Err(e))) => {
                guard.remove_now().await;
                return Err(e);
            }
            Ok(Err(error)) => return Err(io::Error::other(error)),
            Err(_) => {
                return Err(io::Error::other(
                    "language server container creation deadline exceeded; the cleanup \
                     supervisor retains ownership until absence is confirmed",
                ));
            }
        }

        dispatch.check().await?;
        let mut child = match self
            .broker()
            .args(["start", "--attach", "--interactive", &name])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                guard.remove_now().await;
                return Err(e);
            }
        };
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("missing language server stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("missing language server stdout"))?;
        // Stderr is where both servers log. Drain it into a bounded buffer so a chatty server
        // cannot block on a full pipe, and so a startup failure has something to explain it.
        let diagnostics = Arc::new(std::sync::Mutex::new(Vec::new()));
        if let Some(err) = child.stderr.take() {
            let sink = Arc::clone(&diagnostics);
            tokio::spawn(async move {
                let mut reader = tokio::io::BufReader::new(err);
                let mut buffer = [0u8; 4096];
                loop {
                    match reader.read(&mut buffer).await {
                        Ok(0) | Err(_) => return,
                        Ok(n) => {
                            if let Ok(mut sink) = sink.lock() {
                                sink.extend_from_slice(&buffer[..n]);
                                let overflow = sink.len().saturating_sub(64 * 1024);
                                if overflow > 0 {
                                    sink.drain(..overflow);
                                }
                            }
                        }
                    }
                }
            });
        }
        Ok(ServedSession {
            operation_id: operation.id(),
            authority: dispatch.observation(),
            operation,
            runner: self.clone(),
            dispatch,
            inputs_root: capsule,
            name,
            child,
            stdin: Some(stdin),
            stdout: Some(stdout),
            diagnostics,
            guard,
            started_at,
            image_id: image.to_owned(),
        })
    }

    /// Execute an admitted producer inside its private capsule; terminate the entire container on every exit path.
    pub async fn run(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
        cancel: Arc<AtomicBool>,
    ) -> io::Result<ProcessObservation> {
        self.clone()
            .admitted()
            .await?
            .execute(image, capsule, args, cancel, false, Default::default())
            .await
    }

    /// Network access belongs only to validated dependency acquisition, never target execution.
    pub(crate) async fn acquire(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
        cancel: Arc<AtomicBool>,
        outputs: std::collections::BTreeMap<String, OutputKind>,
    ) -> io::Result<ProcessObservation> {
        self.clone()
            .admitted()
            .await?
            .execute(image, capsule, args, cancel, true, outputs)
            .await
    }

    /// Preparation only: return explicitly selected outputs after validated handoff and cleanup.
    pub async fn prepare_outputs(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
        cancel: Arc<AtomicBool>,
        outputs: std::collections::BTreeMap<String, OutputKind>,
    ) -> io::Result<ProcessObservation> {
        self.clone()
            .admitted()
            .await?
            .execute(image, capsule, args, cancel, false, outputs)
            .await
    }

    async fn execute(
        &self,
        image: &str,
        capsule: &Path,
        args: &[String],
        cancel: Arc<AtomicBool>,
        acquisition: bool,
        outputs: std::collections::BTreeMap<String, OutputKind>,
    ) -> io::Result<ProcessObservation> {
        if !Self::valid_image(image) || args.is_empty() {
            return Err(io::Error::other(
                "an immutable sha256 image and concrete producer command are required",
            ));
        }
        let capsule = capsule.canonicalize()?;
        if capsule.to_string_lossy().contains([':', ',', '\n']) {
            return Err(io::Error::other("capsule path contains a mount delimiter"));
        }
        let name = format!("libenr-{}", uuid::Uuid::new_v4().simple());
        let deadline = self.limits.deadline_seconds.clamp(1, 600);
        let operation = self.operation(&name, image, &capsule, args, Mode::Command, outputs)?;
        let dispatch = self
            .authorize_process(image, &operation, acquisition)
            .await?;
        let operation = dispatch.operation(operation);
        let reservation = budget::Reservation::acquire(
            &self.cache,
            if operation.outputs.is_empty() {
                0
            } else {
                operation.data_bytes
            },
            self.limits.capsule_budget_mib.saturating_mul(1024 * 1024),
        )?;
        let until = tokio::time::Instant::now() + Duration::from_secs(deadline);
        let mut cmd = self.broker();
        cmd.args(self.create_args(
            &name,
            image,
            &capsule,
            acquisition,
            Some(deadline + 2),
            false,
        )?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
        let mut operation_record = OperationRecord::write(&self.root, &name, &operation)?;
        let started_at = clock::now_rfc3339();
        let ownership = self.root.join("owned").join(format!("{name}.json"));
        operation_record.ownership = Some(ownership.clone());
        std::fs::create_dir_all(self.root.join("owned"))?;
        let bytes = serde_json::to_vec(
            &serde_json::json!({"name":name,"capsule":capsule,"image":image,"created_at":started_at,"operation_id":operation.id(),"authority":dispatch.observation(),"owner":self.owner,"creation_in_progress":true,"creator_boot_id":std::fs::read_to_string("/proc/sys/kernel/random/boot_id")?.trim()}),
        )?;
        enrichment_store::atomic::write_atomic(&ownership, &bytes)?;
        std::fs::File::open(&ownership)?.sync_all()?;
        std::fs::File::open(self.root.join("owned"))?.sync_all()?;
        // Creation never starts the image. Finish creation before observing cancellation, so a
        // successful removal cannot race with a launcher that has not created its container yet.
        // Ownership is registered before the container exists, so a dropped future, a panic or
        // a failed removal all leave someone holding the obligation. `kill_on_drop` reaps the
        // Podman client; only removal reaps the container.
        let mut guard =
            ContainerGuard::register(self.clone(), Arc::clone(&self.supervisor), name.clone())?;
        operation_record.registered = true;
        let creator = self.create_supervised(cmd, ownership, dispatch.clone());
        let created = tokio::time::timeout_at(
            until.min(tokio::time::Instant::now() + Duration::from_secs(15)),
            creator,
        )
        .await;
        match created {
            Ok(Ok(Ok(output))) if output.status.success() => {}
            Ok(Ok(Ok(output))) => {
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                guard.remove_now().await;
                return Err(io::Error::other(format!(
                    "container creation failed: {stderr}"
                )));
            }
            Ok(Ok(Err(e))) => {
                guard.remove_now().await;
                return Err(e);
            }
            Ok(Err(error)) => return Err(io::Error::other(error)),
            Err(_) => {
                // A killed creation broker is not proof that all its helper processes have
                // stopped. Keep the durable record and let the supervisor retry removal; it
                // holds a worker permit until absence is confirmed.
                return Err(io::Error::other(
                    "container creation deadline exceeded; the cleanup supervisor retains \
                     ownership until absence is confirmed",
                ));
            }
        }
        if cancel.load(Ordering::Acquire) || tokio::time::Instant::now() >= until {
            let end = if cancel.load(Ordering::Acquire) {
                ProcessEnd::Cancelled
            } else {
                ProcessEnd::Deadline
            };
            let cleanup_confirmed = guard.remove_now().await;
            return Ok(ProcessObservation {
                operation_id: operation.id(),
                authority: dispatch.observation(),
                image_id: image.into(),
                command: args.to_vec(),
                started_at,
                finished_at: clock::now_rfc3339(),
                exit_code: None,
                end,
                stdout: String::new(),
                stderr: String::new(),
                cleanup_confirmed,
            });
        }
        let mut start_command = self.broker();
        start_command
            .args(["start", "--attach", &name])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        dispatch.check().await?;
        let mut child = match start_command.spawn() {
            Ok(child) => child,
            Err(e) => {
                guard.remove_now().await;
                return Err(e);
            }
        };
        let overflow = Arc::new(AtomicBool::new(false));
        let limit = self.limits.output_bytes.clamp(1024, 1048576);
        let mut source = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("missing executor stream"))?;
        let err = tokio::spawn(read_bounded(
            child
                .stderr
                .take()
                .ok_or_else(|| io::Error::other("missing broker stderr"))?,
            limit,
            overflow.clone(),
        ));
        let mut interrupted = None;
        let mut captured = {
            let receiver = handoff::receive(&mut source, &operation, &reservation.root);
            tokio::pin!(receiver);
            loop {
                tokio::select! {
                    result = &mut receiver => break Some(result),
                    () = tokio::time::sleep_until(until) => { interrupted = Some(ProcessEnd::Deadline); break None; }
                    () = tokio::time::sleep(Duration::from_millis(10)) => {
                        if cancel.load(Ordering::Acquire) { interrupted = Some(ProcessEnd::Cancelled); break None; }
                        if overflow.load(Ordering::Acquire) { interrupted = Some(ProcessEnd::OutputLimit); break None; }
                    }
                }
            }
        };
        if tokio::time::Instant::now() >= until {
            captured = None;
            interrupted = Some(ProcessEnd::Deadline);
        }
        // Completion is a protocol fact. Removal is an independent whole-container fact.
        let cleanup_confirmed = guard.remove_now().await;
        let _ = child.kill().await;
        let _ = child.wait().await;
        // Any bytes after the terminal frame are rejected. EOF here is teardown only.
        let mut trailing = [0u8; 1];
        let extra = tokio::time::timeout(Duration::from_secs(2), source.read(&mut trailing)).await;
        let broker_stderr = tokio::time::timeout(Duration::from_secs(2), err)
            .await
            .map_err(|_| io::Error::other("broker stderr did not close"))?
            .map_err(io::Error::other)??;
        let (end, exit_code, stdout, mut stderr) = match captured {
            Some(Ok(captured)) => {
                if !matches!(extra, Ok(Ok(0))) {
                    return Err(io::Error::other("trailing or unclosed executor stream"));
                }
                if cleanup_confirmed
                    && captured.end == ProcessEnd::Exited
                    && captured.exit_code == Some(0)
                {
                    let current = inventory::capture(&capsule, operation.data_bytes)?;
                    if current != operation.inputs {
                        return Err(io::Error::other("admitted inputs changed during execution"));
                    }
                    let retained_bytes = current
                        .iter()
                        .filter(|(path, _)| {
                            !operation
                                .outputs
                                .keys()
                                .any(|root| *path == root || path.starts_with(&format!("{root}/")))
                        })
                        .try_fold(0u64, |sum, (_, entry)| {
                            sum.checked_add(entry.bytes())
                                .ok_or_else(|| io::Error::other("preparation size overflow"))
                        })?;
                    let new_bytes =
                        captured
                            .entries
                            .values()
                            .try_fold(retained_bytes, |sum, entry| {
                                sum.checked_add(entry.bytes())
                                    .ok_or_else(|| io::Error::other("preparation size overflow"))
                            })?;
                    if new_bytes > operation.data_bytes {
                        return Err(io::Error::other(
                            "promoted outputs exceed preparation byte reservation",
                        ));
                    }
                    // These are preparation staging roots, never a published retained generation.
                    // The caller publishes its complete manifest only after every stage succeeds.
                    for output in operation.outputs.keys() {
                        let target = capsule.join(output);
                        if target.try_exists()? {
                            if target.is_dir() {
                                std::fs::remove_dir_all(&target)?;
                            } else {
                                std::fs::remove_file(&target)?;
                            }
                        }
                        std::fs::create_dir_all(
                            target
                                .parent()
                                .ok_or_else(|| io::Error::other("output target has no parent"))?,
                        )?;
                        std::fs::rename(reservation.root.join(output), target)?;
                    }
                }
                let _validated_inventory = captured.entries;
                (
                    captured.end,
                    captured.exit_code,
                    captured.stdout,
                    captured.stderr,
                )
            }
            Some(Err(error)) => {
                return Err(io::Error::other(format!(
                    "executor handoff failed: {error}; broker: {}",
                    String::from_utf8_lossy(&broker_stderr)
                )));
            }
            None => (
                interrupted
                    .ok_or_else(|| io::Error::other("execution ended without observation"))?,
                None,
                Vec::new(),
                Vec::new(),
            ),
        };
        stderr.extend_from_slice(
            &broker_stderr[..broker_stderr.len().min(limit.saturating_sub(stderr.len()))],
        );
        Ok(ProcessObservation {
            operation_id: operation.id(),
            authority: dispatch.observation(),
            image_id: image.into(),
            command: args.to_vec(),
            started_at,
            finished_at: clock::now_rfc3339(),
            exit_code,
            end,
            stdout: String::from_utf8_lossy(&stdout).into_owned(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
            cleanup_confirmed,
        })
    }

    async fn remove(&self, name: &str) -> io::Result<()> {
        let path = self.root.join("owned").join(format!("{name}.json"));
        if path.exists() {
            let record: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
            if record["creation_in_progress"] == true {
                return Err(io::Error::other(
                    "container creation is still in progress; absence cannot retire ownership yet",
                ));
            }
        }
        let output = tokio::time::timeout(
            Duration::from_secs(15),
            self.broker()
                .args(["rm", "--force", "--ignore", "--time=0", name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status(),
        )
        .await
        .map_err(|_| io::Error::other("container cleanup deadline exceeded"))??;
        if !output.success() {
            return Err(io::Error::other(
                "container cleanup failed; inspect owned libenr containers before retrying",
            ));
        }
        let status = tokio::time::timeout(
            Duration::from_secs(5),
            self.broker()
                .args(["container", "exists", name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status(),
        )
        .await
        .map_err(|_| io::Error::other("container absence check timed out"))??;
        if status.code() != Some(1) {
            return Err(io::Error::other("container absence was not confirmed"));
        }
        if path.exists() {
            std::fs::remove_file(path)?;
            std::fs::File::open(self.root.join("owned"))?.sync_all()?;
        }
        let operation = self.root.join("operations").join(format!("{name}.json"));
        if operation.try_exists()? {
            std::fs::remove_file(operation)?;
        }
        Ok(())
    }

    /// Reconcile durable ownership before interrupted job journals become terminal after restart.
    pub fn recover_owned(&self) -> io::Result<()> {
        for root in ownership::read(&self.cache)? {
            let prior = Self {
                root: root.path,
                broker: root.broker,
                ..self.clone()
            };
            prior.recover_root()?;
        }
        Ok(())
    }

    fn recover_root(&self) -> io::Result<()> {
        let owned = self.root.join("owned");
        if !owned.exists() {
            return Ok(());
        }
        for entry in std::fs::read_dir(&owned)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                return Err(io::Error::other(
                    "unknown container ownership child; inspect before recovery",
                ));
            }
            let meta = std::fs::symlink_metadata(&path)?;
            if !meta.is_file() || meta.len() > 1_048_576 {
                return Err(io::Error::other("invalid container ownership record"));
            }
            let record: serde_json::Value = serde_json::from_slice(&std::fs::read(&path)?)?;
            if !record["owner"].as_str().is_some_and(|owner| {
                owner.len() == 64 && owner.bytes().all(|b| b.is_ascii_hexdigit())
            }) {
                return Err(io::Error::other("missing or invalid container owner"));
            }
            if record["owner"].as_str() != Some(self.owner.as_str()) {
                continue;
            }
            if record["creation_in_progress"] == true
                && record["creator_boot_id"].as_str().is_none_or(|previous| {
                    std::fs::read_to_string("/proc/sys/kernel/random/boot_id")
                        .map(|current| current.trim() == previous)
                        .unwrap_or(true)
                })
            {
                return Err(io::Error::other(
                    "an interrupted container creator has not confirmed completion; inspect its owned record and creator PID. Automatic reconciliation requires a different boot, which proves the old creator and helpers cannot remain alive",
                ));
            }
            let name = record["name"]
                .as_str()
                .ok_or_else(|| io::Error::other("malformed container ownership"))?;
            if !name
                .strip_prefix("libenr-")
                .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()))
                || path.file_stem().and_then(|s| s.to_str()) != Some(name)
            {
                return Err(io::Error::other("invalid container ownership identity"));
            }
            let status = bounded_status(
                self.broker()
                    .args(["rm", "--force", "--ignore", "--time=0", name]),
            )?;
            if !status.success() {
                return Err(io::Error::other(
                    "cannot reconcile owned container; daemon refuses to start",
                ));
            }
            let absent = bounded_status(self.broker().args(["container", "exists", name]))?;
            if absent.code() != Some(1) {
                return Err(io::Error::other(
                    "owned container absence unconfirmed; daemon refuses to start",
                ));
            }
            let operation = self.root.join("operations").join(format!("{name}.json"));
            if operation.try_exists()? {
                std::fs::remove_file(operation)?;
            }
            std::fs::remove_file(path)?;
        }
        std::fs::File::open(owned)?.sync_all()
    }
}
fn bounded_status(command: &mut Command) -> io::Result<std::process::ExitStatus> {
    command.stdout(Stdio::null()).stderr(Stdio::null());
    let mut child = command.as_std_mut().spawn()?;
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::other("container recovery deadline exceeded"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

async fn read_bounded(
    mut input: impl AsyncRead + Unpin,
    limit: usize,
    overflow: Arc<AtomicBool>,
) -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        let n = input.read(&mut chunk).await?;
        if n == 0 {
            return Ok(result);
        }
        let keep = n.min(limit.saturating_sub(result.len()));
        result.extend_from_slice(&chunk[..keep]);
        if keep < n {
            overflow.store(true, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_execution_ownership_children_prevent_recovery() {
        let dir = tempfile::tempdir().unwrap();
        let supervisor = Supervisor::new(Arc::new(tokio::sync::Semaphore::new(1)), 5, 1);
        let runner = Runner::new(&Execution::default(), dir.path(), supervisor).unwrap();
        let unknown = runner.root.join("owned/unknown");
        std::fs::write(&unknown, b"preserve").unwrap();
        assert!(
            runner
                .recover_owned()
                .unwrap_err()
                .to_string()
                .contains("unknown")
        );
        assert_eq!(std::fs::read(unknown).unwrap(), b"preserve");
    }

    #[tokio::test]
    async fn unclaimed_process_and_unstarted_registration_leave_no_operation_files() {
        let cache = tempfile::tempdir().unwrap();
        let input = tempfile::tempdir().unwrap();
        let config = Execution {
            capsule_budget_mib: 0,
            ..Execution::default()
        };
        let supervisor = Supervisor::new(Arc::new(tokio::sync::Semaphore::new(1)), 5, 1);
        let runner = Runner::new(&config, cache.path(), supervisor).unwrap();
        for _ in 0..3 {
            let result = runner
                .prepare_outputs(
                    &format!("sha256:{}", "a".repeat(64)),
                    input.path(),
                    &["/bin/true".into()],
                    Arc::new(AtomicBool::new(false)),
                    [("out".into(), OutputKind::File)].into(),
                )
                .await;
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("claimed native command")
            );
        }
        assert!(!runner.root.join("operations").exists());
        assert_eq!(
            std::fs::read_dir(runner.root.join("owned"))
                .unwrap()
                .count(),
            0
        );
        let operation = runner
            .operation(
                "unused",
                "image",
                input.path(),
                &["/bin/true".into()],
                Mode::Command,
                Default::default(),
            )
            .unwrap();
        let mut record = OperationRecord::write(&runner.root, "unstarted", &operation).unwrap();
        let owned = runner.root.join("owned/unstarted.json");
        std::fs::write(&owned, b"unstarted registration").unwrap();
        record.ownership = Some(owned.clone());
        drop(record);
        assert!(!owned.exists());
        assert_eq!(
            std::fs::read_dir(runner.root.join("operations"))
                .unwrap()
                .count(),
            0
        );
    }

    #[tokio::test]
    async fn a_failed_spawn_records_that_no_creator_is_in_progress() {
        let dir = tempfile::tempdir().expect("state");
        let supervisor = Supervisor::new(Arc::new(tokio::sync::Semaphore::new(1)), 5, 1);
        let runner = Runner::new(&Execution::default(), dir.path(), supervisor).expect("runner");
        let ownership = runner.root.join("owned/libenr-test.json");
        std::fs::create_dir_all(ownership.parent().expect("parent")).expect("ownership directory");
        std::fs::write(&ownership, br#"{"creation_in_progress":true}"#).expect("record");
        let absent = dir.path().join("no-such-executable");
        let error = runner
            .create_supervised(
                Command::new(absent),
                ownership.clone(),
                Dispatch::Qualification("unspawned-fixture".into()),
            )
            .await
            .expect("task")
            .expect_err("spawn fails");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        let record: serde_json::Value =
            serde_json::from_slice(&std::fs::read(ownership).expect("record")).expect("JSON");
        assert_eq!(record["creation_in_progress"], false);
    }
    #[test]
    fn execution_boundary_requires_immutable_images() {
        assert!(!Runner::valid_image("python:latest"));
        assert!(!Runner::valid_image("--privileged"));
        assert!(Runner::valid_image(&format!("sha256:{}", "a".repeat(64))));
    }
}
