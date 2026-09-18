# `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.extension.UserDefinedLogicalNodeCore.json).

<a id="op-576e74a1b24795c5e0d7bec0"></a>
## UserDefinedLogicalNodeCore

`trait` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore` · datafusion-expr 55.1.0

```rust
trait UserDefinedLogicalNodeCore: fmt::Debug + Eq + PartialOrd + Hash + Sized + Send + Sync + 'static
```

Source: `src/logical_plan/extension.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This trait facilitates implementation of the [`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432).

See the example in
[user_defined_plan.rs](https://github.com/apache/datafusion/blob/main/datafusion/core/tests/user_defined/user_defined_plan.rs)
file for an example of how to use this extension API.

<a id="op-a6f24b3a7f424c4e95465fe9"></a>
## check_invariants

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::check_invariants` · datafusion-expr 55.1.0

```rust
fn check_invariants(&self, _check: InvariantLevel) -> Result<()>
```

Source: `src/logical_plan/extension.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Perform check of invariants for the extension node.

This is the default implementation for extension nodes.

<a id="op-b9d15714be5cb47b7b2a6d4b"></a>
## expressions

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::expressions` · datafusion-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Expr>
```

Source: `src/logical_plan/extension.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all expressions in the current logical plan node. This
should not include expressions of any inputs (aka
non-recursively). These expressions are used for optimizer
passes and rewrites.

<a id="op-d52b9485b6131a23e79311d7"></a>
## fmt_for_explain

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::fmt_for_explain` · datafusion-expr 55.1.0

```rust
fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Source: `src/logical_plan/extension.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Write a single line, human readable string to `f` for use in explain plan.

For example: `TopK: k=10`

<a id="op-cfb8afd2fd8b65f9ed3086da"></a>
## inputs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::inputs` · datafusion-expr 55.1.0

```rust
fn inputs(&self) -> Vec<&LogicalPlan>
```

Source: `src/logical_plan/extension.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the logical plan's inputs.

<a id="op-1dc3081aae000c81bca62820"></a>
## name

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/logical_plan/extension.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the plan's name.

<a id="op-bd6011bd233fda2feeada0c6"></a>
## necessary_children_exprs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::necessary_children_exprs` · datafusion-expr 55.1.0

```rust
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
```

Source: `src/logical_plan/extension.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the necessary input columns for this node required to compute
the columns in the output schema

This is used for projection push-down when DataFusion has determined that
only a subset of the output columns of this node are needed by its parents.
This API is used to tell DataFusion which, if any, of the input columns are no longer
needed.

Return `None`, the default, if this information can not be determined.
Returns `Some(_)` with the column indices for each child of this node that are
needed to compute `output_columns`

<a id="op-c3ddadf426896737ae2422a7"></a>
## prevent_predicate_push_down_columns

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::prevent_predicate_push_down_columns` · datafusion-expr 55.1.0

```rust
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
```

Source: `src/logical_plan/extension.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A list of output columns (e.g. the names of columns in
self.schema()) for which predicates can not be pushed below
this node without changing the output.

By default, this returns all columns and thus prevents any
predicates from being pushed below this node.

<a id="op-137c09d1d046de9e5583fdc2"></a>
## schema

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Source: `src/logical_plan/extension.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the output schema of this logical plan node.

<a id="op-d7dd561cda19891e2059ecae"></a>
## supports_limit_pushdown

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::supports_limit_pushdown` · datafusion-expr 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Source: `src/logical_plan/extension.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns `true` if a limit can be safely pushed down through this
`UserDefinedLogicalNode` node.

If this method returns `true`, and the query plan contains a limit at
the output of this node, DataFusion will push the limit to the input
of this node.

<a id="op-a270bac2988bd61a75649bbe"></a>
## with_exprs_and_inputs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNodeCore::with_exprs_and_inputs` · datafusion-expr 55.1.0

```rust
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Self>
```

Source: `src/logical_plan/extension.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `UserDefinedLogicalNode` with the specified children
and expressions. This function is used during optimization
when the plan is being rewritten and a new instance of the
`UserDefinedLogicalNode` must be created.

Note that exprs and inputs are in the same order as the result
of self.inputs and self.exprs.

So, `self.with_exprs_and_inputs(exprs, ..).expressions() == exprs
