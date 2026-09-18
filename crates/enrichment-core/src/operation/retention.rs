//! Exact durable protection and maintenance commands. These declarations are the
//! shared Arrow, Delta, validation and transport contracts; clocks never revoke ownership.
pub use crate::delta_reference::{CdfWindow, TableSelection};
pub use crate::identity::RowValue;
use crate::native_union::Rule;

crate::native_struct! {
    /// Minimum horizons for unreferenced Delta data and history. Published roots never expire.
    #[serde(default)]
    pub struct RetentionPolicy {
        data_days: u64 => Rule::UnsignedRange { min: 7, max: 36500 },
        log_days: u64 => Rule::UnsignedRange { min: 7, max: 36500 },
        transaction_days: u64 => Rule::UnsignedRange { min: 7, max: 36500 },
    }
}
impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            data_days: 7,
            log_days: 30,
            transaction_days: 30,
        }
    }
}
impl RetentionPolicy {
    pub fn identity(&self) -> datafusion::common::Result<crate::identity::RetentionPolicyId> {
        crate::identity::RetentionPolicyId::try_from_record(self)
    }
}

crate::native_struct! { pub struct NativeProcess {
    machine: String => Rule::NonEmpty,
    boot: String => Rule::NonEmpty,
    pid_namespace: String => Rule::NonEmpty,
    pid: u32 => Rule::UnsignedRange { min: 1, max: u32::MAX as u64 },
    start_ticks: u64 => Rule::Text,
} }
crate::native_struct! { pub struct ProcessObservation {
    process: NativeProcess => Rule::Text,
    observer: NativeProcess => Rule::Text,
    present: Option<bool> => Rule::Text,
    start_ticks: Option<u64> => Rule::Text,
} }

