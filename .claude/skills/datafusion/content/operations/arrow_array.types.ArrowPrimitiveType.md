# `arrow_array::types::ArrowPrimitiveType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.ArrowPrimitiveType.json).

<a id="op-ddd581d2aed24174207ba803"></a>
## ArrowPrimitiveType

`trait` · `arrow_array::types::ArrowPrimitiveType` · arrow-array 59.3.0

```rust
trait ArrowPrimitiveType: primitive::PrimitiveTypeSealed + 'static
```

Source: `src/types.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Trait for [primitive values].

This trait bridges the dynamic-typed nature of Arrow
(via [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)) with the static-typed nature of rust types
([`ArrowNativeType`]) for all types that implement [`ArrowNativeType`].

[primitive values]: https://arrow.apache.org/docs/format/Columnar.html#fixed-size-primitive-layout
[`ArrowNativeType`]: arrow_buffer::ArrowNativeType

<a id="op-4e3fd55a44d0ecd0ff3d8d43"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::ArrowPrimitiveType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Source: `src/types.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

the corresponding Arrow data type of this primitive type.

<a id="op-7074851a90dd4b9a4c61423b"></a>
## Native

`assoc_type` · `arrow_array::types::ArrowPrimitiveType::Native` · arrow-array 59.3.0

```rust
Native
```

Source: `src/types.rs:69`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Corresponding Rust native type for the primitive type.

<a id="op-8817795a14bc7ae1d63919c8"></a>
## default_value

`function` · `arrow_array::types::ArrowPrimitiveType::default_value` · arrow-array 59.3.0

```rust
fn default_value() -> Self::Native
```

Source: `src/types.rs:77`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a default value of this primitive type.

This is useful for aggregate array ops like `sum()`, `mean()`.
