# `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.json).

<a id="op-2111ab49e0384ae550130432"></a>
## UserDefinedLogicalNode

`trait` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode` · datafusion-expr 55.1.0

```rust
trait UserDefinedLogicalNode: fmt::Debug + Send + Sync
```

Source: `src/logical_plan/extension.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

This defines the interface for [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da) nodes that can be
used to extend DataFusion with custom relational operators.

The [`UserDefinedLogicalNodeCore`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNodeCore.md#op-576e74a1b24795c5e0d7bec0) trait is *the recommended way to implement*
this trait and avoids having implementing some required boiler plate code.

<a id="op-18a8357e141fd36eb221899c"></a>
## as_any

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::as_any` · datafusion-expr 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/logical_plan/extension.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a reference to self as Any, to support dynamic downcasting

Typically this will look like:

```
# use std::any::Any;
# struct Dummy { }

# impl Dummy {
// canonical boiler plate
fn as_any(&self) -> &dyn Any {
    self
}
# }
```

<a id="op-e6c8c852a2035ec276a5835d"></a>
## check_invariants

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::check_invariants` · datafusion-expr 55.1.0

```rust
fn check_invariants(&self, check: InvariantLevel) -> Result<()>
```

Source: `src/logical_plan/extension.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Perform check of invariants for the extension node.

<a id="op-acf2e335ce37bf00a3042cfa"></a>
## dyn_eq

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::dyn_eq` · datafusion-expr 55.1.0

```rust
fn dyn_eq(&self, other: &dyn UserDefinedLogicalNode) -> bool
```

Source: `src/logical_plan/extension.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Compare `other`, respecting requirements from [Eq].

Note: consider using [`UserDefinedLogicalNodeCore`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNodeCore.md#op-576e74a1b24795c5e0d7bec0) instead of
[`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) directly.

When `other` has an another type than `self`, then the values
are *not* equal.

