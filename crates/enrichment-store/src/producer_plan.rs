//! DataFusion lowers finite producer declarations. Drivers cannot choose flags or output scope.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::{
    execution::producer::{Command, Invocation, Selection},
    native_union::NativeStruct,
};

/// The finite rustdoc target/defaults are selected before physical preparation. Command
/// lowering consumes these exact options and current-command admission checks their scope.
pub async fn rustdoc_options(
    runtime: &QueryRuntime,
    environment: &enrichment_core::identity::Environment,
) -> Result<enrichment_core::execution::producer::RustdocOptions> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "rustdoc_environment",
        enrichment_core::identity::Environment::batch(std::slice::from_ref(environment))?,
    )?;
    runtime.require_empty(session.sql("SELECT 'rustdoc_target_not_installed' AS witness FROM rustdoc_environment WHERE target IS NOT NULL AND target<>$1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(enrichment_core::execution::producer::RUST_TARGET)])?,"rustdoc_target_not_installed","producer_preparation").await?;
    let frame=session.sql("SELECT coalesce(target,$1) AS target,coalesce(features,CAST([] AS VARCHAR[])) AS features,default_features IS NOT FALSE AS default_features FROM rustdoc_environment").await?.with_param_values(vec![datafusion::common::ScalarValue::from(enrichment_core::execution::producer::RUST_TARGET)])?;
    let frame = crate::native_delta::project(
        frame,
        &arrow::datatypes::Schema::new(
            enrichment_core::execution::producer::RustdocOptions::fields(),
        ),
    )?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("rustdoc options missing".into()))
}

