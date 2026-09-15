//! Incremental typed records to bounded Parquet batches. No corpus of normalized payloads exists.
use crate::{
    admission::{EvidenceFile, Relation},
    dataset::{BoundedFile, WriteLimits},
    projection,
};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{
    canonical,
    evidence::{
        execution::ExecutionObservation,
        ingest::EvidenceSink,
        metadata::ReleaseMetadata,
        relational::{
            ApiObservation, CoverageFact, Definition, InputArtifact, PublicBinding,
            RelationshipObservation, TextFragment,
        },
    },
    producer::ProducerRun,
};
use parquet::arrow::ArrowWriter;
use serde::Serialize;
use std::{fs::File, io, path::Path};

pub(crate) struct RelationBuffer<T> {
    relation: Option<Relation>,
    name: String,
    path: std::path::PathBuf,
    writer: ArrowWriter<BoundedFile>,
    pending: Vec<T>,
    pending_bytes: usize,
    group_bytes: usize,
    rows: usize,
    limits: WriteLimits,
    encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
    schema_nodes: usize,
}
impl<T: Serialize> RelationBuffer<T> {
    fn new(
        root: &Path,
        relation: Relation,
        limits: &WriteLimits,
        encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
    ) -> io::Result<Self> {
        let mut writer = Self::staging_with_key(
            root,
            relation.name(),
            limits,
            encode,
            crate::native_policy::bloom_key(relation, limits.observation_bloom),
        )?;
        writer.relation = Some(relation);
        Ok(writer)
    }
    pub(crate) fn staging(
        root: &Path,
        name: &str,
        limits: &WriteLimits,
        encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
    ) -> io::Result<Self> {
        Self::staging_with_key(root, name, limits, encode, None)
    }
    fn staging_with_key(
        root: &Path,
        name: &str,
        limits: &WriteLimits,
        encode: fn(&[T]) -> Result<RecordBatch, ArrowError>,
        bloom_key: Option<&str>,
    ) -> io::Result<Self> {
        let path = root.join(format!("{name}.parquet"));
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        let properties = crate::native_policy::writer_properties(limits.row_group_rows, bloom_key)
            .map_err(io::Error::other)?;
        let schema = encode(&[]).map_err(io::Error::other)?.schema();
        let schema_nodes = schema
            .fields()
            .iter()
            .map(|field| schema_nodes(field.data_type()))
            .sum();
        let writer = ArrowWriter::try_new(
            BoundedFile {
                file,
                bytes: 0,
                limit: limits.file_bytes,
            },
            schema,
            Some(properties),
        )
        .map_err(io::Error::other)?;
        Ok(Self {
            relation: None,
            name: name.into(),
            path,
            writer,
            pending: Vec::new(),
            pending_bytes: 0,
            group_bytes: 0,
            rows: 0,
            limits: limits.clone(),
            encode,
            schema_nodes,
        })
    }
    fn construction_budget(&self, rows: usize, bytes: usize) -> usize {
        // Preflight space for nested offset/validity arrays, rounded small allocations and
        // derived scalar columns, as well as UTF-8 data. Final Arrow capacity is checked too.
        if rows == 0 {
            return 0;
        }
        bytes.saturating_mul(2).saturating_add(
            self.schema_nodes
                .saturating_mul(256usize.saturating_add(rows.saturating_mul(8))),
        )
    }
    pub(crate) fn push(&mut self, value: T) -> io::Result<()> {
        let bytes = canonical::serialized_size(&value, self.limits.record_bytes)?;
        if self.rows >= self.limits.table_rows {
            return Err(io::Error::other("relation row limit exceeded"));
        }
        if self.pending.len() >= self.limits.batch_rows
            || self.construction_budget(
                self.pending.len() + 1,
                self.pending_bytes.saturating_add(bytes),
            ) > self.limits.batch_bytes
        {
            self.flush()?;
        }
        if self.construction_budget(1, bytes) > self.limits.batch_bytes {
            return Err(io::Error::other("record cannot fit batch"));
        }
        self.rows += 1;
        self.pending_bytes += bytes;
        self.pending.push(value);
        Ok(())
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.pending.is_empty() {
            return Ok(());
        }
        if self.writer.flushed_row_groups().len() >= self.limits.row_groups {
            return Err(io::Error::other("row-group metadata limit exceeded"));
        }
        let rows = std::mem::take(&mut self.pending);
        let batch = (self.encode)(&rows).map_err(io::Error::other)?;
        if batch.get_array_memory_size() > self.limits.batch_bytes {
            return Err(io::Error::other(format!(
                "Arrow batch byte limit exceeded: relation={}, rows={}, serialized={}, arrow={}, limit={}",
                self.name,
                rows.len(),
                self.pending_bytes,
                batch.get_array_memory_size(),
                self.limits.batch_bytes
            )));
        }
        drop(rows);
        self.pending_bytes = 0;
        crate::dataset::write_bounded(
            &mut self.writer,
            &batch,
            &self.limits,
            &mut self.group_bytes,
        )?;
        Ok(())
    }
    fn finish(self) -> io::Result<EvidenceFile> {
        let relation = self
            .relation
            .ok_or_else(|| io::Error::other("staging buffer is not a published relation"))?;
        let (path, sha256, bytes, rows) = self.finish_staging()?;
        Ok(EvidenceFile {
            relation,
            path,
            sha256,
            bytes,
            rows,
        })
    }
    pub(crate) fn finish_staging(mut self) -> io::Result<(std::path::PathBuf, String, u64, u64)> {
        self.flush()?;
        self.writer.finish().map_err(io::Error::other)?;
        self.writer.inner().file.sync_all()?;
        let (sha256, bytes) =
            canonical::sha256_reader(File::open(&self.path)?, self.limits.file_bytes)?;
        Ok((self.path, sha256, bytes, self.rows as u64))
    }
}

