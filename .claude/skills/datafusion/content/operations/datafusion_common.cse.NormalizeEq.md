# `datafusion_common::cse::NormalizeEq`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.cse.NormalizeEq.json).

<a id="op-edbcc7233ddfeb1d626dc6cf"></a>
## NormalizeEq

`trait` · `datafusion_common::cse::NormalizeEq` · datafusion-common 55.1.0

```rust
trait NormalizeEq: Eq + Normalizeable
```

Source: `src/cse.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The `NormalizeEq` trait extends `Eq` and `Normalizeable` to provide a method for comparing
normalized nodes in optimizations like Common Subexpression Elimination (CSE).

The `normalize_eq` method ensures that two nodes that are semantically equivalent (after normalization)
are considered equal in CSE optimization, even if their original forms differ.

This trait allows for equality comparisons between nodes with equivalent semantics, regardless of their
internal representations.

<a id="op-9d5a9c468b6a75bc045b2e4c"></a>
## normalize_eq

`function` · `datafusion_common::cse::NormalizeEq::normalize_eq` · datafusion-common 55.1.0

```rust
fn normalize_eq(&self, other: &Self) -> bool
```

Source: `src/cse.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
