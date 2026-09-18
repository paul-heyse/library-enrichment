# `datafusion_common::tree_node`

Crate `datafusion-common` · 11 public items · structured records in [`model/datafusion_common.tree_node.json`](../model/datafusion_common.tree_node.json)

## TreeNodeRecursion

`enum` · `datafusion_common::tree_node::TreeNodeRecursion`

```rust
enum TreeNodeRecursion
```

**Variants**: `Continue`, `Jump`, `Stop`

**Derives**: Clone, Copy, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn visit_children<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
fn visit_parent<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
fn visit_sibling<F: FnOnce() -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeRecursion.md).


Controls how [`TreeNode`] recursions should proceed.

---

## Transformed

`struct` · `datafusion_common::tree_node::Transformed`

```rust
struct Transformed<T>
```

**Fields**: `data`, `transformed`, `tnr`

**Derives**: Debug, PartialEq, StructuralPartialEq

**Methods** (11)

```rust
fn complete(data: T) -> Self
fn map_data<U, F: FnOnce(T) -> Result<U>>(self, f: F) -> Result<Transformed<U>>
fn new(data: T, transformed: bool, tnr: TreeNodeRecursion) -> Self
fn new_transformed(data: T, transformed: bool) -> Self
fn no(data: T) -> Self
fn transform_children<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
fn transform_data<U, F: FnOnce(T) -> Result<Transformed<U>>>(self, f: F) -> Result<Transformed<U>>
fn transform_parent<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
fn transform_sibling<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
fn update_data<U, F: FnOnce(T) -> U>(self, f: F) -> Transformed<U>
fn yes(data: T) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.Transformed.md).


Result of tree walk / transformation APIs

`Transformed` is a wrapper around the tree node data (e.g. `Expr` or
`LogicalPlan`). It is used to indicate whether the node was transformed
and how the recursion should proceed.

[`TreeNode`] API users control the transformation by returning:
- The resulting (possibly transformed) node,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`] specifying how to proceed with the recursion.

At the end of the transformation, the return value will contain:
- The final (possibly transformed) tree,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`] specifying how the recursion ended.

See also
* [`Transformed::update_data`] to modify the node without changing the `transformed` flag
* [`Transformed::map_data`] for fallable operation that return the same type
* [`Transformed::transform_data`] to chain fallable transformations
* [`TransformedResult`] for working with `Result<Transformed<U>>`

# Examples

Use [`Transformed::yes`] and [`Transformed::no`] to signal that a node was
rewritten and the recursion should continue:

```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();

// Create a new `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
assert!(!ret.transformed);

// Create a new `Transformed` object signaling the node was rewritten
let ret = Transformed::yes(expr);
assert!(ret.transformed)
```

Access the node within the `Transformed` object:
```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();

// `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
// Access the inner object using .data
assert_eq!(expr, ret.data);
```

Transform the node within the `Transformed` object.

```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();
let ret = Transformed::no(expr.clone())
    .transform_data(|expr| {
        // closure returns a result and potentially transforms the node
        // in this example, it does transform the node
        let new_expr = make_new_expr(expr);
        Ok(Transformed::yes(new_expr))
    })
    .unwrap();
// transformed flag is the union of the original ans closure's  transformed flag
assert!(ret.transformed);
```
# Example APIs that use `TreeNode`
- [`TreeNode`],
- [`TreeNode::rewrite`],
- [`TreeNode::transform_down`],
- [`TreeNode::transform_up`],
- [`TreeNode::transform_down_up`]

---

## ConcreteTreeNode

`trait` · `datafusion_common::tree_node::ConcreteTreeNode`

```rust
trait ConcreteTreeNode: Sized
```

**Implementors** (2)

- `datafusion_physical_expr_common::tree_node::ExprContext`
- `datafusion_physical_plan::tree_node::PlanContext`

**Methods** (3)

```rust
fn children(&self) -> &[Self]
fn take_children(self) -> (Self, Vec<Self>)
fn with_new_children(self, children: Vec<Self>) -> Result<Self>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.ConcreteTreeNode.md).


