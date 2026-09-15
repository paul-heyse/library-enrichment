use std::{collections::BTreeMap, path::PathBuf, sync::Arc};
use async_trait::async_trait;
use datafusion::{
    arrow::{array::Int32Array, datatypes::{DataType, Field, Schema, SchemaRef}, record_batch::RecordBatch},
    catalog::{CatalogProvider, MemoryCatalogProvider, SchemaProvider, Session, TableProvider},
    common::{Result, Statistics, stats::Precision, types::logical_string},
    datasource::{file_format::{FileFormat, parquet::ParquetFormat}, listing::PartitionedFile,
        physical_plan::{FileScanConfigBuilder, ParquetSource}, table_schema::TableSchema},
    execution::{memory_pool::{FairSpillPool, MemoryConsumer, MemoryPool, PeakRecordingPool}, object_store::ObjectStoreUrl},
    logical_expr::{Coercion, Expr, Signature, TableType, TypeSignatureClass, Volatility,
        type_coercion::functions::{UDFCoercionExt, fields_with_udf}},
    parquet::arrow::ArrowWriter,
    physical_plan::{ExecutionPlan, displayable},
    prelude::{SessionConfig, SessionContext},
};

#[derive(Debug)]
struct Source { schema: SchemaRef, path: PathBuf, physical_stats: bool, provider_stats: bool }
impl Source {
    fn stats(&self) -> Statistics {
        let mut stats = Statistics::new_unknown(&self.schema);
        stats.num_rows = Precision::Exact(3);
        stats
    }
}
#[async_trait]
impl TableProvider for Source {
    fn schema(&self) -> SchemaRef { self.schema.clone() }
    fn table_type(&self) -> TableType { TableType::Base }
    fn statistics(&self) -> Option<Statistics> { self.provider_stats.then(|| self.stats()) }
    async fn scan(&self, state: &dyn Session, projection: Option<&Vec<usize>>, _filters: &[Expr], limit: Option<usize>) -> Result<Arc<dyn ExecutionPlan>> {
        let source = ParquetSource::new(TableSchema::from(self.schema.clone()));
        let mut builder = FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), Arc::new(source))
            .with_file(PartitionedFile::new(self.path.to_string_lossy().trim_start_matches('/').to_string(), std::fs::metadata(&self.path)?.len()))
            .with_projection_indices(projection.cloned())?.with_limit(limit);
        if self.physical_stats { builder = builder.with_statistics(self.stats()); }
        ParquetFormat::default().create_physical_plan(state, builder.build()).await
    }
}

#[derive(Debug)]
struct FrozenSchema(BTreeMap<String, Arc<dyn TableProvider>>);
#[async_trait]
impl SchemaProvider for FrozenSchema {
    fn table_names(&self) -> Vec<String> { self.0.keys().cloned().collect() }
    fn table_exist(&self, name: &str) -> bool { self.0.contains_key(name) }
    async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>> { Ok(self.0.get(name).cloned()) }
    async fn table_type(&self, name: &str) -> Result<Option<TableType>> { Ok(self.0.get(name).map(|p|p.table_type())) }
}

struct CoercionProbe(Signature);
impl UDFCoercionExt for CoercionProbe {
    fn name(&self) -> &str { "review_string" }
    fn signature(&self) -> &Signature { &self.0 }
    fn coerce_types(&self, _: &[DataType]) -> Result<Vec<DataType>> { unreachable!("declarative coercion") }
}

