# `datafusion_tracing::rule_instrumentation`

Crate `datafusion-tracing` · 26 public items · structured records in [`model/datafusion_tracing.rule_instrumentation.json`](../model/datafusion_tracing.rule_instrumentation.json)

## OPTIMIZER_PASS_TRACKER

`constant` · `datafusion_tracing::rule_instrumentation::OPTIMIZER_PASS_TRACKER`

```rust
const OPTIMIZER_PASS_TRACKER: thread::LocalKey<std::cell::RefCell<OptimizerPassTracker>> = _
```

Tracks optimizer pass count and the parent span ID to detect new queries.

---

## PLANNING_CONTEXT

`constant` · `datafusion_tracing::rule_instrumentation::PLANNING_CONTEXT`

```rust
const PLANNING_CONTEXT: thread::LocalKey<std::cell::RefCell<Option<PlanningContext>>> = _
```

The current planning context, if any phase is active.

---

## PlanningPhase

`enum` · `datafusion_tracing::rule_instrumentation::PlanningPhase`

```rust
enum PlanningPhase
```

**Variants**: `Analyzer`, `Optimizer`, `PhysicalOptimizer`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The phase of query planning currently active.

---

## close_phase_span

`function` · `datafusion_tracing::rule_instrumentation::close_phase_span`

```rust
fn close_phase_span<P: FormatPlan>(ctx: PlanningContext, plan_after: &P)
```

Closes a phase span, recording effective rules and plan diff if enabled.

The span is closed when `ctx` is dropped at the end of this function,
as `ctx._entered` holds the entered span guard.

---

## detect_and_record_modification

`function` · `datafusion_tracing::rule_instrumentation::detect_and_record_modification`

```rust
fn detect_and_record_modification(before_str: &str, after_str: &str, span: &tracing::Span, record_diff: bool, rule_name: &str)
```

Detects if a plan was modified and updates span attributes accordingly.

This function:
1. Compares plan string representations to detect actual changes
2. Generates and records a unified diff if `record_diff` is true
3. Updates the span's `otel.name` to indicate modification
4. Records the rule name in the planning context for effective_rules tracking

---

## drop_planning_context

`function` · `datafusion_tracing::rule_instrumentation::drop_planning_context`

```rust
fn drop_planning_context()
```

Drops the current planning context on a fatal rule error, closing the active
phase span and resetting `OPTIMIZER_PASS_TRACKER` for `Optimizer` and
`PhysicalOptimizer` phases so the next query starts at pass 0.

---

## generate_plan_diff

`function` · `datafusion_tracing::rule_instrumentation::generate_plan_diff`

```rust
fn generate_plan_diff(before: &str, after: &str) -> String
```

Generates a unified diff between two plan strings.

---

## instrument_analyzer_rules

`function` · `datafusion_tracing::rule_instrumentation::instrument_analyzer_rules`

```rust
fn instrument_analyzer_rules(rules: Vec<std::sync::Arc<dyn AnalyzerRule + Send + Sync>>, options: &rule_options::RuleInstrumentationOptions, span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>, phase_span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Vec<std::sync::Arc<dyn AnalyzerRule + Send + Sync>>
```

Instruments analyzer rules with phase sentinel and optional rule-level spans.

---

## instrument_optimizer_rules

`function` · `datafusion_tracing::rule_instrumentation::instrument_optimizer_rules`

```rust
fn instrument_optimizer_rules(rules: Vec<std::sync::Arc<dyn OptimizerRule + Send + Sync>>, options: &rule_options::RuleInstrumentationOptions, span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>, phase_span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Vec<std::sync::Arc<dyn OptimizerRule + Send + Sync>>
```

Instruments optimizer rules with phase sentinel and optional rule-level spans.

---

## instrument_physical_optimizer_rules

`function` · `datafusion_tracing::rule_instrumentation::instrument_physical_optimizer_rules`