Instead of implementing [`TreeNode`], it's recommended to implement a [`ConcreteTreeNode`] for
trees that contain nodes with payloads. This approach ensures safe execution of algorithms
involving payloads, by enforcing rules for detaching and reattaching child nodes.

---

## DynTreeNode

`trait` · `datafusion_common::tree_node::DynTreeNode`

```rust
trait DynTreeNode
```

**Methods** (2)

```rust
fn arc_children(&self) -> Vec<&Arc<Self>>
fn with_new_arc_children(&self, arc_self: Arc<Self>, new_children: Vec<Arc<Self>>) -> Result<Arc<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.DynTreeNode.md).


Helper trait for implementing [`TreeNode`] that have children stored as
`Arc`s. If some trait object, such as `dyn T`, implements this trait,
its related `Arc<dyn T>` will automatically implement [`TreeNode`].

---

## TransformedResult

`trait` · `datafusion_common::tree_node::TransformedResult`

```rust
trait TransformedResult<T>
```

**Implementors** (1)

- `datafusion_common::error::Result`

**Methods** (3)

```rust
fn data(self) -> Result<T>
fn tnr(self) -> Result<TreeNodeRecursion>
fn transformed(self) -> Result<bool>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TransformedResult.md).


Transformation helper to access [`Transformed`] fields in a [`Result`] easily.

# Example
Access the internal data of a `Result<Transformed<T>>`
as a `Result<T>` using the `data` method:
```
# use datafusion_common::Result;
# use datafusion_common::tree_node::{Transformed, TransformedResult};
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn update_expr() -> i64 { 1 }
# fn main() -> Result<()> {
let transformed: Result<Transformed<_>> = Ok(Transformed::yes(update_expr()));
// access the internal data of the transformed result, or return the error
let transformed_expr = transformed.data()?;
# Ok(())
# }
```

---

## TreeNode

`trait` · `datafusion_common::tree_node::TreeNode`

```rust
trait TreeNode: Sized
```

**Implementors** (3)

- `alloc::sync::Arc`
- `datafusion_expr::expr::Expr`
- `datafusion_expr::logical_plan::plan::LogicalPlan`

**Methods** (10)

