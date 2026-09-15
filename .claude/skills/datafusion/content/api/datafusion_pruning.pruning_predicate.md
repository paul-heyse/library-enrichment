# `datafusion_pruning::pruning_predicate`

Crate `datafusion-pruning` · 7 public items · structured records in [`model/datafusion_pruning.pruning_predicate.json`](../model/datafusion_pruning.pruning_predicate.json)

## MAX_IN_LIST_SIZE

`constant` · `datafusion_pruning::pruning_predicate::MAX_IN_LIST_SIZE`

Also reachable as `datafusion_physical_optimizer::pruning::MAX_IN_LIST_SIZE`, `datafusion_pruning::MAX_IN_LIST_SIZE`

```rust
const MAX_IN_LIST_SIZE: usize = 20
```

Default maximum number of entries in an `IN (...)` list that will be
rewritten into a chain of per-value min/max checks by
`build_predicate_expression`. Callers threading a [`PredicateRewriter`]
can override this via [`PredicateRewriter::with_max_in_list_size`], and
query engines can wire it from the
`datafusion.execution.parquet.max_in_list_size` config option.

---

## build_pruning_predicate

`function` · `datafusion_pruning::pruning_predicate::build_pruning_predicate`

Also reachable as `datafusion_physical_optimizer::pruning::build_pruning_predicate`, `datafusion_pruning::build_pruning_predicate`

```rust
fn build_pruning_predicate(predicate: std::sync::Arc<dyn PhysicalExpr>, file_schema: &arrow::datatypes::SchemaRef, predicate_creation_errors: &datafusion_physical_plan::metrics::Count) -> Option<std::sync::Arc<PruningPredicate>>
```

Build a pruning predicate from an optional predicate expression.
If the predicate is None or the predicate cannot be converted to a pruning
predicate, return None.
If there is an error creating the pruning predicate it is recorded by incrementing
the `predicate_creation_errors` counter.

---

## PredicateRewriter

`struct` · `datafusion_pruning::pruning_predicate::PredicateRewriter`

Also reachable as `datafusion_physical_optimizer::pruning::PredicateRewriter`, `datafusion_pruning::PredicateRewriter`

```rust
struct PredicateRewriter
```

**Derives**: Default

**Methods** (4)

```rust
fn new() -> Self
fn rewrite_predicate_to_statistics_predicate(&self, expr: &Arc<dyn PhysicalExpr>, schema: &Schema) -> Arc<dyn PhysicalExpr>
fn with_max_in_list_size(self, max_in_list_size: usize) -> Self
fn with_unhandled_hook(self, unhandled_hook: Arc<dyn UnhandledPredicateHook>) -> Self
```

Rewrite a predicate expression in terms of statistics (min/max/null_counts)
for use as a [`PruningPredicate`].

---

## PruningPredicate

`struct` · `datafusion_pruning::pruning_predicate::PruningPredicate`

Also reachable as `datafusion_physical_optimizer::pruning::PruningPredicate`, `datafusion_pruning::PruningPredicate`

```rust
struct PruningPredicate
```

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn always_true(&self) -> bool
fn literal_columns(&self) -> Vec<String>
fn literal_guarantees(&self) -> &[LiteralGuarantee]
fn orig_expr(&self) -> &Arc<dyn PhysicalExpr>
fn predicate_expr(&self) -> &Arc<dyn PhysicalExpr>
fn prune<S: PruningStatistics + ?Sized>(&self, statistics: &S) -> Result<Vec<bool>>
fn required_columns(&self) -> &RequiredColumns
fn schema(&self) -> &SchemaRef
fn try_new(expr: Arc<dyn PhysicalExpr>, schema: SchemaRef) -> Result<Self>
```

Used to prove that arbitrary predicates (boolean expression) can not
possibly evaluate to `true` given information about a column provided by
[`PruningStatistics`].

# Introduction

`PruningPredicate` analyzes filter expressions using statistics such as
min/max values and null counts, attempting to prove a "container" (e.g.
Parquet Row Group) can be skipped without reading the actual data,
potentially leading to significant performance improvements.

For example, `PruningPredicate`s are used to prune Parquet Row Groups based
on the min/max values found in the Parquet metadata. If the
`PruningPredicate` can prove that the filter can never evaluate to `true`
for any row in the Row Group, the entire Row Group is skipped during query
execution.

The `PruningPredicate` API is general, and can be used for pruning other
types of containers (e.g. files) based on statistics that may be known from
external catalogs (e.g. Delta Lake) or other sources. How this works is a
subtle topic.  See the Background and Implementation section for details.

`PruningPredicate` supports:

1. Arbitrary expressions (including user defined functions)

2. Vectorized evaluation (provide more than one set of statistics at a time)
   so it is suitable for pruning 1000s of containers.

3. Any source of information that implements the [`PruningStatistics`] trait
   (not just Parquet metadata).

# Example

See the [`pruning.rs` example in the `datafusion-examples`] for a complete
example of how to use `PruningPredicate` to prune files based on min/max
values.

[`pruning.rs` example in the `datafusion-examples`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/query_planning/pruning.rs

Given an expression like `x = 5` and statistics for 3 containers (Row
Groups, files, etc) `A`, `B`, and `C`:

```text
  A: {x_min = 0, x_max = 4}
  B: {x_min = 2, x_max = 10}
  C: {x_min = 5, x_max = 8}
