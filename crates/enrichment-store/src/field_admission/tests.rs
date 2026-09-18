use super::*;
use crate::{
    native_catalog::{self, BindingKind, BoundCatalog, Tables},
    runtime::QueryRuntime,
};
use arrow::record_batch::RecordBatch;
use enrichment_core::{identity::ReleaseId, native_union::NativeStruct};
use std::{collections::BTreeMap, sync::Arc};

#[tokio::test]
async fn union_identity_projection_keeps_binary_domain_and_ignores_inactive_children() -> Result<()>
{
    use arrow::{
        array::{ArrayRef, StringArray, UnionArray},
        datatypes::UnionFields,
    };
    use enrichment_core::{
        identity::SnapshotId,
        native_union::{Cell, field},
    };
    enrichment_core::native_struct! { struct Retained {
        owner: String => Rule::Text,
        value: SnapshotId => Rule::Text,
    } }
    let id: SnapshotId = format!("snap_{}", "a".repeat(64)).try_into().unwrap();
    let branch = field::<Option<SnapshotId>>("snapshot", Rule::Text);
    let fields = UnionFields::try_new(
        vec![1, 7],
        vec![branch, Field::new("other", DataType::Utf8, true)],
    )?;
    let ids = SnapshotId::encode(&[Some(&id), None])?;
    let values: ArrayRef = Arc::new(UnionArray::try_new(
        fields,
        vec![1, 1, 7].into(),
        Some(vec![0, 1, 0].into()),
        vec![ids, Arc::new(StringArray::from(vec![id.to_string()]))],
    )?);
    let schema = Arc::new(Schema::new(vec![
        Field::new("owner", DataType::Utf8, false),
        Field::new("payload", values.data_type().clone(), true),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["selected", "absent", "decoy"])),
            values,
        ],
    )?;
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let frame = native_catalog::batch(&runtime.session(), "encoded_identities", batch)?;
    let selected = identity_values(frame, &schema, "owner", Domain::Snapshot)?
        .expect("declared identity branch");
    assert_eq!(
        runtime.records::<Retained>(selected, 3).await?,
        vec![Retained {
            owner: "selected".into(),
            value: id
        }]
    );
    runtime.close_diagnostics().await?;
    Ok(())
}

enrichment_core::native_struct! { struct Collections {
    owner: String => Rule::NonEmpty,
    values: Vec<Option<String>> => Rule::Set,
    sequence: Vec<Option<String>> => Rule::Sequence,
} }

