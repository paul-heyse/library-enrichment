//! Bounded operational observations copied from the physical plan that actually ran.
use datafusion::{
    logical_expr::LogicalPlan,
    physical_plan::{ExecutionPlan, display::DisplayableExecutionPlan, metrics::MetricValue},
};
use serde::{Deserialize, Serialize};
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
const FAILURE_HISTORY: usize = 32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metric {
    pub node: usize,
    pub name: String,
    pub partition: Option<usize>,
    pub labels: Vec<(String, String)>,
    /// Counts/gauges/time values are copied, never live shared counters.
    pub value: Option<usize>,
    pub unit: String,
    pub pruned: Option<usize>,
    pub matched: Option<usize>,
    pub part: Option<usize>,
    pub total: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryDiagnostics {
    pub query_id: u64,
    pub operation_id: Option<String>,
    pub binding: Option<crate::runtime::OperationBinding>,
    pub relations: Vec<String>,
    pub functions: Vec<String>,
    pub inventory_truncated: bool,
    pub logical: String,
    pub stage: String,
    pub family: Option<String>,
    pub analyzed: String,
    pub analysis_micros: u64,
    pub optimization_micros: u64,
    pub physical: String,
    pub truncated: bool,
    pub metrics_truncated: bool,
    /// False includes failed execution, timeout and dropped requests; counters may be partial.
    pub completed: bool,
    /// Time waiting for the shared query permit before native planning starts.
    #[serde(default)]
    pub queue_micros: u64,
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
    pub index_materializations: u64,
    pub index_spill_bytes: u64,
    pub index_reads: u64,
    pub queue_micros: u64,
    pub output_rows: u64,
    pub output_arrow_bytes: u64,
}
impl Summary {
    pub(crate) fn query(&mut self, query: &QueryDiagnostics) {
        self.executions = self.executions.saturating_add(1);
        self.completed = self.completed.saturating_add(u64::from(query.completed));
        self.planning_micros = self.planning_micros.saturating_add(query.planning_micros);
        self.elapsed_micros = self.elapsed_micros.saturating_add(query.elapsed_micros);
        self.queue_micros = self.queue_micros.saturating_add(query.queue_micros);
        self.output_rows = self.output_rows.saturating_add(query.output_rows as u64);
        self.output_arrow_bytes = self
            .output_arrow_bytes
            .saturating_add(query.output_arrow_bytes as u64);
    }
}
#[derive(Default)]
struct HistoryState {
    next_query_id: u64,
    entries: VecDeque<QueryDiagnostics>,
    summary: Summary,
    failures: VecDeque<Failure>,
    operations: VecDeque<crate::runtime::OperationDiagnostics>,
}
#[derive(Clone, Serialize, Deserialize)]
struct Failure {
    recorded_at: String,
    diagnostic: enrichment_core::wire::Diagnostic,
    /// The most recent executed query for this operation, if available. This is supporting
    /// operation context, not a claim that preparation necessarily reached physical execution.
    last_operation_query: Option<QueryDiagnostics>,
}
#[derive(Clone, Default)]
pub(crate) struct History(Arc<Mutex<HistoryState>>, Option<std::path::PathBuf>);
impl History {
    pub(crate) fn persistent(path: std::path::PathBuf) -> std::io::Result<Self> {
        use std::io::Read;
        let failures = match std::fs::File::open(&path) {
            Ok(file) => {
                let mut bytes = Vec::new();
                file.take(8 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
                if bytes.len() > 8 * 1024 * 1024 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "native failure history exceeds bound",
                    ));
                }
                serde_json::from_slice::<VecDeque<Failure>>(&bytes)?
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => VecDeque::new(),
            Err(e) => return Err(e),
        };
        if failures.len() > FAILURE_HISTORY {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "native failure history exceeds entry bound",
            ));
        }
        Ok(Self(
            Arc::new(Mutex::new(HistoryState {
                failures,
                ..Default::default()
            })),
            Some(path),
        ))
    }

    pub(crate) fn failure(&self, diagnostic: enrichment_core::wire::Diagnostic) {
        let Ok(mut history) = self.0.lock() else {
            return;
        };
        let last_operation_query = diagnostic.correlation_id.as_ref().and_then(|id| {
            history
                .entries
                .iter()
                .rev()
                .find(|q| q.operation_id.as_ref() == Some(id))
                .cloned()
        });
        while history.failures.len() >= FAILURE_HISTORY {
            history.failures.pop_front();
        }
        history.failures.push_back(Failure {
            recorded_at: enrichment_core::clock::now_rfc3339(),
            diagnostic,
            last_operation_query,
        });
        if let Some(path) = &self.1 {
            let written = (|| -> std::io::Result<()> {
                loop {
                    let bytes = serde_json::to_vec(&history.failures)?;
                    if bytes.len() <= 4 * 1024 * 1024 {
                        return crate::atomic::write_atomic(path, &bytes);
                    }
                    if history.failures.len() > 1 {
                        history.failures.pop_front();
                    } else if let Some(failure) = history.failures.front_mut()
                        && failure.last_operation_query.is_some()
                    {
                        failure.last_operation_query = None;
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::OutOfMemory,
                            "failure diagnostic exceeds retained history byte bound",
                        ));
                    }
                }
            })();
            if let Err(error) = written {
                eprintln!("native failure diagnostics could not be retained: {error}");
            }
        }
    }
    pub(crate) fn read(&self) -> Vec<QueryDiagnostics> {
        self.0
            .lock()
            .map(|h| h.entries.iter().cloned().collect())
            .unwrap_or_default()
    }
    pub(crate) fn summary(&self) -> Summary {
        self.0.lock().map(|h| h.summary.clone()).unwrap_or_default()
    }
    pub(crate) fn materialized_index(&self, bytes: u64) {
        if let Ok(mut history) = self.0.lock() {
            history.summary.index_materializations =
                history.summary.index_materializations.saturating_add(1);
            history.summary.index_spill_bytes =
                history.summary.index_spill_bytes.saturating_add(bytes);
        }
    }
    pub(crate) fn index_read(&self) {
        if let Ok(mut history) = self.0.lock() {
            history.summary.index_reads = history.summary.index_reads.saturating_add(1);
        }
    }
    pub(crate) fn operation(&self, observation: crate::runtime::OperationDiagnostics) {
        if let Ok(mut history) = self.0.lock() {
            while history.operations.len() >= 32 {
                history.operations.pop_front();
            }
            history.operations.push_back(observation);
        }
    }
    pub(crate) fn operations(&self) -> Vec<crate::runtime::OperationDiagnostics> {
        self.0
            .lock()
            .map(|history| history.operations.iter().cloned().collect())
            .unwrap_or_default()
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
    plan: Option<Arc<dyn ExecutionPlan>>,
    history: History,
    start: Instant,
    diagnostic: QueryDiagnostics,
    operation: crate::runtime::OperationContext,
}
impl Trace {
    pub(crate) fn new(logical: &LogicalPlan, history: History, start: Instant) -> Self {
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
        let query_id = history
            .0
            .lock()
            .map(|mut h| {
                h.next_query_id = h.next_query_id.saturating_add(1);
                h.next_query_id
            })
            .unwrap_or(0);
        Self {
            operation: crate::runtime::capture_operation(),
            plan: None,
            history,
            start,
            diagnostic: QueryDiagnostics {
                query_id,
                stage: "analysis".into(),
                family: None,
                analyzed: String::new(),
                analysis_micros: 0,
                optimization_micros: 0,
                operation_id: crate::runtime::operation_id(),
                binding: crate::runtime::operation_binding(),
                relations: Vec::new(),
                functions: Vec::new(),
                inventory_truncated: false,
                logical: text.text,
                physical: String::new(),
                truncated: text.truncated,
                metrics_truncated: false,
                completed: false,
                queue_micros: u64::try_from(start.elapsed().as_micros()).unwrap_or(u64::MAX),
                planning_micros: 0,
                elapsed_micros: 0,
                output_rows: 0,
                output_arrow_bytes: 0,
                metrics: vec![],
            },
        }
    }
    pub(crate) fn family(&mut self, family: crate::preparation::QueryFamily) {
        self.diagnostic.family = Some(format!("{family:?}"));
    }

