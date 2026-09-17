//! Isolated value/codec units. No daemon, filesystem store, producer or MCP journey.
use arrow::{array::AsArray, datatypes::DataType};
use enrichment_core::{
    identity::{
        Context, ContextId, Ecosystem, Environment, Release, ReleaseId, ReleaseKey, ResearchMode,
    },
    native_union::{Cell, NativeStruct, Rule},
};

enrichment_core::native_struct! {
    struct SignedValues {
        value: i64 => Rule::Text,
        optional: Option<i64> => Rule::Text,
    }
}
enrichment_core::native_struct! {
    struct Numbers {
        unsigned: u64 => Rule::Text,
        optional: Option<u64> => Rule::Text,
        sequence: Vec<u64> => Rule::Sequence,
        bounded: u32 => Rule::Text,
    }
}

#[test]
fn signed_boundaries_remain_exact_across_both_wire_encoders() {
    for value in [i64::MIN, -(1i64 << 53), -1, 0, 1, 1i64 << 53, i64::MAX] {
        let values = SignedValues {
            value,
            optional: Some(value),
        };
        let wire = serde_json::to_value(&values).unwrap();
        assert_eq!(wire["value"], value.to_string());
        assert_eq!(
            serde_json::from_value::<SignedValues>(wire.clone()).unwrap(),
            values
        );
        let array = <SignedValues as Cell>::encode(&[Some(&values)]).unwrap();
        let field = std::sync::Arc::new(enrichment_core::native_union::field::<SignedValues>(
            "signed",
            Rule::Text,
        ));
        let mut bytes = Vec::new();
        enrichment_core::native_json::write_value(&mut bytes, 512, &field, array.as_ref(), 0)
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            wire
        );
    }
    for value in [
        serde_json::json!(0),
        serde_json::json!(i64::MIN),
        serde_json::json!("-0"),
        serde_json::json!("01"),
        serde_json::json!("+1"),
        serde_json::json!("9223372036854775808"),
        serde_json::json!("-9223372036854775809"),
    ] {
        assert!(
            serde_json::from_value::<SignedValues>(
                serde_json::json!({"value": value, "optional": null})
            )
            .is_err()
        );
    }
    let schema =
        enrichment_core::native_wire::schema(schemars::schema_for!(SignedValues).to_value());
    assert_eq!(schema["properties"]["value"]["format"], "decimal-int64");
    let pattern = schema["properties"]["value"]["pattern"].as_str().unwrap();
    use datafusion::{
        common::{DFSchema, ScalarValue},
        logical_expr::{Expr, simplify::SimplifyContext},
        optimizer::simplify_expressions::ExprSimplifier,
        prelude::lit,
    };
    let context = SimplifyContext::builder()
        .with_schema(std::sync::Arc::new(DFSchema::empty()))
        .build();
    let simplifier = ExprSimplifier::new(context);
    for (value, accepted) in [
        ("-9223372036854775808", true),
        ("9223372036854775807", true),
        ("0", true),
        ("-1", true),
        ("9223372036854775808", false),
        ("-9223372036854775809", false),
        ("-0", false),
        ("01", false),
    ] {
        let expression =
            datafusion::functions::regex::expr_fn::regexp_like(lit(value), lit(pattern), None);
        assert!(
            matches!(simplifier.simplify(expression).unwrap(), Expr::Literal(ScalarValue::Boolean(Some(found)), _) if found == accepted),
            "{value}"
        );
    }
}

