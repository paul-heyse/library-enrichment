//! Exact private Arrow IPC inputs consumed by DataFusion's native Arrow source.
use crate::{provider::FileWitness, runtime::QueryRuntime};
use arrow::datatypes::SchemaRef;
use datafusion::{
    catalog::TableProvider,
    datasource::file_format::options::ArrowReadOptions,
    error::{DataFusionError, Result},
};
use std::{path::Path, sync::Arc};

pub(crate) async fn provider(
    runtime: &QueryRuntime,
    path: &Path,
    schema: SchemaRef,
) -> Result<Arc<dyn TableProvider>> {
    FileWitness::read(path)?;
    let url = url::Url::from_file_path(path)
        .map_err(|()| DataFusionError::Plan("Arrow input path is not absolute".into()))?;
    let frame = runtime
        .session()
        .read_arrow(url.as_str(), ArrowReadOptions::default().schema(&schema))
        .await?;
    // Each exact source has its own native relation qualifier. Distinct input files can
    // participate in one join without inheriting read_arrow's shared `?table?` name.
    Ok(frame.alias(url.as_str())?.into_view())
}
