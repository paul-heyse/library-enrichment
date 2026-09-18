//! Native authorization of the service's one static Python extractor. A retained launch
//! describes an effect; only its private grant and a fresh live-command check permit it.
use crate::{
    control_jobs::Grant, immutable_definitions::Definitions, native_catalog, retention::Dependency,
    runtime::QueryRuntime,
};
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    dataframe::DataFrame,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::arrow_model::expressions::{literal, record},
    execution::static_worker::{Effect, Launch, MEMORY_BYTES, STDERR_BYTES},
    native_union::{Cell, NativeStruct, Rule},
    producer::python::{WorkerFile, WorkerRequest, worker},
};

enrichment_core::native_struct! { pub struct Source {
    root: String => Rule::NonEmpty,
    files: Vec<WorkerFile> => Rule::SequenceBounds { min: 0, max: worker::MAX_FILES as u64 },
    package: String => Rule::NonEmpty,
    artifact_id: String => Rule::ArtifactIdentity { digest: "artifact_sha256".into() },
    artifact_sha256: String => Rule::Sha256,
    output_root: String => Rule::NonEmpty,
} }
enrichment_core::native_struct! { struct Inventory {
    inputs: enrichment_core::capsule_protocol::inventory::Inventory => Rule::Map,
} }

pub struct Prepared {
    parent: Grant,
    value: Effect,
    _binding: enrichment_core::delta_reference::DeltaVersionRef,
    source: std::sync::Arc<crate::PrivateDirectory>,
    output: std::sync::Arc<crate::PrivateDirectory>,
}
impl Prepared {
    pub fn launch(&self) -> &Launch {
        &self.value.launch
    }
    pub async fn check(&self) -> Result<()> {
        self.parent.check().await
    }
    async fn check_inputs(&self, runtime: &QueryRuntime) -> Result<()> {
        let input = self.source.clone();
        let root = self.value.launch.request.root.clone();
        let captured = runtime
            .blocking(move || {
                input.directory(
                    crate::private_directory::Kind::Source,
                    std::path::Path::new(&root),
                )?;
                capture(&root)
            })
            .await??;
        let session = runtime.session();
        for (name, inputs) in [
            ("worker_expected", self.value.inputs.clone()),
            ("worker_actual", captured),
        ] {
            native_catalog::input(&session, name, Inventory::batch(&[Inventory { inputs }])?)?;
        }
        runtime.require_empty(session.sql(r#"
            SELECT 'static_worker_inputs_changed' AS witness FROM worker_actual a CROSS JOIN worker_expected e
              WHERE cardinality(array_except(native_map_entries(a.inputs),native_map_entries(e.inputs)))>0
                 OR cardinality(array_except(native_map_entries(e.inputs),native_map_entries(a.inputs)))>0
        "#).await?,"static_worker_exact_inputs","worker_dispatch").await
    }
    /// A running parser has no reusable grant. Cancellation, expiry or loss of the
    /// durable claim interrupts its owner even while stdout is still making progress.
    pub async fn revoked(&self) -> DataFusionError {
        self.parent.revoked().await
    }
}

/// The native producer contract supplies request limits, protocol, argv and the complete
/// environment. No target PATH, module path, observation bound or program is caller chosen.
pub async fn lower(
    runtime: &QueryRuntime,
    source: &Source,
    config: &enrichment_core::config::PythonProducers,
) -> Result<Launch> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "static_source",
        Source::batch(std::slice::from_ref(source))?,
    )?;
    native_catalog::input(&session, "static_files", WorkerFile::batch(&source.files)?)?;
    let executable = config
        .worker_python
        .as_ref()
        .ok_or_else(|| invalid("static worker interpreter is not configured; generate an installed launch configuration"))?
        .to_str()
        .ok_or_else(|| invalid("worker executable is not UTF-8"))?;
    let frame = session.table("static_source").await?;
    if let Some(violations) = enrichment_core::native_schema::intrinsic_violations(
        frame.clone(),
        &arrow::datatypes::Schema::new(Source::fields()),
    )? {
        runtime
            .require_empty(
                violations.select(vec![lit("static_source").alias("witness")])?,
                "static_source_contract",
                "worker_admission",
            )
            .await?;
    }
    runtime.require_empty(session.sql(r#"
        SELECT 'static_worker_paths_or_deadline' AS witness FROM static_source
          WHERE NOT starts_with(root,'/') OR NOT starts_with(output_root,'/')
            OR root=output_root OR NOT starts_with($1,'/') OR contains($1,chr(0)) OR $2<1 OR $2>600
        UNION ALL SELECT 'duplicate_worker_file' AS witness FROM static_files GROUP BY file HAVING count(*)<>1
        UNION ALL SELECT 'static_worker_inventory_bytes' AS witness FROM static_files
          HAVING sum(octet_length(file)+octet_length(module))>2097152
        UNION ALL SELECT 'static_worker_source_kind' AS witness FROM static_files
          WHERE NOT ((origin='source' AND ends_with(file,'.py')) OR (origin='stub' AND ends_with(file,'.pyi')))
    "#).await?.with_param_values(vec![ScalarValue::from(executable), config.worker_timeout_seconds.into()])?,
        "static_worker_parameters", "worker_admission").await?;
    let request = record(
        &WorkerRequest::data_type(),
        &[
            ("schema_version", lit(worker::PROTOCOL)),
            ("root", col("root")),
            ("files", col("files")),
            ("max_observations", lit(worker::MAX_OBSERVATIONS as u64)),
            ("max_memory_bytes", lit(MEMORY_BYTES)),
            ("max_cpu_seconds", lit(config.worker_timeout_seconds)),
        ],
    )?;
    let frame = frame.select(vec![
        lit(executable).alias("executable"),
        literal(&vec![
            "-I".to_owned(),
            "-B".into(),
            "-m".into(),
            "enrichment_worker".into(),
        ])?
        .alias("argv"),
        datafusion::functions_nested::map::map(
            vec![lit("HOME"), lit("TMPDIR"), lit("LANG")],
            vec![col("output_root"), col("output_root"), lit("C.UTF-8")],
        )
        .alias("environment"),
        col("output_root").alias("cwd"),
        lit("worker.arrow").alias("output"),
        lit(worker::MAX_BYTES).alias("output_bytes"),
        lit(STDERR_BYTES).alias("stderr_bytes"),
        lit(config.worker_timeout_seconds).alias("deadline_seconds"),
        request.alias("request"),
    ])?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("native worker launch absent"))
}

