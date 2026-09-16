//! Native logical projections and physical immutable-artifact witnesses.

use datafusion::{common::DFSchema, error::Result, logical_expr::Expr};
use std::{path::Path, sync::Arc};

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
