# `buoyant_kernel::transforms::expression`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.transforms.expression.json`](../model/buoyant_kernel.transforms.expression.json)

## ExpressionDepthChecker

`struct` · `buoyant_kernel::transforms::expression::ExpressionDepthChecker`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transforms.expression.ExpressionDepthChecker.md)

Also reachable as `buoyant_kernel::transforms::ExpressionDepthChecker`, `delta_kernel::transforms::expression::ExpressionDepthChecker`

```rust
struct ExpressionDepthChecker
```

**Implements**: `buoyant_kernel::transforms::expression::ExpressionTransform`

**Methods** (2)

```rust
fn check_expr(expr: &Expression, depth_limit: usize) -> usize
fn check_pred(pred: &Predicate, depth_limit: usize) -> usize
```

**via `buoyant_kernel::transforms::expression::ExpressionTransform`**

```rust
fn transform_expr_binary(&mut self, expr: &'a BinaryExpression) -> DeltaResult<()>
fn transform_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> DeltaResult<()>
fn transform_expr_opaque(&mut self, expr: &'a OpaqueExpression) -> DeltaResult<()>
fn transform_expr_pred(&mut self, pred: &'a Predicate) -> DeltaResult<()>
fn transform_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> DeltaResult<()>
fn transform_pred_binary(&mut self, pred: &'a BinaryPredicate) -> DeltaResult<()>
fn transform_pred_junction(&mut self, pred: &'a JunctionPredicate) -> DeltaResult<()>
fn transform_pred_not(&mut self, pred: &'a Predicate) -> DeltaResult<()>
fn transform_pred_opaque(&mut self, pred: &'a OpaquePredicate) -> DeltaResult<()>
fn transform_pred_unary(&mut self, pred: &'a UnaryPredicate) -> DeltaResult<()>
```

An expression "transform" that doesn't actually change the expression at all. Instead, it
measures the maximum depth of a expression, with a depth limit to prevent stack overflow. Useful
for verifying that a expression has reasonable depth before attempting to work with it.

---

## ExpressionTransform

`trait` · `buoyant_kernel::transforms::expression::ExpressionTransform`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md)

Also reachable as `buoyant_kernel::transforms::ExpressionTransform`, `delta_kernel::transforms::expression::ExpressionTransform`

```rust
trait ExpressionTransform<'a>
```

**Implementors** (4)

- `buoyant_kernel::expressions::GetColumnReferences`
- `buoyant_kernel::scan::ApplyColumnMappings`
- `buoyant_kernel::scan::PrefixColumns`
- `buoyant_kernel::transforms::expression::ExpressionDepthChecker`

**Methods** (33)

