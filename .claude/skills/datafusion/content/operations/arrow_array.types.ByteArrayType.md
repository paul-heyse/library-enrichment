# `arrow_array::types::ByteArrayType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.ByteArrayType.json).

<a id="op-f2bd8184abd4b6c92211ea20"></a>
## ByteArrayType

`trait` · `arrow_array::types::ByteArrayType` · arrow-array 59.3.0

```rust
trait ByteArrayType: 'static + Send + Sync + bytes::ByteArrayTypeSealed
```

Source: `src/types.rs:1636`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A trait over the variable-size byte array types

See [Variable Size Binary Layout](https://arrow.apache.org/docs/format/Columnar.html#variable-size-binary-layout)

<a id="op-73b69052b34c9ad5166b9edb"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::ByteArrayType::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Source: `src/types.rs:1648`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Datatype of array elements

<a id="op-24bbe9ce1c57565d37b56759"></a>
## Native

`assoc_type` · `arrow_array::types::ByteArrayType::Native` · arrow-array 59.3.0

```rust
Native
```

Source: `src/types.rs:1642`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Type for representing its equivalent rust type i.e
Utf8Array will have native type has &str
BinaryArray will have type as [u8]

Unresolved upstream links (retained, not inferred): `u8`.

<a id="op-a993977a9410001ef7dd8d69"></a>
## Offset

`assoc_type` · `arrow_array::types::ByteArrayType::Offset` · arrow-array 59.3.0

```rust
Offset
```

Source: `src/types.rs:1638`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Type of offset i.e i32/i64

<a id="op-990a035288215ba9477785bd"></a>
## PREFIX

`assoc_const` · `arrow_array::types::ByteArrayType::PREFIX` · arrow-array 59.3.0

```rust
PREFIX
```

Source: `src/types.rs:1645`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

"Binary" or "String", for use in error messages

<a id="op-343d1729c3e8ad4d83a8dea0"></a>
## validate

`function` · `arrow_array::types::ByteArrayType::validate` · arrow-array 59.3.0

```rust
fn validate(offsets: &OffsetBuffer<Self::Offset>, values: &Buffer) -> Result<(), ArrowError>
```

Source: `src/types.rs:1651`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Verifies that every consecutive pair of `offsets` denotes a valid slice of `values`
