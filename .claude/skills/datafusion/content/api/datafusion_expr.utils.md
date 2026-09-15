# `datafusion_expr::utils`

Crate `datafusion-expr` · 36 public items · structured records in [`model/datafusion_expr.utils.json`](../model/datafusion_expr.utils.json)

## add_filter

`function` · `datafusion_expr::utils::add_filter`

```rust
fn add_filter(plan: LogicalPlan, predicates: &[&Expr]) -> datafusion_common::Result<LogicalPlan>
```

Returns a new [LogicalPlan] that filters the output of  `plan` with a
[LogicalPlan::Filter] with all `predicates` ANDed.

# Example
Before:
```text
plan
```

After:
```text
Filter(predicate)
  plan
```

---

## can_hash

`function` · `datafusion_expr::utils::can_hash`

```rust
fn can_hash(data_type: &arrow::datatypes::DataType) -> bool
```

Can this data type be used in hash join equal conditions??
Data types here come from function 'equal_rows', if more data types are supported
in create_hashes, add those data types here to generate join logical plan.

---

## check_all_columns_from_schema

`function` · `datafusion_expr::utils::check_all_columns_from_schema`

```rust
fn check_all_columns_from_schema(columns: &std::collections::HashSet<&datafusion_common::Column>, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<bool>
```

Check whether all columns are from the schema.

---

## collect_subquery_cols

`function` · `datafusion_expr::utils::collect_subquery_cols`

```rust
fn collect_subquery_cols(exprs: &[Expr], subquery_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<std::collections::BTreeSet<datafusion_common::Column>>
```

Determine the set of [`Column`]s produced by the subquery.

---

## columnize_expr

`function` · `datafusion_expr::utils::columnize_expr`

```rust
fn columnize_expr(e: Expr, input: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Convert an expression into Column expression if it's already provided as input plan.

For example, it rewrites:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), sum(col("c2"))?
```

Into:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), col("SUM(c2)")?
```

---

## compare_sort_expr

`function` · `datafusion_expr::utils::compare_sort_expr`

```rust
fn compare_sort_expr(sort_expr_a: &expr::Sort, sort_expr_b: &expr::Sort, schema: &datafusion_common::DFSchemaRef) -> std::cmp::Ordering
```

Compare the sort expr as PostgreSQL's common_prefix_cmp():
<https://github.com/postgres/postgres/blob/master/src/backend/optimizer/plan/planner.c>

---

## conjunction

`function` · `datafusion_expr::utils::conjunction`

```rust
fn conjunction(filters: impl IntoIterator<Item = Expr>) -> Option<Expr>
```

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical AND.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::conjunction;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use conjunction to join them together with `AND`
assert_eq!(conjunction(split), Some(expr));
```

---

## disjunction

`function` · `datafusion_expr::utils::disjunction`

