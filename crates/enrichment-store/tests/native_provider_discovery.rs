use arrow::{
    array::{Int64Array, StructArray},
    datatypes::{DataType, Field, Schema},
    record_batch::RecordBatch,
};
use datafusion::{
    catalog::{
        SchemaProvider, Session, TableProvider, TableProviderFactory,
        listing_schema::ListingSchemaProvider,
    },
    datasource::MemTable,
    error::{DataFusionError, Result},
    logical_expr::CreateExternalTable,
};
use enrichment_store::{
    native_delta::{DeltaStore, StorageContract},
    runtime::QueryRuntime,
};
use object_store::{ObjectStoreExt, path::Path};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Debug)]
struct InventoryFactory {
    calls: AtomicUsize,
    refuse: AtomicBool,
}
#[async_trait::async_trait]
impl TableProviderFactory for InventoryFactory {
    async fn create(
        &self,
        _: &dyn Session,
        _: &CreateExternalTable,
    ) -> Result<Arc<dyn TableProvider>> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        if self.refuse.load(Ordering::Relaxed) {
            return Err(DataFusionError::Plan("fixture factory refusal".into()));
        }
        Ok(Arc::new(MemTable::try_new(
            Arc::new(Schema::empty()),
            vec![vec![]],
        )?))
    }
}

#[tokio::test]
async fn native_listing_bounds_collisions_and_atomic_refresh() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(root.path(), Default::default())?;
    let context = runtime.session();
    let store = Arc::new(object_store::memory::InMemory::new());
    let factory = Arc::new(InventoryFactory {
        calls: AtomicUsize::new(0),
        refuse: AtomicBool::new(false),
    });
    store
        .put(
            &Path::from("root/first/file"),
            bytes::Bytes::from_static(b"one").into(),
        )
        .await?;
    let listing = ListingSchemaProvider::new(
        "memory://".into(),
        Path::from("root"),
        factory.clone(),
        store.clone(),
        "fixture".into(),
    )
    .with_discovery_limits(8, 2)?;
    listing.refresh(&context.state()).await?;
    assert_eq!(listing.table_names(), vec!["first"]);
    let retained = listing.table("first").await?.unwrap();
    store
        .put(
            &Path::from("root/second/file"),
            bytes::Bytes::from_static(b"two").into(),
        )
        .await?;
    factory.refuse.store(true, Ordering::Relaxed);
    assert!(listing.refresh(&context.state()).await.is_err());
    assert_eq!(listing.table_names(), vec!["first"]);
    assert!(Arc::ptr_eq(
        &retained,
        &listing.table("first").await?.unwrap()
    ));
    factory.refuse.store(false, Ordering::Relaxed);
    store
        .put(
            &Path::from("root/first.collision/file"),
            bytes::Bytes::from_static(b"three").into(),
        )
        .await?;
    let calls = factory.calls.load(Ordering::Relaxed);
    assert!(
        listing
            .refresh(&context.state())
            .await
            .unwrap_err()
            .to_string()
            .contains("ambiguous")
    );
    assert_eq!(factory.calls.load(Ordering::Relaxed), calls);
    assert_eq!(listing.table_names(), vec!["first"]);
    let limited = ListingSchemaProvider::new(
        "memory://".into(),
        Path::from("root"),
        factory.clone(),
        store,
        "fixture".into(),
    )
    .with_discovery_limits(1, 1)?;
    assert!(limited.refresh(&context.state()).await.is_err());
    assert!(limited.table_names().is_empty());
    assert_eq!(factory.calls.load(Ordering::Relaxed), calls);
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn native_factory_capture_and_discovery_preserve_version_and_fields() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default())?;
    let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
    let field = Field::new("id", DataType::Int64, false)
        .with_metadata([("enrichment.rule".into(), "{\"kind\":\"text\"}".into())].into());
    let schema = Arc::new(Schema::new(vec![field]));
    let contract = StorageContract::new(schema.clone())?;
    let table = store.create("facts", &contract, true).await?;
    let frame = |value| {
        runtime.session().read_batch(
            RecordBatch::try_new(
                schema.clone(),
                vec![Arc::new(Int64Array::from(vec![value]))],
            )
            .unwrap(),
        )
    };
    let table = store.append(table, &contract, frame(1)?, vec![]).await?;
    let captured = store.provider(&table, &contract).await?;
    let first = store.discover(256, 8).await?;
    let capture = first.captures.iter().find(|c| c.name == "facts").unwrap();
    assert_eq!(capture.version, table.version().unwrap());
    assert_eq!(capture.contract_id, contract.identity());
    assert_eq!(capture.table_id, table.snapshot().unwrap().metadata().id());
    assert_eq!(captured.schema().fields(), schema.fields());
    assert_eq!(first.tables["facts"].schema().fields(), schema.fields());
    let advanced = store.append(table, &contract, frame(2)?, vec![]).await?;
    assert!(advanced.version().unwrap() > capture.version);
    assert_eq!(
        runtime
            .execute(runtime.session().read_table(captured)?)
            .await?
            .rows,
        1
    );
    assert_eq!(
        runtime
            .execute(
                runtime
                    .session()
                    .read_table(first.tables["facts"].clone())?
            )
            .await?
            .rows,
        1
    );
    assert_eq!(
        runtime
            .execute(
                runtime
                    .session()
                    .read_table(store.provider(&advanced, &contract).await?)?
            )
            .await?
            .rows,
        2
    );
    runtime.close_diagnostics().await?;
    Ok(())
}

#[tokio::test]
async fn nested_check_in_roundtrips_and_rejects_invalid_rows() -> Result<()> {
    let root = tempfile::tempdir()?;
    let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default())?;
    let store = DeltaStore::new(&root.path().join("tables"), runtime.clone())?;
    let fields = vec![Arc::new(Field::new("status", DataType::Int64, false))].into();
    let schema = Arc::new(Schema::new(vec![Field::new(
        "response",
        DataType::Struct(fields),
        false,
    )]));
    let contract = StorageContract::new(schema.clone())?;
    let table = store
        .create_with_rules(
            "checked",
            &contract,
            false,
            &[
                ("positive", "response.status IN (200, 304)".into()),
                ("negative", "response.status NOT IN (400, 500)".into()),
            ],
        )
        .await?;
    let config = table.snapshot().unwrap().metadata().configuration();
    assert!(!config["delta.constraints.positive"].contains("Utf8("));
    let frame = |value| {
        let DataType::Struct(fields) = schema.field(0).data_type() else {
            unreachable!()
        };
        let array = StructArray::try_new(
            fields.clone(),
            vec![Arc::new(Int64Array::from(vec![value]))],
            None,
        )?;
        runtime
            .session()
            .read_batch(RecordBatch::try_new(schema.clone(), vec![Arc::new(array)])?)
    };
    let table = store.append(table, &contract, frame(200)?, vec![]).await?;
    let version = table.version().unwrap();
    let reopened = store.load("checked", Some(version)).await?;
    assert!(
        store
            .append(reopened, &contract, frame(500)?, vec![])
            .await
            .is_err()
    );
    assert_eq!(store.load("checked", None).await?.version(), Some(version));
    runtime.close_diagnostics().await?;
    Ok(())
}
