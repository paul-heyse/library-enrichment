//! `CanonicalTable` -- the one custom `TableProvider`.
//!
//! delta-rs's own provider implements only `scan`, `schema`, `table_type`,
//! `supports_filters_pushdown`, `insert_into`, `get_logical_plan` and `get_table_definition`.
//! Everything else sits at the trait default, and one of those defaults costs real query
//! performance.
//!
//! # Why this wrapper exists: `constraints()`
//!
//! Probe PB02 measured it, with the wrapper used in *both* arms so that declaring a key was the
//! only variable. Declaring `Constraint::PrimaryKey([0])` propagates into the scan schema as a
//! functional dependence covering every column, and two query shapes the retrieval layer leans on
//! re-plan as a result:
//!
//! ```text
//! SELECT DISTINCT entity_id   Aggregate + TableScan  ->  TableScan       (aggregation eliminated)
//! self-join on the key        Inner Join + Projection ->  LeftSemi Join
//! ```
//!
//! `Constraints::new_unverified` is a declaration DataFusion trusts rather than checks, so the
//! matching Delta CHECK constraint is not redundant with it -- it is what makes the declaration
//! true.
//!
//! # Why `statistics()` is deliberately absent
//!
//! An earlier draft listed it as a second reason to build this wrapper. Probe PB02b showed that
//! was wrong: delta-rs already supplies accurate statistics at the `ExecutionPlan` level --
//! `DeltaScanExec` reports `Rows=Exact(n)` from the transaction log plus per-column min/max from
//! Parquet -- whether or not the provider implements `statistics()`. Implementing it here adds
//! nothing and is ignored, so it is left out rather than written and misunderstood.

use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::catalog::{Session, TableProvider};
use datafusion::common::{Constraints, Result as DFResult};
use datafusion::datasource::TableType;
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown};
use datafusion::physical_plan::ExecutionPlan;

/// Wraps a Delta provider and supplies the declarations DataFusion's optimiser can use.
#[derive(Debug)]
pub struct CanonicalTable {
    inner: Arc<dyn TableProvider>,
    constraints: Option<Constraints>,
}

impl CanonicalTable {
    /// Wrap a provider, declaring `key_column_indices` as its primary key.
    ///
    /// An empty index list declares nothing, which is the honest representation of a table with
    /// no natural key -- better than declaring a key that is not one, since DataFusion trusts the
    /// declaration without verifying it.
    pub fn new(inner: Arc<dyn TableProvider>, key_column_indices: Vec<usize>) -> Self {
        let constraints = if key_column_indices.is_empty() {
            None
        } else {
            Some(Constraints::new_unverified(vec![
                datafusion::common::Constraint::PrimaryKey(key_column_indices),
            ]))
        };
        Self { inner, constraints }
    }
}

#[async_trait]
impl TableProvider for CanonicalTable {
    fn schema(&self) -> SchemaRef {
        self.inner.schema()
    }

    fn table_type(&self) -> TableType {
        self.inner.table_type()
    }

    /// The reason this type exists. See the module docs.
    fn constraints(&self) -> Option<&Constraints> {
        self.constraints.as_ref()
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> DFResult<Vec<TableProviderFilterPushDown>> {
        self.inner.supports_filters_pushdown(filters)
    }

    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> DFResult<Arc<dyn ExecutionPlan>> {
        self.inner.scan(state, projection, filters, limit).await
    }
}
