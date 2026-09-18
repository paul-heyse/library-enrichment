# `buoyant_kernel::engine::ensure_data_types::ValidationMode`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.ensure_data_types.ValidationMode.json).

<a id="op-6fb2bbb0327bb0fa70d05659"></a>
## ValidationMode

`enum` · `buoyant_kernel::engine::ensure_data_types::ValidationMode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ValidationMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L19).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Controls how `ensure_data_types` validates struct fields and metadata.

<a id="op-6079111f0c7f17741480a53b"></a>
## Full

`variant` · `buoyant_kernel::engine::ensure_data_types::ValidationMode::Full` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Full
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check types, names, nullability, and metadata.

<a id="op-ae6ed6517212de4d73ac300b"></a>
## TypesAndNames

`variant` · `buoyant_kernel::engine::ensure_data_types::ValidationMode::TypesAndNames` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TypesAndNames
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check types and match struct fields by name, but skip nullability and metadata.
Used by the parquet reader where fields are already resolved by name upstream.

<a id="op-ba687c49ae91eb865241c1d5"></a>
## TypesOnly

`variant` · `buoyant_kernel::engine::ensure_data_types::ValidationMode::TypesOnly` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TypesOnly
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Check types only. Struct fields are matched by ordinal position, not by name.
Nullability and metadata are not checked.

<a id="op-903e26c6e84c998b83653d7b"></a>
## clone

`function` · `buoyant_kernel::engine::ensure_data_types::ValidationMode::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ValidationMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L17).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::engine::ensure_data_types::ValidationMode", "path": "ValidationMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 10], "end": [17, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
