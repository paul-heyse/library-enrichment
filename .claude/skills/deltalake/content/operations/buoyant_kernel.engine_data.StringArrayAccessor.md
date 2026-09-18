# `buoyant_kernel::engine_data::StringArrayAccessor`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine_data.StringArrayAccessor.json).

<a id="op-802ca76bf3bcd8ef3d667bbb"></a>
## StringArrayAccessor

`trait` · `buoyant_kernel::engine_data::StringArrayAccessor` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait StringArrayAccessor
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Uniform read access to a string array, abstracting over the various string representations
that list and map columns may use (e.g. Utf8, LargeUtf8, Utf8View). Engines implement this
for their string array types so that [`ListItem`](../operations/buoyant_kernel.engine_data.ListItem.md#op-65f266dfcf5c42f50a245940) and [`MapItem`](../operations/buoyant_kernel.engine_data.MapItem.md#op-4618344877f857b56da06722) can resolve the concrete
type once at construction and access elements via virtual dispatch thereafter.

<a id="op-16f6e1d558ae5b76717f8f5f"></a>
## is_empty

`function` · `buoyant_kernel::engine_data::StringArrayAccessor::is_empty` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_empty(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L117).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:117`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns whether the array has no elements.

<a id="op-28823efa23cb72748568e828"></a>
## is_valid

`function` · `buoyant_kernel::engine_data::StringArrayAccessor::is_valid` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_valid(&self, index: usize) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L123).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:123`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns whether the value at the given index is non-null.

<a id="op-42e16f98e67ebcef52310e3c"></a>
## len

`function` · `buoyant_kernel::engine_data::StringArrayAccessor::len` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn len(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the number of elements in the array.

<a id="op-58b2cf752c9bed1603c9e273"></a>
## value

`function` · `buoyant_kernel::engine_data::StringArrayAccessor::value` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn value(&self, index: usize) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine_data.rs#L121).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine_data.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the string value at the given index. The caller must ensure `index < len()`.