```rust
fn apply<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
fn apply_children<'n, F: FnMut(&'n Self) -> Result<TreeNodeRecursion>>(&'n self, f: F) -> Result<TreeNodeRecursion>
fn exists<F: FnMut(&Self) -> Result<bool>>(&self, f: F) -> Result<bool>
fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn rewrite<R: TreeNodeRewriter<Node = Self>>(self, rewriter: &mut R) -> Result<Transformed<Self>>
fn transform<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn transform_down<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn transform_down_up<FD: FnMut(Self) -> Result<Transformed<Self>>, FU: FnMut(Self) -> Result<Transformed<Self>>>(self, f_down: FD, f_up: FU) -> Result<Transformed<Self>>
fn transform_up<F: FnMut(Self) -> Result<Transformed<Self>>>(self, f: F) -> Result<Transformed<Self>>
fn visit<'n, V: TreeNodeVisitor<'n, Node = Self>>(&'n self, visitor: &mut V) -> Result<TreeNodeRecursion>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNode.md).


API for inspecting and rewriting tree data structures.

The `TreeNode` API is used to express algorithms separately from traversing
the structure of `TreeNode`s, avoiding substantial code duplication.

This trait is implemented for plans ([`ExecutionPlan`], [`LogicalPlan`]) and
expression trees ([`PhysicalExpr`], [`Expr`]) as well as Plan+Payload
combinations [`PlanContext`] and [`ExprContext`].

# Overview
There are three categories of TreeNode APIs:

1. "Inspecting" APIs to traverse a tree of `&TreeNodes`:
   [`apply`], [`visit`], [`exists`].

2. "Transforming" APIs that traverse and consume a tree of `TreeNode`s
   producing possibly changed `TreeNode`s: [`transform`], [`transform_up`],
   [`transform_down`], [`transform_down_up`], and [`rewrite`].

3. Internal APIs used to implement the `TreeNode` API: [`apply_children`],
   and [`map_children`].

| Traversal Order | Inspecting | Transforming |
| --- | --- | --- |
| top-down | [`apply`], [`exists`] | [`transform_down`]|
| bottom-up | | [`transform`] , [`transform_up`]|
| combined with separate `f_down` and `f_up` closures | | [`transform_down_up`] |
| combined with `f_down()` and `f_up()` in an object | [`visit`]  | [`rewrite`] |

**Note**:while there is currently no in-place mutation API that uses `&mut
TreeNode`, the transforming APIs are efficient and optimized to avoid
cloning.

[`apply`]: Self::apply
[`visit`]: Self::visit
[`exists`]: Self::exists
[`transform`]: Self::transform
[`transform_up`]: Self::transform_up
[`transform_down`]: Self::transform_down
[`transform_down_up`]: Self::transform_down_up
[`rewrite`]: Self::rewrite
[`apply_children`]: Self::apply_children
[`map_children`]: Self::map_children

# Terminology
The following terms are used in this trait

* `f_down`: Invoked before any children of the current node are visited.
* `f_up`: Invoked after all children of the current node are visited.
* `f`: closure that is applied to the current node.
* `map_*`: applies a transformation to rewrite owned nodes
* `apply_*`:  invokes a function on borrowed nodes
* `transform_`: applies a transformation to rewrite owned nodes

<!-- Since these are in the datafusion-common crate, can't use intra doc links) -->
[`ExecutionPlan`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html
[`PhysicalExpr`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.PhysicalExpr.html
[`LogicalPlan`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/logical_plan/enum.LogicalPlan.html
[`Expr`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/expr/enum.Expr.html
[`PlanContext`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/tree_node/struct.PlanContext.html
[`ExprContext`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/tree_node/struct.ExprContext.html

---

## TreeNodeContainer

`trait` · `datafusion_common::tree_node::TreeNodeContainer`

```rust
trait TreeNodeContainer<'a, T: 'a>: Sized
```

**Implementors** (10)

- `alloc::boxed::Box`
- `alloc::sync::Arc`
- `alloc::vec::Vec`
- `core::option::Option`
- `datafusion_expr::expr::Expr`
- `datafusion_expr::expr::Sort`
- `datafusion_expr::logical_plan::ddl::CreateFunctionBody`
- `datafusion_expr::logical_plan::ddl::OperateFunctionArg`
- `datafusion_expr::logical_plan::plan::LogicalPlan`
- `std::collections::hash::map::HashMap`

**Methods** (2)

```rust
fn apply_elements<F: FnMut(&'a T) -> Result<TreeNodeRecursion>>(&'a self, f: F) -> Result<TreeNodeRecursion>
fn map_elements<F: FnMut(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<Self>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeContainer.md).


[`TreeNodeContainer`] contains elements that a function can be applied on or mapped.
The elements of the container are siblings so the continuation rules are similar to
[`TreeNodeRecursion::visit_sibling`] / [`Transformed::transform_sibling`].

---

## TreeNodeIterator

`trait` · `datafusion_common::tree_node::TreeNodeIterator`

```rust
trait TreeNodeIterator: Iterator
```

**Methods** (2)

