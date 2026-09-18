//! Read-only DataFusion projection of the effective Rust query policy.

use datafusion::{
    catalog::Session,
    common::config::{ConfigEntry, ConfigExtension, ExtensionOptions, TableParquetOptions},
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
        parquet.global.max_predicate_cache_size = Some(limits.caches.predicate_bytes);
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
                "native_workers",
                self.limits.concurrency.to_string(),
                "Native async workers per owned compute/I/O lane",
            ),
            (
                "native_blocking_threads",
                self.limits.native.blocking_threads.to_string(),
                "Native blocking-worker ceiling per owned compute/I/O lane",
            ),
            (
                "native_worker_stack_bytes",
                self.limits.native.worker_stack_bytes.to_string(),
                "Stack capacity of each native worker",
            ),
            (
                "parser_concurrency",
                self.limits.native.parser_concurrency.to_string(),
                "Shared Rust/Python parser admission held through physical child reaping",
            ),
            (
                "native_stack_capacity_bytes",
                (2 * self.limits.native.worker_stack_bytes
                    * (self.limits.concurrency + self.limits.native.blocking_threads))
                    .to_string(),
                "Both owned lanes' maximum thread stack capacity; separate from Arrow reservations",
            ),
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
                "Writer Bloom filter on declared observation_id fields",
            ),
            (
                "bloom_filter_on_read",
                self.parquet.global.bloom_filter_on_read.to_string(),
                "Native Parquet Bloom metadata consumer",
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
                "storage_row_group_bytes",
                self.limits.native.row_group_bytes.to_string(),
                "Native Parquet row-group byte target",
            ),
            (
                "target_file_bytes",
                self.limits.native.target_file_bytes.to_string(),
                "Native Delta writer file-size target",
            ),
            (
                "claim_lease_seconds",
                self.limits.native.claim_lease_seconds.to_string(),
                "Captured native claim horizon; expiry does not prove owner cleanup",
            ),
            (
                "normalization_depth",
                self.limits.native.normalization_depth.to_string(),
                "Captured native closure depth; exhaustion produces partial evidence",
            ),
            (
                "metadata_cache_bytes",
                self.limits.caches.metadata_bytes.to_string(),
                "Shared native Parquet metadata capacity",
            ),
            (
                "snapshot_cache_bytes",
                self.limits.caches.snapshots_bytes.to_string(),
                "Shared exact Delta snapshot capacity",
            ),
            (
                "snapshot_entry_bytes",
                self.limits.caches.snapshot_entry_bytes.to_string(),
                "Per-snapshot replay admission and resident bound",
            ),
            (
                "provider_cache_bytes",
                self.limits.caches.providers_bytes.to_string(),
                "Typed immutable provider ingredient capacity",
            ),
            (
                "contract_cache_bytes",
                self.limits.caches.contracts_bytes.to_string(),
                "Positive semantic contract proof capacity",
            ),
            (
                "predicate_cache_bytes",
                self.limits.caches.predicate_bytes.to_string(),
                "Native Parquet predicate cache limit per row group",
            ),
            (
                "control_checkpoint_interval",
                self.limits.caches.control_checkpoint_interval.to_string(),
                "Native control commit checkpoint cadence",
            ),
            (
                "checkpoint_interval",
                self.limits.caches.checkpoint_interval.to_string(),
                "Native non-control commit checkpoint cadence",
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
    row_group_bytes: usize,
    bloom_key: Option<&str>,
) -> Result<parquet::file::properties::WriterProperties> {
    let mut builder = parquet::file::properties::WriterProperties::builder()
        .set_compression(parquet::basic::Compression::ZSTD(
            parquet::basic::ZstdLevel::try_new(3)?,
        ))
        .set_max_row_group_row_count(Some(row_group_rows))
        .set_max_row_group_bytes(Some(row_group_bytes));
    if let Some(key) = bloom_key {
        builder = builder
            .set_column_bloom_filter_enabled(key.into(), true)
            .set_column_bloom_filter_max_ndv(key.into(), row_group_rows as u64);
    }
    Ok(builder.build())
}

