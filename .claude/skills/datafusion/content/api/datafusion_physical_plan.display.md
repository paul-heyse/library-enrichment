# `datafusion_physical_plan::display`

Crate `datafusion-physical-plan` · 7 public items · structured records in [`model/datafusion_physical_plan.display.json`](../model/datafusion_physical_plan.display.json)

## DisplayFormatType

`enum` · `datafusion_physical_plan::display::DisplayFormatType`

Also reachable as `datafusion::physical_plan::DisplayFormatType`, `datafusion_physical_plan::DisplayFormatType`, `datafusion_physical_plan::execution_plan::DisplayFormatType`

```rust
enum DisplayFormatType
```

**Variants**: `Default`, `Verbose`, `TreeRender`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.DisplayFormatType.md).


Options for controlling how each [`ExecutionPlan`] should format itself

---

## display_orderings

`function` · `datafusion_physical_plan::display::display_orderings`

```rust
fn display_orderings(f: &mut std::fmt::Formatter<'_>, orderings: &[datafusion_physical_expr::LexOrdering]) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.display_orderings.md).


---

## DefaultDisplay

`struct` · `datafusion_physical_plan::display::DefaultDisplay`

Also reachable as `datafusion::physical_plan::DefaultDisplay`, `datafusion_physical_plan::DefaultDisplay`, `datafusion_physical_plan::execution_plan::DefaultDisplay`

```rust
struct DefaultDisplay<T>
```

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.DefaultDisplay.md).


A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

---

## DisplayableExecutionPlan

`struct` · `datafusion_physical_plan::display::DisplayableExecutionPlan`

```rust
struct DisplayableExecutionPlan<'a>
```

**Derives**: Clone, Debug

**Methods** (16)

```rust
fn graphviz(&self) -> impl fmt::Display + 'a
fn indent(&self, verbose: bool) -> impl fmt::Display + 'a
fn new(inner: &'a dyn ExecutionPlan) -> Self
fn one_line(&self) -> impl fmt::Display + 'a
fn pgjson(&self, verbose: bool) -> impl fmt::Display + 'a
fn set_metric_categories(self, metric_categories: Option<Vec<MetricCategory>>) -> Self
fn set_metric_names(self, metric_names: Vec<String>) -> Self
fn set_metric_types(self, metric_types: Vec<MetricType>) -> Self
fn set_show_schema(self, show_schema: bool) -> Self
fn set_show_statistics(self, show_statistics: bool) -> Self
fn set_summary(self, total_rows: Option<usize>, duration: Option<Duration>) -> Self
fn set_tree_maximum_render_width(self, width: usize) -> Self
fn to_stringified(&self, verbose: bool, plan_type: PlanType, explain_format: DisplayFormatType) -> StringifiedPlan
fn tree_render(&self) -> impl fmt::Display + 'a
fn with_full_metrics(inner: &'a dyn ExecutionPlan) -> Self
fn with_metrics(inner: &'a dyn ExecutionPlan) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.DisplayableExecutionPlan.md).


Wraps an `ExecutionPlan` with various methods for formatting


# Example
```
# use std::sync::Arc;
# use arrow::datatypes::{Field, Schema, DataType};
# use datafusion_expr::Operator;
# use datafusion_physical_expr::expressions::{binary, col, lit};
# use datafusion_physical_plan::{displayable, ExecutionPlan};
# use datafusion_physical_plan::empty::EmptyExec;
# use datafusion_physical_plan::filter::FilterExec;
# let schema = Schema::new(vec![Field::new("i", DataType::Int32, false)]);
# let plan = EmptyExec::new(Arc::new(schema));
# let i = col("i", &plan.schema()).unwrap();
# let predicate = binary(i, Operator::Eq, lit(1), &plan.schema()).unwrap();
# let plan: Arc<dyn ExecutionPlan> = Arc::new(FilterExec::try_new(predicate, Arc::new(plan)).unwrap());
// Get a one line description (Displayable)
let display_plan = displayable(plan.as_ref());

// you can use the returned objects to format plans
// where you can use `Display` such as  format! or println!
assert_eq!(
   &format!("The plan is: {}", display_plan.one_line()),
  "The plan is: FilterExec: i@0 = 1\n"
);
// You can also print out the plan and its children in indented mode
assert_eq!(display_plan.indent(false).to_string(),
  "FilterExec: i@0 = 1\
  \n  EmptyExec\
  \n"
);
```

---

## ProjectSchemaDisplay

`struct` · `datafusion_physical_plan::display::ProjectSchemaDisplay`

```rust
struct ProjectSchemaDisplay<'a>
```

**Implements**: `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.ProjectSchemaDisplay.md).


A wrapper to customize partitioned file display

---

## VerboseDisplay

`struct` · `datafusion_physical_plan::display::VerboseDisplay`

Also reachable as `datafusion::physical_plan::VerboseDisplay`, `datafusion_physical_plan::VerboseDisplay`, `datafusion_physical_plan::execution_plan::VerboseDisplay`

```rust
struct VerboseDisplay<T>
```

**Implements**: `core::fmt::Display`

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.VerboseDisplay.md).


A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

---

## DisplayAs

`trait` · `datafusion_physical_plan::display::DisplayAs`

Also reachable as `datafusion::physical_plan::DisplayAs`, `datafusion_physical_plan::DisplayAs`, `datafusion_physical_plan::execution_plan::DisplayAs`

```rust
trait DisplayAs
```

**Implementors** (53)

- `datafusion_datasource::display::FileGroupDisplay`
- `datafusion_datasource::file_scan_config::FileScanConfig`
- `datafusion_datasource::memory::MemSink`
- `datafusion_datasource::sink::DataSinkExec`
- `datafusion_datasource::source::DataSourceExec`
- `datafusion_datasource_csv::file_format::CsvSink`
- `datafusion_datasource_json::file_format::JsonSink`
- `datafusion_datasource_parquet::sink::ParquetSink`
- `datafusion_ffi::execution_plan::ForeignExecutionPlan`
- `datafusion_ffi::execution_plan::tests::EmptyExec`
- `datafusion_physical_optimizer::output_requirements::OutputRequirementExec`
- `datafusion_physical_plan::aggregates::AggregateExec`
- `datafusion_physical_plan::analyze::AnalyzeExec`
- `datafusion_physical_plan::async_func::AsyncFuncExec`
- `datafusion_physical_plan::buffer::BufferExec`
- `datafusion_physical_plan::coalesce_batches::CoalesceBatchesExec`
- `datafusion_physical_plan::coalesce_partitions::CoalescePartitionsExec`
- `datafusion_physical_plan::coop::CooperativeExec`
- `datafusion_physical_plan::empty::EmptyExec`
- `datafusion_physical_plan::explain::ExplainExec`
- `datafusion_physical_plan::filter::FilterExec`
- `datafusion_physical_plan::joins::cross_join::CrossJoinExec`
- `datafusion_physical_plan::joins::hash_join::exec::HashJoinExec`
- `datafusion_physical_plan::joins::nested_loop_join::NestedLoopJoinExec`
- `datafusion_physical_plan::joins::piecewise_merge_join::exec::PiecewiseMergeJoinExec`
- `datafusion_physical_plan::joins::sort_merge_join::exec::SortMergeJoinExec`
- `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec`
- `datafusion_physical_plan::limit::GlobalLimitExec`
- `datafusion_physical_plan::limit::LocalLimitExec`
- `datafusion_physical_plan::memory::LazyMemoryExec`
- `datafusion_physical_plan::placeholder_row::PlaceholderRowExec`
- `datafusion_physical_plan::projection::ProjectionExec`
- `datafusion_physical_plan::recursive_query::RecursiveQueryExec`
- `datafusion_physical_plan::repartition::RepartitionExec`
- `datafusion_physical_plan::scalar_subquery::ScalarSubqueryExec`
- `datafusion_physical_plan::sorts::partial_sort::PartialSortExec`
- `datafusion_physical_plan::sorts::partitioned_topk::PartitionedTopKExec`
- `datafusion_physical_plan::sorts::sort::SortExec`
- `datafusion_physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec`
- `datafusion_physical_plan::streaming::StreamingTableExec`
- `datafusion_physical_plan::test::TestMemoryExec`
- `datafusion_physical_plan::test::exec::BarrierExec`
- `datafusion_physical_plan::test::exec::BlockingExec`
- `datafusion_physical_plan::test::exec::ErrorExec`
- `datafusion_physical_plan::test::exec::MockExec`
- `datafusion_physical_plan::test::exec::PanicExec`
- `datafusion_physical_plan::test::exec::StatisticsExec`
- `datafusion_physical_plan::union::InterleaveExec`
- `datafusion_physical_plan::union::UnionExec`
- `datafusion_physical_plan::unnest::UnnestExec`
- `datafusion_physical_plan::windows::bounded_window_agg_exec::BoundedWindowAggExec`
- `datafusion_physical_plan::windows::window_agg_exec::WindowAggExec`
- `datafusion_physical_plan::work_table::WorkTableExec`

**Methods** (1)

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_physical_plan.display.DisplayAs.md).


Trait for types which could have additional details when formatted in `Verbose` mode

---