```rust
fn apply_until_stop<F: FnMut(Self::Item) -> Result<TreeNodeRecursion>>(self, f: F) -> Result<TreeNodeRecursion>
fn map_until_stop_and_collect<F: FnMut(Self::Item) -> Result<Transformed<Self::Item>>>(self, f: F) -> Result<Transformed<Vec<Self::Item>>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeIterator.md).


Transformation helper to process a sequence of iterable tree nodes that are siblings.

---

## TreeNodeRefContainer

`trait` · `datafusion_common::tree_node::TreeNodeRefContainer`

```rust
trait TreeNodeRefContainer<'a, T: 'a>: Sized
```

**Implementors** (1)

- `alloc::vec::Vec`

**Methods** (1)

```rust
fn apply_ref_elements<F: FnMut(&'a T) -> Result<TreeNodeRecursion>>(&self, f: F) -> Result<TreeNodeRecursion>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeRefContainer.md).


[`TreeNodeRefContainer`] contains references to elements that a function can be
applied on. The elements of the container are siblings so the continuation rules are
similar to [`TreeNodeRecursion::visit_sibling`].

This container is similar to [`TreeNodeContainer`], but the lifetime of the reference
elements (`T`) are not derived from the container's lifetime.
A typical usage of this container is in `Expr::apply_children` when we need to
construct a temporary container to be able to call `apply_ref_elements` on a
collection of tree node references. But in that case the container's temporary
lifetime is different to the lifetime of tree nodes that we put into it.
Please find an example use case in `Expr::apply_children` with the `Expr::Case` case.

Most of the cases we don't need to create a temporary container with
`TreeNodeRefContainer`, but we can just call `TreeNodeContainer::apply_elements`.
Please find an example use case in `Expr::apply_children` with the `Expr::GroupingSet`
case.

---

## TreeNodeRewriter

`trait` · `datafusion_common::tree_node::TreeNodeRewriter`

```rust
trait TreeNodeRewriter: Sized
```

**Implementors** (4)

- `datafusion_expr::expr_rewriter::guarantees::GuaranteeRewriter`
- `datafusion_optimizer::analyzer::type_coercion::TypeCoercionRewriter`
- `datafusion_optimizer::decorrelate::PullUpCorrelatedExpr`
- `datafusion_physical_plan::column_rewriter::PhysicalColumnRewriter`

**Methods** (2)

```rust
fn f_down(&mut self, node: Self::Node) -> Result<Transformed<Self::Node>>
fn f_up(&mut self, node: Self::Node) -> Result<Transformed<Self::Node>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeRewriter.md).


A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
rewriting [`TreeNode`]s via [`TreeNode::rewrite`].

For example you can implement this trait on a struct to rewrite `Expr` or
`LogicalPlan` that needs to track state during the rewrite.

See [`TreeNode`] for more details on available APIs

When passed to [`TreeNode::rewrite`], [`TreeNodeRewriter::f_down`] and
[`TreeNodeRewriter::f_up`] are invoked recursively on the tree.
See [`TreeNodeRecursion`] for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`] for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeRewriter::f_up`] or
[`TreeNodeRewriter::f_down`] that do nothing, consider using
[`TreeNode::transform_up`] or [`TreeNode::transform_down`] instead.

# See Also:
* [`TreeNode::visit`] to inspect borrowed `TreeNode`s

---

## TreeNodeVisitor

`trait` · `datafusion_common::tree_node::TreeNodeVisitor`

```rust
trait TreeNodeVisitor<'n>: Sized
```

**Implementors** (3)

- `datafusion_expr::logical_plan::display::GraphvizVisitor`
- `datafusion_expr::logical_plan::display::IndentVisitor`
- `datafusion_expr::logical_plan::display::PgJsonVisitor`

**Methods** (2)

```rust
fn f_down(&mut self, _node: &'n Self::Node) -> Result<TreeNodeRecursion>
fn f_up(&mut self, _node: &'n Self::Node) -> Result<TreeNodeRecursion>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.tree_node.TreeNodeVisitor.md).


A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
inspecting [`TreeNode`]s via [`TreeNode::visit`].

See [`TreeNode`] for more details on available APIs

When passed to [`TreeNode::visit`], [`TreeNodeVisitor::f_down`] and
[`TreeNodeVisitor::f_up`] are invoked recursively on the tree.
See [`TreeNodeRecursion`] for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`] for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeVisitor::f_up`] or
[`TreeNodeVisitor::f_down`] that do nothing, consider using
[`TreeNode::apply`] instead.

# See Also:
* [`TreeNode::rewrite`] to rewrite owned `TreeNode`s

---
