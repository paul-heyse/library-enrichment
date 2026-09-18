# `datafusion_common::tree_node::TransformedResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.tree_node.TransformedResult.json).

<a id="op-908e5add91b8c1def34e590f"></a>
## TransformedResult

`trait` · `datafusion_common::tree_node::TransformedResult` · datafusion-common 55.1.0

```rust
trait TransformedResult<T>
```

Source: `src/tree_node.rs:1241`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Transformation helper to access [`Transformed`](../operations/datafusion_common.tree_node.Transformed.md#op-dd3d2e82b5362ef73363ba21) fields in a [`Result`](../operations/datafusion_common.error.Result.md#op-b73a5a953660113193cd6983) easily.

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

<a id="op-763a956f8bc1ac2d79fa6e22"></a>
## data

`function` · `datafusion_common::tree_node::TransformedResult::data` · datafusion-common 55.1.0

```rust
fn data(self) -> Result<T>
```

Source: `src/tree_node.rs:1242`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-134caa6df6d25f49e6c9801f"></a>
## tnr

`function` · `datafusion_common::tree_node::TransformedResult::tnr` · datafusion-common 55.1.0

```rust
fn tnr(self) -> Result<TreeNodeRecursion>
```

Source: `src/tree_node.rs:1246`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5c43a6f89c8e5a88eb8f98d"></a>
## transformed

`function` · `datafusion_common::tree_node::TransformedResult::transformed` · datafusion-common 55.1.0

```rust
fn transformed(self) -> Result<bool>
```

Source: `src/tree_node.rs:1244`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