This method is required to support Eq on [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s.  To
implement it, typically the type implementing
[`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) typically implements [`Eq`] and
then the following boiler plate is used:

# Example:
```
# use datafusion_expr::UserDefinedLogicalNode;
// User defined node that derives Eq
#[derive(Hash, Debug, PartialEq, Eq)]
struct MyNode {
    val: u64,
}

// impl UserDefinedLogicalNode {
// ...
# impl MyNode {
// Boiler plate to call the derived Eq impl
fn dyn_eq(&self, other: &dyn UserDefinedLogicalNode) -> bool {
    match other.as_any().downcast_ref::<Self>() {
        Some(o) => self == o,
        None => false,
    }
}
// }
# }
```
Note: [`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) is not constrained by [`Eq`]
directly because it must remain object safe.

Unresolved upstream links (retained, not inferred): `Eq`, ``Eq``.

<a id="op-8fb1b94e8e08fbc52ce9a985"></a>
## dyn_hash

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::dyn_hash` · datafusion-expr 55.1.0

```rust
fn dyn_hash(&self, state: &mut dyn Hasher)
```

Source: `src/logical_plan/extension.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Update the hash `state` with this node requirements from
[`Hash`].

Note: consider using [`UserDefinedLogicalNodeCore`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNodeCore.md#op-576e74a1b24795c5e0d7bec0) instead of
[`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) directly.

This method is required to support hashing [`LogicalPlan`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-2f2092c4f87ff1cc0b33c3da)s.  To
implement it, typically the type implementing
[`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) typically implements [`Hash`] and
then the following boiler plate is used:

# Example:
```
// User defined node that derives Hash
#[derive(Hash, Debug, PartialEq, Eq)]
struct MyNode {
    val: u64,
}

// impl UserDefinedLogicalNode {
// ...
# impl MyNode {
// Boiler plate to call the derived Hash impl
fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
    use std::hash::Hash;
    let mut s = state;
    self.hash(&mut s);
}
// }
# }
```
Note: [`UserDefinedLogicalNode`](../operations/datafusion_expr.logical_plan.extension.UserDefinedLogicalNode.md#op-2111ab49e0384ae550130432) is not constrained by [`Hash`]
directly because it must remain object safe.

Unresolved upstream links (retained, not inferred): ``Hash``.

<a id="op-74faa8c15eb5d39b07995346"></a>
## dyn_ord

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::dyn_ord` · datafusion-expr 55.1.0

```rust
fn dyn_ord(&self, other: &dyn UserDefinedLogicalNode) -> Option<Ordering>
```

Source: `src/logical_plan/extension.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Compare `other`, respecting requirements from [PartialOrd].
Must return `Some(Equal)` if and only if `self.dyn_eq(other)`.

Unresolved upstream links (retained, not inferred): `PartialOrd`.

<a id="op-9368b137b050b31705548695"></a>
## expressions

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::expressions` · datafusion-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Expr>
```

Source: `src/logical_plan/extension.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns all expressions in the current logical plan node. This should
not include expressions of any inputs (aka non-recursively).

These expressions are used for optimizer
passes and rewrites. See [`LogicalPlan::expressions`](../operations/datafusion_expr.logical_plan.plan.LogicalPlan.md#op-586c0dd5f6fcdb0f90eeae74) for more details.

<a id="op-cd2e11c6532b49affafdc39d"></a>
## fmt_for_explain

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::fmt_for_explain` · datafusion-expr 55.1.0

```rust
fn fmt_for_explain(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Source: `src/logical_plan/extension.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Write a single line, human readable string to `f` for use in explain plan.

For example: `TopK: k=10`

<a id="op-7d1087e9ab182ec2253c8a7c"></a>
## inputs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::inputs` · datafusion-expr 55.1.0

```rust
fn inputs(&self) -> Vec<&LogicalPlan>
```

Source: `src/logical_plan/extension.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the logical plan's inputs.

<a id="op-bf7638fdbd59010d9123eb72"></a>
## name

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/logical_plan/extension.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the plan's name.

<a id="op-0ea622f1e33ece7b57ace995"></a>
## necessary_children_exprs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::necessary_children_exprs` · datafusion-expr 55.1.0

```rust
fn necessary_children_exprs(&self, _output_columns: &[usize]) -> Option<Vec<Vec<usize>>>
```

Source: `src/logical_plan/extension.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the necessary input columns for this node required to compute
the columns in the output schema

This is used for projection push-down when DataFusion has determined that
only a subset of the output columns of this node are needed by its parents.
This API is used to tell DataFusion which, if any, of the input columns are no longer
needed.

Return `None`, the default, if this information can not be determined.
Returns `Some(_)` with the column indices for each child of this node that are
needed to compute `output_columns`

<a id="op-3b88cbca34c43f5d5cc6b257"></a>
## prevent_predicate_push_down_columns

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::prevent_predicate_push_down_columns` · datafusion-expr 55.1.0

```rust
fn prevent_predicate_push_down_columns(&self) -> HashSet<String>
```

Source: `src/logical_plan/extension.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A list of output columns (e.g. the names of columns in
self.schema()) for which predicates can not be pushed below
this node without changing the output.

By default, this returns all columns and thus prevents any
predicates from being pushed below this node.

<a id="op-6650e82aaec656036fb4571f"></a>
## schema

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::schema` · datafusion-expr 55.1.0

```rust
fn schema(&self) -> &DFSchemaRef
```

Source: `src/logical_plan/extension.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the output schema of this logical plan node.

<a id="op-4e9d0f557a70be20497c9b9e"></a>
## supports_limit_pushdown

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::supports_limit_pushdown` · datafusion-expr 55.1.0

```rust
fn supports_limit_pushdown(&self) -> bool
```

Source: `src/logical_plan/extension.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns `true` if a limit can be safely pushed down through this
`UserDefinedLogicalNode` node.

If this method returns `true`, and the query plan contains a limit at
the output of this node, DataFusion will push the limit to the input
of this node.

<a id="op-e6e167a1718fd8c73c3025f1"></a>
## with_exprs_and_inputs

`function` · `datafusion_expr::logical_plan::extension::UserDefinedLogicalNode::with_exprs_and_inputs` · datafusion-expr 55.1.0

```rust
fn with_exprs_and_inputs(&self, exprs: Vec<Expr>, inputs: Vec<LogicalPlan>) -> Result<Arc<dyn UserDefinedLogicalNode>>
```

Source: `src/logical_plan/extension.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `UserDefinedLogicalNode` with the specified children
and expressions. This function is used during optimization
when the plan is being rewritten and a new instance of the
`UserDefinedLogicalNode` must be created.

Note that exprs and inputs are in the same order as the result
of self.inputs and self.exprs.

So, `self.with_exprs_and_inputs(exprs, ..).expressions() == exprs