pub async fn lower(runtime: &QueryRuntime, invocation: &Invocation) -> Result<Command> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "producer_request",
        Selection::batch(&[Selection {
            invocation: invocation.clone(),
        }])?,
    )?;
    runtime.require_empty(session.sql(r#"
        SELECT 'invalid_producer_parameter' AS witness FROM producer_request p
        WHERE (p.invocation.kind='probe' AND ((p.invocation.probe.ecosystem='python' AND p.invocation.probe.mode='compile') OR (p.invocation.probe.ecosystem='rust' AND p.invocation.probe.mode='typecheck')))
           OR (p.invocation.kind='rustdoc_fetch' AND NOT regexp_like(p.invocation.rustdoc_fetch.source_root,'^[A-Za-z0-9_][A-Za-z0-9_.-]{0,254}$'))
           OR (p.invocation.kind='rustdoc_build' AND (
             NOT regexp_like(p.invocation.rustdoc_build.source_root,'^[A-Za-z0-9_][A-Za-z0-9_.-]{0,254}$')
             OR NOT regexp_like(p.invocation.rustdoc_build.lib_name,'^[A-Za-z_][A-Za-z0-9_]{0,254}$')
             OR cardinality(p.invocation.rustdoc_build.features)>256
             OR array_any_match(p.invocation.rustdoc_build.features, f -> NOT regexp_like(f,'^[A-Za-z0-9_][A-Za-z0-9_+./?-]{0,254}$'))))
    "#).await?,"finite_producer_parameters","process_admission").await?;
    let frame = session.sql(r#"
      SELECT CASE WHEN p.invocation.kind='language_server' THEN 'language_server' ELSE 'command' END AS mode,
        CASE WHEN p.invocation.kind IN ('rust_fetch','rustdoc_fetch') THEN 'registry' ELSE 'offline' END AS network,
        CASE p.invocation.kind
          WHEN 'python_identity' THEN ['/usr/local/bin/python3','-I','-S','-c',concat('import sys,os; assert os.getuid()==65532; assert sys.version_info[:3]==tuple(map(int,"',$5,'".split("."))); print(sys.version)')]
          WHEN 'rust_identity' THEN ['/usr/local/cargo/bin/rustc',$1,'-vV']
          WHEN 'ty_identity' THEN ['/opt/producers/bin/ty','--version']
          WHEN 'rustdoc_identity' THEN ['/usr/local/cargo/bin/rustc',$2,'-vV']
          WHEN 'python_install' THEN ['/opt/producers/bin/uv','--no-config','--no-python-downloads','pip','install','--python=/usr/local/bin/python3','--offline','--no-index','--find-links=/capsule/wheelhouse','--only-binary=:all:','--require-hashes','--no-deps','--target=/capsule/python','--link-mode=copy','-r','/capsule/requirements.txt']
          WHEN 'rust_fetch' THEN ['/usr/local/cargo/bin/cargo',$1,'fetch','--manifest-path=/capsule/Cargo.toml',$3]
          WHEN 'runtime_object' THEN ['/usr/local/bin/python3','-I','-S','/capsule/runtime_object.py']
          WHEN 'language_server' THEN CASE p.invocation.language_server.ecosystem WHEN 'python' THEN ['/opt/producers/bin/ty','server'] ELSE ['/usr/local/cargo/bin/rust-analyzer'] END
          WHEN 'probe' THEN CASE WHEN p.invocation.probe.ecosystem='rust' THEN
            ['/usr/local/cargo/bin/cargo',$1,CASE WHEN p.invocation.probe.mode='runtime' THEN 'run' ELSE 'check' END,'--frozen','--manifest-path=/capsule/Cargo.toml',$3]
            WHEN p.invocation.probe.mode='typecheck' THEN
            ['/opt/producers/bin/ty','check','--project=/capsule','--python=/usr/local/bin/python3','--extra-search-path=/capsule/python','--python-version=3.14','--python-platform=linux','--output-format=concise','--color=never','--no-progress','--config-file=/capsule/probe-config/ty.toml','/capsule/consumer.py']
            ELSE ['/usr/local/bin/python3','-I','-S','-c','import sys,runpy; sys.path.insert(0,''/capsule/python''); runpy.run_path(''/capsule/consumer.py'',run_name=''__main__'')'] END
          WHEN 'rustdoc_fetch' THEN ['/usr/local/cargo/bin/cargo',$2,'fetch',concat('--manifest-path=/capsule/source/',p.invocation.rustdoc_fetch.source_root,'/Cargo.toml'),$3]
          WHEN 'rustdoc_build' THEN array_concat(
            ['/usr/local/cargo/bin/cargo',$2,'rustdoc','--frozen','--lib',concat('--manifest-path=/capsule/source/',p.invocation.rustdoc_build.source_root,'/Cargo.toml'),$3],
            CASE WHEN p.invocation.rustdoc_build.default_features THEN CAST([] AS VARCHAR[]) ELSE ['--no-default-features'] END,
            array_transform(p.invocation.rustdoc_build.features, f -> concat('--features=',f)),
            ['--','-Z','unstable-options','--output-format','json'])
        END AS argv,
        CASE p.invocation.kind WHEN 'rust_fetch' THEN ['Cargo.lock']
          WHEN 'rustdoc_fetch' THEN [concat('source/',p.invocation.rustdoc_fetch.source_root,'/Cargo.lock')]
          WHEN 'rustdoc_build' THEN [concat($4,p.invocation.rustdoc_build.lib_name,'.json')]
          ELSE CAST([] AS VARCHAR[]) END AS files,
        CASE p.invocation.kind WHEN 'python_install' THEN ['python']
          WHEN 'rust_fetch' THEN ['cargo-home'] WHEN 'rustdoc_fetch' THEN ['cargo-home']
          ELSE CAST([] AS VARCHAR[]) END AS directories,
        CASE p.invocation.kind WHEN 'python_install' THEN ['requirements.txt','probe-config/ty.toml']
          WHEN 'rust_fetch' THEN ['Cargo.toml','src/main.rs']
          WHEN 'runtime_object' THEN ['runtime_object.py','runtime-selection.json']
          WHEN 'rustdoc_fetch' THEN [concat('source/',p.invocation.rustdoc_fetch.source_root,'/Cargo.toml')]
          WHEN 'rustdoc_build' THEN [concat('source/',p.invocation.rustdoc_build.source_root,'/Cargo.toml'),concat('source/',p.invocation.rustdoc_build.source_root,'/Cargo.lock')]
          WHEN 'probe' THEN CASE WHEN p.invocation.probe.ecosystem='rust' THEN ['Cargo.toml','Cargo.lock','src/main.rs'] ELSE ['consumer.py','probe-config/ty.toml'] END
          WHEN 'language_server' THEN CASE WHEN p.invocation.language_server.ecosystem='rust' THEN ['Cargo.toml','Cargo.lock','src/main.rs'] ELSE ['probe-config/ty.toml'] END
          ELSE CAST([] AS VARCHAR[]) END AS required_files
      FROM producer_request p
    "#).await?.with_param_values(vec![
        datafusion::common::ScalarValue::from(format!("+{}",enrichment_core::execution::producer::STABLE)),
        format!("+{}",enrichment_core::execution::producer::RUSTDOC_TOOLCHAIN).into(),
        format!("--target={}",enrichment_core::execution::producer::RUST_TARGET).into(),
        format!("target/{}/doc/",enrichment_core::execution::producer::RUST_TARGET).into(),
        enrichment_core::execution::producer::PYTHON_VERSION.into(),
    ])?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("finite producer projection missing".into()))
}

