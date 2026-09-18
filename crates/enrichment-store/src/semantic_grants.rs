//! A warm server retains a process, while each semantic conversation derives new authority
//! from the live command and an exact admitted snapshot. No caller supplies document bytes.
use crate::{
    control_jobs::Grant,
    immutable_definitions::Definitions,
    process_grants::ProcessGrant,
    query::SnapshotReader,
    retention::{Dependency, LeaseGuard, ProtectionKind, RetentionStore},
};
use datafusion::functions::core::expr_ext::FieldAccessor;
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    prelude::{col, lit},
};
use enrichment_core::{
    capsule_protocol::Mode,
    evidence::execution::SemanticMethod,
    native_semantics::{Consumer, Conversation, Input, PositionEncoding, Scope},
    native_union::{Cell, NativeStruct},
};
use std::sync::Arc;

enrichment_core::native_struct! { struct Capture {
    snapshot_id: enrichment_core::identity::SnapshotId => enrichment_core::native_union::Rule::Text,
    environment_id: enrichment_core::identity::EnvironmentId => enrichment_core::native_union::Rule::Text,
    context_id: enrichment_core::identity::ContextId => enrichment_core::native_union::Rule::Text,
} }

/// Private construction prevents an arbitrary string or cached plan from becoming permission.
pub struct Prepared {
    parent: Grant,
    scope: Scope,
    dependencies: Vec<Dependency>,
    protection: Arc<LeaseGuard>,
}
impl Prepared {
    pub fn consumer(&self) -> &Consumer {
        &self.scope.consumer
    }
}

/// Read-only retained-result selection uses the same native constructor as fresh effects.
pub async fn consumer(
    runtime: &crate::runtime::QueryRuntime,
    symbol: &enrichment_core::evidence::SymbolHeader,
    ecosystem: enrichment_core::identity::Ecosystem,
    options: &enrichment_core::request::InspectionOptions,
) -> Result<Consumer> {
    let input = Input {
        path: symbol.path.clone(),
        name: symbol.name.clone(),
        ecosystem,
        options: options.clone(),
    };
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "semantic_grants",
        Input::batch(&[input])?,
    )?;
    let input = enrichment_core::evidence::arrow_model::expressions::record(
        &Input::data_type(),
        &Input::fields()
            .iter()
            .map(|field| (field.name().as_str(), col(field.name())))
            .collect::<Vec<_>>(),
    )?;
    let frame = frame.select(vec![
        enrichment_core::native_semantics::consumer()
            .call(vec![input])
            .alias("consumer"),
    ])?;
    let frame = frame.select(
        Consumer::fields()
            .iter()
            .map(|field| col("consumer").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("native consumer missing"))
}

/// Select the same single definition as inspection, then derive the exact document from the
/// retained request. Caller-supplied symbol IDs narrow this selection; they never broaden it.
pub async fn prepare(parent: Grant, reader: &SnapshotReader, symbol_id: &str) -> Result<Prepared> {
    parent.check().await?;
    let jobs = parent.store();
    let runtime = jobs.runtime();
    let dependencies = reader
        .manifest()
        .tables
        .iter()
        .map(crate::retention::dependency)
        .collect::<Vec<_>>();
    let protection = RetentionStore::new(parent.control(), runtime.clone())
        .enroll_rooted(
            parent.id().to_string(),
            ProtectionKind::Query,
            dependencies.clone(),
            &[reader.manifest().snapshot_id.to_string()],
        )
        .await?;
    let pin = jobs.pin().await?;
    let session = pin.session(runtime).await?;
    crate::native_catalog::work(
        &session,
        "semantic_headers",
        reader
            .session()
            .table("snapshot.domain.symbol_headers")
            .await?
            .into_view(),
    )?;
    crate::native_catalog::work(
        &session,
        "semantic_capture",
        crate::native_catalog::batch(
            &session,
            "semantic_grants",
            Capture::batch(&[Capture {
                snapshot_id: reader.manifest().snapshot_id.clone(),
                environment_id: reader.manifest().metadata.environment_id.clone(),
                context_id: reader.manifest().metadata.context_id.clone(),
            }])?,
        )?
        .into_view(),
    )?;
    for (alias, source) in [
        ("semantic_claims", "state.records.claims"),
        ("semantic_commands", "state.records.commands"),
    ] {
        crate::native_catalog::work(&session, alias, session.table(source).await?.into_view())?;
    }
    let selected = scope_plan(
        &session,
        parent.id(),
        symbol_id,
        jobs.config().limits.verification_input_bytes as u64,
    )
    .await?;
    let scope = runtime
        .records::<Scope>(selected, 1)
        .await?
        .pop()
        .ok_or_else(|| {
            invalid("semantic conversation requires one exact requested symbol and execution scope")
        })?;
    parent.check().await?;
    Ok(Prepared {
        parent,
        scope,
        dependencies,
        protection,
    })
}