#[test]
fn inventory_identities_bind_modes_sizes_and_names_with_declared_order() {
    use enrichment_core::{
        capsule_protocol::inventory::{self, Entry},
        native_key::Key,
        operation::identities::{PhysicalFile, PhysicalInventory},
    };
    let mut files = inventory::Inventory::from([(
        "artifact".into(),
        Entry::File {
            mode: 0o644,
            bytes: u64::MAX,
            sha256: "a".repeat(64),
        },
    )]);
    let digest = inventory::digest(&files).unwrap();
    files.insert(
        "artifact".into(),
        Entry::File {
            mode: 0o444,
            bytes: u64::MAX,
            sha256: "a".repeat(64),
        },
    );
    assert_ne!(digest, inventory::digest(&files).unwrap());
    let file = PhysicalFile {
        path: "artifact".into(),
        device: 2,
        inode: 3,
        mode: 0o644,
        bytes: u64::MAX,
        modified_seconds: i64::MIN,
        modified_nanoseconds: 9,
        changed_seconds: i64::MAX,
        changed_nanoseconds: 8,
    };
    let mut other = file.clone();
    other.path = "second".into();
    let first = Key::PhysicalInventory
        .hex_digest(&PhysicalInventory {
            files: vec![file.clone(), other.clone()],
        })
        .unwrap();
    assert_eq!(
        first,
        Key::PhysicalInventory
            .hex_digest(&PhysicalInventory {
                files: vec![other.clone(), file.clone()]
            })
            .unwrap()
    );
    other.inode += 1;
    assert_ne!(
        first,
        Key::PhysicalInventory
            .hex_digest(&PhysicalInventory {
                files: vec![file, other]
            })
            .unwrap()
    );
}

fn release() -> Release {
    Release::new(ReleaseKey {
        ecosystem: Ecosystem::Rust,
        registry: "crates.io".into(),
        package: "native-value-unit".into(),
        version: "1.0.0".into(),
        artifact_digest: None,
    })
}

#[test]
fn core_identities_are_binary_in_records_and_domain_tagged_on_the_wire() {
    let release = release();
    let batch = Release::batch(std::slice::from_ref(&release)).unwrap();
    let field = batch
        .schema()
        .field_with_name("release_id")
        .unwrap()
        .clone();
    assert_eq!(field.data_type(), &DataType::FixedSizeBinary(32));
    assert_eq!(
        batch
            .column_by_name("release_id")
            .unwrap()
            .as_fixed_size_binary()
            .value(0),
        release.release_id.as_bytes()
    );
    assert_eq!(std::mem::size_of::<ReleaseId>(), 32);
    assert!(field.metadata()["ARROW:extension:metadata"].contains("release"));
    let wire = serde_json::to_value(&release).unwrap();
    assert_eq!(wire["release_id"], release.release_id.to_string());
    assert_eq!(serde_json::from_value::<Release>(wire).unwrap(), release);
    assert!(ContextId::try_from(release.release_id.to_string()).is_err());
}

#[test]
fn native_context_roundtrip_keeps_domains_and_environment_knowledge() {
    let release = release();
    let unknown = Environment::unspecified();
    let empty = Environment::declared(None, Some(vec![]), None);
    assert_ne!(unknown.environment_id, empty.environment_id);
    let context = Context::new(
        release.release_id,
        unknown.environment_id,
        ResearchMode::Upstream,
    );
    let batch = Context::batch(std::slice::from_ref(&context)).unwrap();
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
    assert_eq!(
        <Context as NativeStruct>::decode(rows.row(0)).unwrap(),
        context
    );
    let schema = batch.schema();
    assert!(
        enrichment_core::native_analysis::compatible(
            schema.field_with_name("context_id").unwrap(),
            schema.field_with_name("release_id").unwrap(),
            "identity equality"
        )
        .is_err()
    );
}

#[test]
fn exact_unsigned_values_have_one_shape_in_serde_arrow_and_schema() {
    for value in [0, 1, (1_u64 << 53) - 1, 1_u64 << 53, u64::MAX] {
        let numbers = Numbers {
            unsigned: value,
            optional: Some(value),
            sequence: vec![value],
            bounded: u32::MAX,
        };
        let serde = serde_json::to_value(&numbers).unwrap();
        assert_eq!(serde["unsigned"], value.to_string());
        assert_eq!(serde["optional"], value.to_string());
        assert_eq!(serde["sequence"][0], value.to_string());
        assert_eq!(serde["bounded"], u32::MAX);
        assert_eq!(
            serde_json::from_value::<Numbers>(serde.clone()).unwrap(),
            numbers
        );
        let array = <Numbers as Cell>::encode(&[Some(&numbers)]).unwrap();
        let field = std::sync::Arc::new(enrichment_core::native_union::field::<Numbers>(
            "numbers",
            Rule::Text,
        ));
        let mut bytes = Vec::new();
        enrichment_core::native_json::write_value(&mut bytes, 1024, &field, array.as_ref(), 0)
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            serde
        );
    }
    let schema = enrichment_core::native_wire::schema(schemars::schema_for!(Numbers).to_value());
    assert_eq!(schema["properties"]["unsigned"]["type"], "string");
    assert_eq!(schema["properties"]["bounded"]["type"], "integer");
}

