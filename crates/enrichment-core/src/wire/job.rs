//! Job handles and pagination (blueprint §7.3, §8.3).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The persisted job states (blueprint §8.3).
///
/// The values are, in order: `queued`, `running`, `succeeded`, `partial`, `failed`,
/// `cancel_requested`, `cancelled`. `cancel_requested` is distinct from `cancelled` because
/// cancelling one caller's interest must not kill work another caller still needs.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(inline)]
pub enum JobState {
    Queued,
    Running,
    Succeeded,
    Partial,
    Failed,
    CancelRequested,
    Cancelled,
}

/// A receipt for submitted work. Never evidence that the work finished.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct JobHandle {
    /// Durable core job identity. Distinct from the request ID: jobs are shared and reusable.
    pub job_id: String,
    /// Where the job is now.
    pub state: JobState,
    /// Which stage is executing, for a caller to report progress.
    pub stage: String,
    /// Advisory next-poll delay. Not a completion-time guarantee.
    pub poll_after_ms: u64,
}

/// Bounded-result accounting (blueprint §7.3).
///
/// A first page is not the whole result set. A cursor binds the query digest, snapshot, filters
/// and sort, so a mismatched cursor is rejected with `INVALID_CURSOR` rather than producing an
/// inconsistent page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Pagination {
    /// How many entries this page carries.
    pub returned: u64,
    /// Total matches when known. `null` means "not counted", never "zero".
    pub total_matches: Option<u64>,
    /// Whether output was cut short by a budget.
    pub truncated: bool,
    /// Opaque cursor for the next page, or `null` at the end.
    pub next_cursor: Option<String>,
}
