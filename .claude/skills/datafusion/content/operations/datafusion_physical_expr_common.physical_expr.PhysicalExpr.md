# `datafusion_physical_expr_common::physical_expr::PhysicalExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.physical_expr.PhysicalExpr.json).

<a id="op-fe8284c43330456b0d4e6af7"></a>
## PhysicalExpr

`trait` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr` · datafusion-physical-expr-common 55.1.0

```rust
trait PhysicalExpr: Any + Send + Sync + Display + Debug + DynEq + DynHash
```

Source: `src/physical_expr.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

[`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

`PhysicalExpr` knows its type, nullability and can be evaluated directly on
a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) (see [`Self::evaluate`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-1fe79d89368238daf9372c3d)).

`PhysicalExpr` are the physical counterpart to [`Expr`] used in logical
planning. They are typically created from [`Expr`] by a [`PhysicalPlanner`]
invoked from a higher level API

Some important examples of `PhysicalExpr` are:
* [`Column`]: Represents a column at a given index in a RecordBatch

To create `PhysicalExpr` from  `Expr`, see
* [`SessionContext::create_physical_expr`]: A high level API
* [`create_physical_expr`]: A low level API

# Formatting `PhysicalExpr` as strings
There are three ways to format `PhysicalExpr` as a string:
* [`Debug`]: Standard Rust debugging format (e.g. `Constant { value: ... }`)
* [`Display`]: Detailed SQL-like format that shows expression structure (e.g. (`Utf8 ("foobar")`). This is often used for debugging and tests
* [`Self::fmt_sql`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-7c15755c74044b7b06f8a4d8): SQL-like human readable format (e.g. ('foobar')`), See also [`sql_fmt`]