    fn inventory(&mut self, logical: &LogicalPlan) {
        use datafusion::{
            common::tree_node::{TreeNode, TreeNodeRecursion},
            logical_expr::Expr,
        };
        use std::collections::BTreeSet;
        let mut relations = BTreeSet::new();
        let mut functions = BTreeSet::new();
        let mut nodes = 0;
        let mut expressions = 0;
        let mut truncated = false;
        let result = logical.apply_with_subqueries(|plan| {
            nodes += 1;
            if nodes > NODES {
                truncated = true;
                return Ok(TreeNodeRecursion::Stop);
            }
            if let LogicalPlan::TableScan(scan) = plan {
                let schema = scan.source.schema();
                let label = format!(
                    "{} [contract={}, relation={}]",
                    scan.table_name,
                    schema
                        .metadata()
                        .get("enrichment.contract")
                        .map_or("native", String::as_str),
                    schema
                        .metadata()
                        .get("enrichment.relation")
                        .map_or("transient", String::as_str)
                );
                relations.insert(short(&label, 512, &mut truncated));
            }
            for field in plan.schema().fields() {
                if let Some(identity) = field.metadata().get("enrichment.function") {
                    functions.insert(short(identity, 512, &mut truncated));
                }
            }
            plan.apply_expressions(|expression| {
                expression.apply(|expression| {
                    expressions += 1;
                    if expressions > 512 {
                        truncated = true;
                        return Ok(TreeNodeRecursion::Stop);
                    }
                    let name = match expression {
                        Expr::ScalarFunction(function) => Some(function.func.name()),
                        Expr::AggregateFunction(function) => Some(function.func.name()),
                        Expr::WindowFunction(function) => Some(function.fun.name()),
                        _ => None,
                    };
                    if let Some(name) = name {
                        functions.insert(short(name, 256, &mut truncated));
                    }
                    Ok(TreeNodeRecursion::Continue)
                })
            })?;
            if relations.len() > 64 || functions.len() > 64 || expressions > 512 {
                truncated = true;
                return Ok(TreeNodeRecursion::Stop);
            }
            Ok(TreeNodeRecursion::Continue)
        });
        self.diagnostic.inventory_truncated = truncated || result.is_err();
        self.diagnostic.relations = relations.into_iter().take(64).collect();
        self.diagnostic.functions = functions.into_iter().take(64).collect();
    }
    pub(crate) fn analyzed(&mut self, logical: &LogicalPlan, micros: u64) {
        let mut text = Capped::new(TEXT_BYTES);
        let _ = write!(text, "{}", logical.display_indent());
        self.diagnostic.analyzed = text.text;
        self.diagnostic.truncated |= text.truncated;
        self.diagnostic.analysis_micros = micros;
        self.diagnostic.stage = "optimization".into();
    }
    pub(crate) fn optimized(&mut self, logical: &LogicalPlan, micros: u64) {
        self.inventory(logical);
        let mut text = Capped::new(TEXT_BYTES);
        let _ = write!(text, "{}", logical.display_indent());
        self.diagnostic.logical = text.text;
        self.diagnostic.truncated |= text.truncated;
        self.diagnostic.optimization_micros = micros;
        self.diagnostic.stage = "physical_planning".into();
    }
    pub(crate) fn physical(&mut self, plan: Arc<dyn ExecutionPlan>, micros: u64) {
        self.plan = Some(plan);
        self.diagnostic.planning_micros = micros;
        self.diagnostic.stage = "execution".into();
    }
    pub(crate) fn completed(&mut self, rows: usize, bytes: usize) {
        self.diagnostic.completed = true;
        self.diagnostic.stage = "completed".into();
        self.diagnostic.output_rows = rows;
        self.diagnostic.output_arrow_bytes = bytes;
    }
}
impl Drop for Trace {
    fn drop(&mut self) {
        let mut text = Capped::new(TEXT_BYTES);
        let mut stack = self
            .plan
            .iter()
            .map(|plan| (Arc::clone(plan), 0usize))
            .collect::<Vec<_>>();
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
                        unit: unit.into(),
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
        self.operation.query(&self.diagnostic);
        if let Ok(mut history) = self.history.0.lock() {
            history.summary.query(&self.diagnostic);

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
