# `buoyant_kernel::engine::ensure_data_types::DataTypeCompat`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.ensure_data_types.DataTypeCompat.json).

<a id="op-0d571d6c92264faffc88007f"></a>
## DataTypeCompat

`enum` · `buoyant_kernel::engine::ensure_data_types::DataTypeCompat` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DataTypeCompat
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L60).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:60`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Capture the compatibility between two data-types, as passed to [`ensure_data_types`](../operations/buoyant_kernel.engine.ensure_data_types.ensure_data_types.md#op-6a09270f4b3ca4f205abd9f0)

<a id="op-56a2d2143ba3bf912d44331e"></a>
## Identical

`variant` · `buoyant_kernel::engine::ensure_data_types::DataTypeCompat::Identical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Identical
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L62).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The two types are the same

<a id="op-0f4264bf6625aed7a82bceb8"></a>
## NeedsCast

`variant` · `buoyant_kernel::engine::ensure_data_types::DataTypeCompat::NeedsCast` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NeedsCast
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L64).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:64`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

What is read from parquet needs to be cast to the associated type

<a id="op-a92bb3d1e45bbd9839c62d83"></a>
## Nested

`variant` · `buoyant_kernel::engine::ensure_data_types::DataTypeCompat::Nested` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Nested
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/ensure_data_types.rs#L67).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/ensure_data_types.rs:67`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Types are compatible, but are nested types. This is used when comparing types where casting
is not desired (i.e. in the expression evaluator)