```rust
fn recurse_into_expr_binary(&mut self, b: &'a BinaryExpression) -> Self::Output<BinaryExpression>
fn recurse_into_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> Self::Output<MapToStructExpression>
fn recurse_into_expr_opaque(&mut self, o: &'a OpaqueExpression) -> Self::Output<OpaqueExpression>
fn recurse_into_expr_parse_json(&mut self, expr: &'a ParseJsonExpression) -> Self::Output<ParseJsonExpression>
fn recurse_into_expr_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
fn recurse_into_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> Self::Output<[ExpressionRef]>
fn recurse_into_expr_unary(&mut self, u: &'a UnaryExpression) -> Self::Output<UnaryExpression>
fn recurse_into_expr_variadic(&mut self, v: &'a VariadicExpression) -> Self::Output<VariadicExpression>
fn recurse_into_pred_binary(&mut self, b: &'a BinaryPredicate) -> Self::Output<BinaryPredicate>
fn recurse_into_pred_junction(&mut self, j: &'a JunctionPredicate) -> Self::Output<JunctionPredicate>
fn recurse_into_pred_not(&mut self, p: &'a Predicate) -> Self::Output<Predicate>
fn recurse_into_pred_opaque(&mut self, o: &'a OpaquePredicate) -> Self::Output<OpaquePredicate>
fn recurse_into_pred_unary(&mut self, u: &'a UnaryPredicate) -> Self::Output<UnaryPredicate>
fn transform_expr(&mut self, expr: &'a Expression) -> Self::Output<Expression>
fn transform_expr_binary(&mut self, expr: &'a BinaryExpression) -> Self::Output<BinaryExpression>
fn transform_expr_column(&mut self, name: &'a ColumnName) -> Self::Output<ColumnName>
fn transform_expr_literal(&mut self, value: &'a Scalar) -> Self::Output<Scalar>
fn transform_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> Self::Output<MapToStructExpression>
fn transform_expr_opaque(&mut self, expr: &'a OpaqueExpression) -> Self::Output<OpaqueExpression>
fn transform_expr_parse_json(&mut self, expr: &'a ParseJsonExpression) -> Self::Output<ParseJsonExpression>
fn transform_expr_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
fn transform_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> Self::Output<[ExpressionRef]>
fn transform_expr_struct_patch(&mut self, patch: &'a ExpressionStructPatch) -> Self::Output<ExpressionStructPatch>
fn transform_expr_unary(&mut self, expr: &'a UnaryExpression) -> Self::Output<UnaryExpression>
fn transform_expr_unknown(&mut self, name: &'a String) -> Self::Output<String>
fn transform_expr_variadic(&mut self, expr: &'a VariadicExpression) -> Self::Output<VariadicExpression>
fn transform_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
fn transform_pred_binary(&mut self, pred: &'a BinaryPredicate) -> Self::Output<BinaryPredicate>
fn transform_pred_junction(&mut self, pred: &'a JunctionPredicate) -> Self::Output<JunctionPredicate>
fn transform_pred_not(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
fn transform_pred_opaque(&mut self, pred: &'a OpaquePredicate) -> Self::Output<OpaquePredicate>
fn transform_pred_unary(&mut self, pred: &'a UnaryPredicate) -> Self::Output<UnaryPredicate>
fn transform_pred_unknown(&mut self, name: &'a String) -> Self::Output<String>
```

Generic framework for recursive bottom-up transforms of expressions and predicates.

The transform entry point is generally [`Self::transform_expr`] or [`Self::transform_pred`] (for
expressions or predicates, respectively), but callers can also directly invoke the transform
for a specific expression/predicate variant (e.g. [`Self::transform_expr_column`] for
[`ColumnName`] or [`Self::transform_pred_unary`] for [`UnaryPredicate`]).

The provided `transform_xxx` methods all default to no-op (usually by invoking the corresponding
recursive helper method), and implementations should selectively override specific
`transform_xxx` methods as needed for the task at hand.

# Recursive helper methods

The provided `recurse_into_xxx` methods encapsulate the boilerplate work of recursing into the
child expression of each expression type. Except as specifically noted otherwise, these
recursive helpers all behave uniformly, based on the number of children the parent has:

* Leaf (no children) - Leaf `transform_xxx` methods simply return their argument unchanged, and
  no corresponding `recurse_into_xxx` method is provided.

* Unary (single child) - If the child was filtered out, filter out the parent. If the child
  changed, build a new parent around it. Otherwise, return the parent unchanged.

* Binary (two children) - If either child was filtered out, filter out the parent. If at least
  one child changed, build a new parent around them. Otherwise, return the parent unchanged.

* Variadic (0+ children) - If no children remain (all filtered out), filter out the parent.
  Otherwise, if at least one child changed or was filtered out, build a new parent around the
  children. Otherwise, return the parent unchanged.

Implementations can call these as needed but will generally not need to override them.

# Transform carrier selection

Implementations choose an output [`Carrier`] instance based on the operation to be
performed. That carrier determines the return type of each transform method.

For example, a simple read-only visitor would use `()` as a carrier, while a validity checker
could use `DeltaResult<()>` instead. A mutating transform uses `Cow<_>`, returning `Cow::Owned`
for changed/replaced nodes, and a filtering transform uses `Option<Cow<_>>`, where `None`
indicates the node should be dropped rather than replaced. `DeltaResult<Cow<_>>` and
`Result<Option<Cow<_>>, E>` round out the set as fallible mutating and fitering transforms that
short circuit immediately upon `Err`.

---