#[test]
fn unsafe_or_noncanonical_unsigned_ingress_is_rejected() {
    for value in [
        serde_json::json!(0),
        serde_json::json!(1),
        serde_json::json!(u64::MAX),
        serde_json::json!("18446744073709551616"),
        serde_json::json!("01"),
        serde_json::json!("+1"),
        serde_json::json!("-1"),
    ] {
        let encoded =
            serde_json::json!({"unsigned":value,"optional":null,"sequence":[],"bounded":0});
        assert!(serde_json::from_value::<Numbers>(encoded).is_err());
    }
}

#[test]
fn operator_configuration_projects_exact_integer_widths_at_its_boundary() {
    let config: enrichment_core::config::Config = enrichment_core::config::parse(
        "[arrow]\nmemory_bytes=34359738368\nspill_bytes=68719476736\n",
    )
    .unwrap();
    assert_eq!(config.arrow.memory_bytes, 32 * 1024 * 1024 * 1024);
    assert_eq!(config.arrow.spill_bytes, 64 * 1024 * 1024 * 1024);
    let value = serde_json::to_value(&config).unwrap();
    assert_eq!(value["arrow"]["memory_bytes"], "34359738368");
}

#[test]
fn native_field_bounds_are_projected_into_the_request_schema() {
    let schema = enrichment_core::native_wire::schema(
        schemars::schema_for!(enrichment_core::wire::research::AspectSelection).to_value(),
    );
    let pattern = schema["properties"]["max_items"]["pattern"]
        .as_str()
        .unwrap();
    use datafusion::{
        common::{DFSchema, ScalarValue},
        logical_expr::{Expr, simplify::SimplifyContext},
        optimizer::simplify_expressions::ExprSimplifier,
        prelude::lit,
    };
    let context = SimplifyContext::builder()
        .with_schema(std::sync::Arc::new(DFSchema::empty()))
        .build();
    let matches = |value: &str| {
        let expr =
            datafusion::functions::regex::expr_fn::regexp_like(lit(value), lit(pattern), None);
        matches!(
            ExprSimplifier::new(context.clone()).simplify(expr).unwrap(),
            Expr::Literal(ScalarValue::Boolean(Some(true)), _)
        )
    };
    for value in ["1", "32", "1024"] {
        assert!(matches(value));
    }
    for value in ["0", "1025", "9999", "01", "18446744073709551615"] {
        assert!(!matches(value));
    }
}

#[test]
fn exact_encoding_obeys_the_encoded_byte_limit() {
    let numbers = Numbers {
        unsigned: u64::MAX,
        optional: None,
        sequence: vec![0],
        bounded: 1,
    };
    let array = <Numbers as Cell>::encode(&[Some(&numbers)]).unwrap();
    let field = std::sync::Arc::new(enrichment_core::native_union::field::<Numbers>(
        "numbers",
        Rule::Text,
    ));
    let expected = serde_json::to_vec(&numbers).unwrap();
    let mut exact = Vec::new();
    assert_eq!(
        enrichment_core::native_json::write_value(
            &mut exact,
            expected.len(),
            &field,
            array.as_ref(),
            0
        )
        .unwrap(),
        expected.len()
    );
    let mut bounded = Vec::new();
    assert!(
        enrichment_core::native_json::write_value(
            &mut bounded,
            expected.len() - 1,
            &field,
            array.as_ref(),
            0
        )
        .is_err()
    );
    assert!(bounded.len() < expected.len());
}

