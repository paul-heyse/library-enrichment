//! One installed-input contract for retained capsule reuse and process admission.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::Result, dataframe::DataFrame, functions::core::expr_ext::FieldAccessor, prelude::*,
};
use enrichment_core::{
    execution::environment::Target, identity::Ecosystem, native_key::Key,
    native_union::NativeStruct, operation::ownership::PreparedCapsule,
};

/// The caller supplies a native PreparedCapsule relation, not a claimed validity flag.
pub(crate) async fn refusals(session: &SessionContext, mut frame: DataFrame) -> Result<DataFrame> {
    for (key, source, id, alias) in [
        (
            Key::Release,
            col("inputs").field("release").field("key"),
            col("inputs").field("release").field("release_id"),
            "release_valid",
        ),
        (
            Key::Context,
            col("inputs").field("context"),
            col("inputs").field("context").field("context_id"),
            "context_valid",
        ),
        (
            Key::Environment,
            col("inputs").field("environment"),
            col("inputs").field("environment").field("environment_id"),
            "parent_environment_valid",
        ),
        (
            Key::Environment,
            col("environment"),
            col("environment").field("environment_id"),
            "environment_valid",
        ),
    ] {
        let members = key
            .schema()
            .fields()
            .iter()
            .map(|field| source.clone().field(field.name()))
            .collect();
        frame = frame.with_column(alias, id.eq(key.identity_expression(members)?))?;
    }
    native_catalog::work(session, "prepared", frame.into_view())?;
    // Bind the same finite target declaration used before physical preparation. The image
    // discriminator is added in SQL for every row rather than accepting a caller's toolchain.
    native_catalog::input(
        session,
        "targets",
        Target::batch(&[
            Target::consumer(Ecosystem::Python, ""),
            Target::consumer(Ecosystem::Rust, ""),
        ])?,
    )?;
    session.sql(r#"
      SELECT 'prepared_capsule_contract' AS witness FROM prepared p
      LEFT JOIN targets t ON p.inputs.release.key.ecosystem=t.ecosystem
      WHERE NOT coalesce(p.release_valid AND p.context_valid AND p.parent_environment_valid AND p.environment_valid
        AND p.inputs.context.release_id=p.inputs.release.release_id
        AND p.inputs.context.environment_id=p.inputs.environment.environment_id
        AND p.inputs.profile='build'
        AND p.environment.resolution='resolved'
        AND p.environment.toolchain=concat(t.toolchain,p.inputs.image)
        AND p.environment.target=t.target
        AND p.environment.lock_digest=encode(sha256(p.lock),'hex')
        AND array_any_match(native_map_entries(p.inventory), e -> get_field(e,'key')='enrichment.lock'
          AND get_field(get_field(e,'value'),'kind')='file'
          AND get_field(get_field(get_field(e,'value'),'file'),'sha256')=p.environment.lock_digest
          AND get_field(get_field(get_field(e,'value'),'file'),'bytes')=octet_length(p.lock))
        AND (p.inputs.environment.toolchain IS NULL OR p.inputs.environment.toolchain=p.environment.toolchain OR array_has(t.toolchain_aliases,p.inputs.environment.toolchain))
        AND (p.inputs.environment.target IS NULL OR p.inputs.environment.target=p.environment.target OR array_has(t.target_aliases,p.inputs.environment.target))
        AND p.environment.features=coalesce(p.inputs.environment.features,CAST([] AS VARCHAR[]))
        AND (p.environment.default_features IS NOT DISTINCT FROM CASE WHEN t.ecosystem='rust' THEN coalesce(p.inputs.environment.default_features,true) ELSE NULL END)
        AND (p.inputs.environment.lock_digest IS NULL OR p.inputs.environment.lock_digest=p.environment.lock_digest), false)
    "#).await
}

/// The complete descriptor, including its baseline inventory, is part of the immutable
/// Operation retained by ProcessGrant. Only declared per-command overlays can differ.
pub(crate) async fn admit_operation(
    runtime: &QueryRuntime,
    operation: &enrichment_core::capsule_protocol::Operation,
) -> Result<()> {
    use enrichment_core::execution::producer::Invocation;
    let requires = matches!(
        operation.invocation,
        Some(
            Invocation::Probe { .. }
                | Invocation::LanguageServer { .. }
                | Invocation::RuntimeObject
        )
    );
    let Some(prepared) = &operation.prepared else {
        return if requires {
            Err(datafusion::common::DataFusionError::Execution(
                "producer requires an exact prepared capsule".into(),
            ))
        } else {
            Ok(())
        };
    };
    if !requires {
        return Err(datafusion::common::DataFusionError::Execution(
            "preparation producer cannot claim an installed capsule".into(),
        ));
    }
    let session = runtime.session();
    runtime
        .require_empty(
            refusals(
                &session,
                crate::native_catalog::batch(
                    &session,
                    "prepared_capsule",
                    PreparedCapsule::batch(std::slice::from_ref(prepared))?,
                )?,
            )
            .await?,
            "prepared_capsule",
            "process_admission",
        )
        .await?;
    native_catalog::input(
        &session,
        "operation",
        enrichment_core::capsule_protocol::Operation::batch(std::slice::from_ref(operation))?,
    )?;
    runtime.require_empty(session.sql(r#"
      WITH scoped AS (
        SELECT *, CASE invocation.kind
          WHEN 'probe' THEN CASE invocation.probe.ecosystem WHEN 'python' THEN ['consumer.py'] ELSE ['src/main.rs'] END
          WHEN 'runtime_object' THEN ['runtime_object.py','runtime-selection.json'] ELSE CAST([] AS VARCHAR[]) END AS overlays
        FROM operation
      ), entries AS (
        SELECT *,
          array_filter(native_map_entries(inputs), e -> NOT array_has(overlays,get_field(e,'key'))) AS actual,
          array_filter(native_map_entries(prepared.inventory), e -> NOT array_has(overlays,get_field(e,'key'))) AS installed
        FROM scoped
      ) SELECT 'prepared_input_closure_mismatch' AS witness FROM entries
        WHERE prepared.inputs.image<>launch.image OR prepared.inputs.containment<>launch.containment
          OR cardinality(array_except(actual,installed))>0 OR cardinality(array_except(installed,actual))>0
    "#).await?, "prepared_input_closure", "process_admission").await
}

pub(crate) async fn runtime_refusals(
    runtime: &QueryRuntime,
    frame: DataFrame,
) -> Result<DataFrame> {
    use enrichment_core::{evidence::arrow_model::expressions::record, native_union::Cell};
    let input = record(
        &enrichment_core::native_runtime::SelectionTransport::data_type(),
        &[
            (
                "selection",
                col("arguments")
                    .field("inspect")
                    .field("request")
                    .field("execution")
                    .field("runtime"),
            ),
            ("max_output_bytes", col("launch").field("output_bytes")),
        ],
    )?;
    let frame = frame.with_column(
        "expected_selection",
        enrichment_core::native_runtime::selection_transport().call(vec![input]),
    )?;
    let session = runtime.session();
    native_catalog::work(&session, "runtime_inputs", frame.into_view())?;
    session.sql(r#"
      SELECT 'runtime_program_or_selection_mismatch' AS witness FROM runtime_inputs p
      WHERE NOT array_any_match(native_map_entries(p.inputs), e -> get_field(e,'key')='runtime_object.py'
        AND get_field(get_field(e,'value'),'kind')='file'
        AND get_field(get_field(get_field(e,'value'),'file'),'sha256')=encode(sha256($1),'hex')
        AND get_field(get_field(get_field(e,'value'),'file'),'bytes')=octet_length($1))
      OR NOT array_any_match(native_map_entries(p.inputs), e -> get_field(e,'key')='runtime-selection.json'
        AND get_field(get_field(e,'value'),'kind')='file'
        AND get_field(get_field(get_field(e,'value'),'file'),'sha256')=encode(sha256(p.expected_selection),'hex')
        AND get_field(get_field(get_field(e,'value'),'file'),'bytes')=octet_length(p.expected_selection))
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::from(enrichment_core::execution::producer::RUNTIME_OBJECT_HELPER)])
}

/// Native command, claim and exact selected snapshot/context must name the descriptor's parent.
pub(crate) async fn command_refusals(
    runtime: &QueryRuntime,
    frame: DataFrame,
) -> Result<DataFrame> {
    let session = runtime.session();
    native_catalog::work(&session, "prepared_command", frame.into_view())?;
    session.sql(r#"
      SELECT 'prepared_command_scope' AS witness FROM prepared_command p
      WHERE p.prepared IS NOT NULL AND NOT coalesce(
        ((p.arguments.kind='verify' AND p.prepared.inputs.context.context_id=p.arguments.verify.request.context_id)
          OR (p.arguments.kind='inspect' AND p.prepared.inputs.context.context_id=p.arguments.inspect.request.context_id))
        AND p.prepared.inputs.context.context_id=p.context_id
        AND p.prepared.inputs.environment.environment_id=p.environment_id
        AND p.prepared.inputs.environment.environment_id=p.context_environment_id
        AND p.prepared.inputs.release.release_id=p.release_id
        AND p.prepared.inputs.context.mode=p.mode,false)
    "#).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        capsule_protocol::{Launch, Operation, inventory::Entry},
        execution::{ProbeMode, producer::Invocation},
        identity::{Context, Environment, Release, ReleaseKey, ResearchMode},
        native_union::Rule,
        operation::{Arguments, identities::CapsuleIdentity},
        policy::ExecutionProfile,
        request::{InspectRequest, InspectionIntent, InspectionOptions, RuntimeSelection},
    };
    fn file(bytes: &str) -> Entry {
        Entry::File {
            mode: 0o600,
            bytes: bytes.len() as u64,
            sha256: enrichment_core::canonical::sha256_hex(bytes.as_bytes()),
        }
    }
    fn fixture() -> PreparedCapsule {
        let release = Release::new(ReleaseKey {
            ecosystem: Ecosystem::Python,
            registry: "pypi".into(),
            package: "fixture".into(),
            version: "1.0.0".into(),
            artifact_digest: None,
        });
        let environment = Environment::unspecified();
        let image = format!("sha256:{}", "a".repeat(64));
        let target = Target::consumer(Ecosystem::Python, &image);
        PreparedCapsule {
            inputs: CapsuleIdentity {
                context: Context::new(
                    release.release_id.clone(),
                    environment.environment_id.clone(),
                    ResearchMode::Project,
                ),
                release,
                environment,
                image,
                containment: "qualified-helper".into(),
                profile: ExecutionProfile::Build,
            },
            environment: Environment::resolved(
                target.toolchain,
                target.target,
                vec![],
                None,
                enrichment_core::canonical::sha256_hex(b"lock"),
            ),
            lock: "lock".into(),
            inventory: [
                ("enrichment.lock".into(), file("lock")),
                ("python/fixture.py".into(), file("installed")),
            ]
            .into(),
        }
    }
    async fn operation(runtime: &QueryRuntime, invocation: Invocation) -> Result<Operation> {
        let prepared = fixture();
        let command = crate::producer_plan::lower(runtime, &invocation).await?;
        let launch = Launch::for_execution(
            &Default::default(),
            &prepared.inputs.image,
            &prepared.inputs.containment,
            false,
        )?;
        Ok(Operation {
            version: enrichment_core::capsule_protocol::VERSION,
            invocation: Some(invocation),
            prepared: Some(prepared.clone()),
            mode: command.mode,
            argv: command.argv.clone(),
            inputs: prepared.inventory,
            outputs: command.outputs(),
            launch,
        })
    }
    #[tokio::test]
    async fn plan19_prepared_capsule_binds_environment_lock_and_exact_installed_closure()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut original = operation(
            &runtime,
            Invocation::Probe {
                ecosystem: Ecosystem::Python,
                mode: ProbeMode::Runtime,
            },
        )
        .await?;
        original
            .inputs
            .insert("consumer.py".into(), file("print(1)"));
        admit_operation(&runtime, &original).await?;
        for variant in 0..12 {
            let mut changed = original.clone();
            match variant {
                0 => changed.prepared = None,
                1 => changed.prepared.as_mut().unwrap().lock.push('x'),
                2 => {
                    changed.prepared.as_mut().unwrap().environment.toolchain = Some("other".into())
                }
                3 => {
                    changed.prepared.as_mut().unwrap().inputs.context.mode = ResearchMode::Upstream
                }
                4 => {
                    changed.prepared.as_mut().unwrap().inputs.image =
                        format!("sha256:{}", "b".repeat(64))
                }
                5 => changed.launch.containment = "other".into(),
                6 => {
                    changed
                        .inputs
                        .insert("python/fixture.py".into(), file("changed"));
                }
                7 => {
                    changed.inputs.insert("extra.py".into(), file("extra"));
                }
                8 => {
                    changed.inputs.remove("python/fixture.py");
                }
                9 => {
                    changed
                        .inputs
                        .insert("enrichment.lock".into(), file("different lock"));
                }
                10 => {
                    changed
                        .prepared
                        .as_mut()
                        .unwrap()
                        .inputs
                        .release
                        .key
                        .version = "2.0.0".into()
                }
                _ => {
                    changed.prepared.as_mut().unwrap().environment = Environment::resolved(
                        "python-3.14.7;wrong-image".into(),
                        "linux-x86_64".into(),
                        vec![],
                        None,
                        enrichment_core::canonical::sha256_hex(b"lock"),
                    )
                }
            }
            assert!(
                admit_operation(&runtime, &changed).await.is_err(),
                "accepted changed prepared field {variant}"
            );
        }
        let mut server = operation(
            &runtime,
            Invocation::LanguageServer {
                ecosystem: Ecosystem::Python,
            },
        )
        .await?;
        admit_operation(&runtime, &server).await?;
        server
            .inputs
            .insert("consumer.py".into(), file("unauthorized on-disk document"));
        assert!(admit_operation(&runtime, &server).await.is_err());
        runtime.close_diagnostics().await
    }
    enrichment_core::native_struct! { struct CommandScope {
        prepared: Option<PreparedCapsule> => Rule::Text,
        arguments: Arguments => Rule::Text,
        environment_id: Option<enrichment_core::identity::EnvironmentId> => Rule::Text,
        context_environment_id: Option<enrichment_core::identity::EnvironmentId> => Rule::Text,
        context_id: Option<enrichment_core::identity::ContextId> => Rule::Text,
        release_id: Option<enrichment_core::identity::ReleaseId> => Rule::Text,
        mode: Option<ResearchMode> => Rule::Text,
    } }
    #[tokio::test]
    async fn plan19_prepared_scope_requires_exact_command_claim_and_snapshot_context() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let prepared = fixture();
        let scope = CommandScope {
            environment_id: Some(prepared.inputs.environment.environment_id.clone()),
            context_environment_id: Some(prepared.inputs.environment.environment_id.clone()),
            context_id: Some(prepared.inputs.context.context_id.clone()),
            release_id: Some(prepared.inputs.release.release_id.clone()),
            mode: Some(prepared.inputs.context.mode),
            arguments: Arguments::Verify {
                request: enrichment_core::execution::VerifyRequest {
                    context_id: prepared.inputs.context.context_id.clone(),
                    snapshot_id: None,
                    snippet: "pass".into(),
                    mode: ProbeMode::Runtime,
                    profile: ExecutionProfile::Runtime,
                    test_intent: None,
                    max_bytes: None,
                },
            },
            prepared: Some(prepared),
        };
        for variant in 0..7 {
            let mut scope = scope.clone();
            match variant {
                1 => scope.environment_id = None,
                2 => scope.context_environment_id = None,
                3 => scope.context_id = None,
                4 => scope.release_id = None,
                5 => scope.mode = Some(ResearchMode::Upstream),
                6 => {
                    let Arguments::Verify { request } = &mut scope.arguments else {
                        unreachable!()
                    };
                    request.context_id = Context::new(
                        scope.release_id.clone().unwrap(),
                        scope.environment_id.clone().unwrap(),
                        ResearchMode::Upstream,
                    )
                    .context_id;
                }
                _ => {}
            }
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "prepared_capsule",
                CommandScope::batch(&[scope])?,
            )?;
            assert_eq!(
                runtime
                    .execute(command_refusals(&runtime, frame).await?)
                    .await?
                    .rows
                    > 0,
                variant != 0,
                "prepared command scope {variant}"
            );
        }
        runtime.close_diagnostics().await
    }
    enrichment_core::native_struct! { struct RuntimeScope {
        inputs: enrichment_core::capsule_protocol::inventory::Inventory => Rule::Map,
        launch: Launch => Rule::Text,
        arguments: Arguments => Rule::Text,
    } }
    #[tokio::test]
    async fn plan19_runtime_program_and_transport_are_bound_to_requested_selection() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let operation = operation(&runtime, Invocation::RuntimeObject).await?;
        let selection = RuntimeSelection {
            module: "fixture".into(),
            attributes: vec!["Widget".into()],
        };
        let transport = crate::runtime_object_plan::selection_transport(
            &runtime,
            &selection,
            operation.launch.output_bytes as usize,
        )
        .await?;
        let input = RuntimeScope {
            inputs: [
                (
                    "runtime_object.py".into(),
                    file(enrichment_core::execution::producer::RUNTIME_OBJECT_HELPER),
                ),
                ("runtime-selection.json".into(), file(&transport)),
            ]
            .into(),
            launch: operation.launch,
            arguments: Arguments::Inspect {
                request: InspectRequest {
                    context_id: operation.prepared.unwrap().inputs.context.context_id,
                    snapshot_id: None,
                    symbol_path: "fixture.Widget".into(),
                    definition_id: None,
                    selection: Default::default(),
                    max_bytes: None,
                    execution: Some(InspectionOptions {
                        intent: InspectionIntent::ExecuteOnMiss,
                        profile: Some(ExecutionProfile::Runtime),
                        runtime: Some(selection),
                        ..Default::default()
                    }),
                },
            },
        };
        for variant in 0..5 {
            let mut input = input.clone();
            match variant {
                1 => {
                    input
                        .inputs
                        .insert("runtime_object.py".into(), file("forged helper"));
                }
                2 => {
                    input
                        .inputs
                        .insert("runtime-selection.json".into(), file("forged selection"));
                }
                3 => input.launch.output_bytes += 1,
                4 => {
                    input.inputs.remove("runtime-selection.json");
                }
                _ => {}
            }
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "prepared_capsule",
                RuntimeScope::batch(&[input])?,
            )?;
            let refused = runtime
                .execute(runtime_refusals(&runtime, frame).await?)
                .await?
                .rows
                > 0;
            assert_eq!(refused, variant != 0, "runtime program case {variant}");
        }
        runtime.close_diagnostics().await
    }
}
