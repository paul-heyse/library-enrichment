# `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_builder.BinaryLikeArrayBuilder.json).

<a id="op-278de8f9614c3081b55d5e63"></a>
## BinaryLikeArrayBuilder

`trait` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder` · arrow-array 59.3.0

```rust
trait BinaryLikeArrayBuilder: ArrayBuilder
```

Source: `src/builder/generic_bytes_builder.rs:440`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for binary-like array builders

This trait provides unified interface for builders that append binary-like data
such as [`GenericBinaryBuilder<O>`](../operations/arrow_array.builder.generic_bytes_builder.GenericBinaryBuilder.md#op-8d05f60326b509bd9f5d4d69) and [`crate::builder::BinaryViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.BinaryViewBuilder.md#op-e2f59013d1dccace932df585)

<a id="op-7ea2cea2c59183c458c9df83"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Source: `src/builder/generic_bytes_builder.rs:451`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null value to the builder.

<a id="op-5ba0bbf13a6352a005eb638a"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &[u8])
```

Source: `src/builder/generic_bytes_builder.rs:448`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a non-null string value to the builder.

<a id="op-351659fdee43651035b312d9"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Source: `src/builder/generic_bytes_builder.rs:442`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a human-readable type name for the builder.

<a id="op-768bc85657f96f624b898d56"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_builder::BinaryLikeArrayBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Source: `src/builder/generic_bytes_builder.rs:445`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new builder with the given row capacity.
