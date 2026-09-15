//! Bounded operational observations copied from the physical plan that actually ran.
use datafusion::{
    logical_expr::LogicalPlan,
    physical_plan::{ExecutionPlan, display::DisplayableExecutionPlan, metrics::MetricValue},
};
use serde::Serialize;
use std::{
    collections::VecDeque,
    fmt::{self, Write},
    sync::{Arc, Mutex},
    time::Instant,
};

const TEXT_BYTES: usize = 16 * 1024;
const NODES: usize = 128;
const METRICS: usize = 1024;
const HISTORY: usize = 8;

#[derive(Clone, Debug, Serialize)]
pub struct Metric {
    pub node: usize,
    pub name: String,
    pub partition: Option<usize>,
    pub labels: Vec<(String, String)>,
    /// Counts/gauges/time values are copied, never live shared counters.
    pub value: Option<usize>,
    pub unit: &'static str,
    pub pruned: Option<usize>,
    pub matched: Option<usize>,
    pub part: Option<usize>,
    pub total: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryDiagnostics {
    pub query_id: u64,
    pub logical: String,
    pub physical: String,
    pub truncated: bool,
    pub metrics_truncated: bool,
    /// False includes failed execution, timeout and dropped requests; counters may be partial.
    pub completed: bool,
    pub planning_micros: u64,
    pub elapsed_micros: u64,
    pub output_rows: usize,
    pub output_arrow_bytes: usize,
    pub metrics: Vec<Metric>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Summary {
    pub executions: u64,
    pub completed: u64,
    pub planning_micros: u64,
    /// Sum of query elapsed durations; concurrent durations overlap and are not wall time.
    pub elapsed_micros: u64,
}
#[derive(Default)]
struct HistoryState {
    entries: VecDeque<QueryDiagnostics>,
    summary: Summary,
}
#[derive(Clone, Default)]
pub(crate) struct History(Arc<Mutex<HistoryState>>);
impl History {
    pub(crate) fn read(&self) -> Vec<QueryDiagnostics> {
        self.0
            .lock()
            .map(|h| h.entries.iter().cloned().collect())
            .unwrap_or_default()
    }
    pub(crate) fn summary(&self) -> Summary {
        self.0.lock().map(|h| h.summary.clone()).unwrap_or_default()
    }
}

struct Capped {
    text: String,
    limit: usize,
    truncated: bool,
}
impl Capped {
    fn new(limit: usize) -> Self {
        Self {
            text: String::new(),
            limit,
            truncated: false,
        }
    }
}
impl Write for Capped {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let available = self.limit.saturating_sub(self.text.len());
        if text.len() <= available {
            self.text.push_str(text);
            return Ok(());
        }
        let mut end = available;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        self.text.push_str(&text[..end]);
        self.truncated = true;
        Err(fmt::Error)
    }
}
fn short(text: &str, cap: usize, truncated: &mut bool) -> String {
    let mut out = Capped::new(cap);
    let _ = out.write_str(text);
    *truncated |= out.truncated;
    out.text
}

/// Owns the executed plan until its stream has dropped, including early-return paths.
pub(crate) struct Trace {
    plan: Arc<dyn ExecutionPlan>,
    history: History,
    start: Instant,
    diagnostic: QueryDiagnostics,
}
impl Trace {
    pub(crate) fn new(
        plan: Arc<dyn ExecutionPlan>,
        logical: &LogicalPlan,
        history: History,
        start: Instant,
        planning_micros: u64,
    ) -> Self {
        let mut text = Capped::new(TEXT_BYTES);
        let mut nodes = vec![(logical, 0usize)];
        let mut count = 0;
        while let Some((node, depth)) = nodes.pop() {
            if count >= NODES || depth > 32 {
                text.truncated = true;
                break;
            }
            count += 1;
            if writeln!(text, "{depth}: {}", node.display()).is_err() {
                break;
            }
            let children = node.inputs();
            if children.len() + nodes.len() > NODES {
                text.truncated = true;
                break;
            }
            nodes.extend(children.into_iter().rev().map(|child| (child, depth + 1)));
        }
        Self {
            plan,
            history,
            start,
            diagnostic: QueryDiagnostics {
                query_id: 0,
                logical: text.text,
                physical: String::new(),
                truncated: text.truncated,
                metrics_truncated: false,
                completed: false,
                planning_micros,
                elapsed_micros: 0,
                output_rows: 0,
                output_arrow_bytes: 0,
                metrics: vec![],
            },
        }
    }
    pub(crate) fn completed(&mut self, rows: usize, bytes: usize) {
        self.diagnostic.completed = true;
        self.diagnostic.output_rows = rows;
        self.diagnostic.output_arrow_bytes = bytes;
    }
}
impl Drop for Trace {
    fn drop(&mut self) {
        let mut text = Capped::new(TEXT_BYTES);
        let mut stack = vec![(Arc::clone(&self.plan), 0usize)];
        let mut count = 0;
        while let Some((plan, depth)) = stack.pop() {
            if count >= NODES || depth > 32 {
                text.truncated = true;
                self.diagnostic.metrics_truncated = true;
                break;
            }
            let index = count;
            count += 1;
            if !text.truncated {
                let _ = writeln!(
                    text,
                    "{index} depth={depth}: {}",
                    DisplayableExecutionPlan::new(plan.as_ref()).one_line()
                );
            }
            if let Some(metrics) = plan.metrics() {
                for metric in metrics.iter() {
                    // A closed diagnostic selection leaves room for every scan instead of
                    // allowing one file's auxiliary timings to crowd out later operators.
                    if !selected_metric(metric.value()) {
                        continue;
                    }
                    if self.diagnostic.metrics.len() >= METRICS {
                        self.diagnostic.truncated = true;
                        self.diagnostic.metrics_truncated = true;
                        break;
                    }
                    let value = metric.value();
                    let (scalar, unit, pruned, matched, part, total) = match value {
                        MetricValue::PruningMetrics {
                            pruning_metrics, ..
                        } => (
                            None,
                            "pruning",
                            Some(pruning_metrics.pruned()),
                            Some(pruning_metrics.matched()),
                            None,
                            None,
                        ),
                        MetricValue::Ratio { ratio_metrics, .. } => (
                            None,
                            "ratio",
                            None,
                            None,
                            Some(ratio_metrics.part()),
                            Some(ratio_metrics.total()),
                        ),
                        MetricValue::StartTimestamp(_)
                        | MetricValue::EndTimestamp(_)
                        | MetricValue::Custom { .. } => {
                            (None, "unavailable", None, None, None, None)
                        }
                        MetricValue::ElapsedCompute(_) | MetricValue::Time { .. } => (
                            Some(value.as_usize()),
                            "nanoseconds",
                            None,
                            None,
                            None,
                            None,
                        ),
                        _ => (
                            Some(value.as_usize()),
                            "operator_defined",
                            None,
                            None,
                            None,
                            None,
                        ),
                    };
                    let truncated = &mut self.diagnostic.metrics_truncated;
                    if metric.labels().len() > 4 {
                        *truncated = true;
                    }
                    self.diagnostic.metrics.push(Metric {
                        node: index,
                        name: short(value.name(), 128, truncated),
                        partition: metric.partition(),
                        labels: metric
                            .labels()
                            .iter()
                            .take(4)
                            .map(|label| {
                                (
                                    short(label.name(), 64, truncated),
                                    short(label.value(), 128, truncated),
                                )
                            })
                            .collect(),
                        value: scalar,
                        unit,
                        pruned,
                        matched,
                        part,
                        total,
                    });
                }
            }
            let children = plan.children();
            if children.len() + stack.len() > NODES {
                text.truncated = true;
                self.diagnostic.metrics_truncated = true;
                break;
            }
            stack.extend(
                children
                    .into_iter()
                    .rev()
                    .map(|child| (Arc::clone(child), depth + 1)),
            );
        }
        self.diagnostic.physical = text.text;
        self.diagnostic.truncated |= text.truncated || self.diagnostic.metrics_truncated;
        self.diagnostic.elapsed_micros =
            u64::try_from(self.start.elapsed().as_micros()).unwrap_or(u64::MAX);
        if let Ok(mut history) = self.history.0.lock() {
            history.summary.executions = history.summary.executions.saturating_add(1);
            history.summary.completed = history
                .summary
                .completed
                .saturating_add(u64::from(self.diagnostic.completed));
            history.summary.planning_micros = history
                .summary
                .planning_micros
                .saturating_add(self.diagnostic.planning_micros);
            history.summary.elapsed_micros = history
                .summary
                .elapsed_micros
                .saturating_add(self.diagnostic.elapsed_micros);
            self.diagnostic.query_id = history.summary.executions;
            while history.entries.len() >= HISTORY {
                history.entries.pop_front();
            }
            history.entries.push_back(self.diagnostic.clone());
        }
    }
}

fn selected_metric(value: &MetricValue) -> bool {
    match value {
        MetricValue::Count { name, .. }
        | MetricValue::Gauge { name, .. }
        | MetricValue::Time { name, .. } => matches!(
            name.as_ref(),
            "bytes_scanned" | "predicate_evaluation_errors" | "metadata_load_time"
        ),
        MetricValue::StartTimestamp(_)
        | MetricValue::EndTimestamp(_)
        | MetricValue::Custom { .. } => false,
        _ => true,
    }
}
