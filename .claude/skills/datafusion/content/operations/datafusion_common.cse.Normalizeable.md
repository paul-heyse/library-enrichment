# `datafusion_common::cse::Normalizeable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.Normalizeable.json).

<a id="op-6e36008347fd4b1a66e3cbb4"></a>
## Normalizeable

`trait` · `datafusion_common::cse::Normalizeable` · datafusion-common 55.1.0

```rust
trait Normalizeable
```

Source: `src/cse.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The `Normalizeable` trait defines a method to determine whether a node can be normalized.

Normalization is the process of converting a node into a canonical form that can be used
to compare nodes for equality. This is useful in optimizations like Common Subexpression Elimination (CSE),
where semantically equivalent nodes (e.g., `a + b` and `b + a`) should be treated as equal.

<a id="op-f65f6994ed839c90c604a36c"></a>
## can_normalize

`function` · `datafusion_common::cse::Normalizeable::can_normalize` · datafusion-common 55.1.0

```rust
fn can_normalize(&self) -> bool
```

Source: `src/cse.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
