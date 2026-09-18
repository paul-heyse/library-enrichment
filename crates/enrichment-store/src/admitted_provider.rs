//! Proven relational constraints over a native read-only source or view.
use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::{
    catalog::{Session, TableProvider},
    common::{Constraints, Result, Statistics},
    logical_expr::{Expr, LogicalPlan, TableProviderFilterPushDown, TableType},
    physical_plan::ExecutionPlan,
};
use std::{borrow::Cow, sync::Arc};

#[derive(Debug)]
pub(crate) struct AdmittedProvider {
    inner: Arc<dyn TableProvider>,
    constraints: Constraints,
    captured_batch: bool,
}
impl AdmittedProvider {
    pub(crate) fn new(inner: Arc<dyn TableProvider>, constraints: Constraints) -> Self {
        Self {
            inner,
            constraints,
            captured_batch: false,
        }
    }
    /// Own an immutable Arrow ingress without exposing MemTable's mutation handle.
    /// This grants no uniqueness, foreign-key, or row-semantic assertion.
    pub(crate) fn from_batch(
        batch: arrow::record_batch::RecordBatch,
        pool: &Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
    ) -> Result<Self> {
        Self::from_batches(vec![batch], pool)
    }
    pub(crate) fn from_batches(
        batches: Vec<arrow::record_batch::RecordBatch>,
        pool: &Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
    ) -> Result<Self> {
        let schema = batches
            .first()
            .ok_or_else(|| {
                datafusion::common::plan_datafusion_err!(
                    "captured input requires a declared batch schema"
                )
            })?
            .schema();
        if batches.iter().any(|batch| batch.schema() != schema) {
            return datafusion::common::plan_err!("captured batches disagree on their full schema");
        }
        for batch in &batches {
            crate::owned_batch::claim(batch, pool, "native-input")?;
        }
        Ok(Self {
            inner: Arc::new(datafusion::datasource::MemTable::try_new(
                schema,
                vec![batches],
            )?),
            constraints: Constraints::new_unverified(vec![]),
            captured_batch: true,
        })
    }
    pub(crate) fn is_captured_batch(&self) -> bool {
        self.captured_batch
    }
    pub(crate) fn input(&self) -> &Arc<dyn TableProvider> {
        &self.inner
    }
}
#[async_trait]
impl TableProvider for AdmittedProvider {
    fn schema(&self) -> SchemaRef {
        self.inner.schema()
    }
    fn table_type(&self) -> TableType {
        self.inner.table_type()
    }
    fn constraints(&self) -> Option<&Constraints> {
        Some(&self.constraints)
    }
    fn statistics(&self) -> Option<Statistics> {
        self.inner.statistics()
    }
    fn get_logical_plan(&self) -> Option<Cow<'_, LogicalPlan>> {
        self.inner.get_logical_plan()
    }
    fn get_table_definition(&self) -> Option<&str> {
        self.inner.get_table_definition()
    }
    fn get_column_default(&self, column: &str) -> Option<&Expr> {
        self.inner.get_column_default(column)
    }
    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        self.inner.supports_filters_pushdown(filters)
    }
    async fn scan(
        &self,
        session: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        self.inner.scan(session, projection, filters, limit).await
    }
    async fn scan_with_args<'a>(
        &self,
        session: &dyn Session,
        args: datafusion::catalog::ScanArgs<'a>,
    ) -> Result<datafusion::catalog::ScanResult> {
        self.inner.scan_with_args(session, args).await
    }
}