```rust
fn instrument_physical_optimizer_rules(rules: Vec<std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>>, options: &rule_options::RuleInstrumentationOptions, span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>, phase_span_create_fn: &std::sync::Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Vec<std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>>
```

Instruments physical optimizer rules with phase sentinel and optional rule-level spans.

---

## record_modified_rule_in_context

`function` · `datafusion_tracing::rule_instrumentation::record_modified_rule_in_context`

```rust
fn record_modified_rule_in_context(rule_name: &str)
```

Records that a rule modified the plan in the current phase.

---

## AnalyzerPhaseSentinel

`struct` · `datafusion_tracing::rule_instrumentation::AnalyzerPhaseSentinel`

```rust
struct AnalyzerPhaseSentinel
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, _config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

Sentinel analyzer rule that toggles the phase span.
First call opens the span, second call closes it.

---

## ErrorCleanupAnalyzerRule

`struct` · `datafusion_tracing::rule_instrumentation::ErrorCleanupAnalyzerRule`

```rust
struct ErrorCleanupAnalyzerRule
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

---

## ErrorCleanupOptimizerRule

`struct` · `datafusion_tracing::rule_instrumentation::ErrorCleanupOptimizerRule`

```rust
struct ErrorCleanupOptimizerRule
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

---

## ErrorCleanupPhysicalOptimizerRule

`struct` · `datafusion_tracing::rule_instrumentation::ErrorCleanupPhysicalOptimizerRule`

```rust
struct ErrorCleanupPhysicalOptimizerRule
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

---

## InstrumentedAnalyzerRule

`struct` · `datafusion_tracing::rule_instrumentation::InstrumentedAnalyzerRule`

```rust
struct InstrumentedAnalyzerRule
```

