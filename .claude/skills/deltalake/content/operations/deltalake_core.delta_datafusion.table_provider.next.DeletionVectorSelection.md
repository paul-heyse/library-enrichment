# `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.table_provider.next.DeletionVectorSelection.json).

<a id="op-e466c7efebebc25ac848dd42"></a>
## DeletionVectorSelection

`struct` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeletionVectorSelection
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L506).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:506`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Deletion vector selection for one data file.

<a id="op-a02d247955333b82a3092480"></a>
## clone

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeletionVectorSelection
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L505).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection", "path": "DeletionVectorSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 17], "end": [505, 22], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4179f3821a48df273763bc6c"></a>
## eq

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DeletionVectorSelection) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L505).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection", "path": "DeletionVectorSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 24], "end": [505, 33], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49774a64812847dc0ecc4e86"></a>
## filepath

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection::filepath` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
filepath: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L508).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:508`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Fully-qualified file URI.

<a id="op-217215f9a16ffcf0a5bba84f"></a>
## fmt

`function` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L505).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection", "path": "DeletionVectorSelection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [505, 10], "end": [505, 15], "filename": "crates/core/src/delta_datafusion/table_provider/next/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:505`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a93d99cd6f78d2fa7428f622"></a>
## keep_mask

`struct_field` · `deltalake_core::delta_datafusion::table_provider::next::DeletionVectorSelection::keep_mask` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
keep_mask: Vec<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/mod.rs#L510).

Source: `crates/core/src/delta_datafusion/table_provider/next/mod.rs:510`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Row-level keep mask where `true` means keep and `false` means deleted.
