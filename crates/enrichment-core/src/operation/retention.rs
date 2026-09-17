//! Exact durable protection and maintenance commands. These declarations are the
//! shared Arrow, Delta, validation and transport contracts; clocks never revoke ownership.
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
    pub fn identity(&self) -> datafusion::common::Result<String> {
        crate::native_key::Key::RetentionPolicy.record(self)
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

crate::native_struct! { pub struct TableVersion {
    table_uri: String => Rule::NonEmpty,
    table_id: String => Rule::NonEmpty,
    version: u64 => Rule::Text,
    contract_id: String => Rule::NonEmpty,
    cohort_id: Option<String> => Rule::Text,
} }
crate::native_union! { pub enum Dependency {
    TableScope = "table_scope" { table_uri: String => Rule::NonEmpty },
    Table = "table" { value: TableVersion => Rule::Text },
    Artifact = "artifact" { artifact_id: String => Rule::NonEmpty },
    Definition = "definition" { table: TableVersion => Rule::Text, definition_id: String => Rule::NonEmpty },
    PhysicalOwner = "physical_owner" { name: String => Rule::NonEmpty },
    CdfWindow = "cdf_window" {
        table_uri: String => Rule::NonEmpty,
        table_id: String => Rule::NonEmpty,
        contract_id: String => Rule::NonEmpty,
        start: u64 => Rule::Text,
        end: u64 => Rule::Text,
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
crate::native_struct! { pub struct RetentionLease {
    lease_id: String => Rule::NonEmpty,
    owner: String => Rule::NonEmpty,
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
    run_id: String => Rule::NonEmpty,
    table_uri: String => Rule::NonEmpty,
    owner: String => Rule::NonEmpty,
    process: NativeProcess => Rule::Text,
    generation: u64 => Rule::Text,
    predecessor: u64 => Rule::Text,
    policy_id: String => Rule::NonEmpty,
    policy: RetentionPolicy => Rule::Text,
    state: MaintenanceState => Rule::Text,
    protected: Vec<Dependency> => Rule::SequenceBounds { min: 0, max: 8192 },
    sequence: u64 => Rule::Text,
} }
crate::native_struct! { pub struct CleanupObligation {
    obligation_id: String => Rule::NonEmpty,
    owner: String => Rule::NonEmpty,
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
