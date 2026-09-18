# `buoyant_kernel::transforms::expression::ExpressionTransform`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transforms.expression.ExpressionTransform.json).

<a id="op-55a6339eca69df19277a218c"></a>
## ExpressionTransform

`trait` · `buoyant_kernel::transforms::expression::ExpressionTransform` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait ExpressionTransform<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L59).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:59`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generic framework for recursive bottom-up transforms of expressions and predicates.

The transform entry point is generally [`Self::transform_expr`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-cec9a1e249dc2c10ae60b41c) or [`Self::transform_pred`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-80078ce011ddf500902dd9b1) (for
expressions or predicates, respectively), but callers can also directly invoke the transform
for a specific expression/predicate variant (e.g. [`Self::transform_expr_column`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-cf3368630f0876d4e5cee6b5) for
[`ColumnName`](../operations/buoyant_kernel.expressions.column_names.ColumnName.md#op-9a9657c136296c6d9c579ddd) or [`Self::transform_pred_unary`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-85c7e85b257da27f55bbbc1d) for [`UnaryPredicate`](../operations/buoyant_kernel.expressions.UnaryPredicate.md#op-a6c7fccb3b3f819bb5b11c53)).

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

Implementations choose an output [`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d) instance based on the operation to be
performed. That carrier determines the return type of each transform method.

For example, a simple read-only visitor would use `()` as a carrier, while a validity checker
could use `DeltaResult<()>` instead. A mutating transform uses `Cow<_>`, returning `Cow::Owned`
for changed/replaced nodes, and a filtering transform uses `Option<Cow<_>>`, where `None`
indicates the node should be dropped rather than replaced. `DeltaResult<Cow<_>>` and
`Result<Option<Cow<_>>, E>` round out the set as fallible mutating and fitering transforms that
short circuit immediately upon `Err`.

<a id="op-027a42ee10b194c67646e321"></a>
## Output

`assoc_type` · `buoyant_kernel::transforms::expression::ExpressionTransform::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L64).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d) output type for transformed nodes.

Implementations can use [`crate::transforms::transform_output_type`](../operations/buoyant_kernel.transform_output_type.md#op-ad05bd806f416e4d185977fc) to define `Output`
and `Residual` together.

<a id="op-42ed18db077f8ab2a69ff23f"></a>
## Residual

`assoc_type` · `buoyant_kernel::transforms::expression::ExpressionTransform::Residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Residual
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L80).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:80`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Residual type propagated by this transform's output [`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d).