#[tokio::test]
async fn sets_refuse_duplicates_without_changing_sequence_or_null_semantics() -> Result<()> {
    use arrow::array::StringArray;
    use enrichment_core::native_wire::Owned;
    use std::collections::BTreeSet;
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let batch = Collections::batch(&[
        Collections {
            owner: "ordered".into(),
            values: vec![Some("b".into()), Some("a".into()), None],
            sequence: vec![None, None],
        },
        Collections {
            owner: "repeated".into(),
            values: vec![Some("a".into()), Some("a".into())],
            sequence: vec![],
        },
        Collections {
            owner: "nulls".into(),
            values: vec![None, None],
            sequence: vec![],
        },
        Collections {
            owner: "empty".into(),
            values: vec![],
            sequence: vec![Some("a".into()), Some("a".into())],
        },
    ])?;
    let schema = batch.schema();
    let frame = native_catalog::batch(&runtime.session(), "collections", batch)?;
    let check =
        enrichment_core::native_schema::intrinsic_witness(frame, &schema, "owner")?.unwrap();
    let mut ids = Vec::new();
    for batch in runtime.execute(check).await?.batches {
        ids.extend(
            batch
                .column(0)
                .as_any()
                .downcast_ref::<StringArray>()
                .unwrap()
                .iter()
                .flatten()
                .map(str::to_owned),
        );
    }
    ids.sort();
    assert_eq!(ids, ["nulls", "repeated"]);
    assert!(serde_json::from_str::<Owned<BTreeSet<String>>>(r#"["a","a"]"#).is_err());
    assert_eq!(
        serde_json::from_str::<Owned<BTreeSet<String>>>(r#"["b","a"]"#)
            .unwrap()
            .0
            .len(),
        2
    );
    let wire = serde_json::to_value(schemars::schema_for!(Collections)).unwrap();
    assert_eq!(wire["properties"]["values"]["uniqueItems"], true);
    assert!(wire["properties"]["sequence"].get("uniqueItems").is_none());
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn encoded_containers_preserve_value_rules_references_and_inactive_presence() -> Result<()> {
    use arrow::{
        array::{
            Array, ArrayRef, DictionaryArray, Int8Array, Int32Array, LargeListViewArray,
            ListViewArray, RunArray, StringArray, StructArray, UnionArray,
        },
        datatypes::{Int8Type, Int32Type, UnionFields},
    };
    let member: Arc<Field> = Arc::new(
        Field::new("value", DataType::Utf8, true).with_metadata(
            [
                (
                    "enrichment.rule".into(),
                    serde_json::to_string(&Rule::ForeignKey {
                        table: "target".into(),
                        field: vec!["value".into()],
                        scope: vec![],
                    })
                    .expect("finite reference declaration"),
                ),
                ("enrichment.null".into(), "forbidden".into()),
            ]
            .into(),
        ),
    );
    let records: ArrayRef = Arc::new(StructArray::try_new(
        vec![member].into(),
        vec![Arc::new(StringArray::from(vec![
            Some("retained"),
            Some("dangling"),
            None,
            Some(""),
            Some("inactive"),
        ]))],
        Some(vec![true, true, true, true, false].into()),
    )?);
    let item = Arc::new(Field::new("item", records.data_type().clone(), true));
    let branches = UnionFields::try_new(
        vec![1, 7],
        vec![
            item.as_ref().clone().with_name("record"),
            Field::new("other", DataType::Utf8, true),
        ],
    )?;
    let containers: Vec<(&str, ArrayRef)> = vec![
        (
            "dictionary",
            Arc::new(DictionaryArray::<Int8Type>::try_new(
                Int8Array::from(vec![Some(0), Some(1), Some(2), Some(3), None]),
                records.clone(),
            )?),
        ),
        (
            "runs",
            Arc::new(RunArray::<Int32Type>::try_new(
                &Int32Array::from(vec![1, 2, 3, 4, 5]),
                records.as_ref(),
            )?),
        ),
        (
            "list_view",
            Arc::new(ListViewArray::try_new(
                item.clone(),
                vec![0, 1, 2, 3, 4].into(),
                vec![1, 1, 1, 1, 0].into(),
                records.clone(),
                None,
            )?),
        ),
        (
            "large_list_view",
            Arc::new(LargeListViewArray::try_new(
                item,
                vec![0i64, 1, 2, 3, 4].into(),
                vec![1i64, 1, 1, 1, 0].into(),
                records.clone(),
                None,
            )?),
        ),
        (
            "union",
            Arc::new(UnionArray::try_new(
                branches,
                vec![1, 1, 1, 1, 7].into(),
                Some(vec![0, 1, 2, 3, 0].into()),
                vec![records, Arc::new(StringArray::from(vec!["other"]))],
            )?),
        ),
    ];
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let release: ReleaseId = format!("rel_{}", "a".repeat(64)).try_into().unwrap();
    for (kind, values) in containers {
        let schema = Arc::new(Schema::new(vec![
            Field::new("owner", DataType::Utf8, false),
            Field::new("encoded", values.data_type().clone(), true),
        ]));
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(vec![
                    "good", "dangling", "missing", "empty", "inactive",
                ])),
                values,
            ],
        )?;
        let session = bind(
            &runtime,
            "source",
            batch,
            Target::batch(&[Target {
                value: "retained".into(),
                scope: None,
            }])?,
        )?;
        let source = TableReference::full("candidate", "evidence", "source");
        let frame = session.table(source.clone()).await?;
        let intrinsic = enrichment_core::native_schema::intrinsic_witness(frame, &schema, "owner")?
            .expect("value checks");
        let mut ids = Vec::new();
        for batch in runtime.execute(intrinsic).await?.batches {
            ids.extend(
                batch
                    .column(0)
                    .as_any()
                    .downcast_ref::<StringArray>()
                    .unwrap()
                    .iter()
                    .flatten()
                    .map(str::to_owned),
            );
        }
        ids.sort();
        assert_eq!(ids, ["empty", "missing"], "{kind} intrinsic rules");
        let checks = violations(
            &session,
            source,
            &schema,
            "owner",
            ReferenceNamespace::Evidence(&release),
        )
        .await?;
        assert_eq!(checks.len(), 1, "{kind} reference declaration");
        for (_, check) in checks {
            assert_eq!(
                runtime.execute(check).await?.rows,
                1,
                "{kind} reference membership"
            );
        }
    }
    runtime.close_diagnostics().await?;
    Ok(())
}

// The same new declaration must reach Arrow, admission, manifest and metadata discovery.
enrichment_core::native_struct! { struct Member {
    value: String => Rule::ForeignKey {
        table: "target".into(), field: vec!["value".into()],
        scope: vec![ScopeKey::exact(&["scope"], &["scope"])],
    },
    scope: Option<String> => Rule::Text,
} }
enrichment_core::native_struct! { struct Owner {
    owner: String => Rule::NonEmpty,
    members: Vec<Member> => Rule::Sequence,
    optional: Option<Member> => Rule::Text,
} }
enrichment_core::native_struct! { struct Target {
    value: String => Rule::NonEmpty,
    scope: Option<String> => Rule::Text,
} }
enrichment_core::native_struct! { struct Library {
    owner: String => Rule::NonEmpty,
    release: ReleaseId => Rule::Reference(Domain::Release),
} }

fn bind(
    runtime: &QueryRuntime,
    name: &str,
    table: arrow::record_batch::RecordBatch,
    target: arrow::record_batch::RecordBatch,
) -> Result<SessionContext> {
    let base = runtime.session();
    runtime.bound_session(BTreeMap::from([(
        "candidate".into(),
        Arc::new(BoundCatalog::default().with_schema(
            BindingKind::CandidateEvidence,
            Tables::from([
                (
                    name.into(),
                    native_catalog::batch(&base, name, table)?.into_view(),
                ),
                (
                    "target".into(),
                    native_catalog::batch(&base, "target", target)?.into_view(),
                ),
            ]),
        )) as Arc<dyn datafusion::catalog::CatalogProvider>,
    )]))
}

#[tokio::test]
async fn field_declared_scopes_optional_lists_and_typed_release_are_authoritative() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let release: ReleaseId = format!("rel_{}", "a".repeat(64)).try_into().unwrap();
    for (value, scope, optional, expected) in [
        ("retained", Some("scope-a"), false, 0),
        ("retained", Some("scope-b"), false, 1),
        ("missing", Some("scope-a"), false, 1),
        ("retained", None, false, 1),
        ("retained", Some("scope-a"), true, 0),
        ("missing", Some("scope-a"), true, 1),
    ] {
        let member = Member {
            value: value.into(),
            scope: scope.map(str::to_owned),
        };
        let owner = if optional {
            Owner {
                owner: "one".into(),
                members: vec![],
                optional: Some(member),
            }
        } else {
            Owner {
                owner: "one".into(),
                members: vec![member],
                optional: None,
            }
        };
        let batch = Owner::batch(&[owner])?;
        let schema = batch.schema();
        let session = bind(
            &runtime,
            "source",
            batch,
            Target::batch(&[Target {
                value: "retained".into(),
                scope: Some("scope-a".into()),
            }])?,
        )?;
        let plans = violations(
            &session,
            TableReference::full("candidate", "evidence", "source"),
            schema.as_ref(),
            "owner",
            ReferenceNamespace::Evidence(&release),
        )
        .await?;
        assert_eq!(plans.len(), 2);
        let mut count = 0;
        for (_, plan) in plans {
            count += runtime.execute(plan).await?.rows;
        }
        assert_eq!(
            count, expected,
            "value={value} scope={scope:?} optional={optional}"
        );
        let metadata = runtime.execute(session.sql("SELECT scope FROM operation.metadata.rules WHERE kind = 'scoped_reference'").await?).await?;
        assert_eq!(metadata.rows, 2);
    }
    for (suffix, expected) in [('a', 0), ('b', 1)] {
        let value = Library {
            owner: "one".into(),
            release: format!("rel_{}", suffix.to_string().repeat(64))
                .try_into()
                .unwrap(),
        };
        let batch = Library::batch(&[value])?;
        let schema = batch.schema();
        let session = bind(&runtime, "library", batch, Target::batch(&[])?)?;
        let checks = violations(
            &session,
            TableReference::full("candidate", "evidence", "library"),
            schema.as_ref(),
            "owner",
            ReferenceNamespace::Evidence(&release),
        )
        .await?;
        assert_eq!(checks.len(), 1);
        for (_, plan) in checks {
            assert_eq!(runtime.execute(plan).await?.rows, expected);
        }
    }
    runtime.close_diagnostics().await?;
    Ok(())
}