/// Every qualified Delta builder consumes the same resolved native writer policy.
pub(crate) fn delta_writer_properties(
    state: &dyn Session,
    schema: Option<&arrow_schema::Schema>,
) -> Result<parquet::file::properties::WriterProperties> {
    let policy = state
        .config_options()
        .extensions
        .get::<NativePolicy>()
        .ok_or_else(|| {
            DataFusionError::Configuration("Delta writer requires bound native policy".into())
        })?;
    let key = (policy.limits.native.observation_bloom
        && schema.is_some_and(|schema| schema.field_with_name("observation_id").is_ok()))
    .then_some("observation_id");
    writer_properties(
        policy.limits.native.row_group_rows,
        policy.limits.native.row_group_bytes,
        key,
    )
}

/// Batching and target file size come from the same captured session policy as Parquet layout.
pub(crate) fn delta_write_options(state: &dyn Session) -> Result<(usize, std::num::NonZeroU64)> {
    let policy = state
        .config_options()
        .extensions
        .get::<NativePolicy>()
        .ok_or_else(|| {
            DataFusionError::Configuration("Delta writer requires bound native policy".into())
        })?;
    let target =
        std::num::NonZeroU64::new(policy.limits.native.target_file_bytes).ok_or_else(|| {
            DataFusionError::Configuration("Delta file target must be positive".into())
        })?;
    Ok((policy.limits.batch_rows, target))
}

pub(crate) fn claim_lease_seconds(state: &dyn Session) -> Result<u64> {
    let seconds = state
        .config_options()
        .extensions
        .get::<NativePolicy>()
        .map(|policy| policy.limits.native.claim_lease_seconds)
        .ok_or_else(|| {
            DataFusionError::Configuration("claim requires bound native policy".into())
        })?;
    if !(1..=86400).contains(&seconds) {
        return Err(DataFusionError::Configuration(
            "claim lease must be in 1..=86400 seconds".into(),
        ));
    }
    Ok(seconds)
}

/// Native UTC time and interval arithmetic own both initial and renewed lease deadlines.
pub(crate) fn claim_deadline(state: &dyn Session) -> Result<datafusion::logical_expr::Expr> {
    let seconds = claim_lease_seconds(state)?;
    Ok(enrichment_core::native_time::expression(
        enrichment_core::native_types::ClockMeaning::Expiry,
        enrichment_core::native_time::now_instant()
            + datafusion::prelude::lit(datafusion::common::ScalarValue::new_interval_mdn(
                0,
                0,
                i64::try_from(seconds * 1_000_000_000).map_err(|_| {
                    DataFusionError::Configuration("native lease interval overflow".into())
                })?,
            )),
    ))
}

pub(crate) fn normalization_depth(state: &dyn Session) -> Result<u32> {
    let depth = state
        .config_options()
        .extensions
        .get::<NativePolicy>()
        .ok_or_else(|| {
            DataFusionError::Configuration("normalization requires bound native policy".into())
        })?
        .limits
        .native
        .normalization_depth;
    if !(1..=4096).contains(&depth) {
        return Err(DataFusionError::Configuration(
            "normalization depth must be in 1..=4096".into(),
        ));
    }
    Ok(depth)
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
        let format = state
            .config_options()
            .extensions
            .get::<NativePolicy>()
            .unwrap()
            .table_options();
        assert!(format.global.pushdown_filters);
        assert!(format.global.reorder_filters);
        assert!(!format.global.skip_metadata);
        assert!(
            state
                .config_mut()
                .options_mut()
                .set("enrichment.decoder_filter", "false")
                .is_err()
        );
        assert!(
            state
                .config_options()
                .extensions
                .get::<NativePolicy>()
                .unwrap()
                .table_options()
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
            runtime
                .operational_counters()
                .await
                .unwrap()
                .effective_native_settings["enrichment.storage_row_group_rows"],
            "4096"
        );
    }
}
