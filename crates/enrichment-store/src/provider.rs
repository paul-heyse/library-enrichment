//! Exact-file Parquet providers. A missing file never becomes a directory listing.
//!
//! No custom execution plan: projection, predicates, statistics, cache integration and scans
//! remain DataFusion/Parquet responsibilities. Admission supplies the immutable file witness.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::catalog::{Session, TableProvider};
use datafusion::common::{Constraints, DFSchema};
use datafusion::datasource::{
    file_format::{FileFormat, parquet::ParquetFormat},
    listing::PartitionedFile,
    physical_plan::{FileGroup, FileScanConfigBuilder, ParquetSource},
    table_schema::TableSchema,
};
use datafusion::error::{DataFusionError, Result};
use datafusion::execution::object_store::ObjectStoreUrl;
use datafusion::logical_expr::{Expr, TableProviderFilterPushDown, TableType, utils::conjunction};
use datafusion::physical_plan::ExecutionPlan;
use object_store::{ObjectStoreExt, local::LocalFileSystem};

/// A derived relation has its own schema metadata. DataFusion 55 logical UNION intersects
/// metadata while physical UNION merges it; inheriting a source table's relation identity
/// would therefore be both semantically wrong and physically inconsistent for mixed axes.
/// This private boundary reconstructs output metadata without changing any base provider.
#[derive(Debug)]
pub(crate) struct DerivedRelation {
    input: Arc<dyn TableProvider>,
    schema: SchemaRef,
}

impl DerivedRelation {
    pub(crate) fn new(input: Arc<dyn TableProvider>, relation: &str) -> Self {
        let schema = Arc::new(arrow_schema::Schema::new_with_metadata(
            input.schema().fields().clone(),
            std::collections::HashMap::from([
                (
                    "enrichment.contract".into(),
                    crate::projection::VERSION.into(),
                ),
                ("enrichment.relation".into(), relation.into()),
            ]),
        ));
        Self { input, schema }
    }
}

#[async_trait]
impl TableProvider for DerivedRelation {
    fn schema(&self) -> SchemaRef {
        Arc::clone(&self.schema)
    }
    fn table_type(&self) -> TableType {
        TableType::View
    }
    async fn scan(
        &self,
        state: &dyn Session,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        use datafusion::{
            physical_expr::{
                PhysicalExpr,
                expressions::{CastExpr, Column},
            },
            physical_plan::projection::ProjectionExec,
        };
        let input = self.input.scan(state, projection, filters, limit).await?;
        let schema = match projection {
            Some(indices) => Arc::new(self.schema.project(indices)?),
            None => Arc::clone(&self.schema),
        };
        let expressions = schema
            .fields()
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let column = Arc::new(Column::new(field.name(), i)) as Arc<dyn PhysicalExpr>;
                // Parquet can return Utf8View although the logical source declares Utf8. Adapt
                // this small derived index to its declared types, preserving explicit field
                // metadata, rather than disabling string views for all evidence scans.
                (
                    Arc::new(CastExpr::new_with_target_field(
                        column,
                        Arc::clone(field),
                        None,
                    )) as Arc<dyn PhysicalExpr>,
                    field.name().clone(),
                )
            })
            .collect::<Vec<_>>();
        Ok(Arc::new(ProjectionExec::try_new_with_schema_metadata(
            expressions,
            input,
            schema.as_ref(),
        )?))
    }
    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        self.input.supports_filters_pushdown(filters)
    }
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
    groups: Vec<FileGroup>,
    constraints: Constraints,
    format: Arc<ParquetFormat>,
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
        let groups = vec![FileGroup::new(partitions)];
        Ok(Self {
            files,
            schema,
            groups,
            constraints,
            format: Arc::new(ParquetFormat::default().with_skip_metadata(false)),
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
        let config =
            FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), Arc::new(source))
                .with_file_groups(self.groups.clone())
                .with_constraints(self.constraints.clone())
                .with_projection_indices(projection.cloned())?
                .with_limit(if filters.is_empty() { limit } else { None })
                .build();
        self.format.create_physical_plan(state, config).await
    }

    fn supports_filters_pushdown(
        &self,
        filters: &[&Expr],
    ) -> Result<Vec<TableProviderFilterPushDown>> {
        Ok(vec![TableProviderFilterPushDown::Inexact; filters.len()])
    }
}
