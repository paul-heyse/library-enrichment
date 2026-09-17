//! Isolated native cache/codec fixtures; no service publication or client journey.
use super::*;
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::execution::memory_pool::GreedyMemoryPool;

fn key() -> Key {
    Key {
        table: TableReference::bare("evidence"),
        root: "file:///fixture/".into(),
        table_id: "id".into(),
        version: 2,
        cohort: "cohort".into(),
        contract: "semantic".into(),
        definition: DEFINITION_REVISION,
        codec: CODEC,
        options: vec![],
    }
}

#[test]
fn native_eviction_does_not_release_an_outstanding_value() -> Result<()> {
    let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1024 * 1024));
    let cache = ProviderCache::new(1024 * 1024);
    let key = key();
    let mut encoder = Encoder::new(&pool, key.size())?;
    encoder.write_all(b"descriptor")?;
    cache.0.put(&key, encoder.finish());
    let held = cache.0.get(&key).unwrap();
    let reserved = pool.reserved();
    assert!(reserved > 0);
    cache.0.update_cache_limit(0);
    assert_eq!(cache.0.memory_used(), 0);
    assert_eq!(pool.reserved(), reserved);
    drop(held);
    assert_eq!(pool.reserved(), 0);
    Ok(())
}

#[test]
fn encoding_refuses_before_unreserved_allocation() -> Result<()> {
    let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(4096));
    let mut encoder = Encoder::new(&pool, 0)?;
    assert!(encoder.write_all(&vec![0; 8192]).is_err());
    assert_eq!(encoder.bytes.capacity(), 0);
    drop(encoder);
    assert_eq!(pool.reserved(), 0);
    Ok(())
}

#[tokio::test]
async fn immutable_descriptor_rebinds_and_rejects_changed_contracts() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime =
        crate::runtime::QueryRuntime::new(&root.path().join("spill"), Default::default())?;
    let delta = DeltaStore::new(&root.path().join("delta"), runtime.clone())?;
    let contract = StorageContract::new(Arc::new(Schema::new(vec![Field::new(
        "value",
        DataType::Int32,
        true,
    )])))?;
    // An empty native Delta table is sufficient to exercise snapshot encoding and
    // rebinding; no publication, jobs, CDF, exports or service control are involved.
    let table = delta.create("fixture", &contract, false).await?;
    let binding = DeltaBinding {
        relation: "fixture".into(),
        table_uri: "fixture".into(),
        table_id: table
            .snapshot()
            .map_err(|e| DataFusionError::External(Box::new(e)))?
            .metadata()
            .id()
            .into(),
        version: table.version().unwrap(),
        cohort_id: "fixture".into(),
        contract_id: contract.identity().into(),
        rows: 0,
    };
    crate::leases::initialize(root.path())?;
    let protection = ReadProtection::Root(crate::leases::shared(root.path())?);
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
    let entries = runtime.descriptors.0.list_entries();
    assert_eq!(entries.values().next().unwrap().hits, 1);
    let encoded = &entries.values().next().unwrap().value.0.bytes;
    let session = runtime.session();
    let decoded = DeltaLogicalCodec {}.try_decode_table_provider(
        encoded,
        &TableReference::bare("fixture"),
        contract.semantic_schema(),
        &session.task_ctx(),
    )?;
    let scan = decoded.downcast_ref::<DeltaScanNext>().unwrap();
    let mut wrong = binding.clone();
    wrong.version += 1;
    assert!(validate_identity(scan, &wrong, &contract, &session.state()).is_err());
    wrong = binding.clone();
    wrong.table_id = "wrong-table".into();
    assert!(validate_identity(scan, &wrong, &contract, &session.state()).is_err());
    let mut config = crate::native_discovery::scan_config(&session.state(), &contract);
    config.enable_parquet_pushdown = !config.enable_parquet_pushdown;
    assert!(
        scan.clone()
            .rebind_immutable(delta.log_store("fixture")?, &config)
            .is_err()
    );
    let config = crate::native_discovery::scan_config(&session.state(), &contract);
    assert!(
        scan.clone()
            .rebind_immutable(delta.log_store("foreign")?, &config)
            .is_err()
    );
    let non_delta = session.read_empty()?.into_view();
    assert!(
        DeltaLogicalCodec {}
            .encode_immutable_provider(non_delta.as_ref(), std::io::sink())
            .is_err()
    );
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
    Ok(())
}
