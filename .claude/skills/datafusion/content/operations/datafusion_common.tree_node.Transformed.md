# `datafusion_common::tree_node::Transformed`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.Transformed.json).

<a id="op-dd3d2e82b5362ef73363ba21"></a>
## Transformed

`struct` · `datafusion_common::tree_node::Transformed` · datafusion-common 55.1.0

```rust
struct Transformed<T>
```

Source: `src/tree_node.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Result of tree walk / transformation APIs

`Transformed` is a wrapper around the tree node data (e.g. `Expr` or
`LogicalPlan`). It is used to indicate whether the node was transformed
and how the recursion should proceed.

[`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29) API users control the transformation by returning:
- The resulting (possibly transformed) node,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) specifying how to proceed with the recursion.

At the end of the transformation, the return value will contain:
- The final (possibly transformed) tree,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) specifying how the recursion ended.

See also
* [`Transformed::update_data`](../operations/datafusion_common.tree_node.Transformed.md#op-88a0704cb1891d830cbd571b) to modify the node without changing the `transformed` flag
* [`Transformed::map_data`](../operations/datafusion_common.tree_node.Transformed.md#op-0661c3f826cfeb162dacb938) for fallable operation that return the same type
* [`Transformed::transform_data`](../operations/datafusion_common.tree_node.Transformed.md#op-762e0ad11f2a9339aa005ddc) to chain fallable transformations
* [`TransformedResult`](../operations/datafusion_common.tree_node.TransformedResult.md#op-908e5add91b8c1def34e590f) for working with `Result<Transformed<U>>`

# Examples

Use [`Transformed::yes`](../operations/datafusion_common.tree_node.Transformed.md#op-f899f58948a58a086085aa10) and [`Transformed::no`](../operations/datafusion_common.tree_node.Transformed.md#op-268dad84b616771bdd05cb5a) to signal that a node was
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
- [`TreeNode`](../operations/datafusion_common.tree_node.TreeNode.md#op-19aa424c8e392fc74e9acd29),
- [`TreeNode::rewrite`](../operations/datafusion_common.tree_node.TreeNode.md#op-af82c9ed79a9f7ee9db9ed17),
- [`TreeNode::transform_down`](../operations/datafusion_common.tree_node.TreeNode.md#op-7f8f7d15d1860327f1638078),
- [`TreeNode::transform_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-86d9d244e6b5fe16b173f4ed),
- [`TreeNode::transform_down_up`](../operations/datafusion_common.tree_node.TreeNode.md#op-e30b76102a8ade63c654b8d6)

<a id="op-6d3e744c2fa412b2c96f1b7e"></a>
## complete

`function` · `datafusion_common::tree_node::Transformed::complete` · datafusion-common 55.1.0

```rust
fn complete(data: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapper for transformed data with [`TreeNodeRecursion::Stop`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-fbbedd1f0409ac786df535a1) statement.

<a id="op-7429216abcb499749a996c30"></a>
## data

`struct_field` · `datafusion_common::tree_node::Transformed::data` · datafusion-common 55.1.0

```rust
data: T
```

Source: `src/tree_node.rs:659`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-646cf8ea74287106de431203"></a>
## eq

`function` · `datafusion_common::tree_node::Transformed::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Transformed<T>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "$crate::cmp::PartialEq"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [657, 10], "end": [657, 19], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/tree_node.rs:657`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e68292dc3fca0d994d9ef34"></a>
## fmt

`function` · `datafusion_common::tree_node::Transformed::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [657, 21], "end": [657, 26], "filename": "src/tree_node.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/tree_node.rs:657`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0661c3f826cfeb162dacb938"></a>
## map_data

`function` · `datafusion_common::tree_node::Transformed::map_data` · datafusion-common 55.1.0

```rust
fn map_data<U, F: FnOnce(T) -> Result<U>>(self, f: F) -> Result<Transformed<U>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:702`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies a fallible `f` (returns `Result`) to the data of this
[`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) object, without modifying the `transformed` flag.

<a id="op-3fd43ca51489c67c28d9402b"></a>
## new

`function` · `datafusion_common::tree_node::Transformed::new` · datafusion-common 55.1.0

```rust
fn new(data: T, transformed: bool, tnr: TreeNodeRecursion) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new `Transformed` object with the given information.

<a id="op-aa8267b7b589fef835193f1a"></a>
## new_transformed

`function` · `datafusion_common::tree_node::Transformed::new_transformed` · datafusion-common 55.1.0

```rust
fn new_transformed(data: T, transformed: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a `Transformed` with `transformed` and [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c).

<a id="op-268dad84b616771bdd05cb5a"></a>
## no

`function` · `datafusion_common::tree_node::Transformed::no` · datafusion-common 55.1.0

```rust
fn no(data: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:690`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapper for unchanged data with [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c) statement.

<a id="op-6230ccc7157309b448ccbfcf"></a>
## tnr

`struct_field` · `datafusion_common::tree_node::Transformed::tnr` · datafusion-common 55.1.0

```rust
tnr: TreeNodeRecursion
```

Source: `src/tree_node.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff61c065b103e87511464fc3"></a>
## transform_children

`function` · `datafusion_common::tree_node::Transformed::transform_children` · datafusion-common 55.1.0

```rust
fn transform_children<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:724`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maps the [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) object to the result of the given `f` depending on the
current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) value and the fact that `f` is changing the current
node's children.

<a id="op-762e0ad11f2a9339aa005ddc"></a>
## transform_data

`function` · `datafusion_common::tree_node::Transformed::transform_data` · datafusion-common 55.1.0

```rust
fn transform_data<U, F: FnOnce(T) -> Result<Transformed<U>>>(self, f: F) -> Result<Transformed<U>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies a fallible transforming `f` to the data of this [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21)
object.

The returned `Transformed` object has the `transformed` flag set if either
`self` or the return value of `f` have the `transformed` flag set.

<a id="op-2f10118fd558f7c24145ce5a"></a>
## transform_parent

`function` · `datafusion_common::tree_node::Transformed::transform_parent` · datafusion-common 55.1.0

```rust
fn transform_parent<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:764`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maps the [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) object to the result of the given `f` depending on the
current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) value and the fact that `f` is changing the current
node's parent.

<a id="op-ec1488aced780e3217f315bd"></a>
## transform_sibling

`function` · `datafusion_common::tree_node::Transformed::transform_sibling` · datafusion-common 55.1.0

```rust
fn transform_sibling<F: FnOnce(T) -> Result<Transformed<T>>>(self, f: F) -> Result<Transformed<T>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:746`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maps the [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) object to the result of the given `f` depending on the
current [`TreeNodeRecursion`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-71e77b162ffdf243709ca1dc) value and the fact that `f` is changing the current
node's sibling.

<a id="op-1bec66bd4c374aac30f7c141"></a>
## transformed

`struct_field` · `datafusion_common::tree_node::Transformed::transformed` · datafusion-common 55.1.0

```rust
transformed: bool
```

Source: `src/tree_node.rs:660`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88a0704cb1891d830cbd571b"></a>
## update_data

`function` · `datafusion_common::tree_node::Transformed::update_data` · datafusion-common 55.1.0

```rust
fn update_data<U, F: FnOnce(T) -> U>(self, f: F) -> Transformed<U>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Applies an infallible `f` to the data of this [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) object,
without modifying the `transformed` flag.

<a id="op-f899f58948a58a086085aa10"></a>
## yes

`function` · `datafusion_common::tree_node::Transformed::yes` · datafusion-common 55.1.0

```rust
fn yes(data: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_common::tree_node::Transformed", "path": "Transformed"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [664, 1], "end": [776, 2], "filename": "src/tree_node.rs"}, "trait": null, "trait_path": null}`

Source: `src/tree_node.rs:680`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Wrapper for transformed data with [`TreeNodeRecursion::Continue`](../operations/datafusion_common.tree_node.TreeNodeRecursion.md#op-128238c101f7ed2798f6d11c) statement.
