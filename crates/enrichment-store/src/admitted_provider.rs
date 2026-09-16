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
}
impl AdmittedProvider {
    pub(crate) fn new(inner: Arc<dyn TableProvider>, constraints: Constraints) -> Self {
        Self { inner, constraints }
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