#[tokio::test]
async fn native_parameters_preserve_domains_before_shared_analyzer_coercion() {
    use datafusion::{
        common::ParamValues, execution::session_state::SessionStateBuilder, prelude::SessionContext,
    };
    let template = SessionContext::new().state();
    let mut rules = template.analyzer().rules.clone();
    rules.insert(
        0,
        std::sync::Arc::new(enrichment_core::native_analysis::SemanticAnalyzer),
    );
    let state = SessionStateBuilder::from(template)
        .with_analyzer_rules(rules)
        .with_extension_type_registry(enrichment_core::native_types::registry().unwrap())
        .build();
    let session = SessionContext::new_with_state(state);
    let release = release();
    let context = Context::new(
        release.release_id.clone(),
        Environment::unspecified().environment_id,
        ResearchMode::Upstream,
    );
    session
        .register_batch(
            "contexts",
            Context::batch(std::slice::from_ref(&context)).unwrap(),
        )
        .unwrap();
    let query = session
        .sql("SELECT * FROM contexts WHERE context_id=$1")
        .await
        .unwrap();
    let selected = query
        .clone()
        .with_param_values(ParamValues::List(vec![context.context_id.parameter()]))
        .unwrap();
    assert_eq!(
        selected
            .collect()
            .await
            .unwrap()
            .iter()
            .map(|batch| batch.num_rows())
            .sum::<usize>(),
        1
    );
    let incompatible =
        query.with_param_values(ParamValues::List(vec![release.release_id.parameter()]));
    if let Ok(frame) = incompatible {
        assert!(frame.collect().await.is_err());
    }
    let key = enrichment_core::native_key::Key::Context;
    let derived = key
        .identity_expression(
            key.schema()
                .fields()
                .iter()
                .map(|field| datafusion::prelude::col(field.name()))
                .collect(),
        )
        .unwrap();
    let invalid = session
        .table("contexts")
        .await
        .unwrap()
        .filter(datafusion::prelude::col("context_id").not_eq(derived))
        .unwrap();
    assert_eq!(
        invalid
            .collect()
            .await
            .unwrap()
            .iter()
            .map(|batch| batch.num_rows())
            .sum::<usize>(),
        0
    );
    let frame = session.table("contexts").await.unwrap();
    let displayed = enrichment_core::native_id::diagnostic(
        frame
            .schema()
            .field_with_unqualified_name("context_id")
            .unwrap(),
        datafusion::prelude::col("context_id"),
    )
    .unwrap();
    let batches = frame
        .select(vec![displayed.alias("witness")])
        .unwrap()
        .collect()
        .await
        .unwrap();
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batches[0]).unwrap();
    assert_eq!(
        rows.row(0).text("witness").unwrap(),
        context.context_id.to_string()
    );
}

#[test]
fn finite_operations_share_rpc_durable_and_transport_declarations() {
    use enrichment_core::{
        operation::{Definition, Durability},
        request::{ResearchRequest, operation_definitions},
    };
    let definitions = operation_definitions();
    assert_eq!(definitions.len(), 10);
    assert_eq!(
        definitions.iter().filter(|value| value.published).count(),
        9
    );
    assert_eq!(
        definitions
            .iter()
            .filter(|value| value.durability == Durability::Durable)
            .count(),
        4
    );
    assert_eq!(Definition::batch(&definitions).unwrap().num_rows(), 10);
    let input = ResearchRequest::from_rpc(
        "library.overview",
        serde_json::json!({
            "context_id": format!("ctx_{}", "b".repeat(64)), "max_bytes": "18446744073709551615"
        }),
    )
    .unwrap()
    .unwrap();
    assert_eq!(input.operation().rpc(), "library.overview");
    assert_eq!(input.requested_budget(), Some(usize::MAX));
    let schema = enrichment_core::request::request_schema();
    assert_eq!(
        schema["$defs"]["OverviewRequest"]["properties"]["max_bytes"]["format"],
        "decimal-uint64"
    );
}

