# `arrow_array::types::ByteViewType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.ByteViewType.json).

<a id="op-db53530cebe592e4d127f752"></a>
## ByteViewType

`trait` · `arrow_array::types::ByteViewType` · arrow-array 59.3.0

```rust
trait ByteViewType: byte_view::Sealed + 'static + PartialEq + Send + Sync
```

Source: `src/types.rs:1743`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait over the variable length bytes view array types

<a id="op-62f61835a0978e409bf698e9"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::ByteViewType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Source: `src/types.rs:1748`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Datatype of array elements

<a id="op-86e10071f211bb73f886cb15"></a>
## IS_UTF8

`assoc_const` · `arrow_array::types::ByteViewType::IS_UTF8` · arrow-array 59.3.0

```rust
IS_UTF8
```

Source: `src/types.rs:1745`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

If element in array is utf8 encoded string.

<a id="op-438b6963d26d04dbca69df81"></a>
## Native

`assoc_type` · `arrow_array::types::ByteViewType::Native` · arrow-array 59.3.0

```rust
Native
```

Source: `src/types.rs:1760`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Type for representing its equivalent rust type i.e
Utf8Array will have native type has &str
BinaryArray will have type as [u8]

Unresolved upstream links (retained, not inferred): `u8`.

<a id="op-7af2fb564e5bee04ba81ad83"></a>
## Owned

`assoc_type` · `arrow_array::types::ByteViewType::Owned` · arrow-array 59.3.0

```rust
Owned
```

Source: `src/types.rs:1763`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Type for owned corresponding to `Native`

<a id="op-d8095f1213dd84ae14e1761e"></a>
## PREFIX

`assoc_const` · `arrow_array::types::ByteViewType::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Source: `src/types.rs:1755`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

"Binary" or "String", for use in displayed or error messages

<a id="op-9cd62882abef2950e75e1ec5"></a>
## validate

`function` · `arrow_array::types::ByteViewType::validate` · arrow-array 59.3.0

```rust
fn validate(views: &[u128], buffers: &[Buffer]) -> Result<(), ArrowError>
```

Source: `src/types.rs:1766`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Verifies that the provided buffers are valid for this array type