Implementations can use [`crate::transforms::transform_output_type`](../operations/buoyant_kernel.transform_output_type.md#op-ad05bd806f416e4d185977fc) to define `Output`
and `Residual` together. Or, define it manually like this:
```rust,no_run
# use std::borrow::Cow;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transforms::{Carrier, ExpressionTransform};
# struct X;
# impl<'a> ExpressionTransform<'a> for X {
#     type Output<T: std::borrow::ToOwned + ?Sized + 'a> = Cow<'a, T>;
type Residual = <Self::Output<()> as Carrier<'a, ()>>::Residual;
# }
```
(required because associated type defaults are not stable rust yet)

<a id="op-bb30d164565200641b8f3481"></a>
## recurse_into_expr_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_binary(&mut self, b: &'a BinaryExpression) -> Self::Output<BinaryExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L385).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:385`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a binary expression's children (binary).

<a id="op-a8e9d6596fd2763721b98132"></a>
## recurse_into_expr_map_to_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_map_to_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> Self::Output<MapToStructExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L333).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:333`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms the child expression of a map-to-struct expression (unary).

<a id="op-7a970b5064eef62a0c730e0e"></a>
## recurse_into_expr_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_opaque(&mut self, o: &'a OpaqueExpression) -> Self::Output<OpaqueExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L342).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:342`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms the children of an opaque expression (variadic).

<a id="op-337293604ff8dbf5a742743b"></a>
## recurse_into_expr_parse_json

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_parse_json(&mut self, expr: &'a ParseJsonExpression) -> Self::Output<ParseJsonExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L324).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:324`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms the child expression of a parse-json expression (unary).

<a id="op-71b392dc333fd461523545db"></a>
## recurse_into_expr_pred

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L352).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:352`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms the child of a predicate expression (unary).

<a id="op-ef47f417ec6379fd6e488905"></a>
## recurse_into_expr_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> Self::Output<[ExpressionRef]>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L313).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:313`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a struct's child expressions (variadic).

<a id="op-c6e30dfb50bb455de736a67f"></a>
## recurse_into_expr_unary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_unary(&mut self, u: &'a UnaryExpression) -> Self::Output<UnaryExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L379).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:379`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a unary expression's child (unary).

<a id="op-882e33d1e82c318247a77a21"></a>
## recurse_into_expr_variadic

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_expr_variadic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_expr_variadic(&mut self, v: &'a VariadicExpression) -> Self::Output<VariadicExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L396).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:396`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a variadic expression's children (variadic).

<a id="op-a16ae5ed3a8660c5988d1e76"></a>
## recurse_into_pred_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_pred_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_pred_binary(&mut self, b: &'a BinaryPredicate) -> Self::Output<BinaryPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L368).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a binary predicate's children (binary).

<a id="op-196fb155ba6b312e5039db80"></a>
## recurse_into_pred_junction

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_pred_junction(&mut self, j: &'a JunctionPredicate) -> Self::Output<JunctionPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L405).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:405`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a junction predicate's children (variadic).

<a id="op-6c0e5cea110464b90f08684b"></a>
## recurse_into_pred_not

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_pred_not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_pred_not(&mut self, p: &'a Predicate) -> Self::Output<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L357).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:357`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms the child of a not predicate expression (unary).

<a id="op-8524cce93022637f1a21a744"></a>
## recurse_into_pred_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_pred_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_pred_opaque(&mut self, o: &'a OpaquePredicate) -> Self::Output<OpaquePredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L414).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:414`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms an opaque predicate's children (variadic).

<a id="op-f64e0a407bf9a34f56763f36"></a>
## recurse_into_pred_unary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::recurse_into_pred_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_pred_unary(&mut self, u: &'a UnaryPredicate) -> Self::Output<UnaryPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L362).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:362`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a unary predicate's child (unary).

<a id="op-cec9a1e249dc2c10ae60b41c"></a>
## transform_expr

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr(&mut self, expr: &'a Expression) -> Self::Output<Expression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L220).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

General entry point for transforming an expression. This method will dispatch to the
specific transform for each expression variant. Also invoked internally in order to recurse
on the child(ren) of non-leaf expressions.

<a id="op-19554ec874b3ff4d5f0eaba3"></a>
## transform_expr_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_binary(&mut self, expr: &'a BinaryExpression) -> Self::Output<BinaryExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L169).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:169`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each binary expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_binary`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-bb30d164565200641b8f3481).

<a id="op-cf3368630f0876d4e5cee6b5"></a>
## transform_expr_column

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_column(&mut self, name: &'a ColumnName) -> Self::Output<ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L88).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each column reference encountered during the traversal (leaf).

<a id="op-2f2503c611d745008b34da97"></a>
## transform_expr_literal

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_literal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_literal(&mut self, value: &'a Scalar) -> Self::Output<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L83).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:83`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each literal encountered during the traversal (leaf).

<a id="op-605decf090a4d01fd06d3f31"></a>
## transform_expr_map_to_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_map_to_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_map_to_struct(&mut self, expr: &'a MapToStructExpression) -> Self::Output<MapToStructExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L136).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:136`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each map-to-struct expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_map_to_struct`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-a8e9d6596fd2763721b98132).

<a id="op-6d5592eec90ead830fad966a"></a>
## transform_expr_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_opaque(&mut self, expr: &'a OpaqueExpression) -> Self::Output<OpaqueExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L103).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each opaque expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_opaque`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-7a970b5064eef62a0c730e0e).

<a id="op-e03b4df10815adc50533ad91"></a>
## transform_expr_parse_json

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_parse_json(&mut self, expr: &'a ParseJsonExpression) -> Self::Output<ParseJsonExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L127).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each parse-json expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_parse_json`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-337293604ff8dbf5a742743b).

<a id="op-7c1ba2ecbc779759571d8e4b"></a>
## transform_expr_pred

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L145).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:145`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for the child of each predicate expression encountered during the
traversal. The provided implementation just forwards to [`Self::recurse_into_expr_pred`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-71b392dc333fd461523545db).

<a id="op-be0a69385bb6a20a982402e7"></a>
## transform_expr_struct

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_struct(&mut self, fields: &'a [ExpressionRef]) -> Self::Output<[ExpressionRef]>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L94).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:94`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for the expression list of each struct expression encountered during the
traversal. The provided implementation just forwards to [`Self::recurse_into_expr_struct`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-ef47f417ec6379fd6e488905).

<a id="op-ff5ab6d6a2aa6a34c8cbc987"></a>
## transform_expr_struct_patch

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_struct_patch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_struct_patch(&mut self, patch: &'a ExpressionStructPatch) -> Self::Output<ExpressionStructPatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L118).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:118`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each struct patch expression encountered during the traversal (leaf).

The provided implementation does _NOT_ recurse into its children.

<a id="op-f1354f6f841b4a270ea683bd"></a>
## transform_expr_unary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_unary(&mut self, expr: &'a UnaryExpression) -> Self::Output<UnaryExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L157).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:157`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each unary expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_unary`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-c6e30dfb50bb455de736a67f).

<a id="op-95f86dee13fa7d1e11a47ddb"></a>
## transform_expr_unknown

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_unknown(&mut self, name: &'a String) -> Self::Output<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L111).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:111`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each unknown expression encountered during the traversal (leaf).

<a id="op-396e9a6b5b2b8bcd7cbec065"></a>
## transform_expr_variadic

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_expr_variadic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_expr_variadic(&mut self, expr: &'a VariadicExpression) -> Self::Output<VariadicExpression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L187).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:187`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each variadic expression encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_expr_variadic`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-882e33d1e82c318247a77a21).

<a id="op-80078ce011ddf500902dd9b1"></a>
## transform_pred

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L276).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:276`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

General entry point for transforming a predicate. This method will dispatch to the specific
transform for each predicate variant. Also invoked internally in order to recurse on the
child(ren) of non-leaf variants.

<a id="op-e6e42792b1a204198e023bd4"></a>
## transform_pred_binary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_binary(&mut self, pred: &'a BinaryPredicate) -> Self::Output<BinaryPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L178).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:178`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each binary predicate encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_pred_binary`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-a16ae5ed3a8660c5988d1e76).

<a id="op-03b1a27a36306302258571ee"></a>
## transform_pred_junction

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_junction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_junction(&mut self, pred: &'a JunctionPredicate) -> Self::Output<JunctionPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L196).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:196`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each junction predicate encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_pred_junction`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-196fb155ba6b312e5039db80).

<a id="op-06b47c5cd22e46d16096edaf"></a>
## transform_pred_not

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_not` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_not(&mut self, pred: &'a Predicate) -> Self::Output<Predicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L151).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:151`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for the child of each NOT predicate encountered during the
traversal. The provided implementation just forwards to [`Self::recurse_into_pred_not`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-6c0e5cea110464b90f08684b).

<a id="op-ad2b3db6c1598ad774395854"></a>
## transform_pred_opaque

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_opaque` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_opaque(&mut self, pred: &'a OpaquePredicate) -> Self::Output<OpaquePredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L205).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:205`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each opaque predicate encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_pred_opaque`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-8524cce93022637f1a21a744).

<a id="op-85c7e85b257da27f55bbbc1d"></a>
## transform_pred_unary

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_unary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_unary(&mut self, pred: &'a UnaryPredicate) -> Self::Output<UnaryPredicate>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L163).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:163`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each unary predicate encountered during the traversal. The provided
implementation just forwards to [`Self::recurse_into_pred_unary`](../operations/buoyant_kernel.transforms.expression.ExpressionTransform.md#op-f64e0a407bf9a34f56763f36).

<a id="op-d245e1d8b4f6cae7104cacdd"></a>
## transform_pred_unknown

`function` · `buoyant_kernel::transforms::expression::ExpressionTransform::transform_pred_unknown` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_pred_unknown(&mut self, name: &'a String) -> Self::Output<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/expression.rs#L213).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/expression.rs:213`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each unknown predicate encountered during the traversal (leaf).