#[test]
fn native_cursor_bindings_reject_changes_without_json_identity_hashing() {
    use enrichment_core::{
        identity::{SnapshotId, SnapshotInputs},
        search::{
            page::{SearchCursor, SearchKey, SearchScope},
            row_page::RowCursor,
        },
    };
    let context = Context::new(
        release().release_id,
        Environment::unspecified().environment_id,
        ResearchMode::Upstream,
    );
    let snapshot = SnapshotId::derive(&SnapshotInputs {
        schema_version: "10".into(),
        normalizer_version: "unit".into(),
        context_id: context.context_id.clone(),
        input_digests: Default::default(),
        producers: Default::default(),
    });
    let row = RowCursor::encode(&snapshot, "selection-a", "row-a".into()).unwrap();
    assert_eq!(
        RowCursor::decode(&row, &snapshot, "selection-a")
            .unwrap()
            .after,
        "row-a"
    );
    assert!(RowCursor::decode(&row, &snapshot, "selection-b").is_err());
    let scope = SearchScope {
        context_id: context.context_id,
        snapshot_id: snapshot,
    };
    let cursor = SearchCursor::new(
        scope.clone(),
        "query-a".into(),
        u64::MAX,
        SearchKey {
            score: 1,
            hit_order: 0,
            subject: "pkg::f".into(),
            candidate_id: "candidate-a".into(),
        },
    );
    let encoded = cursor.encode().unwrap();
    assert_eq!(
        SearchCursor::decode(&encoded, &scope, "query-a")
            .unwrap()
            .returned_before,
        u64::MAX
    );
    assert!(SearchCursor::decode(&encoded, &scope, "query-b").is_err());
}

#[tokio::test]
async fn contract_changes_report_paths_rules_layout_and_codec_independently() {
    use arrow::datatypes::{Field, Schema};
    use datafusion::prelude::SessionContext;
    use enrichment_core::native_contract::{Change, Manifest, changes};
    let field = Field::new("literal.dot", DataType::UInt64, false);
    let semantic = Schema::new(vec![field.clone()]);
    let storage = Schema::new(vec![Field::new(
        "literal.dot",
        DataType::Decimal128(20, 0),
        false,
    )]);
    let before = Manifest::new(&semantic, &storage).unwrap();
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("enrichment.role".into(), "cosmetic".into());
    let cosmetic = Manifest::new(
        &Schema::new(vec![field.clone().with_metadata(metadata)]),
        &storage,
    )
    .unwrap();
    assert_eq!(before.identity().unwrap(), cosmetic.identity().unwrap());
    let after = Manifest::new(
        &Schema::new(vec![
            field.with_nullable(true),
            Field::new("added", DataType::Utf8, true),
        ]),
        &storage,
    )
    .unwrap();
    assert_ne!(before.identity().unwrap(), after.identity().unwrap());
    let session = SessionContext::new();
    let batches = changes(&session, &before, &after)
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    let mut observed = vec![];
    for batch in batches {
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        for index in 0..batch.num_rows() {
            observed.push(<Change as NativeStruct>::decode(rows.row(index)).unwrap());
        }
    }
    assert_eq!(observed.len(), 2);
    assert_eq!(observed[0].path, vec!["added"]);
    assert_eq!(observed[0].kind, "added");
    assert_eq!(observed[1].path, vec!["literal.dot"]);
    assert_eq!(observed[1].kind, "changed");
    assert_eq!(
        observed[1].before.as_ref().unwrap().properties["nullable"],
        "false"
    );
    let mut codec = before.clone();
    codec.fields[0]
        .properties
        .insert("wire".into(), "new-codec".into());
    let session = SessionContext::new();
    let batches = changes(&session, &before, &codec)
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();
    assert_eq!(batches.iter().map(|b| b.num_rows()).sum::<usize>(), 1);
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batches[0]).unwrap();
    let change = <Change as NativeStruct>::decode(rows.row(0)).unwrap();
    assert_eq!(change.projection, "contract");
    assert!(change.path.is_empty());
}