```

`PruningPredicate` will conclude that the rows in container `A` can never
be true (as the maximum value is only `4`), so it can be pruned:

```text
A: false (no rows could possibly match x = 5)
B: true  (rows might match x = 5)
C: true  (rows might match x = 5)
```

See [`PruningPredicateBuilder`] and [`PruningPredicate::prune`] for more information.

# Background

## Boolean Tri-state logic

To understand the details of the rest of this documentation, it is important
to understand how the tri-state boolean logic in SQL works. As this is
somewhat esoteric, we review it here.

SQL has a notion of `NULL` that represents the value is `“unknown”` and this
uncertainty propagates through expressions. SQL `NULL` behaves very
differently than the `NULL` in most other languages where it is a special,
sentinel value (e.g. `0` in `C/C++`). While representing uncertainty with
`NULL` is powerful and elegant, SQL `NULL`s are often deeply confusing when
first encountered as they behave differently than most programmers may
expect.

In most other programming languages,
* `a == NULL` evaluates to `true` if `a` also had the value `NULL`
* `a == NULL` evaluates to `false` if `a` has any other value

However, in SQL `a = NULL` **always** evaluates to `NULL` (never `true` or
`false`):

Expression    | Result
------------- | ---------
`1 = NULL`    | `NULL`
`NULL = NULL` | `NULL`

Also important is how `AND` and `OR` works with tri-state boolean logic as
(perhaps counterintuitively) the result is **not** always NULL. While
consistent with the notion of `NULL` representing “unknown”, this is again,
often deeply confusing 🤯 when first encountered.

Expression       | Result    | Intuition
---------------  | --------- | -----------
`NULL AND true`  |   `NULL`  | The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change
`NULL AND false` |  `false`  | If the `NULL` was either `true` or `false` the overall expression is still `false`
`NULL AND NULL`  | `NULL`    |

Expression      | Result    | Intuition
--------------- | --------- | ----------
`NULL OR true`  | `true`    |  If the `NULL` was either `true` or `false` the overall expression is still `true`
`NULL OR false` | `NULL`    |  The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change
`NULL OR NULL`  |  `NULL`   |

## SQL Filter Semantics

The SQL `WHERE` clause has a boolean expression, often called a filter or
predicate. The semantics of this predicate are that the query evaluates the
predicate for each row in the input tables and:

* Rows that evaluate to `true` are returned in the query results

* Rows that evaluate to `false` are not returned (“filtered out” or “pruned” or “skipped”).

* Rows that evaluate to `NULL` are **NOT** returned (also “filtered out”).
  Note: *this treatment of `NULL` is **DIFFERENT** than how `NULL` is treated
  in the rewritten predicate described below.*

# `PruningPredicate` Implementation

Armed with the information in the Background section, we can now understand
how the `PruningPredicate` logic works.

## Interface

**Inputs**
1. An input schema describing what columns exist

2. A predicate (expression that evaluates to a boolean)

3. [`PruningStatistics`] that provides information about columns in that
   schema, for multiple “containers”. For each column in each container, it
   provides optional information on contained values, min_values, max_values,
   null_counts counts, and row_counts counts.

**Outputs**:
A (non null) boolean value for each container:
* `true`: There MAY be rows that match the predicate

* `false`: There are no rows that could possibly match the predicate (the
  predicate can never possibly be true). The container can be pruned (skipped)
  entirely.

While `PruningPredicate` will never return a `NULL` value, the
rewritten predicate (as returned by `build_predicate_expression` and used internally
by `PruningPredicate`) may evaluate to `NULL` when some of the min/max values
or null / row counts are not known.

In order to be correct, `PruningPredicate` must return false
**only** if it can determine that for all rows in the container, the
predicate could never evaluate to `true` (always evaluates to either `NULL`
or `false`).

## Contains Analysis and Min/Max Rewrite

`PruningPredicate` works by first analyzing the predicate to see what
[`LiteralGuarantee`] must hold for the predicate to be true.

Then, the `PruningPredicate` rewrites the original predicate into an
expression that references the min/max values of each column in the original
predicate.

When the min/max values are actually substituted in to this expression and
evaluated, the result means

* `true`: there MAY be rows that pass the predicate, **KEEPS** the container

* `NULL`: there MAY be rows that pass the predicate, **KEEPS** the container
  Note that rewritten predicate can evaluate to NULL when some of
  the min/max values are not known. *Note that this is different than
  the SQL filter semantics where `NULL` means the row is filtered
  out.*

* `false`: there are no rows that could possibly match the predicate,
  **PRUNES** the container

For example, given a column `x`, the `x_min`, `x_max`, `x_null_count`, and
`x_row_count` represent the minimum and maximum values, the null count of
column `x`, and the row count of column `x`, provided by the `PruningStatistics`.
`x_null_count` and `x_row_count` are used to handle the case where the column `x`
is known to be all `NULL`s. Note this is different from knowing nothing about
the column `x`, which confusingly is encoded by returning `NULL` for the min/max
values from [`PruningStatistics::max_values`] and [`PruningStatistics::min_values`].

Here are some examples of the rewritten predicates:

Original Predicate | Rewritten Predicate
------------------ | --------------------
`x = 5` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)`
`x < 5` | `x_null_count != x_row_count AND (x_min < 5)`
`x = 5 AND y = 10` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max) AND y_null_count != y_row_count (y_min <= 10 AND 10 <= y_max)`
`x IS NULL`  | `x_null_count > 0`
`x IS NOT NULL`  | `x_null_count != row_count`
`CAST(x as int) = 5` | `x_null_count != x_row_count (CAST(x_min as int) <= 5 AND 5 <= CAST(x_max as int))`

## Predicate Evaluation
The PruningPredicate works in two passes

**First pass**:  For each `LiteralGuarantee` calls
[`PruningStatistics::contained`] and rules out containers where the
LiteralGuarantees are not satisfied

**Second Pass**: Evaluates the rewritten expression using the
min/max/null_counts/row_counts values for each column for each container. For any
container that this expression evaluates to `false`, it rules out those
containers.


### Example 1

Given the predicate, `x = 5 AND y = 10`, the rewritten predicate would look like:

```sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for a given container, `x` is between `1 and 100` and we know that
`y` is between `4` and `7`, we know nothing about the null count and row count of
`x` and `y`, the input statistics might look like:

Column   | Value
-------- | -----
`x_min`  | `1`
`x_max`  | `100`
`x_null_count` | `null`
`x_row_count`  | `null`
`y_min`  | `4`
`y_max`  | `7`
`y_null_count` | `null`
`y_row_count`  | `null`

When these statistics values are substituted in to the rewritten predicate and
simplified, the result is `false`:

* `null != null AND (1 <= 5 AND 5 <= 100) AND null != null AND (4 <= 10 AND 10 <= 7)`
* `null = null` is `null` which is not true, so the AND moves on to the next clause
* `null and (1 <= 5 AND 5 <= 100) AND null AND (4 <= 10 AND 10 <= 7)`
* evaluating the clauses further we get:
* `null and true and null and false`
* `null and false`
* `false`

Returning `false` means the container can be pruned, which matches the
intuition that  `x = 5 AND y = 10` can’t be true for any row if all values of `y`
are `7` or less.

Note that if we had ended up with `null AND true AND null AND true` the result
would have been `null`.
`null` is treated the same as`true`, because we can't prove that the predicate is `false.`

If, for some other container, we knew `y` was between the values `4` and
`15`, then the rewritten predicate evaluates to `true` (verifying this is
left as an exercise to the reader -- are you still here?), and the container
**could not** be pruned. The intuition is that there may be rows where the
predicate *might* evaluate to `true`, and the only way to find out is to do
more analysis, for example by actually reading the data and evaluating the
predicate row by row.

### Example 2

Given the same predicate, `x = 5 AND y = 10`, the rewritten predicate would
look like the same as example 1:

```sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for another given container, `x_min` is NULL and `x_max` is
NULL (the min/max values are unknown), `x_null_count` is `100` and `x_row_count`
 is `100`; we know that `y` is between `4` and `7`, but we know nothing about
the null count and row count of `y`. The input statistics might look like:

Column   | Value
-------- | -----
`x_min`  | `null`
`x_max`  | `null`
`x_null_count` | `100`
`x_row_count`  | `100`
`y_min`  | `4`
`y_max`  | `7`
`y_null_count` | `null`
`y_row_count`  | `null`

When these statistics values are substituted in to the rewritten predicate and
simplified, the result is `false`:

* `100 != 100 AND (null <= 5 AND 5 <= null) AND null = null AND (4 <= 10 AND 10 <= 7)`
* `false AND null AND null AND false`
* `false AND false`
* `false`

Returning `false` means the container can be pruned, which matches the
intuition that  `x = 5 AND y = 10` can’t be true because all values in `x`
are known to be NULL.

# Related Work

[`PruningPredicate`] implements the type of min/max pruning described in
Section `3.3.3` of the [`Snowflake SIGMOD Paper`]. The technique is
described by various research such as [small materialized aggregates], [zone
maps], and [data skipping].

[`Snowflake SIGMOD Paper`]: https://dl.acm.org/doi/10.1145/2882903.2903741
[small materialized aggregates]: https://www.vldb.org/conf/1998/p476.pdf
[zone maps]: https://dl.acm.org/doi/10.1007/978-3-642-03730-6_10
[data skipping]: https://dl.acm.org/doi/10.1145/2588555.2610515

---

## PruningPredicateBuilder

`struct` · `datafusion_pruning::pruning_predicate::PruningPredicateBuilder`

Also reachable as `datafusion_physical_optimizer::pruning::PruningPredicateBuilder`, `datafusion_pruning::PruningPredicateBuilder`

```rust
struct PruningPredicateBuilder<'a>
```

**Derives**: Default

**Methods** (6)

```rust
fn build(self, predicate: Arc<dyn PhysicalExpr>) -> Option<Arc<PruningPredicate>>
fn new() -> Self
fn try_build(self, predicate: Arc<dyn PhysicalExpr>) -> Result<PruningPredicate>
fn with_error_counter(self, error_counter: &'a Count) -> Self
fn with_file_schema(self, file_schema: SchemaRef) -> Self
fn with_max_in_list_size(self, max_in_list_size: usize) -> Self
```

Builder for a [`PruningPredicate`]. Groups optional configuration —
`IN (...)` rewrite cap, error counter — so future additions do not
churn the top-level API.

The two entry points are:
 - [`Self::build`]: convenience for scan sites that already track a
   `predicate_creation_errors` counter. Returns `Some(Arc<..>)` when the
   resulting predicate can actually prune, `None` when it is trivially
   true or when construction failed (in which case the error counter is
   incremented if one was supplied).
 - [`Self::try_build`]: returns a raw `Result<PruningPredicate>` for
   callers that want to surface errors themselves.

---

## RequiredColumns

`struct` · `datafusion_pruning::pruning_predicate::RequiredColumns`

Also reachable as `datafusion_physical_optimizer::pruning::RequiredColumns`, `datafusion_pruning::RequiredColumns`

```rust
struct RequiredColumns
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default

**Methods** (1)

```rust
fn single_column(&self) -> Option<&phys_expr::Column>
```

**via `core::convert::From`**

```rust
fn from(columns: Vec<(phys_expr::Column, StatisticsType, Field)>) -> Self
```

Describes which columns statistics are necessary to evaluate a
[`PruningPredicate`].

This structure permits reading and creating the minimum number statistics,
which is important since statistics may be non trivial to read (e.g. large
strings or when there are 1000s of columns).

Handles creating references to the min/max statistics
for columns as well as recording which statistics are needed

---

## UnhandledPredicateHook

`trait` · `datafusion_pruning::pruning_predicate::UnhandledPredicateHook`

Also reachable as `datafusion_physical_optimizer::pruning::UnhandledPredicateHook`, `datafusion_pruning::UnhandledPredicateHook`

```rust
trait UnhandledPredicateHook
```

**Methods** (1)

```rust
fn handle(&self, expr: &Arc<dyn PhysicalExpr>) -> Arc<dyn PhysicalExpr>
```

Rewrites predicates that [`PredicateRewriter`] can not handle, e.g. certain
complex expressions or predicates that reference columns that are not in the
schema.

---