[`SessionContext::create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr
[`PhysicalPlanner`]: https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html
[`Expr`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html
[`create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/fn.create_physical_expr.html
[`Column`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/expressions/struct.Column.html

Unresolved upstream links (retained, not inferred): ``Debug``, ``Display``.

<a id="op-17502ace8d8d415416182b66"></a>
## children

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::children` · datafusion-physical-expr-common 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Source: `src/physical_expr.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Get a list of child PhysicalExpr that provide the input for this expr.

<a id="op-e367ff9ff197323a7bd2586a"></a>
## data_type

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::data_type` · datafusion-physical-expr-common 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Source: `src/physical_expr.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Get the data type of this expression, given the schema of the input.
Returns an error if the data type cannot be determined, ex. if the
schema is missing a required field.

<a id="op-1fe79d89368238daf9372c3d"></a>
## evaluate

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::evaluate` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Source: `src/physical_expr.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Evaluate an expression against a RecordBatch

<a id="op-76b56dba1bb605b93be55a68"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::evaluate_bounds` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_bounds(&self, _children: &[&Interval]) -> Result<Interval>
```

Source: `src/physical_expr.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Computes the output interval for the expression, given the input
intervals.

# Parameters

* `children` are the intervals for the children (inputs) of this
  expression.

# Returns

A `Result` containing the output interval for the expression in
case of success, or an error object in case of failure.

Note that the output bounds must form an **envelope** that contains all
possible outputs of the expression given the input bounds. While
expressions should output the tightest possible bounds, they do not need
to be exact and can be conservative.

# Example

If the expression is `a + b`, and the input intervals are `a: [1, 2]`
and `b: [3, 4]`, then the output interval would be `[4, 6]`.

If the expression is `sin(a)`, it is correct (though not precise) to
produce the interval `[-1, 1]` for any input interval for `a`.

<a id="op-4206377250688d9166a2a426"></a>
## evaluate_selection

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::evaluate_selection` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_selection(&self, batch: &RecordBatch, selection: &BooleanArray) -> Result<ColumnarValue>
```

Source: `src/physical_expr.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Evaluate an expression against a RecordBatch after first applying a validity array

# Errors

Returns an `Err` if the expression could not be evaluated or if the length of the
`selection` validity array and the number of row in `batch` is not equal.

<a id="op-59940a3f827de91c5c3a5ac8"></a>
## evaluate_statistics

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::evaluate_statistics` · datafusion-physical-expr-common 55.1.0

```rust
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
```

Source: `src/physical_expr.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Computes the output statistics for the expression, given the input
statistics.

# Parameters

* `children` are the statistics for the children (inputs) of this
  expression.

# Returns

A `Result` containing the output statistics for the expression in
case of success, or an error object in case of failure.

Expressions (should) implement this function and utilize the independence
assumption, match on children distribution types and compute the output
statistics accordingly. The default implementation simply creates an
unknown output distribution by combining input ranges. This logic loses
distribution information, but is a safe default.

<a id="op-40e9c4db3e31f20fd22444d4"></a>
## expression_id

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::expression_id` · datafusion-physical-expr-common 55.1.0

```rust
fn expression_id(&self) -> Option<u64>
```

Source: `src/physical_expr.rs:477`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Return a stable, globally-unique identifier for this [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7), if it
has one.

This identifier tracks which expressions which are connected (e.g. `DynamicFilterPhysicalExpr`
where two expressions may be different but store the same mutable inner state). Tracking
connected expressions helps preserve referential integrity within plan nodes
during serialization and deserialization.

This id must be preserved across [`PhysicalExpr::with_new_children`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-88d2e4279fdf8b0302cb6dd2) or any other
methods which may want to preserve identity.

Default is `None`: the expression has no identity worth preserving across a
serialization boundary.

<a id="op-7c15755c74044b7b06f8a4d8"></a>
## fmt_sql

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::fmt_sql` · datafusion-physical-expr-common 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Source: `src/physical_expr.rs:369`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Format this `PhysicalExpr` in nice human readable "SQL" format

Specifically, this format is designed to be readable by humans, at the
expense of details. Use `Display` or `Debug` for more detailed
representation.

See the [`fmt_sql`](../operations/datafusion_physical_expr_common.physical_expr.fmt_sql.md#op-5fc2d3b3a4f26d7a3eff8172) function for an example of printing `PhysicalExpr`s as SQL.

<a id="op-2958a7acd05bbd3c8d8baaec"></a>
## get_properties

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::get_properties` · datafusion-physical-expr-common 55.1.0

```rust
fn get_properties(&self, _children: &[ExprProperties]) -> Result<ExprProperties>
```

Source: `src/physical_expr.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Calculates the properties of this [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) based on its
children's properties (i.e. order and range), recursively aggregating
the information from its children. In cases where the [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)
has no children (e.g., `Literal` or `Column`), these properties should
be specified externally, as the function defaults to unknown properties.

<a id="op-e37287ffad98248435abbe99"></a>
## is_volatile_node

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::is_volatile_node` · datafusion-physical-expr-common 55.1.0

```rust
fn is_volatile_node(&self) -> bool
```

Source: `src/physical_expr.rs:450`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns true if the expression node is volatile, i.e. whether it can return
different results when evaluated multiple times with the same input.

Note: unlike [`is_volatile`](../operations/datafusion_physical_expr_common.physical_expr.is_volatile.md#op-87a14151dca85e57889ff9c0), this function does not consider inputs:
- `random()` returns `true`,
- `a + random()` returns `false` (because the operation `+` itself is not volatile.)

The default to this function was set to `false` when it was created
to avoid imposing API churn on implementers, but this is not a safe default in general.
It is highly recommended that volatile expressions implement this method and return `true`.
This default may be removed in the future if it causes problems or we decide to
eat the cost of the breaking change and require all implementers to make a choice.

<a id="op-13a2991991c4980ed1692861"></a>
## nullable

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::nullable` · datafusion-physical-expr-common 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Source: `src/physical_expr.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Determine whether this expression is nullable, given the schema of the input

<a id="op-b6e07d30cd07cfae5220612e"></a>
## placement

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::placement` · datafusion-physical-expr-common 55.1.0

```rust
fn placement(&self) -> ExpressionPlacement
```

Source: `src/physical_expr.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns placement information for this expression.

This is used by optimizers to make decisions about expression placement,
such as whether to push expressions down through projections.

The default implementation returns [`ExpressionPlacement::KeepInPlace`](../operations/datafusion_expr_common.placement.ExpressionPlacement.md#op-600eef0885718f7165bfa8f5).

<a id="op-fc731633d2f041faaf4e5da3"></a>
## propagate_constraints

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::propagate_constraints` · datafusion-physical-expr-common 55.1.0

```rust
fn propagate_constraints(&self, _interval: &Interval, _children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Source: `src/physical_expr.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Updates bounds for child expressions, given a known interval for this
expression.

This is used to propagate constraints down through an expression tree.

# Parameters

* `interval` is the currently known interval for this expression.
* `children` are the current intervals for the children of this expression.

# Returns

A `Result` containing a `Vec` of new intervals for the children (in order)
in case of success, or an error object in case of failure.

If constraint propagation reveals an infeasibility for any child, returns
[`None`]. If none of the children intervals change as a result of
propagation, may return an empty vector instead of cloning `children`.
This is the default (and conservative) return value.

# Example

If the expression is `a + b`, the current `interval` is `[4, 5]` and the
inputs `a` and `b` are respectively given as `[0, 2]` and `[-∞, 4]`, then
propagation would return `[0, 2]` and `[2, 4]` as `b` must be at least
`2` to make the output at least `4`.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-f31c1b2602bf19b5bf890028"></a>
## propagate_statistics

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::propagate_statistics` · datafusion-physical-expr-common 55.1.0

```rust
fn propagate_statistics(&self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>>
```

Source: `src/physical_expr.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Updates children statistics using the given parent statistic for this
expression.

This is used to propagate statistics down through an expression tree.

# Parameters

* `parent` is the currently known statistics for this expression.
* `children` are the current statistics for the children of this expression.

# Returns

A `Result` containing a `Vec` of new statistics for the children (in order)
in case of success, or an error object in case of failure.

If statistics propagation reveals an infeasibility for any child, returns
[`None`]. If none of the children statistics change as a result of
propagation, may return an empty vector instead of cloning `children`.
This is the default (and conservative) return value.

Expressions (should) implement this function and apply Bayes rule to
reconcile and update parent/children statistics. This involves utilizing
the independence assumption, and matching on distribution types. The
default implementation simply creates an unknown distribution if it can
narrow the range by propagating ranges. This logic loses distribution
information, but is a safe default.

Unresolved upstream links (retained, not inferred): ``None``.

<a id="op-d276a76f139c27b4d6be4a2d"></a>
## return_field

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::return_field` · datafusion-physical-expr-common 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Source: `src/physical_expr.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

The output field associated with this expression

<a id="op-eac5395e633cbe5d577748bf"></a>
## snapshot

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::snapshot` · datafusion-physical-expr-common 55.1.0

```rust
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
```

Source: `src/physical_expr.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Take a snapshot of this `PhysicalExpr`, if it is dynamic.

"Dynamic" in this case means containing references to structures that may change
during plan execution, such as hash tables.

This method is used to capture the current state of `PhysicalExpr`s that may contain
dynamic references to other operators in order to serialize it over the wire
or treat it via downcast matching.

You should not call this method directly as it does not handle recursion.
Instead use [`snapshot_physical_expr`](../operations/datafusion_physical_expr_common.physical_expr.snapshot_physical_expr.md#op-9112be2d6306fc04a981dae3) to handle recursion and capture the
full state of the `PhysicalExpr`.

This is expected to return "simple" expressions that do not have mutable state
and are composed of DataFusion's built-in `PhysicalExpr` implementations.
Callers however should *not* assume anything about the returned expressions
since callers and implementers may not agree on what "simple" or "built-in"
means.
In other words, if you need to serialize a `PhysicalExpr` across the wire
you should call this method and then try to serialize the result,
but you should handle unknown or unexpected `PhysicalExpr` implementations gracefully
just as if you had not called this method at all.

In particular, consider:
* A `PhysicalExpr` that references the current state of a `datafusion::physical_plan::TopK`
  that is involved in a query with `SELECT * FROM t1 ORDER BY a LIMIT 10`.
  This function may return something like `a >= 12`.
* A `PhysicalExpr` that references the current state of a `datafusion::physical_plan::joins::HashJoinExec`
  from a query such as `SELECT * FROM t1 JOIN t2 ON t1.a = t2.b`.
  This function may return something like `t2.b IN (1, 5, 7)`.

A system or function that can only deal with a hardcoded set of `PhysicalExpr` implementations
or needs to serialize this state to bytes may not be able to handle these dynamic references.
In such cases, we should return a simplified version of the `PhysicalExpr` that does not
contain these dynamic references.

Systems that implement remote execution of plans, e.g. serialize a portion of the query plan
and send it across the wire to a remote executor may want to call this method after
every batch on the source side and broadcast / update the current snapshot to the remote executor.

Note for implementers: this method should *not* handle recursion.
Recursion is handled in [`snapshot_physical_expr`](../operations/datafusion_physical_expr_common.physical_expr.snapshot_physical_expr.md#op-9112be2d6306fc04a981dae3).

<a id="op-3304e292f30207a701cfa61d"></a>
## snapshot_generation

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::snapshot_generation` · datafusion-physical-expr-common 55.1.0

```rust
fn snapshot_generation(&self) -> u64
```

Source: `src/physical_expr.rs:429`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns the generation of this `PhysicalExpr` for snapshotting purposes.
The generation is an arbitrary u64 that can be used to track changes
in the state of the `PhysicalExpr` over time without having to do an exhaustive comparison.
This is useful to avoid unnecessary computation or serialization if there are no changes to the expression.
In particular, dynamic expressions that may change over time; this allows cheap checks for changes.
Static expressions that do not change over time should return 0, as does the default implementation.
You should not call this method directly as it does not handle recursion.
Instead use [`snapshot_generation`](../operations/datafusion_physical_expr_common.physical_expr.snapshot_generation.md#op-297e75607be761bf81f57611) to handle recursion and capture the
full state of the `PhysicalExpr`.

<a id="op-3afb516a11e28f2442ad2f38"></a>
## try_to_proto

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::try_to_proto` · datafusion-physical-expr-common 55.1.0

```rust
fn try_to_proto(&self, _ctx: &proto_encode::PhysicalExprEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>>
```

Source: `src/physical_expr.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Serialize this expression to a [`PhysicalExprNode`] proto message.

Returning `Ok(None)` means "this expression does not know how to
serialize itself"; the caller (typically `datafusion-proto`) will fall
back to its existing codec / extension paths. This matches today's
behavior for expressions that aren't built into `datafusion-proto`.

Returning `Ok(Some(node))` means the expression has serialized itself
fully; the caller should not try any further fallback path.

Returning `Err(_)` means a real serialization failure (e.g. the
expression knows it should serialize but a child failed).

The motivating use case is letting expressions with private state
(e.g. `DynamicFilterPhysicalExpr`'s `RwLock`-protected inner fields)
reach into their own internals for `try_to_proto`/`try_from_proto`
without having to expose `pub` accessors to `datafusion-proto`. See
<https://github.com/apache/datafusion/issues/21835>.

The `try_` prefix matches the fallible `try_from_proto` decode
constructors; both sides of the round-trip are fallible and named
consistently.

[`PhysicalExprNode`]: datafusion_proto_models::protobuf::PhysicalExprNode

<a id="op-88d2e4279fdf8b0302cb6dd2"></a>
## with_new_children

`function` · `datafusion_physical_expr_common::physical_expr::PhysicalExpr::with_new_children` · datafusion-physical-expr-common 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Source: `src/physical_expr.rs:170`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Returns a new PhysicalExpr where all children were replaced by new exprs.

If the implementation returns a [`PhysicalExpr::expression_id`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-40e9c4db3e31f20fd22444d4), then
the identifier should be preserved by the new expression.
