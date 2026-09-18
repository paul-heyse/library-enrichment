# `datafusion_common::datatype::DataTypeExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.datatype.DataTypeExt.json).

<a id="op-23c592feb5e83fa784a191e9"></a>
## DataTypeExt

`trait` · `datafusion_common::datatype::DataTypeExt` · datafusion-common 55.1.0

```rust
trait DataTypeExt
```

Source: `src/datatype.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

DataFusion extension methods for Arrow [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)

<a id="op-d4d717ca00c50b1cb573b536"></a>
## into_nullable_field

`function` · `datafusion_common::datatype::DataTypeExt::into_nullable_field` · datafusion-common 55.1.0

```rust
fn into_nullable_field(self) -> Field
```

Source: `src/datatype.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convert the type to field with nullable type and "" name

This is used to track the places where we convert a [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)
into a nameless field to interact with an API that is
capable of representing an extension type and/or nullability.

For example, it will convert a `DataType::Int32` into
`Field::new("", DataType::Int32, true)`.

```
# use datafusion_common::datatype::DataTypeExt;
# use arrow::datatypes::DataType;
let dt = DataType::Utf8;
let field = dt.into_nullable_field();
// result is a nullable Utf8 field with "" name
assert_eq!(field.name(), "");
assert_eq!(field.data_type(), &DataType::Utf8);
assert!(field.is_nullable());
```

<a id="op-9eab0a1d7c7babe26df5f396"></a>
## into_nullable_field_ref

`function` · `datafusion_common::datatype::DataTypeExt::into_nullable_field_ref` · datafusion-common 55.1.0

```rust
fn into_nullable_field_ref(self) -> FieldRef
```

Source: `src/datatype.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convert the type to [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) with nullable type and "" name

Concise wrapper around [`DataTypeExt::into_nullable_field`](../operations/datafusion_common.datatype.DataTypeExt.md#op-d4d717ca00c50b1cb573b536) that
constructs a [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542).
