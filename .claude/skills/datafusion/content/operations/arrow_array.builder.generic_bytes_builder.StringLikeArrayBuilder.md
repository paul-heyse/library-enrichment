# `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_builder.StringLikeArrayBuilder.json).

<a id="op-486ef26a01d06b840a04b0b8"></a>
## StringLikeArrayBuilder

`trait` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder` · arrow-array 59.3.0

```rust
trait StringLikeArrayBuilder: ArrayBuilder
```

Source: `src/builder/generic_bytes_builder.rs:396`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for string-like array builders

This trait provides unified interface for builders that append string-like data
such as [`GenericStringBuilder<O>`](../operations/arrow_array.builder.generic_bytes_builder.GenericStringBuilder.md#op-261f9ccc4c15fcce9de29b74) and [`crate::builder::StringViewBuilder`](../operations/arrow_array.builder.generic_bytes_view_builder.StringViewBuilder.md#op-dd25f73e404dcf18f7926e36)

<a id="op-655126491cec3eb2f3fd49c5"></a>
## append_null

`function` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder::append_null` · arrow-array 59.3.0

```rust
fn append_null(&mut self)
```

Source: `src/builder/generic_bytes_builder.rs:407`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a null value to the builder.

<a id="op-4c9c5cac389b956935c2e20a"></a>
## append_value

`function` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder::append_value` · arrow-array 59.3.0

```rust
fn append_value(&mut self, value: &str)
```

Source: `src/builder/generic_bytes_builder.rs:404`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Appends a non-null string value to the builder.

<a id="op-23f9440f9ece10be109dc434"></a>
## type_name

`function` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder::type_name` · arrow-array 59.3.0

```rust
fn type_name() -> &'static str
```

Source: `src/builder/generic_bytes_builder.rs:398`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a human-readable type name for the builder.

<a id="op-fe699b0f852919f78bcc1d00"></a>
## with_capacity

`function` · `arrow_array::builder::generic_bytes_builder::StringLikeArrayBuilder::with_capacity` · arrow-array 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Source: `src/builder/generic_bytes_builder.rs:401`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new builder with the given row capacity.
