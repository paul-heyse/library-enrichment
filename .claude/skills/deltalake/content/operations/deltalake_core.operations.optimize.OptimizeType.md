# `deltalake_core::operations::optimize::OptimizeType`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.OptimizeType.json).

<a id="op-b6382abef05a53f0986f1e64"></a>
## OptimizeType

`enum` · `deltalake_core::operations::optimize::OptimizeType` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum OptimizeType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L269).

Source: `crates/core/src/operations/optimize.rs:269`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Type of optimization to perform.

<a id="op-8ac0bd12e19647702a9265ff"></a>
## Compact

`variant` · `deltalake_core::operations::optimize::OptimizeType::Compact` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Compact
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L271).

Source: `crates/core/src/operations/optimize.rs:271`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Compact files into pre-determined bins

<a id="op-680b31b3fa20002b55fc91a0"></a>
## ZOrder

`variant` · `deltalake_core::operations::optimize::OptimizeType::ZOrder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ZOrder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L273).

Source: `crates/core/src/operations/optimize.rs:273`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Z-order files based on provided columns

<a id="op-67384d1bd23ff50d90a75ab2"></a>
## fmt

`function` · `deltalake_core::operations::optimize::OptimizeType::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L268).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::OptimizeType", "path": "OptimizeType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 10], "end": [268, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:268`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
