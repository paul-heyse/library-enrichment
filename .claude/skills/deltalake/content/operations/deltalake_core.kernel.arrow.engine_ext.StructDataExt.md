# `deltalake_core::kernel::arrow::engine_ext::StructDataExt`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.arrow.engine_ext.StructDataExt.json).

<a id="op-2e21316c1bdd8c5c38cb1a68"></a>
## StructDataExt

`trait` · `deltalake_core::kernel::arrow::engine_ext::StructDataExt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait StructDataExt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/arrow/engine_ext.rs#L536).

Source: `crates/core/src/kernel/arrow/engine_ext.rs:536`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Extension trait for Kernel's [`StructData`](../operations/buoyant_kernel.expressions.scalars.StructData.md#op-f4f7b624bc21c78ae21a9def).

StructData is the data structure contained in a Struct scalar.
The exposed API on kernels struct data is very minimal and does not allow
for conveniently probing the fields / values contained within [`StructData`](../operations/buoyant_kernel.expressions.scalars.StructData.md#op-f4f7b624bc21c78ae21a9def).

This trait therefore adds convenience methods for accessing fields and values.

<a id="op-1f9f238bb61a5cf0cdda8f8c"></a>
## field

`function` · `deltalake_core::kernel::arrow::engine_ext::StructDataExt::field` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field(&self, name: &str) -> Option<&StructField>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/arrow/engine_ext.rs#L538).

Source: `crates/core/src/kernel/arrow/engine_ext.rs:538`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns a reference to the field with the given name, if it exists.

<a id="op-1555d922c9099598bd41954e"></a>
## index_of

`function` · `deltalake_core::kernel::arrow::engine_ext::StructDataExt::index_of` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn index_of(&self, name: &str) -> Option<usize>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/arrow/engine_ext.rs#L544).

Source: `crates/core/src/kernel/arrow/engine_ext.rs:544`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns the index of the field with the given name, if it exists.

<a id="op-69bb202d27d4b97104389c7f"></a>
## value

`function` · `deltalake_core::kernel::arrow::engine_ext::StructDataExt::value` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn value(&self, index: usize) -> Option<&Scalar>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/arrow/engine_ext.rs#L541).

Source: `crates/core/src/kernel/arrow/engine_ext.rs:541`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Returns a reference to the value with the given index, if it exists.