pub async fn prepare(
    parent: Grant,
    source: &Source,
    input: std::sync::Arc<crate::PrivateDirectory>,
    output: std::sync::Arc<crate::PrivateDirectory>,
) -> Result<Prepared> {
    parent.check().await?;
    let jobs = parent.store();
    let runtime = jobs.runtime();
    let (input_owner, output_owner, root, output_root) = (
        input.clone(),
        output.clone(),
        source.root.clone(),
        source.output_root.clone(),
    );
    let inputs = runtime
        .blocking(move || {
            input_owner.directory(
                crate::private_directory::Kind::Source,
                std::path::Path::new(&root),
            )?;
            output_owner.directory(
                crate::private_directory::Kind::Worker,
                std::path::Path::new(&output_root),
            )?;
            if output_owner.path() != std::path::Path::new(&output_root) {
                return Err(std::io::Error::other(
                    "worker output requires the exact private root",
                ));
            }
            capture(&root)
        })
        .await??;
    let launch = lower(runtime, source, &jobs.config().producers.python).await?;
    input_scope(runtime, &source.files, &inputs).await?;
    let pin = jobs.pin().await?;
    let session = pin.session(runtime).await?;
    for (name, table) in [
        ("static_claims", "state.records.claims"),
        ("static_commands", "state.records.commands"),
    ] {
        native_catalog::work(&session, name, session.table(table).await?.into_view())?;
    }
    let selected = scope(&session, parent.id(), &source.package).await?;
    let effect = selected.select(vec![
        col("grant_id"),
        col("job_id"),
        col("policy_id"),
        lit(&source.package).alias("package"),
        lit(&source.artifact_id).alias("artifact_id"),
        lit(&source.artifact_sha256).alias("artifact_sha256"),
        lit(concat!(
            "static-worker/1/",
            env!("ENR_NATIVE_SOURCE_DIGEST")
        ))
        .alias("implementation"),
        literal(&inputs)?.alias("inputs"),
        literal(&launch)?.alias("launch"),
    ])?;
    let definitions = Definitions::<enrichment_core::identity::StaticWorkerEffectId>::new(
        parent.control(),
        runtime.clone(),
    );
    let (id, binding) = definitions
        .retain_plan(
            effect,
            vec![Dependency::Artifact {
                artifact_id: source.artifact_id.clone(),
            }],
        )
        .await?;
    let retained = definitions
        .read(&id, &binding)
        .await?
        .drop_columns(&["effect_id"])?;
    let value: Effect = runtime
        .records(retained, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("retained worker effect absent"))?;
    parent.check().await?;
    Ok(Prepared {
        parent,
        value,
        _binding: binding,
        source: input,
        output,
    })
}