#[test]
fn all_control_reference_declarations_validate_and_enter_the_contract_witness() -> Result<()> {
    for table in crate::control::Table::ALL {
        enrichment_core::native_schema::validate(table.schema()?.as_ref())?;
    }
    let original = arrow::datatypes::Schema::new(Member::fields());
    let mut changed = Member::fields().to_vec();
    changed[0] = Arc::new(enrichment_core::native_union::field::<String>(
        "value",
        Rule::foreign_key("another_target", "value"),
    ));
    let changed = arrow::datatypes::Schema::new(changed);
    let witness = |schema: &Schema| enrichment_core::native_contract::Manifest::new(schema, schema);
    assert_ne!(witness(&original)?, witness(&changed)?);
    let mut invalid = Member::fields().to_vec();
    invalid[0] = Arc::new(enrichment_core::native_union::field::<String>(
        "value",
        Rule::ForeignKey {
            table: "target".into(),
            field: vec!["value".into()],
            scope: vec![ScopeKey::exact(&["missing_scope"], &["scope"])],
        },
    ));
    assert!(enrichment_core::native_schema::validate(&Schema::new(invalid)).is_err());
    Ok(())
}

#[tokio::test]
async fn native_field_contracts_reject_bad_coordinates_without_decoding_domain_rows() {
    use enrichment_core::evidence::arrow_model::{cells, checks, encode};
    use enrichment_core::{
        evidence::relational::{FactSource, Locator},
        wire::{EvidenceClass, SourceVersionMatch},
    };
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let cases = vec![
        (
            "valid_lines",
            Locator::Lines {
                file: Some("pkg/β.rs".into()),
                start: 1,
                end: 2,
            },
        ),
        (
            "zero",
            Locator::Lines {
                file: None,
                start: 0,
                end: 2,
            },
        ),
        ("reverse", Locator::Bytes { start: 2, end: 1 }),
        ("empty_bytes", Locator::Bytes { start: 0, end: 0 }),
        (
            "escape",
            Locator::ArchiveMember {
                path: "pkg/../file.rs".into(),
            },
        ),
        (
            "absolute",
            Locator::ArchiveMember {
                path: "/file.rs".into(),
            },
        ),
        (
            "backslash",
            Locator::ArchiveMember {
                path: "pkg\\file.rs".into(),
            },
        ),
        (
            "control",
            Locator::RustdocItem {
                item: 1,
                reported_file: Some("pkg\nfile".into()),
                reported_line: None,
            },
        ),
        (
            "valid_reported",
            Locator::RustdocItem {
                item: 1,
                reported_file: Some("/upstream/build/file.rs".into()),
                reported_line: None,
            },
        ),
        (
            "wrong_python_origin",
            Locator::PythonDeclaration {
                file: "pkg.py".into(),
                declaration: "pkg".into(),
                line: None,
                origin: enrichment_core::evidence::relational::ApiOrigin::Rustdoc,
                overload: None,
            },
        ),
    ];
    let sources = cases
        .iter()
        .map(|(_, locator)| FactSource {
            extractor: "fixture".into(),
            extractor_version: "1".into(),
            producer_binding_id: "producer_fixture".into(),
            artifact_id: "art_fixture".into(),
            source_uri: None,
            source_version_match: SourceVersionMatch::Exact,
            evidence_class: EvidenceClass::Declared,
            locator: locator.clone(),
        })
        .collect::<Vec<_>>();
    let batch = cells::batch(
        "coordinate_probe",
        vec![
            cells::column(
                "id",
                cells::text(cases.iter().map(|(id, _)| *id)),
                false,
                "probe",
            ),
            cells::column(
                "source",
                encode::source(&sources.iter().collect::<Vec<_>>()).unwrap(),
                false,
                "fact-source",
            ),
        ],
    )
    .unwrap();
    let schema = batch.schema();
    let frame = native_catalog::batch(&runtime.session(), "field_fixture", batch).unwrap();
    let mut observed = std::collections::BTreeSet::new();
    for (_, plan) in checks::violations(frame, &schema, "id").unwrap() {
        for batch in runtime.execute(plan).await.unwrap().batches {
            let text = cells::TextColumn::new(batch.column(0).as_ref()).unwrap();
            for row in 0..batch.num_rows() {
                observed.insert(text.required(row).unwrap().to_owned());
            }
        }
    }
    assert_eq!(
        observed,
        [
            "zero",
            "reverse",
            "escape",
            "absolute",
            "backslash",
            "control",
            "wrong_python_origin"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
}

#[tokio::test]
async fn nullable_nested_read_layout_still_enforces_required_semantic_fields() {
    use arrow::array::{Array, StringArray, StructArray};
    use enrichment_core::evidence::arrow_model::{cells, checks};
    let root = tempfile::tempdir().unwrap();
    let runtime = QueryRuntime::new(root.path(), Default::default()).unwrap();
    let batch = cells::batch(
        "presence_probe",
        vec![
            cells::column("id", cells::text(["valid", "missing"]), false, "probe"),
            cells::column(
                "source",
                cells::structure(
                    vec![cells::column(
                        "extractor",
                        cells::text(["extractor", "extractor"]),
                        false,
                        "extractor",
                    )],
                    None,
                )
                .unwrap(),
                false,
                "source",
            ),
        ],
    )
    .unwrap();
    let values = batch
        .column(1)
        .as_any()
        .downcast_ref::<StructArray>()
        .unwrap();
    assert!(values.fields()[0].is_nullable());
    assert!(checks::required(&values.fields()[0]));
    let corrupted = Arc::new(
        StructArray::try_new(
            values.fields().clone(),
            vec![Arc::new(StringArray::from(vec![Some("extractor"), None]))],
            values.nulls().cloned(),
        )
        .unwrap(),
    );
    let batch =
        RecordBatch::try_new(batch.schema(), vec![batch.column(0).clone(), corrupted]).unwrap();
    let schema = batch.schema();
    let violations = checks::violations(
        native_catalog::batch(&runtime.session(), "field_fixture", batch).unwrap(),
        &schema,
        "id",
    )
    .unwrap();
    assert_eq!(violations.len(), 1);
    let output = runtime.execute(violations[0].1.clone()).await.unwrap();
    assert_eq!(output.rows, 1);
    assert_eq!(
        cells::TextColumn::new(output.batches[0].column(0).as_ref())
            .unwrap()
            .required(0)
            .unwrap(),
        "missing"
    );
}

#[tokio::test]
async fn intrinsic_rules_ignore_presentation_roles_and_preserve_all_violation_ids() -> Result<()> {
    use arrow::{
        array::{ArrayRef, StringArray},
        datatypes::Field,
    };
    use enrichment_core::{
        evidence::arrow_model::checks, identity::Ecosystem, native_union::field,
    };
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let mut ecosystem = field::<Ecosystem>("ecosystem", Rule::Text).with_nullable(true);
    ecosystem
        .metadata_mut()
        .insert("enrichment.role".into(), "vocabulary:not-a-rule".into());
    let mut description = field::<String>("description", Rule::Text);
    description
        .metadata_mut()
        .insert("enrichment.role".into(), "sha256".into());
    let schema = Arc::new(Schema::new(vec![
        Field::new("row.id", DataType::Utf8, false),
        ecosystem,
        description,
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["valid", "unknown", "absent"])) as ArrayRef,
            Arc::new(StringArray::from(vec![
                Some("rust"),
                Some("unregistered"),
                None,
            ])),
            Arc::new(StringArray::from(vec!["", "", ""])),
        ],
    )?;
    let frame = native_catalog::batch(&runtime.session(), "declared_fields", batch)?;
    let plans = checks::violations(frame, schema.as_ref(), "row.id")?;
    assert_eq!(plans.len(), 1);
    let mut ids = Vec::new();
    for (_, plan) in plans {
        for batch in runtime.execute(plan).await?.batches {
            let text =
                enrichment_core::evidence::arrow_model::TextColumn::new(batch.column(0).as_ref())?;
            ids.extend((0..batch.num_rows()).map(|row| text.required(row).unwrap().to_owned()));
        }
    }
    ids.sort();
    assert_eq!(ids, ["absent", "unknown"]);
    runtime.close_diagnostics().await?;
    Ok(())
}
