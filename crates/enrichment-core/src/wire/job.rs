//! Job handles and pagination (blueprint §7.3, §8.3).

crate::native_vocabulary! {
/// The persisted job states (blueprint §8.3).
///
/// The values are, in order: `queued`, `running`, `succeeded`, `partial`, `failed`,
/// `cancel_requested`, `cancelled`. `cancel_requested` is distinct from `cancelled` because
/// cancelling one caller's interest must not kill work another caller still needs.
///
/// No variant carries a doc comment -- see the module docs in [`super`](crate::wire).
#[derive(Hash)]
#[schemars(inline)]
pub enum JobState {
    Queued = "queued",
    Running = "running",
    Succeeded = "succeeded",
    Partial = "partial",
    Failed = "failed",
    CancelRequested = "cancel_requested",
    Cancelled = "cancelled",
}
}

impl JobState {
    pub const TERMINAL: [Self; 4] = [
        Self::Succeeded,
        Self::Partial,
        Self::Failed,
        Self::Cancelled,
    ];
}

crate::native_struct! {
/// A receipt for submitted work. Never evidence that the work finished.
pub struct JobHandle {
    /// Durable core job identity. Distinct from the request ID: jobs are shared and reusable.
    job_id: String => crate::native_union::Rule::Text,
    /// Where the job is now.
    state: JobState => crate::native_union::Rule::Text,
    /// Which stage is executing, for a caller to report progress.
    stage: String => crate::native_union::Rule::Text,
    /// Advisory next-poll delay. Not a completion-time guarantee.
    poll_after_ms: u64 => crate::native_union::Rule::Text,
}
}

crate::native_struct! { @checked RawPage;
/// Bounded-result accounting (blueprint §7.3).
///
/// A first page is not the whole result set. A cursor binds the query digest, snapshot, filters
/// and sort, so a mismatched cursor is rejected with `INVALID_CURSOR` rather than producing an
/// inconsistent page.
#[serde(try_from = "RawPage")]
#[schemars(deny_unknown_fields, transform = page_conditionals)]
pub struct Page {
    /// How many entries this page carries.
    returned: u64 => crate::native_union::Rule::Text,
    /// Explicit exact, lower-bound or unknown count; never infer absence from a page.
    count: super::research::MatchCount => crate::native_union::Rule::Text,
    /// Whether another nonempty page can be requested with next_cursor.
    has_more: bool => crate::native_union::Rule::Text,
    /// Opaque cursor for the next page, or `null` at the end.
    next_cursor: Option<String> => crate::native_union::Rule::Text,
}
}

impl Page {
    /// Construct a page with an exact count when available, explicitly unknown otherwise.
    #[must_use]
    pub fn new(
        returned: u64,
        total: Option<u64>,
        has_more: bool,
        next_cursor: Option<String>,
    ) -> Self {
        assert_eq!(
            has_more,
            next_cursor.is_some(),
            "native continuation invariant"
        );
        assert!(!has_more || returned > 0, "native pages must make progress");
        assert!(
            total.is_none_or(|n| n >= returned),
            "native exact count invariant"
        );
        Self {
            returned,
            count: total.map_or(super::research::MatchCount::Unknown, |value| {
                super::research::MatchCount::Exact { value }
            }),
            has_more,
            next_cursor,
        }
    }
}

impl TryFrom<RawPage> for Page {
    type Error = &'static str;
    fn try_from(raw: RawPage) -> Result<Self, Self::Error> {
        if raw.has_more != raw.next_cursor.is_some()
            || raw.has_more && raw.returned == 0
            || matches!(raw.count, super::research::MatchCount::Exact { value } if value < raw.returned)
            || raw.next_cursor.as_ref().is_some_and(|cursor| {
                cursor.is_empty() || cursor.len() > 32768 || !cursor.is_ascii()
            })
        {
            return Err(
                "a continuing page requires rows and a bounded nonempty cursor; a final page has no cursor",
            );
        }
        Ok(Self {
            returned: raw.returned,
            count: raw.count,
            has_more: raw.has_more,
            next_cursor: raw.next_cursor,
        })
    }
}

fn page_conditionals(schema: &mut schemars::Schema) {
    schema.insert("allOf".into(), serde_json::json!([
        {
            "if": {"properties": {"has_more": {"const": true}}},
            "then": {"properties": {"returned": {"pattern": "^[1-9][0-9]*$"}, "next_cursor": {"type": "string", "minLength": 1, "maxLength": 32768, "pattern": "^[\\x00-\\x7F]+$"}}},
            "else": {"properties": {"next_cursor": {"type": "null"}}}
        }
    ]));
}

impl Default for Page {
    fn default() -> Self {
        Self::new(0, None, false, None)
    }
}