async fn scope_plan(
    session: &datafusion::prelude::SessionContext,
    grant_id: &enrichment_core::identity::GrantId,
    symbol_id: &str,
    maximum_bytes: u64,
) -> Result<datafusion::dataframe::DataFrame> {
    let request = session.sql(r#"
        SELECT c.grant_id,t.environment_id,t.snapshot_id,c.ecosystem,
          d.arguments.inspect.request AS request
        FROM semantic_claims c JOIN semantic_commands d ON c.job_id=d.job_id CROSS JOIN semantic_capture t
        WHERE c.grant_id=$1 AND c.profile='build'
          AND c.snapshot_id=t.snapshot_id AND c.environment_id=t.environment_id
          AND d.arguments.inspect.request.context_id=t.context_id
          AND d.arguments.inspect.request.execution.intent IN ('execute_on_miss','rerun')
          AND d.arguments.inspect.request.execution.profile='build'
          AND d.arguments.inspect.request.execution.runtime IS NULL
    "#).await?.with_param_values(datafusion::common::ParamValues::List(vec![grant_id.parameter()]))?;
    crate::native_catalog::work(session, "semantic_request", request.into_view())?;
    let selected = session.sql(r#"
        WITH matching AS (
          SELECT h.*,r.grant_id,r.environment_id,r.snapshot_id,r.ecosystem,r.request,
             h.path=r.request.symbol_path AS exact_path
          FROM semantic_headers h CROSS JOIN semantic_request r
          WHERE (h.path=r.request.symbol_path OR
             (NOT contains(r.request.symbol_path,'::') AND NOT contains(r.request.symbol_path,'.')
              AND array_element(h.components,-1)=r.request.symbol_path))
            AND (r.request.definition_id IS NULL OR h.definition_id=r.request.definition_id)
        ), preferred AS (
          SELECT * FROM matching WHERE exact_path OR NOT EXISTS (SELECT 1 FROM matching WHERE exact_path)
        ), selected AS (
          SELECT * FROM preferred WHERE (SELECT count(DISTINCT definition_id) FROM preferred)=1
          ORDER BY is_reexport,path,symbol_id LIMIT 1
        ) SELECT * FROM selected WHERE symbol_id=$1
    "#).await?.with_param_values(vec![ScalarValue::from(symbol_id)])?;
    let input = enrichment_core::evidence::arrow_model::expressions::record(
        &Input::data_type(),
        &[
            ("path", col("path")),
            ("name", col("name")),
            ("ecosystem", col("ecosystem")),
            ("options", col("request").field("execution")),
        ],
    )?;
    let selected = selected
        .with_column(
            "consumer",
            enrichment_core::native_semantics::consumer().call(vec![input]),
        )?
        .filter(
            datafusion::functions::string::expr_fn::octet_length(col("consumer").field("text"))
                .lt_eq(lit(maximum_bytes)),
        )?
        .select(vec![
            col("grant_id"),
            col("request").field("context_id").alias("context_id"),
            col("snapshot_id"),
            col("environment_id"),
            col("symbol_id"),
            col("consumer"),
        ])?;
    Ok(selected)
}

pub struct ConversationGrant {
    process: ProcessGrant,
    id: enrichment_core::identity::SemanticConversationId,
    value: Conversation,
    _binding: enrichment_core::delta_reference::DeltaVersionRef,
    _protection: Arc<LeaseGuard>,
}
impl ConversationGrant {
    pub fn id(&self) -> &enrichment_core::identity::SemanticConversationId {
        &self.id
    }
    pub fn value(&self) -> &Conversation {
        &self.value
    }
    /// A method is a finite native vocabulary, and membership is checked under the current
    /// physical owner immediately before writing. No generic LSP method/params API is exposed.
    pub async fn check(&self, method: SemanticMethod) -> Result<()> {
        self.process.check().await?;
        let runtime = self.process.parent().store().runtime();
        let session = runtime.session();
        crate::native_catalog::work(
            &session,
            "conversation",
            crate::native_catalog::batch(
                &session,
                "semantic_grants",
                Conversation::batch(std::slice::from_ref(&self.value))?,
            )?
            .into_view(),
        )?;
        runtime.require_empty(session.sql("SELECT 'semantic_method_outside_grant' AS witness FROM conversation WHERE NOT array_has(scope.consumer.methods,$1)").await?.with_param_values(vec![ScalarValue::from(method.as_str())])?, "exact_semantic_method", "semantic_dispatch").await
    }
}

pub async fn admit(
    process: ProcessGrant,
    prepared: Prepared,
    encoding: PositionEncoding,
) -> Result<ConversationGrant> {
    prepared.parent.check().await?;
    process.check().await?;
    let runtime = process.parent().store().runtime();
    let session = runtime.session();
    runtime.require_empty(session.sql("SELECT 'semantic_process_scope_mismatch' AS witness WHERE $1<>$2 OR $3<>'language_server'").await?.with_param_values(datafusion::common::ParamValues::List(vec![
        prepared.parent.id().parameter(), process.parent().id().parameter(),
        ScalarValue::from(match process.operation().mode { Mode::LanguageServer=>"language_server",Mode::Command=>"command" }).into(),
    ]))?, "exact_semantic_process", "semantic_admission").await?;
    let value = Conversation {
        scope: prepared.scope,
        process_operation_id: process.witness().operation_id.clone(),
        process_effect_id: process.witness().effect_id.clone(),
        encoding,
        position: None,
    };
    let definitions = Definitions::<enrichment_core::identity::SemanticConversationId>::new(
        process.parent().control(),
        runtime.clone(),
    );
    let mut dependencies = prepared.dependencies;
    let binding = &process.witness().effect_binding;
    dependencies.push(
        Definitions::<enrichment_core::identity::ProcessEffectId>::dependency(
            &process.witness().effect_id,
            binding,
        ),
    );
    let input = crate::native_catalog::batch(
        &session,
        "semantic_grants",
        Conversation::batch(std::slice::from_ref(&value))?,
    )?;
    let position = enrichment_core::evidence::arrow_model::expressions::record(
        &enrichment_core::native_semantics::PositionInput::data_type(),
        &[
            ("text", col("scope").field("consumer").field("text")),
            ("position", col("scope").field("consumer").field("position")),
            ("encoding", col("encoding")),
        ],
    )?;
    let input = input.with_column(
        "position",
        enrichment_core::native_semantics::protocol_position().call(vec![position]),
    )?;
    let (id, binding) = definitions.retain_plan(input, dependencies).await?;
    let retained = definitions
        .read(&id, &binding)
        .await?
        .drop_columns(&["conversation_id"])?;
    let value = runtime
        .records(retained, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("retained conversation missing"))?;
    process.check().await?;
    Ok(ConversationGrant {
        process,
        id,
        value,
        _binding: binding,
        _protection: prepared.protection,
    })
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Execution(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{
        identity::Ecosystem,
        native_union::Rule,
        policy::ExecutionProfile,
        request::{Arguments, InspectRequest, InspectionIntent, InspectionOptions},
    };
    enrichment_core::native_struct! { struct ClaimInput {
        job_id: enrichment_core::identity::JobId=>Rule::Text,grant_id:enrichment_core::identity::GrantId=>Rule::Text,
        profile:ExecutionProfile=>Rule::Text,ecosystem:Ecosystem=>Rule::Text,
        environment_id:Option<enrichment_core::identity::EnvironmentId> =>Rule::Text,
        snapshot_id:Option<enrichment_core::identity::SnapshotId> =>Rule::Text,
    } }
    enrichment_core::native_struct! { struct CommandInput { job_id: enrichment_core::identity::JobId=>Rule::Text,arguments:Arguments=>Rule::Text } }

    #[tokio::test]
    async fn plan19_semantic_scope_comes_from_exact_command_and_selected_symbol() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let capture = Capture {
            snapshot_id: format!("snap_{}", "a".repeat(64)).try_into().unwrap(),
            environment_id: format!("env_{}", "b".repeat(64)).try_into().unwrap(),
            context_id: format!("ctx_{}", "c".repeat(64)).try_into().unwrap(),
        };
        let symbol = enrichment_core::evidence::SymbolHeader {
            symbol_id: format!("sym_{}", "d".repeat(64)),
            definition_id: format!("def_{}", "e".repeat(64)),
            path: "fixture.Widget".into(),
            name: "Widget".into(),
            kind: enrichment_core::evidence::SymbolKind::Class,
            parent_path: Some("fixture".into()),
            is_reexport: false,
            definition_path: "fixture.Widget".into(),
            defined_in_package: "fixture".into(),
            qualifier: None,
        };
        for variant in 0..9 {
            let session = runtime.session();
            let mut claim = ClaimInput {
                job_id: enrichment_core::identity::JobId::try_from(
                    "job_00112233445566778899aabbccddeeff".to_owned(),
                )
                .unwrap(),
                grant_id: format!("grant_{}", "1".repeat(64)).try_into().unwrap(),
                profile: ExecutionProfile::Build,
                ecosystem: Ecosystem::Python,
                environment_id: Some(capture.environment_id.clone()),
                snapshot_id: Some(capture.snapshot_id.clone()),
            };
            let mut request = InspectRequest {
                context_id: capture.context_id.clone(),
                snapshot_id: Some(capture.snapshot_id.clone()),
                symbol_path: "Widget".into(),
                definition_id: None,
                selection: Default::default(),
                max_bytes: None,
                execution: Some(InspectionOptions {
                    intent: InspectionIntent::ExecuteOnMiss,
                    profile: Some(ExecutionProfile::Build),
                    ..Default::default()
                }),
            };
            match variant {
                1 => claim.grant_id = format!("grant_{}", "2".repeat(64)).try_into().unwrap(),
                2 => {
                    claim.snapshot_id = Some(format!("snap_{}", "f".repeat(64)).try_into().unwrap())
                }
                3 => claim.profile = ExecutionProfile::Runtime,
                4 => request.execution.as_mut().unwrap().intent = InspectionIntent::Retained,
                5 => request.symbol_path = "Missing".into(),
                6 => {
                    request.execution.as_mut().unwrap().methods = vec![SemanticMethod::Diagnostics]
                }
                7 => request.execution.as_mut().unwrap().snippet = Some("Widget + Widget".into()),
                _ => {}
            }
            for (name, batch) in [
                (
                    "semantic_capture",
                    Capture::batch(std::slice::from_ref(&capture))?,
                ),
                ("semantic_claims", ClaimInput::batch(&[claim])?),
                (
                    "semantic_commands",
                    CommandInput::batch(&[CommandInput {
                        job_id: enrichment_core::identity::JobId::try_from(
                            "job_00112233445566778899aabbccddeeff".to_owned(),
                        )
                        .unwrap(),
                        arguments: Arguments::Inspect { request },
                    }])?,
                ),
            ] {
                crate::native_catalog::work(
                    &session,
                    name,
                    crate::native_catalog::batch(&session, "semantic_grants", batch)?.into_view(),
                )?;
            }
            let headers = crate::native_catalog::batch(
                &session,
                "semantic_grants",
                enrichment_core::evidence::SymbolHeader::batch(std::slice::from_ref(&symbol))?,
            )?
            .with_column(
                "components",
                datafusion::functions_nested::expr_fn::string_to_array(
                    col("path"),
                    lit("."),
                    lit(ScalarValue::Utf8(None)),
                ),
            )?;
            crate::native_catalog::work(&session, "semantic_headers", headers.into_view())?;
            let plan = scope_plan(
                &session,
                &format!("grant_{}", "1".repeat(64)).try_into().unwrap(),
                &symbol.symbol_id,
                if variant == 8 { 1 } else { 32768 },
            )
            .await?;
            let selected = runtime.records::<Scope>(plan, 1).await;
            if variant == 7 {
                assert!(selected.is_err(), "ambiguous anchor must refuse");
                continue;
            }
            let selected = selected?;
            if matches!(variant, 0 | 6) {
                assert_eq!(selected.len(), 1);
                let consumer = &selected[0].consumer;
                assert_eq!(consumer.text, "import fixture\nfixture.Widget\n");
                assert_eq!(consumer.uri, "file:///capsule/consumer.py");
                if variant == 6 {
                    assert_eq!(consumer.methods, vec![SemanticMethod::Diagnostics]);
                    assert!(consumer.position.is_none());
                } else {
                    assert_eq!(
                        consumer.position,
                        Some(enrichment_core::evidence::execution::Utf8Position {
                            line: 1,
                            byte: 8
                        })
                    );
                }
            } else {
                assert!(selected.is_empty(), "out-of-scope variant {variant}");
            }
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