async fn scope(
    session: &datafusion::prelude::SessionContext,
    grant: &enrichment_core::identity::GrantId,
    package: &str,
) -> Result<DataFrame> {
    session
        .sql(
            r#"
        SELECT c.grant_id,c.job_id,c.policy_id FROM static_claims c
        JOIN static_commands d ON c.job_id=d.job_id
        WHERE c.grant_id=$1 AND c.ecosystem='python' AND d.arguments.kind='resolve'
          AND d.arguments.resolve.request.name=$2
    "#,
        )
        .await?
        .with_param_values(datafusion::common::ParamValues::List(vec![
            grant.parameter(),
            ScalarValue::from(package).into(),
        ]))
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

fn capture(root: &str) -> std::io::Result<enrichment_core::capsule_protocol::inventory::Inventory> {
    let policy = enrichment_core::policy::ArchivePolicy::default();
    enrichment_core::capsule_protocol::inventory::capture_bounded(
        std::path::Path::new(root),
        policy.max_total_bytes,
        policy.max_entries,
    )
}

async fn input_scope(
    runtime: &QueryRuntime,
    files: &[WorkerFile],
    inputs: &enrichment_core::capsule_protocol::inventory::Inventory,
) -> Result<()> {
    let session = runtime.session();
    native_catalog::input(&session, "worker_selected", WorkerFile::batch(files)?)?;
    native_catalog::input(
        &session,
        "worker_inventory",
        Inventory::batch(&[Inventory {
            inputs: inputs.clone(),
        }])?,
    )?;
    runtime
        .require_empty(
            session
                .sql(
                    r#"
        WITH actual AS (SELECT unnest(native_map_entries(inputs)) AS e FROM worker_inventory)
        SELECT f.file AS witness FROM worker_selected f LEFT JOIN actual a ON f.file=a.e.key
          WHERE a.e.value.kind IS DISTINCT FROM 'file'
    "#,
                )
                .await?,
            "static_worker_input_closure",
            "worker_admission",
        )
        .await
}

/// Own the actual OS child and its paid resources. A dropped waiter requests termination;
/// the pre-admitted release task retains all inputs and the permit through physical wait.
pub struct OwnedChild {
    child: Option<tokio::process::Child>,
    runtime: QueryRuntime,
    slot: Option<crate::retention_tasks::ReleaseSlot>,
    owners: Option<Box<dyn Send + Sync>>,
}

/// Admission reserves the declared process address-space ceiling before spawn. This is
/// an external-process envelope, not measured RSS. It stays with the physical child/reaper.
pub(crate) fn parser_memory(
    runtime: &QueryRuntime,
    bytes: u64,
) -> Result<datafusion::execution::memory_pool::MemoryReservation> {
    let bytes = usize::try_from(bytes).map_err(|_| invalid("parser address bound overflow"))?;
    if bytes == 0 {
        return Err(invalid("parser address bound must be positive"));
    }
    let memory =
        datafusion::execution::memory_pool::MemoryConsumer::new("external-parser-address-envelope")
            .register(&runtime.session().runtime_env().memory_pool);
    memory.try_grow(bytes)?;
    Ok(memory)
}

impl OwnedChild {
    pub async fn spawn(
        runtime: &QueryRuntime,
        prepared: &Prepared,
        owners: impl Send + Sync + 'static,
    ) -> Result<Self> {
        prepared.check_inputs(runtime).await?;
        let permit = runtime.parser_permit().await?;
        prepared.check().await?;
        let launch = prepared.launch();
        let memory = parser_memory(runtime, launch.request.max_memory_bytes)?;
        let mut command = tokio::process::Command::new(&launch.executable);
        command
            .args(&launch.argv)
            .current_dir(&launch.cwd)
            .env_clear()
            .envs(&launch.environment)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);
        let owned = Self::start(
            runtime,
            &mut command,
            (
                prepared.source.clone(),
                prepared.output.clone(),
                permit,
                memory,
                owners,
            ),
        )?;
        let pid = owned
            .child
            .as_ref()
            .and_then(tokio::process::Child::id)
            .ok_or_else(|| invalid("static worker PID unavailable"))?;
        prepared.source.protect_child(pid).await?;
        prepared.output.protect_child(pid).await?;
        prepared.check().await?;
        Ok(owned)
    }
    /// Crate-private mechanical launch shared with the fixed native rustdoc executable.
    /// Callers establish their finite producer contract before reaching this boundary.
    pub(crate) fn start(
        runtime: &QueryRuntime,
        command: &mut tokio::process::Command,
        owners: impl Send + Sync + 'static,
    ) -> Result<Self> {
        let slot = runtime.reserve_release()?;
        let child = command.kill_on_drop(true).spawn()?;
        Ok(Self {
            child: Some(child),
            runtime: runtime.clone(),
            slot: Some(slot),
            owners: Some(Box::new(owners)),
        })
    }
    pub fn child(&mut self) -> &mut tokio::process::Child {
        self.child.as_mut().expect("owned worker child")
    }
}
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let (Some(mut child), Some(slot), Some(owners)) =
            (self.child.take(), self.slot.take(), self.owners.take())
        else {
            return;
        };
        if matches!(child.try_wait(), Ok(Some(_))) {
            return;
        }
        let _ = child.start_kill();
        if let Err(error) = self.runtime.release_retention(slot, async move {
            let _owners = owners;
            loop {
                match child.wait().await {
                    Ok(_) => return Ok(()),
                    Err(error) => {
                        // Unknown exit never releases a paid permit or source. Shutdown must
                        // observe this unresolved task instead of reporting false quiescence.
                        eprintln!("static worker reap unresolved: {error}");
                        let _ = child.start_kill();
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        }) && let Some(diagnostic) = crate::query_failure::diagnostic_from_error(&error)
        {
            self.runtime.record_failure(diagnostic);
        }
    }
}

/// Tokio file I/O can outlive the future awaiting it. Keep the output owner until flushing
/// that file has joined its pending operation, including on cancellation of the driver.
pub struct Output {
    file: Option<tokio::fs::File>,
    owner: std::sync::Arc<crate::PrivateDirectory>,
    runtime: QueryRuntime,
    slot: Option<crate::retention_tasks::ReleaseSlot>,
}
impl Output {
    pub async fn create(runtime: &QueryRuntime, prepared: &Prepared) -> Result<Self> {
        let slot = runtime.reserve_release()?;
        let owner = prepared.output.clone();
        let held = owner.clone();
        let name = prepared.launch().output.clone();
        let file = runtime
            .blocking(move || {
                std::fs::File::options()
                    .create_new(true)
                    .write(true)
                    .open(held.path().join(name))
            })
            .await??;
        Ok(Self {
            file: Some(tokio::fs::File::from_std(file)),
            owner,
            runtime: runtime.clone(),
            slot: Some(slot),
        })
    }
    pub fn file(&mut self) -> &mut tokio::fs::File {
        self.file.as_mut().expect("owned worker output")
    }
}
impl Drop for Output {
    fn drop(&mut self) {
        let (Some(mut file), Some(slot)) = (self.file.take(), self.slot.take()) else {
            return;
        };
        let owner = self.owner.clone();
        if let Err(error) = self.runtime.release_retention(slot, async move {
            use tokio::io::AsyncWriteExt;
            let result = file.flush().await;
            drop(file);
            drop(owner);
            result.map_err(DataFusionError::from)
        }) && let Some(diagnostic) = crate::query_failure::diagnostic_from_error(&error)
        {
            self.runtime.record_failure(diagnostic);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{identity::Ecosystem, producer::python::ObservationOrigin};

    #[tokio::test]
    async fn plan19_static_worker_policy_is_native_and_bounded() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let source = Source {
            root: "/owned/source/content".into(),
            output_root: "/owned/worker/output".into(),
            files: vec![WorkerFile {
                file: "pkg/api.py".into(),
                module: "pkg.api".into(),
                origin: ObservationOrigin::Source,
            }],
            package: "fixture".into(),
            artifact_id: format!("art_{}", "a".repeat(64)),
            artifact_sha256: "a".repeat(64),
        };
        let config = enrichment_core::config::PythonProducers {
            worker_python: Some("/installed/bin/python".into()),
            ..Default::default()
        };
        let launch = lower(&runtime, &source, &config).await?;
        assert_eq!(launch.executable, "/installed/bin/python");
        assert_eq!(launch.argv, ["-I", "-B", "-m", "enrichment_worker"]);
        assert_eq!(
            launch.environment,
            std::collections::BTreeMap::from([
                ("HOME".into(), source.output_root.clone()),
                ("TMPDIR".into(), source.output_root.clone()),
                ("LANG".into(), "C.UTF-8".into()),
            ])
        );
        assert_eq!(launch.request.files[0].module, "pkg.api");
        assert_eq!(launch.request.schema_version, worker::PROTOCOL);
        assert_eq!(launch.request.max_observations, worker::MAX_OBSERVATIONS);
        assert_eq!(launch.request.max_memory_bytes, MEMORY_BYTES);
        assert_eq!(
            launch.request.max_cpu_seconds,
            config.worker_timeout_seconds
        );
        assert_eq!(launch.output_bytes, worker::MAX_BYTES);
        assert_eq!(launch.output, "worker.arrow");
        let mut inputs = enrichment_core::capsule_protocol::inventory::Inventory::from([
            (
                "pkg".into(),
                enrichment_core::capsule_protocol::inventory::Entry::Directory { mode: 0o755 },
            ),
            (
                "pkg/api.py".into(),
                enrichment_core::capsule_protocol::inventory::Entry::File {
                    mode: 0o644,
                    bytes: 0,
                    sha256: "b".repeat(64),
                },
            ),
        ]);
        input_scope(&runtime, &source.files, &inputs).await?;
        inputs.remove("pkg/api.py");
        assert!(input_scope(&runtime, &source.files, &inputs).await.is_err());
        inputs.insert(
            "pkg/api.py".into(),
            enrichment_core::capsule_protocol::inventory::Entry::Directory { mode: 0o755 },
        );
        assert!(input_scope(&runtime, &source.files, &inputs).await.is_err());
        for change in 0..10 {
            let mut bad_source = source.clone();
            let mut bad_config = config.clone();
            match change {
                0 => bad_source.files.push(bad_source.files[0].clone()),
                1 => bad_source.files[0].file = "../escape.py".into(),
                2 => bad_source.files[0].origin = ObservationOrigin::Stub,
                3 => bad_source.root = "relative".into(),
                4 => bad_source.output_root = bad_source.root.clone(),
                5 => bad_config.worker_timeout_seconds = 0,
                6 => bad_config.worker_timeout_seconds = 601,
                7 => bad_config.worker_python = Some("python3".into()),
                8 => bad_config.worker_python = None,
                _ => bad_source.artifact_sha256 = "invalid".into(),
            }
            assert!(
                lower(&runtime, &bad_source, &bad_config).await.is_err(),
                "change {change}"
            );
        }
        let mut empty = source;
        empty.files.clear();
        assert!(
            lower(&runtime, &empty, &config)
                .await?
                .request
                .files
                .is_empty()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    enrichment_core::native_struct! { struct Claim {
        grant_id:enrichment_core::identity::GrantId=>Rule::Text,job_id: enrichment_core::identity::JobId=>Rule::Text,policy_id:enrichment_core::identity::OperationPolicyId=>Rule::Text,ecosystem:Ecosystem=>Rule::Text,
    } }
    enrichment_core::native_struct! { struct Command {job_id: enrichment_core::identity::JobId=>Rule::Text,arguments:enrichment_core::operation::Arguments=>Rule::Text} }

    #[tokio::test]
    async fn plan19_static_worker_scope_requires_exact_python_resolution() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        for ecosystem in [Ecosystem::Python, Ecosystem::Rust] {
            let session = runtime.session();
            native_catalog::input(
                &session,
                "static_claims",
                Claim::batch(&[Claim {
                    grant_id: format!("grant_{}", "1".repeat(64)).try_into().unwrap(),
                    job_id: enrichment_core::identity::JobId::try_from(
                        "job_00112233445566778899aabbccddeeff".to_owned(),
                    )
                    .unwrap(),
                    policy_id: enrichment_core::identity::OperationPolicyId::from_record(
                        &enrichment_core::config::Config::default(),
                    ),
                    ecosystem,
                }])?,
            )?;
            let request: enrichment_core::request::ResolveRequest =
                serde_json::from_value(serde_json::json!({"ecosystem":"python","name":"fixture"}))
                    .unwrap();
            native_catalog::input(
                &session,
                "static_commands",
                Command::batch(&[Command {
                    job_id: enrichment_core::identity::JobId::try_from(
                        "job_00112233445566778899aabbccddeeff".to_owned(),
                    )
                    .unwrap(),
                    arguments: enrichment_core::operation::Arguments::Resolve { request },
                }])?,
            )?;
            for (grant, package) in [
                ("grant", "fixture"),
                ("wrong", "fixture"),
                ("grant", "other"),
            ] {
                assert_eq!(
                    runtime
                        .execute(
                            scope(
                                &session,
                                &format!(
                                    "grant_{}",
                                    if grant == "grant" { "1" } else { "2" }.repeat(64)
                                )
                                .try_into()
                                .unwrap(),
                                package
                            )
                            .await?
                        )
                        .await?
                        .rows,
                    usize::from(
                        ecosystem == Ecosystem::Python && grant == "grant" && package == "fixture"
                    )
                );
            }
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn plan19_dropped_static_child_keeps_paid_owner_until_reaped() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(1));
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let child = tokio::process::Command::new("/bin/sleep")
            .arg("30")
            .kill_on_drop(true)
            .spawn()?;
        let pid = child.id().unwrap();
        let owner = OwnedChild {
            child: Some(child),
            runtime: runtime.clone(),
            slot: Some(runtime.reserve_release()?),
            owners: Some(Box::new(permit)),
        };
        assert_eq!(semaphore.available_permits(), 0);
        drop(owner);
        runtime.close_diagnostics().await?;
        assert_eq!(semaphore.available_permits(), 1);
        assert!(!std::path::Path::new(&format!("/proc/{pid}")).exists());
        Ok(())
    }
}
