//! Exact-file Parquet providers. A missing file never becomes a directory listing.
//!
//! No custom execution plan: projection, predicates, statistics, cache integration and scans
//! remain DataFusion/Parquet responsibilities. Admission supplies the immutable file witness.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::catalog::{Session, TableProvider};
use datafusion::common::{Constraints, DFSchema, Statistics, stats::Precision};
use datafusion::datasource::{
    file_format::FileFormat,
    listing::PartitionedFile,
    physical_plan::{FileGroup, FileScanConfigBuilder, ParquetSource},
    table_schema::TableSchema,
};
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::object_store::ObjectStoreUrl;
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown, TableType, utils::conjunction};
use datafusion::physical_plan::ExecutionPlan;
use object_store::{ObjectStoreExt, local::LocalFileSystem};

/// Native projection with an explicit derived contract. Keeping this logical removes the
/// nested physical-planning pass that an opaque provider around a native view would cause.
pub(crate) fn derived(
    frame: datafusion::dataframe::DataFrame,
    relation: &str,
) -> Result<datafusion::dataframe::DataFrame> {
    use datafusion::logical_expr::{Cast, LogicalPlan, Projection};
    let (state, input) = frame.into_parts();
    let schema = Arc::new(arrow_schema::Schema::new_with_metadata(
        input
            .schema()
            .fields()
            .iter()
            .map(|field| {
                let mut field = field.as_ref().clone();
                if relation == "search_candidates" {
                    let mut metadata = field.metadata().clone();
                    metadata.insert(
                        "enrichment.role".into(),
                        format!("search-candidate:{}", field.name()),
                    );
                    field = field.with_metadata(metadata);
                }
                field
            })
            .collect::<Vec<_>>(),
        std::collections::HashMap::from([
            (
                "enrichment.contract".into(),
                crate::projection::VERSION.into(),
            ),
            ("enrichment.relation".into(), relation.into()),
        ]),
    ));
    let expressions = input
        .schema()
        .columns()
        .into_iter()
        .zip(schema.fields())
        .map(|(column, field)| {
            Expr::Cast(Cast::new_from_field(
                Box::new(Expr::Column(column)),
                Arc::clone(field),
            ))
            .alias_with_metadata(
                field.name(),
                Some(datafusion::common::metadata::FieldMetadata::from(
                    field.metadata().clone(),
                )),
            )
        })
        .collect();
    let projection = Projection::try_new_with_schema(
        expressions,
        Arc::new(input),
        Arc::new(DFSchema::try_from(schema)?),
    )?;
    Ok(datafusion::dataframe::DataFrame::new(
        state,
        LogicalPlan::Projection(projection),
    ))
}

/// Cheap change witness only. Digest/schema validation establishes authority before this exists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileWitness {
    bytes: u64,
    modified: std::time::SystemTime,
    #[cfg(unix)]
    inode: (u64, u64, i64, i64),
}

impl FileWitness {
    /// Inspect one regular, non-symlink file. This is not content validation.
    ///
    /// # Errors
    /// Missing files, links and non-regular files are refused.
    pub fn read(path: &Path) -> std::io::Result<Self> {
        let metadata = std::fs::symlink_metadata(path)?;
        if !metadata.is_file() {
            return Err(std::io::Error::other("evidence file is not a regular file"));
        }
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;
        Ok(Self {
            bytes: metadata.len(),
            modified: metadata.modified()?,
            #[cfg(unix)]
            inode: (
                metadata.dev(),
                metadata.ino(),
                metadata.ctime(),
                metadata.ctime_nsec(),
            ),
        })
    }
}

/// Constructed inside storage admission; no public caller may assert unvalidated constraints.
#[derive(Debug, Clone)]
pub(crate) struct ExactParquet {
    files: Vec<(PathBuf, FileWitness)>,
    schema: SchemaRef,
    partitions: Vec<PartitionedFile>,
    constraints: Constraints,
    verified_rows: Option<usize>,
}

impl ExactParquet {
    /// A private read projection over the same admitted files. Native Parquet schema adaptation
    /// prunes the omitted leaf before filters, joins and repartitioning. This is neither another
    /// stored relation nor an admission bypass: only the complete relation can establish trust.
    pub(crate) fn inspection_projection(&self) -> Result<Self> {
        Ok(Self {
            schema: crate::projection::inspection_schema(&self.schema)?,
            ..self.clone()
        })
    }