#[test]
fn worker_and_provenance_records_share_exact_native_wire_values() {
    use enrichment_core::{
        evidence::{Artifact, ArtifactKind, relational::InputArtifact},
        producer::python::WorkerRequest,
    };
    let request = WorkerRequest {
        schema_version: "3".into(),
        root: "/owned/input".into(),
        files: vec![],
        max_observations: 100,
        max_memory_bytes: 32 * 1024 * 1024 * 1024,
        max_cpu_seconds: 60,
    };
    let wire = serde_json::to_value(&request).unwrap();
    assert_eq!(wire["max_memory_bytes"], "34359738368");
    assert_eq!(wire["max_observations"], "100");
    assert_eq!(
        serde_json::from_value::<WorkerRequest>(wire).unwrap(),
        request
    );
    let artifact = Artifact::describe(
        b"source",
        ArtifactKind::Other,
        "text/plain",
        "fixture://source",
        enrichment_core::native_time::AcquisitionTime::now().unwrap(),
    );
    let input = InputArtifact::new("producer_fixture".into(), "input".into(), &artifact).unwrap();
    let batch = InputArtifact::batch(std::slice::from_ref(&input)).unwrap();
    let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
    assert_eq!(
        <InputArtifact as NativeStruct>::decode(rows.row(0)).unwrap(),
        input
    );
    assert_eq!(serde_json::to_value(&input).unwrap()["size_bytes"], "6");
}

#[test]
fn generated_evidence_codecs_preserve_values_and_reject_corrupted_identities() {
    use enrichment_core::{
        evidence::{
            RelationKind, SymbolHeader, SymbolKind, arrow_model, execution::*, relational::*,
        },
        wire::{EvidenceClass, SourceVersionMatch},
    };
    let definition = Definition {
        definition_id: SymbolHeader::definition_id_for(
            "fixture",
            "fixture::f",
            SymbolKind::Function,
            None,
        ),
        kind: SymbolKind::Function,
        definition_path: "fixture::f".into(),
        defined_in_package: "fixture".into(),
        qualifier: None,
    };
    let encoded = arrow_model::definitions(std::slice::from_ref(&definition)).unwrap();
    assert_eq!(encoded.schema().fields(), &Definition::fields());
    assert_eq!(
        arrow_model::decode::definitions(&encoded.project(&[4, 2, 0, 3, 1]).unwrap()).unwrap(),
        [definition.clone()]
    );
    let mut changed = definition;
    changed.definition_path.push_str("_different");
    assert!(arrow_model::decode::definitions(&Definition::batch(&[changed]).unwrap()).is_err());
    let payload = ExecutionPayload::RuntimeObject(RuntimeObject {
        module: "fixture".into(),
        selection: vec![],
        outcome: ExecutionOutcome::Results,
        type_name: None,
        signature: None,
        docstring: Some(String::new()),
        attributes: vec![],
        limitations: vec![],
    });
    let source = FactSource {
        producer_binding_id: "producer".into(),
        extractor: "unit".into(),
        extractor_version: "1".into(),
        artifact_id: enrichment_core::evidence::artifact_id_for(
            &enrichment_core::canonical::sha256_hex(&payload.canonical_bytes().unwrap()),
        ),
        source_uri: None,
        source_version_match: SourceVersionMatch::Exact,
        locator: Locator::Artifact,
        evidence_class: EvidenceClass::RuntimeObserved,
    };
    let observation = ExecutionObservation::new(
        SubjectRef::Symbol {
            symbol_id: format!("sym_{}", "4".repeat(64)),
        },
        Environment::unspecified().environment_id,
        format!("sha256:{}", "1".repeat(64)),
        "2".repeat(64),
        payload,
        source.clone(),
    )
    .unwrap();
    let encoded = arrow_model::execution::encode(std::slice::from_ref(&observation)).unwrap();
    assert_eq!(encoded.schema().fields(), &ExecutionObservation::fields());
    assert_eq!(
        arrow_model::execution::decode(&encoded).unwrap(),
        [observation.clone()]
    );
    let mut changed = observation;
    changed.containment_identity = "3".repeat(64);
    assert!(
        arrow_model::execution::decode(&ExecutionObservation::batch(&[changed]).unwrap()).is_err()
    );
    let relationship = RelationshipObservation::new(
        SubjectRef::Symbol {
            symbol_id: "sym_source".into(),
        },
        TargetRef::External {
            package: Some("external".into()),
            path: "external::f".into(),
        },
        RelationKind::Reexports,
        None,
        source,
    )
    .unwrap();
    let encoded = arrow_model::relationships(std::slice::from_ref(&relationship)).unwrap();
    assert_eq!(
        encoded.schema().fields(),
        &RelationshipObservation::fields()
    );
    assert_eq!(
        arrow_model::relationships_from_batch(&encoded).unwrap(),
        [relationship.clone()]
    );
    let mut changed = relationship;
    changed.target = TargetRef::Unresolved {
        path: "unresolved".into(),
    };
    assert!(
        arrow_model::relationships_from_batch(&RelationshipObservation::batch(&[changed]).unwrap())
            .is_err()
    );
}