crate::native_struct! {
    /// Exact equality selection within a retained native table. Field names are
    /// schema segments, never SQL expressions; absent selection retains the whole version.
    pub struct RowKey {
        column: String => Rule::NonEmpty,
        value: RowValue => Rule::Text,
    }
}
crate::native_struct! {
    /// A service-created sibling staging directory. The parent identity is observed before
    /// durable enrollment; cleanup never follows a replacement parent or a caller-supplied tree.
    pub struct DirectoryParent {
        parent: String => Rule::NonEmpty,
        parent_device: u64 => Rule::Text,
        parent_inode: u64 => Rule::Text,
    }
}
crate::native_vocabulary! { pub enum PrivateDirectoryKind {
    Documents = "documents", Source = "source", Worker = "worker", Rustdoc = "rustdoc",
} }
crate::native_union! { pub enum PrivateDirectoryRef {
    Local = "local" {
        id: crate::identity::PrivateDirectoryId => Rule::Text,
        purpose: PrivateDirectoryKind => Rule::Text,
    },
    Export = "export" {
        id: crate::identity::PrivateDirectoryId => Rule::Text,
        parent: DirectoryParent => Rule::Text,
    },
} }
impl PrivateDirectoryRef {
    pub fn id(&self) -> crate::identity::PrivateDirectoryId {
        match self {
            Self::Local { id, .. } | Self::Export { id, .. } => *id,
        }
    }
}
crate::native_union! { pub enum Dependency {
    TableScope = "table_scope" { table_uri: String => Rule::NonEmpty },
    PendingRow = "pending_row" {
        table_uri: String => Rule::NonEmpty,
        contract_id: crate::identity::SchemaContractId => Rule::Text,
        row: RowKey => Rule::Text,
    },
    Table = "table" { value: TableSelection => Rule::Text },
    Artifact = "artifact" { artifact_id: String => Rule::NonEmpty },
    PrivateDirectory = "private_directory" { value: PrivateDirectoryRef => Rule::Text },
    CdfWindow = "cdf_window" {
        value: CdfWindow => Rule::Text,
    },
} }
crate::native_vocabulary! { pub enum ProtectionKind {
    Query = "query", Writer = "writer", Export = "export", Replay = "replay", Cdf = "cdf",
} }
crate::native_struct! { pub struct RetentionRoot {
    root_id: String => Rule::NonEmpty,
    dependencies: Vec<Dependency> => Rule::SequenceBounds { min: 1, max: 1024 },
    removed: bool => Rule::Text,
    sequence: u64 => Rule::Text,
} }
crate::native_struct! {
    /// Bounded operator preview or acknowledged atomic removal. Physical storage is
    /// reclaimed separately, after exact readers and writers have exited.
    pub struct RootRemoval {
        generation: u64 => Rule::Text,
        applied: bool => Rule::Text,
        roots: Vec<RetentionRoot> => Rule::SequenceBounds { min: 0, max: 256 },
    }
}
crate::native_struct! { pub struct RetentionLease {
    lease_id: crate::identity::RetentionLeaseId => Rule::Text,
    label: String => Rule::NonEmpty,
    process: NativeProcess => Rule::Text,
    kind: ProtectionKind => Rule::Text,
    fence: u64 => Rule::Text,
    predecessor: u64 => Rule::Text,
    dependencies: Vec<Dependency> => Rule::SequenceBounds { min: 1, max: 1024 },
    released: bool => Rule::Text,
    sequence: u64 => Rule::Text,
} }
crate::native_vocabulary! { pub enum MaintenanceState {
    Claimed = "claimed", Completed = "completed", Failed = "failed",
} }
crate::native_struct! { pub struct MaintenanceRun {
    run_id: crate::identity::MaintenanceRunId => Rule::Text,
    table_uri: String => Rule::NonEmpty,
    label: String => Rule::NonEmpty,
    process: NativeProcess => Rule::Text,
    generation: u64 => Rule::Text,
    predecessor: u64 => Rule::Text,
    policy_id: crate::identity::RetentionPolicyId => Rule::Text,
    policy: RetentionPolicy => Rule::Text,
    state: MaintenanceState => Rule::Text,
    protected: Vec<Dependency> => Rule::SequenceBounds { min: 0, max: 8192 },
    /// Committed before physical maintenance; survives errors and unknown acknowledgements.
    selection: Option<MaintenanceSelection> => Rule::Text,
    sequence: u64 => Rule::Text,
} }
crate::native_struct! { pub struct CleanupObligation {
    obligation_id: crate::identity::CleanupObligationId => Rule::Text,
    label: String => Rule::NonEmpty,
    process: NativeProcess => Rule::Text,
    dependencies: Vec<Dependency> => Rule::SequenceBounds { min: 1, max: 1024 },
    physical_released: bool => Rule::Text,
    settled: bool => Rule::Text,
    sequence: u64 => Rule::Text,
} }

crate::native_struct! { pub struct MaintenanceDecision {
    keep_versions: Vec<u64> => Rule::SequenceBounds { min: 0, max: 8192 },
    log_floor: u64 => Rule::Text,
    vacuum_allowed: bool => Rule::Text,
} }
crate::native_struct! { pub struct MaintenanceSelection {
    table: TableSelection => Rule::Text,
    decision: MaintenanceDecision => Rule::Text,
    reclaim: bool => Rule::Text,
    observed_at: crate::native_time::ObservationTime => Rule::Text,
    log_cutoff: crate::native_time::ObservationTime => Rule::Text,
} }
crate::native_struct! { pub struct Reclamation {
    table_uri: String => Rule::NonEmpty,
    version: u64 => Rule::Text,
    deleted_data_files: u64 => Rule::Text,
    deleted_log_files: u64 => Rule::Text,
} }

crate::native_struct! {
    /// Acknowledged removal of unreferenced content. File bytes are logical lengths,
    /// not filesystem blocks or cache/RSS estimates.
    pub struct ArtifactReclamation {
        candidates: u64 => Rule::Text,
        removed_files: u64 => Rule::Text,
        removed_file_bytes: u64 => Rule::Text,
    }
}
