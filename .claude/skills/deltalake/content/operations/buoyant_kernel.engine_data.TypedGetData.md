# `buoyant_kernel::engine_data::TypedGetData`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.TypedGetData.json).

<a id="op-7b8df66b2936f77e7db928fd"></a>
## TypedGetData

`trait` · `buoyant_kernel::engine_data::TypedGetData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait TypedGetData<'a, T>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L278).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:278`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This is a convenience wrapper over `GetData` to allow code like: `let name: Option<String> =
getters[1].get_opt(row_index, "metadata.name")?;`

<a id="op-2a1e5063e8e312a4509957ed"></a>
## get

`function` · `buoyant_kernel::engine_data::TypedGetData::get` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get(&'a self, row_index: usize, field_name: &str) -> DeltaResult<T>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L280).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:280`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4619641cdc122cdc0c82da05"></a>
## get_opt

`function` · `buoyant_kernel::engine_data::TypedGetData::get_opt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_opt(&'a self, row_index: usize, field_name: &str) -> DeltaResult<Option<T>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L279).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:279`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
