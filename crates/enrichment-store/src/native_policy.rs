//! Read-only DataFusion projection of the effective Rust query policy.

use datafusion::{
    catalog::Session,
    common::config::{ConfigEntry, ConfigExtension, ExtensionOptions, TableParquetOptions},
    datasource::file_format::parquet::ParquetFormat,
    error::{DataFusionError, Result},
};
use std::{any::Any, sync::Arc};

#[derive(Debug, Clone)]
pub(crate) struct NativePolicy {
    limits: Arc<crate::runtime::QueryLimits>,
    parquet: TableParquetOptions,
}

impl NativePolicy {
    pub(crate) fn new(limits: Arc<crate::runtime::QueryLimits>) -> Self {
        let mut parquet = TableParquetOptions::default();
        parquet.global.skip_metadata = false;
        parquet.global.pushdown_filters = limits.native.decoder_filter;
        parquet.global.reorder_filters = limits.native.reorder_filters;
        Self { limits, parquet }
    }
    pub(crate) fn table_options(&self) -> TableParquetOptions {
        self.parquet.clone()
    }
}

impl ConfigExtension for NativePolicy {
    const PREFIX: &'static str = "enrichment";
}
impl ExtensionOptions for NativePolicy {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn cloned(&self) -> Box<dyn ExtensionOptions> {
        Box::new(self.clone())
    }
    fn set(&mut self, _: &str, _: &str) -> Result<()> {
        Err(DataFusionError::Configuration(
            "bound enrichment policy is read-only".into(),
        ))
    }
    fn entries(&self) -> Vec<ConfigEntry> {
        [
            (
                "decoder_filter",
                self.parquet.global.pushdown_filters.to_string(),
                "Effective native Parquet decoder filtering",
            ),
            (
                "reorder_filters",
                self.parquet.global.reorder_filters.to_string(),
                "Effective native Parquet filter ordering",
            ),
            (
                "observation_bloom",
                self.limits.native.observation_bloom.to_string(),
                "Writer Bloom filter on admitted observation_id only",
            ),
            (
                "bloom_filter_on_read",
                self.parquet.global.bloom_filter_on_read.to_string(),
                "Native Parquet Bloom metadata consumer",
            ),
            (
                "scan_file_grouping",
                "native_whole_files".into(),
                "Native whole-file grouping bounded by partitions and exact file count",
            ),
            (
                "skip_metadata",
                self.parquet.global.skip_metadata.to_string(),
                "Required schema metadata preservation",
            ),
            (
                "storage_row_group_rows",
                self.limits.native.row_group_rows.to_string(),
                "Storage rows per group, independent of execution batches",
            ),
            (
                "catalog_file_rows",
                self.limits.native.catalog_file_rows.to_string(),
                "Catalog compaction file target independent of transaction rows",
            ),
            (
                "memory_bytes",
                self.limits.memory_bytes.to_string(),
                "Shared managed memory reservation limit",
            ),
            (
                "spill_bytes",
                self.limits.spill_bytes.to_string(),
                "Shared native spill limit",
            ),
            (
                "batch_rows",
                self.limits.batch_rows.to_string(),
                "Native execution batch size",
            ),
            (
                "partitions",
                self.limits.partitions.to_string(),
                "Native target partitions",
            ),
        ]
        .into_iter()
        .map(|(key, value, description)| ConfigEntry {
            key: format!("enrichment.{key}"),
            value: Some(value),
            description,
        })
        .collect()
    }
}

/// Measured writer choices shared by evidence, staging and catalog files.
pub(crate) fn writer_properties(
    row_group_rows: usize,
    bloom_key: Option<&str>,
) -> Result<parquet::file::properties::WriterProperties> {
    let mut builder = parquet::file::properties::WriterProperties::builder()
        .set_compression(parquet::basic::Compression::ZSTD(
            parquet::basic::ZstdLevel::try_new(3)?,
        ))
        .set_max_row_group_row_count(Some(row_group_rows));
    if let Some(key) = bloom_key {
        builder = builder
            .set_column_bloom_filter_enabled(key.into(), true)
            .set_column_bloom_filter_max_ndv(key.into(), row_group_rows as u64);
    }
    Ok(builder.build())
}

pub(crate) fn bloom_key(
    relation: crate::admission::Relation,
    enabled: bool,
) -> Option<&'static str> {
    (enabled && relation == crate::admission::Relation::ApiObservations).then(|| relation.key())
}

pub(crate) fn parquet_format(state: &dyn Session) -> Result<ParquetFormat> {
    let policy = state
        .config_options()
        .extensions
        .get::<NativePolicy>()
        .ok_or_else(|| {
            DataFusionError::Configuration("native source requires bound enrichment policy".into())
        })?;
    Ok(ParquetFormat::default().with_options(policy.table_options()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn read_only_policy_is_the_actual_format_and_settings_authority() {
        let dir = tempfile::tempdir().unwrap();
        let limits = crate::runtime::QueryLimits {
            native: enrichment_core::config::NativeQueryConfig {
                decoder_filter: true,
                reorder_filters: true,
                row_group_rows: 4096,
                ..Default::default()
            },
            ..Default::default()
        };
        let runtime = crate::runtime::QueryRuntime::new(dir.path(), limits).unwrap();
        let session = runtime.session();
        let mut state = session.state();
        let format = parquet_format(&state).unwrap();
        assert!(format.options().global.pushdown_filters);
        assert!(format.options().global.reorder_filters);
        assert!(!format.options().global.skip_metadata);
        assert!(
            state
                .config_mut()
                .options_mut()
                .set("enrichment.decoder_filter", "false")
                .is_err()
        );
        assert!(
            parquet_format(&state)
                .unwrap()
                .options()
                .global
                .pushdown_filters
        );
        let settings = runtime.execute(session.sql("SELECT value FROM information_schema.df_settings WHERE name = 'enrichment.decoder_filter'").await.unwrap()).await.unwrap();
        assert_eq!(settings.rows, 1);
        assert_eq!(
            crate::projection::TextColumn::new(settings.batches[0].column(0).as_ref())
                .unwrap()
                .get(0),
            Some("true")
        );
        assert_eq!(
            runtime.operational_counters().effective_native_settings["enrichment.storage_row_group_rows"],
            "4096"
        );
    }
}
