//! Bounded operational observations copied from the physical plan that actually ran.
use crate::telemetry_history::History;
use datafusion::{
    logical_expr::LogicalPlan,
    physical_plan::{ExecutionPlan, display::DisplayableExecutionPlan, metrics::MetricValue},
};
use enrichment_core::telemetry::{Label, Metric, QueryDiagnostics, RuleTransition};
use std::{
    fmt::{self, Write},
    sync::Arc,
    time::Instant,
};

const TEXT_BYTES: usize = 16 * 1024;
const NODES: usize = 128;
const METRICS: usize = 1024;

fn fingerprint(plan: &LogicalPlan) -> Option<u64> {
    use datafusion::common::tree_node::{TreeNode, TreeNodeRecursion};
    use std::hash::{Hash, Hasher};
    let mut nodes = 0;
    let mut bounded = true;
    let mut expressions = 0;
    let mut literal_bytes = 0usize;
    plan.apply_with_subqueries(|node| {
        nodes += 1;
        if nodes > NODES {
            bounded = false;
            return Ok(TreeNodeRecursion::Stop);
        }
        for expr in node.expressions() {
            expr.apply(|value| {
                expressions += 1;
                if let datafusion::logical_expr::Expr::Literal(value, _) = value {
                    literal_bytes = literal_bytes.saturating_add(value.size());
                }
                if expressions > 1024 || literal_bytes > TEXT_BYTES {
                    bounded = false;
                    return Ok(TreeNodeRecursion::Stop);
                }
                Ok(TreeNodeRecursion::Continue)
            })?;
            if !bounded {
                return Ok(TreeNodeRecursion::Stop);
            }
        }
        Ok(TreeNodeRecursion::Continue)
    })
    .ok()?;
    if !bounded {
        return None;
    }
    let mut hasher = std::hash::DefaultHasher::new();
    plan.hash(&mut hasher);
    Some(hasher.finish())
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
    previous_fingerprint: Option<u64>,
    plan: Option<Arc<dyn ExecutionPlan>>,
    history: History,
    start: Instant,
    diagnostic: QueryDiagnostics,
    _operation: crate::runtime::OperationContext,
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
        let query_id = history.next_query();
        Self {
            previous_fingerprint: fingerprint(logical),
            _operation: crate::runtime::capture_operation(),
            plan: None,
            history,
            start,
            diagnostic: QueryDiagnostics {
                catalog: enrichment_core::telemetry::InventorySummary::default(),
                rules: Vec::new(),
                rules_truncated: false,
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
    pub(crate) fn bound(&mut self, state: &datafusion::execution::SessionState) {
        self.diagnostic.catalog = crate::native_catalog::summary(state);
    }
    pub(crate) fn rule(&mut self, phase: &str, rule: &str, plan: &LogicalPlan) {
        if self.diagnostic.rules.len() == 128 {
            self.diagnostic.rules_truncated = true;
            return;
        }
        let current = fingerprint(plan);
        let changed = self
            .previous_fingerprint
            .zip(current)
            .map(|(before, after)| before != after);
        self.previous_fingerprint = current;
        let rule = short(rule, 128, &mut self.diagnostic.rules_truncated);
        self.diagnostic.rules.push(RuleTransition {
            phase: phase.into(),
            rule,
            changed,
        });
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
    pub(crate) fn properties(
        &self,
    ) -> datafusion::error::Result<Arc<datafusion::physical_plan::PlanProperties>> {
        self.plan
            .as_ref()
            .map(|plan| Arc::clone(plan.properties()))
            .ok_or_else(|| {
                datafusion::error::DataFusionError::Internal(
                    "executed fold has no physical properties".into(),
                )
            })
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
        if !self.history.enabled() {
            return;
        }
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
                            .map(|label| Label {
                                name: short(label.name(), 64, truncated),
                                value: short(label.value(), 128, truncated),
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
        self.history.query(&self.diagnostic);
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

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::execution::memory_pool::{
        FairSpillPool, MemoryConsumer, MemoryPool, PeakRecordingPool, TrackConsumersPool,
    };

    #[tokio::test]
    async fn rule_history_and_shared_memory_peak_are_bounded_and_do_not_reset() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = crate::runtime::QueryRuntime::new(dir.path(), Default::default()).unwrap();
        let session = runtime.session();
        let frame = session.sql("SELECT 1 AS value").await.unwrap();
        let history = History::default();
        let mut trace = Trace::new(frame.logical_plan(), history.clone(), Instant::now());
        for _ in 0..200 {
            trace.rule("test", &"r".repeat(200), frame.logical_plan());
        }
        assert_eq!(trace.diagnostic.rules.len(), 128);
        assert!(trace.diagnostic.rules_truncated);
        assert!(
            trace
                .diagnostic
                .rules
                .iter()
                .all(|rule| rule.rule.len() <= 128 && rule.changed == Some(false))
        );
        let pool = session.runtime_env().memory_pool.clone();
        let first = MemoryConsumer::new("first_operation").register(&pool);
        let second = MemoryConsumer::new("second_operation").register(&pool);
        first.try_grow(1024).unwrap();
        second.try_grow(2048).unwrap();
        assert_eq!(
            runtime
                .operational_counters()
                .await
                .unwrap()
                .managed_memory_reserved_bytes,
            3072
        );
        drop(first);
        assert_eq!(
            runtime
                .operational_counters()
                .await
                .unwrap()
                .managed_memory_peak_bytes,
            3072
        );
        drop(second);
        assert_eq!(
            runtime
                .operational_counters()
                .await
                .unwrap()
                .managed_memory_reserved_bytes,
            0
        );
        assert_eq!(
            runtime
                .operational_counters()
                .await
                .unwrap()
                .managed_memory_peak_bytes,
            3072
        );
    }

    #[test]
    #[ignore = "manual paired P8 TrackConsumersPool overhead measurement"]
    fn paired_consumer_pool_observation() {
        for round in 0..3 {
            for tracked in [false, true] {
                let inner: Arc<dyn MemoryPool> = if tracked {
                    Arc::new(TrackConsumersPool::new(
                        FairSpillPool::new(8192),
                        std::num::NonZeroUsize::new(4).unwrap(),
                    ))
                } else {
                    Arc::new(FairSpillPool::new(8192))
                };
                let pool: Arc<dyn MemoryPool> = Arc::new(PeakRecordingPool::new(inner));
                let start = Instant::now();
                std::thread::scope(|scope| {
                    for client in 0..4 {
                        let pool = pool.clone();
                        scope.spawn(move || {
                            let reservation =
                                MemoryConsumer::new(format!("query_{client}")).register(&pool);
                            for _ in 0..10_000 {
                                reservation.try_grow(128).unwrap();
                                reservation.shrink(128);
                            }
                        });
                    }
                });
                let allocation_us = start.elapsed().as_micros();
                let first = MemoryConsumer::new("retained_index").register(&pool);
                first.try_grow(4096).unwrap();
                let failing = MemoryConsumer::new("query_sort").register(&pool);
                let failure = failing.try_grow(8192).unwrap_err();
                assert!(matches!(
                    failure,
                    datafusion::error::DataFusionError::ResourcesExhausted(_)
                ));
                let message = failure.to_string();
                assert_eq!(message.contains("retained_index"), tracked);
                assert!(message.len() < 2048);
                drop(first);
                drop(failing);
                assert_eq!(pool.reserved(), 0);
                eprintln!(
                    "P14_POOL {}",
                    serde_json::json!({"round":round,"tracked":tracked,"allocation_us":allocation_us,"error":message})
                );
            }
        }
    }
}