    pub(crate) fn with_validated_constraints(&self, constraints: Constraints) -> Self {
        Self {
            constraints,
            ..self.clone()
        }
    }

    pub(crate) fn with_verified_rows(mut self, rows: u64) -> Result<Self> {
        self.verified_rows = Some(usize::try_from(rows).map_err(|_| {
            DataFusionError::Execution("admitted row count exceeds native range".into())
        })?);
        Ok(self)
    }

    fn admitted_statistics(&self) -> Statistics {
        let mut stats = Statistics::new_unknown(&self.schema);
        if let Some(rows) = self.verified_rows {
            stats.num_rows = Precision::Exact(rows);
        }
        stats
    }

    pub(crate) fn unchanged(&self) -> Result<()> {
        for (path, witness) in &self.files {
            if admitted_witness(path)? != *witness {
                return Err(changed(path));
            }
        }
        Ok(())
    }
    pub(crate) async fn new(
        path: PathBuf,
        witness: FileWitness,
        schema: SchemaRef,
        constraints: Constraints,
    ) -> Result<Self> {
        Self::new_many(vec![(path, witness)], schema, constraints).await
    }

    pub(crate) async fn new_many(
        files: Vec<(PathBuf, FileWitness)>,
        schema: SchemaRef,
        constraints: Constraints,
    ) -> Result<Self> {
        let mut partitions = Vec::with_capacity(files.len());
        for (path, witness) in &files {
            if admitted_witness(path)? != *witness {
                return Err(changed(path));
            }
            let location = object_store::path::Path::from_filesystem_path(path)
                .map_err(|e| DataFusionError::External(Box::new(e)))?;
            let meta = LocalFileSystem::new().head(&location).await?;
            partitions.push(PartitionedFile::new_from_meta(meta));
        }
        Ok(Self {
            files,
            schema,
            partitions,
            constraints,
            verified_rows: None,
        })
    }
}

fn changed(path: &Path) -> DataFusionError {
    crate::preparation::InvariantFailure::error(
        "admitted evidence file changed",
        "provider_scan",
        vec![
            path.file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().into_owned()),
        ],
    )
}

fn admitted_witness(path: &Path) -> Result<FileWitness> {
    FileWitness::read(path).map_err(|error| {
        if matches!(
            error.kind(),
            std::io::ErrorKind::NotFound | std::io::ErrorKind::InvalidData
        ) {
            changed(path)
        } else {
            error.into()
        }
    })
}

#[async_trait]
impl TableProvider for ExactParquet {
    fn schema(&self) -> SchemaRef {
        Arc::clone(&self.schema)
    }
    fn table_type(&self) -> TableType {
        TableType::Base
    }
    fn constraints(&self) -> Option<&Constraints> {
        Some(&self.constraints)
    }
    fn statistics(&self) -> Option<Statistics> {
        Some(self.admitted_statistics())
    }

    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        self.unchanged()?;
        let mut source = ParquetSource::new(TableSchema::from(Arc::clone(&self.schema)));
        if let Some(predicate) = conjunction(filters.iter().cloned()) {
            let schema = DFSchema::try_from(Arc::clone(&self.schema))?;
            source = source.with_predicate(state.create_physical_expr(predicate, &schema)?);
        }
        // FileGroupPartitioner 55.1 partitions byte ranges; its order-preserving
        // mode does not split a multi-file group. Keep whole admitted files here.
        let count = state
            .config_options()
            .execution
            .target_partitions
            .min(self.partitions.len())
            .max(1);
        let mut groups = vec![vec![]; count];
        for (index, file) in self.partitions.iter().enumerate() {
            groups[index % count].push(file.clone());
        }
        let groups = groups.into_iter().map(FileGroup::new).collect();
        let config =
            FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), Arc::new(source))
                .with_file_groups(groups)
                .with_statistics(self.admitted_statistics())
                .with_constraints(self.constraints.clone())
                .with_projection_indices(projection.cloned())?
                .with_limit(if filters.is_empty() { limit } else { None })
                .build();
        crate::native_policy::parquet_format(state)?
            .create_physical_plan(state, config)
            .await
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        Ok(vec![TableProviderFilterPushDown::Inexact; filters.len()])
    }
}

#[cfg(test)]
#[path = "physical_measurements.rs"]
mod measurements;
