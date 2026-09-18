# `buoyant_kernel::engine::arrow_utils::ReorderIndex`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.ReorderIndex.json).

<a id="op-cfc83e8a81b22c2e6ebd97c3"></a>
## ReorderIndex

`struct` · `buoyant_kernel::engine::arrow_utils::ReorderIndex` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ReorderIndex
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L332).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:332`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Reordering is specified as a tree. Each level is a vec of `ReorderIndex`s. Each element's
position represents a column that will be in the read parquet data at that level and
position. The `index` of the element is the position that the column should appear in the final
output. The `transform` indicates what, if any, transforms are needed. See the docs for
[`ReorderIndexTransform`](../operations/buoyant_kernel.engine.arrow_utils.ReorderIndexTransform.md#op-ae68d4af5283608cbd6393e5) for the meaning.

<a id="op-d950ab9205cb51c17384a2f6"></a>
## eq

`function` · `buoyant_kernel::engine::arrow_utils::ReorderIndex::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ReorderIndex) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_utils::ReorderIndex", "path": "ReorderIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 17], "end": [330, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:330`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c5bc08a75028967801ae814"></a>
## fmt

`function` · `buoyant_kernel::engine::arrow_utils::ReorderIndex::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L330).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::arrow_utils::ReorderIndex", "path": "ReorderIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [330, 10], "end": [330, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:330`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d906877778761794a53b3370"></a>
## index

`struct_field` · `buoyant_kernel::engine::arrow_utils::ReorderIndex::index` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
index: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L333).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:333`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44b04f609dc2000cc7c57a89"></a>
## transform

`struct_field` · `buoyant_kernel::engine::arrow_utils::ReorderIndex::transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
transform: ReorderIndexTransform
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L334).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:334`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