async fn run() -> Result<()> {
    let path = PathBuf::from(std::env::args().nth(1).expect("scratch parquet path"));
    let schema = Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, false)]));
    let mut writer = ArrowWriter::try_new(std::fs::File::create(&path)?, schema.clone(), None)?;
    writer.write(&RecordBatch::try_new(schema.clone(), vec![Arc::new(Int32Array::from(vec![1,2,3]))])?)?;
    writer.close()?;
    for (name, provider_stats, physical_stats) in [("unknown",false,false),("provider_only",true,false),("physical",true,true)] {
        let session = SessionContext::new();
        session.register_table("evidence", Arc::new(Source { schema:schema.clone(), path:path.clone(), provider_stats, physical_stats }))?;
        let frame = session.sql("SELECT count(*) AS n FROM evidence").await?;
        let plan = frame.clone().create_physical_plan().await?;
        let display = displayable(plan.as_ref()).indent(true).to_string();
        let rows = frame.collect().await?;
        assert_eq!(rows[0].column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0),3);
        let scans = display.contains("DataSourceExec");
        assert_eq!(scans, !physical_stats, "unexpected plan: {display}");
        println!("STATISTICS {name}: count=3 scan={scans}\n{display}");
        let filtered = session.sql("SELECT count(*) FROM evidence WHERE id > 1").await?.collect().await?;
        assert_eq!(filtered[0].column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0),2);
    }
    let source: Arc<dyn TableProvider> = Arc::new(Source{schema:schema.clone(),path,provider_stats:true,physical_stats:true});
    let frozen: Arc<dyn SchemaProvider> = Arc::new(FrozenSchema(BTreeMap::from([("evidence".into(),source.clone())])));
    assert!(frozen.register_table("new".into(),source).is_err());
    assert!(frozen.deregister_table("evidence").is_err());
    assert!(frozen.table("missing").await?.is_none());
    let catalog = Arc::new(MemoryCatalogProvider::new());
    catalog.register_schema("snapshot",frozen)?;
    let session = SessionContext::new_with_config(SessionConfig::new().with_information_schema(true));
    session.register_catalog("research",catalog.clone());
    let inventory = session.sql("SELECT table_name FROM research.information_schema.tables WHERE table_catalog='research' AND table_schema='snapshot'").await?.collect().await?;
    assert_eq!(inventory.iter().map(RecordBatch::num_rows).sum::<usize>(),1);
    let result = session.sql("SELECT count(*) FROM research.snapshot.evidence").await?.collect().await?;
    assert_eq!(result[0].column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0),3);
    println!("CATALOG: qualified lookup, inventory, absent lookup and mutation refusal passed");
    let pinned_plan = session.sql("SELECT count(*) FROM research.snapshot.evidence").await?;
    let cloned_state = SessionContext::new_with_state(session.state());
    let replacement: Arc<dyn TableProvider> = Arc::new(datafusion::datasource::MemTable::try_new(schema.clone(),vec![vec![RecordBatch::try_new(schema.clone(),vec![Arc::new(Int32Array::from(vec![1,2,3,4]))])?]])?);
    catalog.register_schema("snapshot",Arc::new(FrozenSchema(BTreeMap::from([("evidence".into(),replacement)]))))?;
    let old = pinned_plan.collect().await?;
    let new = cloned_state.sql("SELECT count(*) FROM research.snapshot.evidence").await?.collect().await?;
    assert_eq!(old[0].column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0),3);
    assert_eq!(new[0].column(0).as_any().downcast_ref::<datafusion::arrow::array::Int64Array>().unwrap().value(0),4);
    println!("CATALOG SNAPSHOT: retained plan=3; cloned state after shared catalog replacement=4");

    for dtype in [DataType::Utf8,DataType::LargeUtf8,DataType::Utf8View] {
        let fields = vec![Arc::new(Field::new("text",dtype.clone(),true))];
        let exact = fields_with_udf(&fields,&CoercionProbe(Signature::exact(vec![DataType::Utf8],Volatility::Immutable)))?;
        let coercible = fields_with_udf(&fields,&CoercionProbe(Signature::coercible(vec![Coercion::new_exact(TypeSignatureClass::Native(logical_string()))],Volatility::Immutable)))?;
        assert_eq!(exact[0].data_type(),&DataType::Utf8);
        assert_eq!(coercible[0].data_type(),&dtype);
        println!("COERCION: {dtype:?} exact={:?} coercible={:?}",exact[0].data_type(),coercible[0].data_type());
    }
    let pool = Arc::new(PeakRecordingPool::new(Arc::new(FairSpillPool::new(1024))));
    let dynamic: Arc<dyn MemoryPool> = pool.clone();
    let reservation=MemoryConsumer::new("review").register(&dynamic);
    reservation.try_grow(512)?;
    assert!(reservation.try_grow(513).is_err());
    drop(reservation);
    assert_eq!(pool.reserved(),0);
    assert_eq!(pool.peak_reserved(),512);
    println!("MEMORY: bounded delegation preserved; reserved=0 peak=512");
    Ok(())
}
fn main() -> Result<()> { tokio::runtime::Builder::new_multi_thread().enable_all().build()?.block_on(run()) }
