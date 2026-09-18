use arrow_array::{Int64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema, TimeUnit};
use deltalake::DeltaTable;
use std::sync::Arc;

#[test]
fn nested_schema_and_timestamp_normalization_have_explicit_loss_boundaries() {
    use deltalake::kernel::engine::arrow_conversion::{TryIntoArrow, TryIntoKernel};
    let schema = Arc::new(Schema::new(vec![
        Field::new(
            "nested",
            DataType::Struct(vec![Field::new("amount", DataType::Decimal128(20, 3), false)].into()),
            true,
        ),
        Field::new(
            "ts",
            DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".into())),
            true,
        ),
    ]));
    let normalized = deltalake::kernel::schema::cast::normalize_for_delta(&schema);
    assert_eq!(
        normalized.field(1).data_type(),
        &DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".into()))
    );
    let kernel: deltalake::kernel::StructType = normalized.as_ref().try_into_kernel().unwrap();
    let arrow: Schema = (&kernel).try_into_arrow().unwrap();
    assert_eq!(arrow.field(0), normalized.field(0));
    assert_eq!(arrow.field(1).data_type(), normalized.field(1).data_type());
    let unsigned = Schema::new(vec![Field::new("u", DataType::UInt64, false)]);
    let result: Result<deltalake::kernel::StructType, _> = (&unsigned).try_into_kernel();
    let mapped = result.unwrap();
    let roundtrip: Schema = (&mapped).try_into_arrow().unwrap();
    assert_eq!(roundtrip.field(0).data_type(), &DataType::Int64);
}

#[test]
fn configured_memory_pool_enforces_reservation_budget() {
    use datafusion::execution::memory_pool::MemoryConsumer;
    let state =
        deltalake::delta_datafusion::create_session_state_with_spill_config(Some(1024), Some(4096));
    let pool = &state.runtime_env().memory_pool;
    let reservation = MemoryConsumer::new("skill-probe").register(pool);
    reservation.try_grow(512).unwrap();
    assert_eq!(pool.reserved(), 512);
    assert!(reservation.try_grow(1024).is_err());
    drop(reservation);
    assert_eq!(pool.reserved(), 0);
}

#[tokio::test]
async fn existing_object_store_mapping_needs_explicit_replacement() {
    let table = DeltaTable::new_in_memory()
        .write([RecordBatch::try_new(
            Arc::new(Schema::new(vec![Field::new("id", DataType::Int64, false)])),
            vec![Arc::new(Int64Array::from(vec![1]))],
        )
        .unwrap()])
        .await
        .unwrap();
    let ctx = datafusion::prelude::SessionContext::new();
    let root = table.log_store().root_url().clone();
    let wrong: Arc<dyn object_store::ObjectStore> = Arc::new(object_store::memory::InMemory::new());
    ctx.runtime_env()
        .register_object_store(&root, wrong.clone());
    let provider = table.table_provider().await.unwrap();
    let frame = ctx.read_table(provider.clone()).unwrap();
    assert!(frame.collect().await.is_err());
    let current = ctx
        .runtime_env()
        .object_store_registry
        .get_store(&root)
        .unwrap();
    assert!(Arc::ptr_eq(&wrong, &current));
    ctx.runtime_env()
        .register_object_store(&root, table.log_store().root_object_store(None));
    let batches = ctx.read_table(provider).unwrap().collect().await.unwrap();
    assert_eq!(batches.iter().map(RecordBatch::num_rows).sum::<usize>(), 1);
}