#[test]
fn operation_witness_binds_policy_values_and_is_independent_of_set_order() {
    use enrichment_core::{
        native_contract::Manifest, operation::Contract, request::Operation,
        wire::research::InspectionAspect,
    };
    let schema = arrow::datatypes::Schema::empty();
    let contract = Contract::new(
        Operation::Inspect.definition(),
        Manifest::new(&schema, &schema).unwrap(),
    );
    let expected = contract.identity().unwrap();
    let mut reordered = contract.clone();
    reordered.aspects.reverse();
    reordered.discovery.reverse();
    reordered.execution.reverse();
    assert_eq!(reordered.identity().unwrap(), expected);
    let mut changed = contract.clone();
    changed
        .aspects
        .iter_mut()
        .find(|d| d.aspect == InspectionAspect::Documentation)
        .unwrap()
        .default_max_characters = Some(17);
    assert_ne!(changed.identity().unwrap(), expected);
    let mut changed = contract.clone();
    changed.execution[0].evidence_kind = enrichment_core::evidence::EvidenceKind::UsageProbes;
    assert_ne!(changed.identity().unwrap(), expected);
    let mut changed = contract;
    changed.operation.response = enrichment_core::operation::ResponsePolicy::Status;
    assert_ne!(changed.identity().unwrap(), expected);
}

#[test]
fn checked_page_uses_one_exact_wire_and_arrow_decoder() {
    use enrichment_core::{
        native_union::NativeStruct,
        wire::{Page, research::MatchCount},
    };
    for count in [0, 1, u64::MAX] {
        let value = Page::new(count, Some(count), false, None);
        let json = serde_json::to_value(&value).unwrap();
        assert_eq!(json["returned"], count.to_string());
        assert_eq!(serde_json::from_value::<Page>(json).unwrap(), value);
        let batch = Page::batch(std::slice::from_ref(&value)).unwrap();
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert_eq!(<Page as NativeStruct>::decode(rows.row(0)).unwrap(), value);
    }
    for value in [
        Page {
            returned: 0,
            count: MatchCount::Unknown,
            has_more: true,
            next_cursor: Some("next".into()),
        },
        Page {
            returned: 2,
            count: MatchCount::Exact { value: 1 },
            has_more: false,
            next_cursor: None,
        },
        Page {
            returned: 1,
            count: MatchCount::Unknown,
            has_more: false,
            next_cursor: Some("next".into()),
        },
    ] {
        assert!(serde_json::from_value::<Page>(serde_json::to_value(&value).unwrap()).is_err());
        let batch = Page::batch(&[value]).unwrap();
        let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert!(<Page as NativeStruct>::decode(rows.row(0)).is_err());
    }
    let mut wire = serde_json::to_value(Page::default()).unwrap();
    wire["returned"] = serde_json::json!(0);
    assert!(
        serde_json::from_value::<Page>(wire).is_err(),
        "retired numeric representation must be refused"
    );
}
