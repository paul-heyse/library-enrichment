//! Storage journeys stay deferred; the explicitly named codec probe uses only upstream
//! Delta/Arrow APIs and temporary library fixtures, without service storage or authority.
use super::*;
use arrow::datatypes::{DataType, Field, Schema};

#[tokio::test]
async fn plan19_upstream_binary_provider_codec_probe() -> Result<()> {
    use datafusion_proto::logical_plan::LogicalExtensionCodec;
    use deltalake::{delta_datafusion::DeltaLogicalCodec, operations::create::CreateBuilder};
    let root = tempfile::tempdir()?;
    let session = deltalake::delta_datafusion::DeltaSessionContext::new().into_inner();
    let table = CreateBuilder::new()
        .with_location(root.path().to_str().unwrap())
        .with_column("value", deltalake::kernel::DataType::LONG, true, None)
        .await
        .map_err(|error| DataFusionError::External(Box::new(error)))?;
    let schema = Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Int64,
        true,
    )]));
    let batch = arrow::record_batch::RecordBatch::try_new(
        schema,
        vec![Arc::new(arrow::array::Int64Array::from(vec![1, 2, 3]))],
    )?;
    let table = table
        .write(vec![batch])
        .with_session_state(Arc::new(session.state()))
        .await
        .map_err(|error| DataFusionError::External(Box::new(error)))?;
    let config = deltalake::delta_datafusion::DeltaScanConfig::new_from_session(&session.state());
    let provider = table
        .table_provider()
        .with_eager_snapshot(
            table
                .snapshot()
                .map_err(|error| DataFusionError::External(Box::new(error)))?
                .snapshot()
                .clone(),
        )
        .with_scan_config(config.clone())
        .with_session(Arc::new(session.state()))
        .build()
        .await
        .map_err(|error| DataFusionError::External(Box::new(error)))?;
    let schema = provider.schema();
    let codec = DeltaLogicalCodec {};
    let mut bytes = Vec::new();
    codec.encode_immutable_provider(&provider, &mut bytes)?;
    assert_eq!(&bytes[..8], b"DFDELTA\x01");
    assert!(
        bytes.windows(6).any(|value| value == b"ARROW1"),
        "native materialization must be raw IPC bytes"
    );
    let decoded = codec.try_decode_table_provider(
        &bytes,
        &TableReference::bare("probe"),
        schema.clone(),
        &session.task_ctx(),
    )?;
    assert_eq!(decoded.schema(), schema);
    let scan = decoded
        .downcast_ref::<deltalake::delta_datafusion::DeltaScanNext>()
        .unwrap();
    assert_eq!(scan.snapshot().version(), table.version().unwrap());
    assert_eq!(
        scan.snapshot().metadata().id(),
        table
            .snapshot()
            .map_err(|error| DataFusionError::External(Box::new(error)))?
            .metadata()
            .id()
    );
    for payload in [
        &b"DFDELTA\x01\x83\x01\x02"[..],
        &b"DFDELTA\x01\x9f\x01\xff"[..],
        &bytes[..bytes.len() - 1],
    ] {
        assert!(
            codec
                .try_decode_table_provider(
                    payload,
                    &TableReference::bare("probe"),
                    schema.clone(),
                    &session.task_ctx()
                )
                .is_err()
        );
    }
    let mut trailing = bytes.clone();
    trailing.push(0);
    assert!(
        codec
            .try_decode_table_provider(
                &trailing,
                &TableReference::bare("probe"),
                schema.clone(),
                &session.task_ctx()
            )
            .is_err()
    );
    let mut corrupt_footer = bytes.clone();
    let footer_magic = corrupt_footer
        .windows(6)
        .rposition(|part| part == b"ARROW1")
        .unwrap();
    corrupt_footer[footer_magic - 4..footer_magic].copy_from_slice(&i32::MAX.to_le_bytes());
    assert!(
        codec
            .try_decode_table_provider(
                &corrupt_footer,
                &TableReference::bare("probe"),
                schema.clone(),
                &session.task_ctx()
            )
            .is_err()
    );
    let restored = scan.clone().rebind_immutable(table.log_store(), &config)?;
    let result = session.read_table(Arc::new(restored))?.collect().await?;
    assert_eq!(
        result.iter().map(|batch| batch.num_rows()).sum::<usize>(),
        3
    );
    Ok(())
}