```rust
fn disjunction(filters: impl IntoIterator<Item = Expr>) -> Option<Expr>
```

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical OR.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::disjunction;
// a=1 OR b=2
let expr = col("a").eq(lit(1)).or(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use disjunction to join them together with `OR`
assert_eq!(disjunction(split), Some(expr));
```

---

## enumerate_grouping_sets

`function` · `datafusion_expr::utils::enumerate_grouping_sets`

```rust
fn enumerate_grouping_sets(group_expr: Vec<Expr>) -> datafusion_common::Result<Vec<Expr>>
```

Convert multiple grouping expressions into one [`GroupingSet::GroupingSets`],\
if the grouping expression does not contain [`Expr::GroupingSet`] or only has one expression,\
no conversion will be performed.

e.g.

person.id,\
GROUPING SETS ((person.age, person.salary),(person.age)),\
ROLLUP(person.state, person.birth_date)

=>

GROUPING SETS (\
  (person.id, person.age, person.salary),\
  (person.id, person.age, person.salary, person.state),\
  (person.id, person.age, person.salary, person.state, person.birth_date),\
  (person.id, person.age),\
  (person.id, person.age, person.state),\
  (person.id, person.age, person.state, person.birth_date)\
)

---

## expand_qualified_wildcard

`function` · `datafusion_expr::utils::expand_qualified_wildcard`

```rust
fn expand_qualified_wildcard(qualifier: &datafusion_common::TableReference, schema: &datafusion_common::DFSchema, wildcard_options: Option<&expr::WildcardOptions>) -> datafusion_common::Result<Vec<Expr>>
```

Resolves an `Expr::Wildcard` to a collection of qualified `Expr::Column`'s.

---

## expand_wildcard

`function` · `datafusion_expr::utils::expand_wildcard`

```rust
fn expand_wildcard(schema: &datafusion_common::DFSchema, plan: &LogicalPlan, wildcard_options: Option<&expr::WildcardOptions>) -> datafusion_common::Result<Vec<Expr>>
```

Resolves an `Expr::Wildcard` to a collection of `Expr::Column`'s.

---

## expr_as_column_expr

`function` · `datafusion_expr::utils::expr_as_column_expr`

```rust
fn expr_as_column_expr(expr: &Expr, plan: &LogicalPlan) -> datafusion_common::Result<Expr>
```

Convert any `Expr` to an `Expr::Column`.

---

## expr_to_columns

`function` · `datafusion_expr::utils::expr_to_columns`

```rust
fn expr_to_columns(expr: &Expr, accum: &mut std::collections::HashSet<datafusion_common::Column>) -> datafusion_common::Result<()>
```

Recursively walk an expression tree, collecting the unique set of columns
referenced in the expression

---

## exprlist_to_fields

`function` · `datafusion_expr::utils::exprlist_to_fields`

```rust
fn exprlist_to_fields<'a>(exprs: impl IntoIterator<Item = &'a Expr>, plan: &LogicalPlan) -> datafusion_common::Result<Vec<(Option<datafusion_common::TableReference>, std::sync::Arc<arrow::datatypes::Field>)>>
```

Create schema fields from an expression list, for use in result set schema construction

This function converts a list of expressions into a list of complete schema fields,
making comprehensive determinations about each field's properties including:
- **Data type**: Resolved based on expression type and input schema context
- **Nullability**: Determined by expression-specific nullability rules
- **Metadata**: Computed based on expression type (preserving, merging, or generating new metadata)
- **Table reference scoping**: Establishing proper qualified field references

Each expression is converted to a field by calling [`Expr::to_field`], which performs
the complete field resolution process for all field properties.

# Returns

A `Result` containing a vector of `(Option<TableReference>, Arc<Field>)` tuples,
where each Field contains complete schema information (type, nullability, metadata)
and proper table reference scoping for the corresponding expression.

---

## find_aggregate_exprs

`function` · `datafusion_expr::utils::find_aggregate_exprs`

```rust
fn find_aggregate_exprs<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Vec<Expr>
```

Collect all deeply nested `Expr::AggregateFunction`.
They are returned in order of occurrence (depth
first), with duplicates omitted.

---

## find_column_exprs

`function` · `datafusion_expr::utils::find_column_exprs`

```rust
fn find_column_exprs(exprs: &[Expr]) -> Vec<Expr>
```

Collect all deeply nested `Expr::Column`'s. They are returned in order of
appearance (depth first), and may contain duplicates.

---

## find_join_exprs

`function` · `datafusion_expr::utils::find_join_exprs`

```rust
fn find_join_exprs(exprs: Vec<&Expr>) -> datafusion_common::Result<(Vec<Expr>, Vec<Expr>)>
```

Looks for correlating expressions: for example, a binary expression with one field from the subquery, and
one not in the subquery (closed upon from outer scope)

# Arguments

* `exprs` - List of expressions that may or may not be joins

# Return value

Tuple of (expressions containing joins, remaining non-join expressions)

---

## find_out_reference_exprs

`function` · `datafusion_expr::utils::find_out_reference_exprs`

```rust
fn find_out_reference_exprs(expr: &Expr) -> Vec<Expr>
```

Collect all deeply nested `Expr::OuterReferenceColumn`. They are returned in order of occurrence
(depth first), with duplicates omitted.

---

## find_valid_equijoin_key_pair

`function` · `datafusion_expr::utils::find_valid_equijoin_key_pair`

```rust
fn find_valid_equijoin_key_pair(left_key: &Expr, right_key: &Expr, left_schema: &datafusion_common::DFSchema, right_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<Option<(Expr, Expr)>>
```

Give two sides of the equijoin predicate, return a valid join key pair.
If there is no valid join key pair, return None.

A valid join means:
1. All referenced column of the left side is from the left schema, and
   all referenced column of the right side is from the right schema.
2. Or opposite. All referenced column of the left side is from the right schema,
   and the right side is from the left schema.

---

## find_window_exprs

`function` · `datafusion_expr::utils::find_window_exprs`

```rust
fn find_window_exprs<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Vec<Expr>
```

Collect all deeply nested `Expr::WindowFunction`. They are returned in order of occurrence
(depth first), with duplicates omitted.

---

## format_state_name

`function` · `datafusion_expr::utils::format_state_name`

Also reachable as `datafusion_physical_expr::expressions::format_state_name`, `datafusion_physical_plan::execution_plan::expressions::format_state_name`, `datafusion_physical_plan::expressions::format_state_name`

```rust
fn format_state_name(name: &str, state_name: &str) -> String
```

Build state name. State is the intermediate state of the aggregate function.

---

## generate_signature_error_msg

`function` · `datafusion_expr::utils::generate_signature_error_msg`

> **Deprecated** — since 53.0.0: Internal function

```rust
fn generate_signature_error_msg(func_name: &str, func_signature: datafusion_expr_common::signature::Signature, input_expr_types: &[arrow::datatypes::DataType]) -> String
```

Creates a detailed error message for a function with wrong signature.

For example, a query like `select round(3.14, 1.1);` would yield:
```text
Error during planning: No function matches 'round(Float64, Float64)'. You might need to add explicit type casts.
    Candidate functions:
    round(Float64, Int64)
    round(Float32, Int64)
    round(Float64)
    round(Float32)
```

---

## generate_sort_key

`function` · `datafusion_expr::utils::generate_sort_key`

```rust
fn generate_sort_key(partition_by: &[Expr], order_by: &[expr::Sort]) -> datafusion_common::Result<Vec<(expr::Sort, bool)>>
```

Generate a sort key for a given window expr's partition_by and order_by expr

---

## group_window_expr_by_sort_keys

`function` · `datafusion_expr::utils::group_window_expr_by_sort_keys`

```rust
fn group_window_expr_by_sort_keys(window_expr: impl IntoIterator<Item = Expr>) -> datafusion_common::Result<Vec<(Vec<(expr::Sort, bool)>, Vec<Expr>)>>
```

Group a slice of window expression expr by their order by expressions

---

## grouping_set_expr_count

`function` · `datafusion_expr::utils::grouping_set_expr_count`

```rust
fn grouping_set_expr_count(group_expr: &[Expr]) -> datafusion_common::Result<usize>
```

Count the number of distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.

---

## grouping_set_to_exprlist

`function` · `datafusion_expr::utils::grouping_set_to_exprlist`

```rust
fn grouping_set_to_exprlist(group_expr: &[Expr]) -> datafusion_common::Result<Vec<&Expr>>
```

Find all distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.

---

## inspect_expr_pre

`function` · `datafusion_expr::utils::inspect_expr_pre`

```rust
fn inspect_expr_pre<F, E>(expr: &Expr, f: F) -> datafusion_common::Result<(), E> where F: FnMut(&Expr) -> datafusion_common::Result<(), E>
```

Recursively inspect an [`Expr`] and all its children.

---

## iter_conjunction

`function` · `datafusion_expr::utils::iter_conjunction`

```rust
fn iter_conjunction(expr: &Expr) -> impl Iterator<Item = &Expr>
```

Iterate parts in a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

---

## iter_conjunction_owned

`function` · `datafusion_expr::utils::iter_conjunction_owned`

```rust
fn iter_conjunction_owned(expr: Expr) -> impl Iterator<Item = Expr>
```

Iterate parts in a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

---

## merge_schema

`function` · `datafusion_expr::utils::merge_schema`

```rust
fn merge_schema(inputs: &[&LogicalPlan]) -> datafusion_common::DFSchema
```

merge inputs schema into a single schema.

This function merges schemas from multiple logical plan inputs using [`DFSchema::merge`].
Refer to that documentation for details on precedence and metadata handling.

---

## only_or_err

`function` · `datafusion_expr::utils::only_or_err`

```rust
fn only_or_err<T>(slice: &[T]) -> datafusion_common::Result<&T>
```

Returns the first (and only) element in a slice, or an error

# Arguments

* `slice` - The slice to extract from

# Return value

The first element, or an error

---

## powerset

`function` · `datafusion_expr::utils::powerset`

```rust
fn powerset<T>(slice: &[T]) -> datafusion_common::Result<Vec<Vec<&T>>>
```

The [power set] (or powerset) of a set S is the set of all subsets of S, \
including the empty set and S itself.

Example:

If S is the set {x, y, z}, then all the subsets of S are \
 {} \
 {x} \
 {y} \
 {z} \
 {x, y} \
 {x, z} \
 {y, z} \
 {x, y, z} \
 and hence the power set of S is {{}, {x}, {y}, {z}, {x, y}, {x, z}, {y, z}, {x, y, z}}.

[power set]: https://en.wikipedia.org/wiki/Power_set

---

## split_binary

`function` · `datafusion_expr::utils::split_binary`

```rust
fn split_binary(expr: &Expr, op: Operator) -> Vec<&Expr>
```

Splits an binary operator tree [`Expr`] such as `A <OP> B <OP> C` => `[A, B, C]`

See [`split_binary_owned`] for more details and an example.

---

## split_binary_owned

`function` · `datafusion_expr::utils::split_binary_owned`

```rust
fn split_binary_owned(expr: Expr, op: Operator) -> Vec<Expr>
```

Splits an owned binary operator tree [`Expr`] such as `A <OP> B <OP> C` => `[A, B, C]`

This is often used to "split" expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit, Operator};
# use datafusion_expr::utils::split_binary_owned;
# use std::ops::Add;
// a=1 + b=2
let expr = col("a").eq(lit(1)).add(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use split_binary_owned to split them
assert_eq!(split_binary_owned(expr, Operator::Plus), split);
```

---

## split_conjunction

`function` · `datafusion_expr::utils::split_conjunction`

```rust
fn split_conjunction(expr: &Expr) -> Vec<&Expr>
```

Splits a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

---

## split_conjunction_owned

`function` · `datafusion_expr::utils::split_conjunction_owned`

```rust
fn split_conjunction_owned(expr: Expr) -> Vec<Expr>
```

Splits an owned conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

This is often used to "split" filter expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::split_conjunction_owned;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![col("a").eq(lit(1)), col("b").eq(lit(2))];

// use split_conjunction_owned to split them
assert_eq!(split_conjunction_owned(expr), split);
```

---