/// Native preparation outcomes use the exact command declaration and captured physical facts.
/// Cleanup is part of readiness; a zero exit code alone never admits prepared inputs.
pub async fn preparation_result(
    runtime: &QueryRuntime,
    invocation: &Invocation,
    observation: &enrichment_core::execution::ProcessObservation,
    stage: &str,
) -> Result<enrichment_core::execution::producer::PreparationVerdict> {
    use enrichment_core::execution::producer;
    enrichment_core::native_struct! { struct Input {
        invocation: Invocation => enrichment_core::native_union::Rule::Text,
        observed: enrichment_core::execution::ProcessObservation => enrichment_core::native_union::Rule::Text,
        expected: Command => enrichment_core::native_union::Rule::Text,
        stage: String => enrichment_core::native_union::Rule::Text,
    } }
    let expected = lower(runtime, invocation).await?;
    let session = runtime.session();
    native_catalog::input(
        &session,
        "preparation_observed",
        Input::batch(&[Input {
            invocation: invocation.clone(),
            observed: observation.clone(),
            expected,
            stage: stage.into(),
        }])?,
    )?;
    let frame=session.sql(r#"
      WITH decided AS (SELECT *,CASE
        WHEN observed.command<>expected.argv THEN 'identity_mismatch'
        WHEN observed.end<>'exited' OR (observed.exit_code IS DISTINCT FROM 0) OR NOT observed.cleanup_confirmed THEN 'process_failed'
        WHEN invocation.kind='rust_identity' AND NOT (array_has(string_to_array(observed.stdout,chr(10)),concat('release: ',$1)) AND array_has(string_to_array(observed.stdout,chr(10)),concat('host: ',$2))) THEN 'identity_mismatch'
        WHEN invocation.kind='rustdoc_identity' AND NOT (array_has(string_to_array(observed.stdout,chr(10)),concat('release: ',$3)) AND array_any_match(string_to_array(observed.stdout,chr(10)),line -> starts_with(line,concat('commit-hash: ',$4))) AND array_has(string_to_array(observed.stdout,chr(10)),concat('host: ',$2))) THEN 'identity_mismatch'
        WHEN invocation.kind='ty_identity' AND regexp_replace(observed.stdout,'^\s+|\s+$','','g')<>concat('ty ',$5) THEN 'identity_mismatch'
        WHEN invocation.kind='python_identity' AND NOT starts_with(observed.stdout,concat($6,' ')) THEN 'identity_mismatch'
        WHEN invocation.kind NOT IN ('python_identity','rust_identity','ty_identity','rustdoc_identity','python_install','rust_fetch','rustdoc_fetch','rustdoc_build') THEN 'identity_mismatch'
        ELSE 'ready' END AS state FROM preparation_observed)
      SELECT state,CASE state WHEN 'ready' THEN '' WHEN 'identity_mismatch' THEN concat(stage,': producer identity or command differs from the native declaration')
        ELSE concat(stage,CASE WHEN observed.cleanup_confirmed THEN '' ELSE ' (its container was not confirmed removed)' END) END AS detail FROM decided
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::from(producer::STABLE),producer::RUST_TARGET.into(),producer::RUSTDOC_RELEASE.into(),producer::RUSTDOC_COMMIT.into(),producer::TY_VERSION.into(),producer::PYTHON_VERSION.into()])?;
    runtime
        .records::<producer::PreparationVerdict>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("preparation outcome missing".into()))
}