**Implements**: `datafusion_optimizer::analyzer::AnalyzerRule`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn AnalyzerRule + Send + Sync>, options: RuleInstrumentationOptions, span_create_fn: Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Self
```

**via `datafusion_optimizer::analyzer::AnalyzerRule`**

```rust
fn analyze(&self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan>
fn name(&self) -> &str
```

A wrapper for an `AnalyzerRule` that adds tracing instrumentation.

---

## InstrumentedOptimizerRule

`struct` · `datafusion_tracing::rule_instrumentation::InstrumentedOptimizerRule`

```rust
struct InstrumentedOptimizerRule
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug

**Methods** (2)

```rust
fn apply_inner(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn new(inner: Arc<dyn OptimizerRule + Send + Sync>, options: RuleInstrumentationOptions, span_create_fn: Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Self
```

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

A wrapper for an `OptimizerRule` that adds tracing instrumentation.

---

## InstrumentedPhysicalOptimizerRule

`struct` · `datafusion_tracing::rule_instrumentation::InstrumentedPhysicalOptimizerRule`

```rust
struct InstrumentedPhysicalOptimizerRule
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug

**Methods** (1)

```rust
fn new(inner: Arc<dyn PhysicalOptimizerRule + Send + Sync>, options: RuleInstrumentationOptions, span_create_fn: Arc<dyn Fn(&str) -> tracing::Span + Send + Sync>) -> Self
```

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

A wrapper for a `PhysicalOptimizerRule` that adds tracing instrumentation.

---

## OptimizerPassTracker

`struct` · `datafusion_tracing::rule_instrumentation::OptimizerPassTracker`

```rust
struct OptimizerPassTracker
```

**Methods** (3)

```rust
fn get_and_increment(&mut self, current_parent_id: Option<tracing::span::Id>) -> usize
const fn new() -> Self
fn reset(&mut self)
```

Tracks the optimizer pass count, resetting when a new query starts.
A new query is detected by a change in the parent span ID or when
the physical optimizer phase completes.

---

## OptimizerPhaseSentinel

`struct` · `datafusion_tracing::rule_instrumentation::OptimizerPhaseSentinel`

```rust
struct OptimizerPhaseSentinel
```

**Implements**: `datafusion_optimizer::optimizer::OptimizerRule`

**Derives**: Debug

**via `datafusion_optimizer::optimizer::OptimizerRule`**

```rust
fn apply_order(&self) -> Option<ApplyOrder>
fn name(&self) -> &str
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
fn supports_rewrite(&self) -> bool
```

Sentinel optimizer rule that toggles the phase span.
First call opens the span, second call closes it.

---

## PhysicalOptimizerPhaseSentinel

`struct` · `datafusion_tracing::rule_instrumentation::PhysicalOptimizerPhaseSentinel`

```rust
struct PhysicalOptimizerPhaseSentinel
```

**Implements**: `datafusion_session::physical_optimizer::PhysicalOptimizerRule`

**Derives**: Debug

**via `datafusion_session::physical_optimizer::PhysicalOptimizerRule`**

```rust
fn name(&self) -> &str
fn optimize(&self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>>
fn schema_check(&self) -> bool
```

Sentinel physical optimizer rule that toggles the phase span.
First call opens the span, second call closes it.

---

## PlanningContext

`struct` · `datafusion_tracing::rule_instrumentation::PlanningContext`

```rust
struct PlanningContext
```

Context for tracking the active planning phase span.

Stores the current phase, the entered span guard, and state needed for
computing plan diffs and tracking which rules modified the plan.

---

## SingleSpanTreeTraverser

`struct` · `datafusion_tracing::rule_instrumentation::SingleSpanTreeTraverser`

```rust
struct SingleSpanTreeTraverser<'a>
```

**Implements**: `datafusion_common::tree_node::TreeNodeRewriter`

**Methods** (1)

```rust
fn new(apply_order: ApplyOrder, rule: &'a dyn OptimizerRule, config: &'a dyn OptimizerConfig) -> Self
```

**via `datafusion_common::tree_node::TreeNodeRewriter`**

```rust
fn f_down(&mut self, node: LogicalPlan) -> Result<Transformed<LogicalPlan>>
fn f_up(&mut self, node: LogicalPlan) -> Result<Transformed<LogicalPlan>>
```

Applies an optimizer rule across the tree manually, keeping all work under a single span.

When a rule specifies `apply_order()` as `Some(TopDown)` or `Some(BottomUp)`, the optimizer
framework would normally handle tree traversal, calling the rule's `rewrite` method on each
node. This would result in one span per node if we simply wrapped the inner rule.

Instead, `SingleSpanTreeTraverser` takes over tree traversal so that the entire rule
application (across all nodes) is consolidated under a single tracing span. This gives
cleaner traces that show one span per rule rather than one span per (rule × node) combination.

---

## FormatPlan

`trait` · `datafusion_tracing::rule_instrumentation::FormatPlan`

```rust
trait FormatPlan
```

**Implementors** (2)

- `alloc::sync::Arc`
- `datafusion_expr::logical_plan::plan::LogicalPlan`

**Methods** (1)

```rust
fn format_for_diff(&self) -> String
```

Trait for formatting plans as strings for diffing and tracing.

This provides a unified interface for converting different plan types
(logical and physical) to their string representations.

---

## PhaseSpanCreateFn

`type_alias` · `datafusion_tracing::rule_instrumentation::PhaseSpanCreateFn`

```rust
type PhaseSpanCreateFn = dyn Fn(&str) -> tracing::Span + Send + Sync
```

Type alias for a function that creates a phase span given the phase name.
Example phase names: "analyze_logical_plan", "optimize_logical_plan", "optimize_physical_plan".

---

## RuleSpanCreateFn

`type_alias` · `datafusion_tracing::rule_instrumentation::RuleSpanCreateFn`

```rust
type RuleSpanCreateFn = dyn Fn(&str) -> tracing::Span + Send + Sync
```

Function type that creates a span for a rule, given the rule name.

---
