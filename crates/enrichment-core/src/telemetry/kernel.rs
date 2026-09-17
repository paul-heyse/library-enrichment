//! Typed boundary for the pinned kernel's twenty MetricEvent variants.
//! Durations retain their exact seconds/nanoseconds; failure events do not invent durations.
use crate::native_union::Rule;

crate::native_vocabulary! { pub enum TableKind { PathBased = "path_based", CatalogManaged = "catalog_managed" } }
crate::native_vocabulary! { pub enum ScanKind { Sequential = "sequential", Parallel = "parallel", Full = "full" } }
crate::native_vocabulary! { pub enum CommitFailure { Conflict = "conflict", RetryableIo = "retryable_io", Error = "error" } }
crate::native_struct! {
    pub struct Duration {
        seconds: u64 => Rule::Text,
        nanoseconds: u32 => Rule::UnsignedRange { min: 0, max: 999_999_999 },
    }
}
impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self {
            seconds: value.as_secs(),
            nanoseconds: value.subsec_nanos(),
        }
    }
}
crate::native_struct! {
    pub struct Context {
        kernel_operation_id: [u8; 16] => Rule::Text,
        correlation_id: Option<String> => Rule::Text,
        table_kind: TableKind => Rule::Text,
    }
}
crate::native_union! {
    pub enum Metric {
        LogSegmentLoadSuccess = "log_segment_load_success" {
            context: Context => Rule::Text,
            num_commit_files: u64 => Rule::Text,
            num_checkpoint_files: u64 => Rule::Text,
            num_compaction_files: u64 => Rule::Text,
            has_latest_crc_file: bool => Rule::Text,
            duration: Duration => Rule::Text,
        },
        LogSegmentLoadFailure = "log_segment_load_failure" { context: Context => Rule::Text },
        ProtocolMetadataLoadSuccess = "protocol_metadata_load_success" { context: Context => Rule::Text, duration: Duration => Rule::Text },
        ProtocolMetadataLoadFailure = "protocol_metadata_load_failure" { context: Context => Rule::Text },
        SnapshotBuildSuccess = "snapshot_build_success" { context: Context => Rule::Text, version: u64 => Rule::Text, duration: Duration => Rule::Text },
        SnapshotBuildFailure = "snapshot_build_failure" { context: Context => Rule::Text },
        TransactionCommitSuccess = "transaction_commit_success" {
            context: Context => Rule::Text,
            commit_version: u64 => Rule::Text,
            num_add_files: u64 => Rule::Text,
            num_remove_files: u64 => Rule::Text,
            num_dv_updates: u64 => Rule::Text,
            add_files_bytes: u64 => Rule::Text,
            remove_files_bytes: u64 => Rule::Text,
            is_blind_append: bool => Rule::Text,
            data_change: bool => Rule::Text,
            operation: Option<String> => Rule::Text,
            prepare_duration: Duration => Rule::Text,
            committer_duration: Duration => Rule::Text,
            total_duration: Duration => Rule::Text,
        },
        TransactionCommitFailure = "transaction_commit_failure" { context: Context => Rule::Text, reason: CommitFailure => Rule::Text },
        DomainMetadataLoadSuccess = "domain_metadata_load_success" { from_cache: bool => Rule::Text, num_domains_returned: u64 => Rule::Text, duration: Duration => Rule::Text },
        DomainMetadataLoadFailure = "domain_metadata_load_failure",
        SetTransactionLoadSuccess = "set_transaction_load_success" { from_cache: bool => Rule::Text, found: bool => Rule::Text, duration: Duration => Rule::Text },
        SetTransactionLoadFailure = "set_transaction_load_failure",
        CrcReadSuccess = "crc_read_success" { bytes_read: u64 => Rule::Text, duration: Duration => Rule::Text },
        CrcReadFailure = "crc_read_failure",
        JsonReadCompleted = "json_read_completed" { num_files: u64 => Rule::Text, bytes_read: u64 => Rule::Text },
        ParquetReadCompleted = "parquet_read_completed" { num_files: u64 => Rule::Text, bytes_read: u64 => Rule::Text },
        ScanMetadataCompleted = "scan_metadata_completed" {
            context: Context => Rule::Text,
            scan_kind: ScanKind => Rule::Text,
            duration: Duration => Rule::Text,
            num_add_files_seen: u64 => Rule::Text,
            num_active_add_files: u64 => Rule::Text,
            active_add_files_bytes: u64 => Rule::Text,
            num_remove_files_seen: u64 => Rule::Text,
            num_non_file_actions: u64 => Rule::Text,
            num_predicate_filtered: u64 => Rule::Text,
            peak_hash_set_size: usize => Rule::Text,
            dedup_visitor_time: Duration => Rule::Text,
            predicate_eval_time: Duration => Rule::Text,
        },
        StorageListCompleted = "storage_list_completed" { duration: Duration => Rule::Text, num_files: u64 => Rule::Text },
        StorageReadCompleted = "storage_read_completed" { duration: Duration => Rule::Text, num_files: u64 => Rule::Text, bytes_read: u64 => Rule::Text },
        StorageCopyCompleted = "storage_copy_completed" { duration: Duration => Rule::Text },
    }
}
crate::native_struct! {
    pub struct Observation {
        metric: Metric => Rule::Text,
        /// Only bounded textual attributes may be truncated; counters and IDs stay exact.
        attributes_truncated: bool => Rule::Text,
    }
}
crate::native_struct! {
    pub struct Diagnostic {
        operation_id: Option<String> => Rule::Text,
        sequence: u64 => Rule::Text,
        recorded_at: crate::native_time::EventTime => Rule::Text,
        value: Observation => Rule::Text,
    }
}