#[tokio::test]
async fn typed_ingredients_rebind_and_reject_changed_contracts() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime =
        crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
    let delta = DeltaStore::new(&root.path().join("delta"), runtime.clone())?;
    let contract = crate::delta_cohort::contract(Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Int32,
        true,
    )])))?;
    // This is a storage/provider journey, not a pre-barrier unit test.
    let table = delta.create("fixture", &contract, false).await?;
    let binding = DeltaBinding {
        source: enrichment_core::delta_reference::DeltaVersionRef {
            table: enrichment_core::delta_reference::DeltaTableRef {
                table_uri: "fixture".into(),
                table_id: table
                    .snapshot()
                    .map_err(|e| DataFusionError::External(Box::new(e)))?
                    .metadata()
                    .id()
                    .into(),
                contract_id: contract.identity().clone(),
            },
            version: table.version().unwrap(),
        },
        relation: "fixture".into(),
        cohort_id: enrichment_core::identity::CohortId::try_from(
            "cohort_f16d05ec6b29248d2c61adb1e9263f78".to_owned(),
        )
        .unwrap(),
        rows: 0,
    };
    crate::leases::initialize(root.path())?;
    crate::immutable_root::ImmutableRoot::seal(root.path())?;
    let protection = ReadProtection::immutable(
        crate::immutable_root::ImmutableRoot::open(root.path())?,
        vec![crate::retention::dependency(&binding)],
        runtime.clone(),
    )?;
    protection
        .require_tables(&delta.root, std::slice::from_ref(&binding))
        .await?;
    let first = delta
        .immutable_provider(&binding, &contract, &protection)
        .await?;
    assert_eq!(runtime.descriptors.0.len(), 1);
    let second = delta
        .immutable_provider(&binding, &contract, &protection)
        .await?;
    assert_eq!(first.schema(), second.schema());
    let rejected = crate::native_catalog::batch(
        &runtime.session(),
        "tests",
        arrow::record_batch::RecordBatch::new_empty(contract.semantic_schema()),
    )?;
    assert!(
        delta
            .append(table.clone(), &contract, rejected.clone(), vec![])
            .await
            .is_err(),
        "sealed source cannot be appended through a retained table"
    );
    let other_root = root.path().join("other/data/delta");
    std::fs::create_dir_all(&other_root)?;
    let other = DeltaStore::new(&other_root, runtime.clone())?;
    assert!(
        other
            .append(table.clone(), &contract, rejected, vec![])
            .await
            .is_err(),
        "another namespace cannot bypass sealed-source protection"
    );

    let entries = runtime.descriptors.0.list_entries();
    assert_eq!(entries.values().next().unwrap().hits, 1);
    let retained = &entries.values().next().unwrap().value.0.table;
    let session = runtime.session();
    let mut wrong = binding.clone();
    wrong.source.version += 1;
    assert!(validate_identity(retained, &wrong, &contract, &session.state()).is_err());
    wrong = binding.clone();
    wrong.source.table.table_id = "wrong-table".into();
    assert!(validate_identity(retained, &wrong, &contract, &session.state()).is_err());
    assert!(
        second
            .insert_into(
                &session.state(),
                Arc::new(datafusion::physical_plan::empty::EmptyExec::new(
                    second.schema()
                )),
                datafusion::logical_expr::dml::InsertOp::Append
            )
            .await
            .is_err()
    );
    let descriptor = delta
        .read_descriptor(&binding, &contract, &protection)
        .await?;
    delta
        .replay_provider(&descriptor, &binding, &contract, &protection)
        .await?;
    assert!(delta.descriptor_verified(&binding, &contract, &descriptor.digest)?);
    // Deferred CP12 storage journey: keep the namespace and warm allocations alive,
    // but remove the durable commit. Neither cache may manufacture retained history.
    let log = delta.root.join("fixture/_delta_log");
    for entry in std::fs::read_dir(&log)? {
        let path = entry?.path();
        if path.is_file() {
            std::fs::remove_file(path)?;
        }
    }
    assert!(
        delta
            .load("fixture", Some(binding.source.version))
            .await
            .is_err()
    );
    assert!(
        delta
            .immutable_provider(&binding, &contract, &protection)
            .await
            .is_err()
    );
    assert!(
        delta
            .replay_provider(&descriptor, &binding, &contract, &protection)
            .await
            .is_err()
    );
    Ok(())
}