/// Recompute the native command from its request, then compare full values. Retained byte
/// identity is a witness, never proof that the caller selected a permitted producer.
pub async fn admit(
    runtime: &QueryRuntime,
    operation: &enrichment_core::capsule_protocol::Operation,
) -> Result<()> {
    use datafusion::prelude::col;
    use datafusion::prelude::lit;
    use enrichment_core::native_union::{Cell, Rule};
    let invocation = operation
        .invocation
        .as_ref()
        .ok_or_else(|| DataFusionError::Execution("command producer invocation missing".into()))?;
    let selected = lower(runtime, invocation).await?;
    let mut expected = operation.clone();
    expected.argv = selected.argv.clone();
    expected.mode = selected.mode;
    expected.outputs = selected.outputs();
    expected.launch.network = selected.network;
    let session = runtime.session();
    enrichment_core::native_struct! { struct Compared {
        actual: enrichment_core::capsule_protocol::Operation => Rule::Text,
        expected: enrichment_core::capsule_protocol::Operation => Rule::Text,
    } }
    let values = crate::native_catalog::batch(
        &session,
        "producer_plan",
        Compared::batch(&[Compared {
            actual: operation.clone(),
            expected,
        }])?,
    )?;
    let encode = enrichment_core::native_identity::canonical_bytes(
        "enrichment/finite-producer/1",
        vec![std::sync::Arc::new(arrow::datatypes::Field::new(
            "operation",
            enrichment_core::capsule_protocol::Operation::data_type(),
            false,
        ))]
        .into(),
    );
    runtime
        .require_empty(
            values
                .filter(
                    encode
                        .call(vec![col("actual")])
                        .not_eq(encode.call(vec![col("expected")])),
                )?
                .select(vec![lit("producer_contract_mismatch").alias("witness")])?,
            "finite_producer_contract",
            "process_admission",
        )
        .await?;
    // Only files satisfy required inputs; a directory or link with the same name does not.
    native_catalog::input(
        &session,
        "producer_operation",
        enrichment_core::capsule_protocol::Operation::batch(std::slice::from_ref(operation))?,
    )?;
    let required =
        enrichment_core::evidence::arrow_model::expressions::literal(&selected.required_files)?;
    let frame = session
        .read_empty()?
        .select(vec![required.alias("required")])?;
    native_catalog::work(&session, "producer_required", frame.into_view())?;
    runtime.require_empty(session.sql("SELECT 'producer_input_missing' AS witness FROM producer_operation p CROSS JOIN producer_required r WHERE cardinality(array_except(r.required, array_transform(array_filter(native_map_entries(p.inputs), e -> get_field(get_field(e,'value'),'kind')='file'), e -> get_field(e,'key'))))>0").await?,"finite_producer_inputs","process_admission").await
}