fn schema_nodes(data_type: &arrow::datatypes::DataType) -> usize {
    use arrow::datatypes::DataType;
    match data_type {
        DataType::Struct(fields) => {
            1 + fields
                .iter()
                .map(|field| schema_nodes(field.data_type()))
                .sum::<usize>()
        }
        DataType::List(field) | DataType::LargeList(field) | DataType::FixedSizeList(field, _) => {
            1 + schema_nodes(field.data_type())
        }
        _ => 1,
    }
}

/// One sink shared by all producer kinds; ten fixed buffers share the declared batch budget.
/// Staging is private until full relational admission. Dropping after an error publishes nothing.
pub struct RelationWriter {
    definitions: RelationBuffer<Definition>,
    symbols: RelationBuffer<PublicBinding>,
    api_observations: RelationBuffer<ApiObservation>,
    execution_observations: RelationBuffer<ExecutionObservation>,
    relationships: RelationBuffer<RelationshipObservation>,
    fragments: RelationBuffer<TextFragment>,
    producer_runs: RelationBuffer<ProducerRun>,
    input_artifacts: RelationBuffer<InputArtifact>,
    coverage: RelationBuffer<CoverageFact>,
    release_metadata: RelationBuffer<ReleaseMetadata>,
}
impl RelationWriter {
    /// Private unpublished staging root; temporary native reference tables live beneath it.
    #[must_use]
    pub fn staging_root(&self) -> &Path {
        self.definitions
            .path
            .parent()
            .expect("relation path has its staging parent")
    }
    #[must_use]
    pub fn limits(&self) -> &WriteLimits {
        &self.definitions.limits
    }
    /// # Errors
    /// Invalid budgets, duplicate files and unavailable staging fail before ingestion.
    pub fn new(root: &Path, limits: &WriteLimits) -> io::Result<Self> {
        crate::dataset::validate_limits(limits)?;
        std::fs::create_dir_all(root)?;
        // A single large record can flush alone. All ten buffers together are capped by the
        // shared row/byte admission below, independently of the native query pool.
        Ok(Self {
            definitions: RelationBuffer::new(
                root,
                Relation::Definitions,
                limits,
                projection::definitions,
            )?,
            symbols: RelationBuffer::new(root, Relation::Symbols, limits, projection::bindings)?,
            api_observations: RelationBuffer::new(
                root,
                Relation::ApiObservations,
                limits,
                projection::observations,
            )?,
            execution_observations: RelationBuffer::new(
                root,
                Relation::ExecutionObservations,
                limits,
                projection::execution::encode,
            )?,
            relationships: RelationBuffer::new(
                root,
                Relation::Relationships,
                limits,
                projection::relationships,
            )?,
            fragments: RelationBuffer::new(
                root,
                Relation::Fragments,
                limits,
                projection::fragments,
            )?,
            producer_runs: RelationBuffer::new(
                root,
                Relation::ProducerRuns,
                limits,
                projection::producer_runs,
            )?,
            input_artifacts: RelationBuffer::new(
                root,
                Relation::InputArtifacts,
                limits,
                projection::input_artifacts,
            )?,
            coverage: RelationBuffer::new(root, Relation::Coverage, limits, projection::coverage)?,
            release_metadata: RelationBuffer::new(
                root,
                Relation::ReleaseMetadata,
                limits,
                projection::metadata::encode,
            )?,
        })
    }
    /// Current pending typed rows and serialized bytes; writer/Arrow allocations are separate.
    #[must_use]
    pub fn buffered(&self) -> (usize, usize) {
        let bytes = self.definitions.pending_bytes
            + self.symbols.pending_bytes
            + self.api_observations.pending_bytes
            + self.execution_observations.pending_bytes
            + self.relationships.pending_bytes
            + self.fragments.pending_bytes
            + self.producer_runs.pending_bytes
            + self.input_artifacts.pending_bytes
            + self.coverage.pending_bytes
            + self.release_metadata.pending_bytes;
        let rows = self.definitions.pending.len()
            + self.symbols.pending.len()
            + self.api_observations.pending.len()
            + self.execution_observations.pending.len()
            + self.relationships.pending.len()
            + self.fragments.pending.len()
            + self.producer_runs.pending.len()
            + self.input_artifacts.pending.len()
            + self.coverage.pending.len()
            + self.release_metadata.pending.len();
        (rows, bytes)
    }
    fn make_room(&mut self, value: &impl Serialize) -> io::Result<()> {
        let limits = &self.definitions.limits;
        let size = canonical::serialized_size(value, limits.record_bytes)?;
        let (rows, bytes) = self.buffered();
        if rows >= limits.batch_rows || bytes.saturating_add(size) > limits.batch_bytes {
            self.definitions.flush()?;
            self.symbols.flush()?;
            self.api_observations.flush()?;
            self.execution_observations.flush()?;
            self.relationships.flush()?;
            self.fragments.flush()?;
            self.producer_runs.flush()?;
            self.input_artifacts.flush()?;
            self.coverage.flush()?;
            self.release_metadata.flush()?;
        }
        Ok(())
    }
    /// Close and flush every file. Returned files still require full relational admission.
    /// # Errors
    /// A buffer, writer, digest or filesystem failure aborts the unpublished contribution.
    pub fn finish(self) -> io::Result<Vec<EvidenceFile>> {
        Ok(vec![
            self.definitions.finish()?,
            self.symbols.finish()?,
            self.api_observations.finish()?,
            self.execution_observations.finish()?,
            self.relationships.finish()?,
            self.fragments.finish()?,
            self.producer_runs.finish()?,
            self.input_artifacts.finish()?,
            self.coverage.finish()?,
            self.release_metadata.finish()?,
        ])
    }
}
impl EvidenceSink for RelationWriter {
    fn definition(&mut self, value: Definition) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.definitions.push(value).map_err(|e| e.to_string())
    }
    fn binding(&mut self, value: PublicBinding) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.symbols.push(value).map_err(|e| e.to_string())
    }
    fn api(&mut self, value: ApiObservation) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.api_observations.push(value).map_err(|e| e.to_string())
    }
    fn execution(&mut self, value: ExecutionObservation) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.execution_observations
            .push(value)
            .map_err(|e| e.to_string())
    }
    fn relationship(&mut self, value: RelationshipObservation) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.relationships.push(value).map_err(|e| e.to_string())
    }
    fn fragment(&mut self, value: TextFragment) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.fragments.push(value).map_err(|e| e.to_string())
    }
    fn producer(&mut self, value: ProducerRun) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.producer_runs.push(value).map_err(|e| e.to_string())
    }
    fn input(&mut self, value: InputArtifact) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.input_artifacts.push(value).map_err(|e| e.to_string())
    }
    fn coverage(&mut self, value: CoverageFact) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.coverage.push(value).map_err(|e| e.to_string())
    }
    fn metadata(&mut self, value: ReleaseMetadata) -> Result<(), String> {
        self.make_room(&value).map_err(|e| e.to_string())?;
        self.release_metadata.push(value).map_err(|e| e.to_string())
    }
}
