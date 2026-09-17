//! Native scope compatibility and metadata selection for contributions and conflict retries.
use crate::{native_catalog, registry::rows, runtime::QueryRuntime};
use datafusion::{error::Result, prelude::col};
use enrichment_core::evidence::{
    ObservedConfiguration,
    snapshot::{SnapshotDescriptor, SnapshotMetadata},
};
use enrichment_core::native_union::NativeStruct;

pub(crate) async fn merge(
    runtime: &QueryRuntime,
    metadata: &mut SnapshotMetadata,
    previous: &SnapshotDescriptor,
) -> Result<()> {
    let session = runtime.session();
    let schema = crate::projection::publication::descriptor_schema();
    let records = native_catalog::batch(
        &session,
        "publication_descriptors",
        SnapshotDescriptor::batch(&[metadata.descriptor(), previous.clone()])?,
    )?;
    native_catalog::work(
        &session,
        "publication_descriptors",
        records.clone().into_view(),
    )?;
    let scope = records
        .select(
            schema
                .fields()
                .iter()
                .filter(|f| {
                    !matches!(
                        f.name().as_str(),
                        "observed_configuration" | "producer_items"
                    )
                })
                .map(|f| col(f.name()))
                .collect::<Vec<_>>(),
        )?
        .distinct()?;
    native_catalog::work(&session, "publication_scopes", scope.into_view())?;
    runtime.require_empty(session.sql("SELECT 'different scope, package or normalizer bindings' AS witness FROM publication_scopes HAVING count(*)<>1").await?, "publication_scope_compatibility", "publication").await?;
    runtime.require_empty(session.sql("SELECT 'different observed configurations require distinct contexts' AS witness FROM (SELECT DISTINCT observed_configuration FROM publication_descriptors WHERE observed_configuration IS NOT NULL) HAVING count(*)>1").await?, "publication_observed_configuration", "publication").await?;
    enrichment_core::native_struct! {
    struct Selected {
        producer_items: u64 => enrichment_core::native_union::Rule::Text,
        observed_configuration: Option<ObservedConfiguration> => enrichment_core::native_union::Rule::Text,
    }
    }
    let selected: Selected = rows(runtime, session.sql("SELECT max(producer_items) AS producer_items, first_value(observed_configuration) FILTER (WHERE observed_configuration IS NOT NULL) AS observed_configuration FROM publication_descriptors").await?, 1).await?.pop().ok_or_else(|| datafusion::error::DataFusionError::Internal("publication aggregate absent".into()))?;
    metadata.producer_items = selected.producer_items;
    metadata.observed_configuration = selected.observed_configuration;
    Ok(())
}