/// The invocation must be the operation requested by this durable command, in this ecosystem.
/// This pure plan is shared by admission and the independent native truth-table fixture.
pub(crate) async fn scope_refusals(
    runtime: &QueryRuntime,
    scope: datafusion::dataframe::DataFrame,
) -> Result<datafusion::dataframe::DataFrame> {
    let session = runtime.session();
    native_catalog::work(&session, "producer_scope", scope.into_view())?;
    session.sql(r#"
      SELECT 'producer_request_scope' AS witness FROM producer_scope p
      WHERE NOT coalesce(CASE
        WHEN p.invocation.kind IN ('python_identity','ty_identity','python_install') THEN p.ecosystem='python' AND (p.arguments.verify IS NOT NULL OR p.arguments.inspect IS NOT NULL)
        WHEN p.invocation.kind IN ('rust_identity','rust_fetch') THEN p.ecosystem='rust' AND (p.arguments.verify IS NOT NULL OR p.arguments.inspect IS NOT NULL)
        WHEN p.invocation.kind IN ('rustdoc_identity','rustdoc_fetch','rustdoc_build') THEN
          p.ecosystem='rust' AND p.arguments.resolve IS NOT NULL AND p.arguments.resolve.request.allow_local_build
          AND p.arguments.resolve.request.freshness<>'offline'
          AND coalesce(p.arguments.resolve.request.target,'x86_64-unknown-linux-gnu')='x86_64-unknown-linux-gnu'
          AND (p.invocation.kind<>'rustdoc_build' OR (
            p.invocation.rustdoc_build.default_features=coalesce(p.arguments.resolve.request.default_features,true)
            AND cardinality(array_except(p.invocation.rustdoc_build.features,coalesce(p.arguments.resolve.request.features,CAST([] AS VARCHAR[]))))=0
            AND cardinality(array_except(coalesce(p.arguments.resolve.request.features,CAST([] AS VARCHAR[])),p.invocation.rustdoc_build.features))=0))
        WHEN p.invocation.kind='probe' THEN p.arguments.verify IS NOT NULL
          AND p.invocation.probe.ecosystem=p.ecosystem AND p.invocation.probe.mode=p.arguments.verify.request.mode
          AND array_any_match(native_map_entries(p.inputs), entry -> get_field(entry,'key')=CASE WHEN p.ecosystem='python' THEN 'consumer.py' ELSE 'src/main.rs' END
            AND get_field(get_field(entry,'value'),'kind')='file' AND get_field(get_field(get_field(entry,'value'),'file'),'sha256')=encode(sha256(p.arguments.verify.request.snippet),'hex'))
        WHEN p.invocation.kind='language_server' THEN p.arguments.inspect IS NOT NULL
          AND p.invocation.language_server.ecosystem=p.ecosystem
          AND p.arguments.inspect.request.execution.intent IN ('execute_on_miss','rerun')
          AND p.arguments.inspect.request.execution.profile='build'
          AND p.arguments.inspect.request.execution.runtime IS NULL
        WHEN p.invocation.kind='runtime_object' THEN p.ecosystem='python' AND p.arguments.inspect IS NOT NULL
          AND p.arguments.inspect.request.execution.intent IN ('execute_on_miss','rerun')
          AND p.arguments.inspect.request.execution.profile='runtime'
          AND p.arguments.inspect.request.execution.runtime IS NOT NULL
        ELSE false END,false)
    "#).await
}

/// Exact operator fixture scope. The private operator owner constructs the finite allowed
/// contracts; this relation confers no command authority and has no physical driver.
pub async fn qualify(
    runtime: &QueryRuntime,
    operation: &enrichment_core::capsule_protocol::Operation,
    allowed: &[enrichment_core::capsule_protocol::Operation],
    actual_image: &str,
    expected_image: &str,
    acquisition: bool,
) -> Result<()> {
    use datafusion::{common::ScalarValue, prelude::col};
    use enrichment_core::{capsule_protocol::Operation, native_union::NativeStruct};
    let session = runtime.session();
    let encode = enrichment_core::native_identity::canonical_bytes(
        "enrichment/operator-contract/1",
        Operation::fields(),
    );
    for (name, values) in [
        ("operator_actual", std::slice::from_ref(operation)),
        ("operator_allowed", allowed),
    ] {
        let frame =
            crate::native_catalog::batch(&session, "producer_plan", Operation::batch(values)?)?;
        let value = encode.call(
            Operation::fields()
                .iter()
                .map(|field| col(field.name()))
                .collect(),
        );
        native_catalog::work(
            &session,
            name,
            frame.select(vec![value.alias("contract")])?.into_view(),
        )?;
    }
    runtime.require_empty(session.sql("SELECT 'qualification_scope_mismatch' AS witness FROM operator_actual a JOIN operator_allowed e ON a.contract=e.contract WHERE $1=$2 AND NOT CAST($3 AS BOOLEAN) HAVING count(*)<>1").await?.with_param_values(vec![ScalarValue::from(actual_image),expected_image.into(),acquisition.into()])?,"fixed_qualification_contract","operator_qualification").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        capsule_protocol::{
            Launch, Operation,
            inventory::{Entry, Inventory},
        },
        execution::ProbeMode,
        identity::Ecosystem,
        native_union::Rule,
    };
    enrichment_core::native_struct! { struct Scope {
        invocation: Invocation => Rule::Text,
        inputs: Inventory => Rule::Map,
        arguments: enrichment_core::operation::Arguments => Rule::Text,
        ecosystem: Ecosystem => Rule::Text,
    } }
    #[tokio::test]
    async fn native_rustdoc_options_preserve_declared_features_and_reject_uninstalled_target()
    -> Result<()> {
        use enrichment_core::{execution::producer::RUST_TARGET, identity::Environment};
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let requested = Environment::declared(
            Some(RUST_TARGET.into()),
            Some(vec!["serde".into()]),
            Some(false),
        );
        let selected = rustdoc_options(&runtime, &requested).await?;
        assert_eq!(selected.features, ["serde"]);
        assert!(!selected.default_features);
        let defaults = rustdoc_options(&runtime, &Environment::unspecified()).await?;
        assert_eq!(defaults.target, RUST_TARGET);
        assert!(defaults.features.is_empty());
        assert!(defaults.default_features);
        let missing = Environment::declared(Some("aarch64-unknown-linux-gnu".into()), None, None);
        assert!(rustdoc_options(&runtime, &missing).await.is_err());
        runtime.close_diagnostics().await
    }
    #[tokio::test]
    async fn plan19_finite_producers_derive_and_admit_exact_contracts() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let mut invocations = vec![
            Invocation::PythonIdentity,
            Invocation::RustIdentity,
            Invocation::TyIdentity,
            Invocation::RustdocIdentity,
            Invocation::PythonInstall,
            Invocation::RustFetch,
            Invocation::RuntimeObject,
            Invocation::RustdocFetch {
                source_root: "sample-1.0".into(),
            },
            Invocation::RustdocBuild {
                source_root: "sample-1.0".into(),
                lib_name: "sample".into(),
                features: vec!["serde".into(), "dep/feature".into()],
                default_features: false,
            },
        ];
        for ecosystem in [Ecosystem::Python, Ecosystem::Rust] {
            invocations.push(Invocation::LanguageServer { ecosystem });
            for mode in [
                if ecosystem == Ecosystem::Python {
                    ProbeMode::Typecheck
                } else {
                    ProbeMode::Compile
                },
                ProbeMode::Runtime,
            ] {
                invocations.push(Invocation::Probe { ecosystem, mode });
            }
        }
        for invocation in invocations {
            let command = lower(&runtime, &invocation).await?;
            let operation = Operation {
                version: enrichment_core::capsule_protocol::VERSION,
                invocation: Some(invocation.clone()),
                prepared: None,
                mode: command.mode,
                argv: command.argv.clone(),
                outputs: command.outputs(),
                inputs: command
                    .required_files
                    .iter()
                    .map(|path| {
                        (
                            path.clone(),
                            Entry::File {
                                mode: 0o400,
                                bytes: 3,
                                sha256: "a".repeat(64),
                            },
                        )
                    })
                    .collect(),
                launch: Launch::for_execution(
                    &Default::default(),
                    &format!("sha256:{}", "a".repeat(64)),
                    "fixed-image",
                    command.network == enrichment_core::capsule_protocol::Network::Registry,
                )?,
            };
            operation.validate()?;
            admit(&runtime, &operation).await?;
            let mut changed = operation.clone();
            changed.argv.push("--extra".into());
            assert!(admit(&runtime, &changed).await.is_err());
            if !operation.inputs.is_empty() {
                let mut changed = operation.clone();
                changed.inputs.clear();
                assert!(admit(&runtime, &changed).await.is_err());
            }
            if matches!(invocation, Invocation::RustdocBuild { .. }) {
                assert!(command.argv.contains(&"--features=dep/feature".into()));
                assert!(command.argv.contains(&"--no-default-features".into()));
                assert!(command.argv.contains(&"--frozen".into()));
                assert_eq!(
                    command.files,
                    ["target/x86_64-unknown-linux-gnu/doc/sample.json"]
                );
            }
        }
        assert!(
            lower(
                &runtime,
                &Invocation::RustdocFetch {
                    source_root: "../outside".into()
                }
            )
            .await
            .is_err()
        );
        assert!(
            lower(
                &runtime,
                &Invocation::RustdocBuild {
                    source_root: "package".into(),
                    lib_name: "../outside".into(),
                    features: vec![],
                    default_features: true
                }
            )
            .await
            .is_err()
        );
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn plan19_producer_scope_requires_requested_mode_and_snippet_bytes() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let request = enrichment_core::execution::VerifyRequest {
            context_id: format!("ctx_{}", "1".repeat(64)).try_into().unwrap(),
            snapshot_id: None,
            snippet: "print(1)".into(),
            mode: ProbeMode::Runtime,
            profile: enrichment_core::policy::ExecutionProfile::Runtime,
            test_intent: None,
            max_bytes: None,
        };
        let mut scope = Scope {
            invocation: Invocation::Probe {
                ecosystem: Ecosystem::Python,
                mode: ProbeMode::Runtime,
            },
            arguments: enrichment_core::operation::Arguments::Verify {
                request: request.clone(),
            },
            ecosystem: Ecosystem::Python,
            inputs: [(
                "consumer.py".into(),
                Entry::File {
                    mode: 0o400,
                    bytes: 8,
                    sha256: enrichment_core::canonical::sha256_hex(request.snippet.as_bytes()),
                },
            )]
            .into(),
        };
        async fn refuses(runtime: &QueryRuntime, scope: &Scope) -> Result<bool> {
            let frame = crate::native_catalog::batch(
                &runtime.session(),
                "producer_plan",
                Scope::batch(std::slice::from_ref(scope))?,
            )?;
            Ok(runtime
                .execute(scope_refusals(runtime, frame).await?)
                .await?
                .rows
                != 0)
        }
        assert!(!refuses(&runtime, &scope).await?);
        scope.invocation = Invocation::Probe {
            ecosystem: Ecosystem::Python,
            mode: ProbeMode::Typecheck,
        };
        assert!(refuses(&runtime, &scope).await?);
        scope.invocation = Invocation::RustdocIdentity;
        assert!(refuses(&runtime, &scope).await?);
        scope.invocation = Invocation::Probe {
            ecosystem: Ecosystem::Python,
            mode: ProbeMode::Runtime,
        };
        scope.inputs.clear();
        assert!(refuses(&runtime, &scope).await?);
        runtime.close_diagnostics().await
    }

    #[tokio::test]
    async fn native_preparation_requires_exact_command_identity_and_cleanup() -> Result<()> {
        use enrichment_core::execution::{
            ProcessAuthority, ProcessEnd, ProcessObservation,
            producer::{self, PreparationState},
        };
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        for (invocation, stdout) in [
            (
                Invocation::RustIdentity,
                format!(
                    "release: {}\nhost: {}\n",
                    producer::STABLE,
                    producer::RUST_TARGET
                ),
            ),
            (
                Invocation::RustdocIdentity,
                format!(
                    "release: {}\ncommit-hash: {}0123456\nhost: {}\n",
                    producer::RUSTDOC_RELEASE,
                    producer::RUSTDOC_COMMIT,
                    producer::RUST_TARGET
                ),
            ),
            (
                Invocation::PythonIdentity,
                format!("{} (qualified)\n", producer::PYTHON_VERSION),
            ),
            (
                Invocation::TyIdentity,
                format!("ty {}\n", producer::TY_VERSION),
            ),
        ] {
            let observation = ProcessObservation {
                operation_id: format!("process_{}", "1".repeat(64)).try_into().unwrap(),
                authority: ProcessAuthority::Qualification {
                    definition_id: format!("process_{}", "1".repeat(64)).try_into().unwrap(),
                },
                image_id: "image".into(),
                command: lower(&runtime, &invocation).await?.argv,
                started_at: enrichment_core::native_time::ObservationTime::from_micros(1)?,
                finished_at: enrichment_core::native_time::ObservationTime::from_micros(2)?,
                exit_code: Some(0),
                end: ProcessEnd::Exited,
                stdout,
                stderr: String::new(),
                cleanup_confirmed: true,
            };
            assert_eq!(
                preparation_result(&runtime, &invocation, &observation, "identity")
                    .await?
                    .state,
                PreparationState::Ready,
                "{invocation:?}"
            );
            let mut changed = observation.clone();
            changed.stdout = "unrecorded".into();
            assert_eq!(
                preparation_result(&runtime, &invocation, &changed, "identity")
                    .await?
                    .state,
                PreparationState::IdentityMismatch
            );
            changed = observation.clone();
            changed.cleanup_confirmed = false;
            assert_eq!(
                preparation_result(&runtime, &invocation, &changed, "identity")
                    .await?
                    .state,
                PreparationState::ProcessFailed
            );
            changed = observation.clone();
            changed.command.push("--extra".into());
            assert_eq!(
                preparation_result(&runtime, &invocation, &changed, "identity")
                    .await?
                    .state,
                PreparationState::IdentityMismatch
            );
            changed = observation;
            changed.exit_code = None;
            assert_eq!(
                preparation_result(&runtime, &invocation, &changed, "identity")
                    .await?
                    .state,
                PreparationState::ProcessFailed
            );
        }
        runtime.close_diagnostics().await
    }
}
