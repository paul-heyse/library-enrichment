# `datafusion_common::datatype::FieldExt`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.datatype.FieldExt.json).

<a id="op-54697fa903a072da45f5dc64"></a>
## FieldExt

`trait` · `datafusion_common::datatype::FieldExt` · datafusion-common 55.1.0

```rust
trait FieldExt
```

Source: `src/datatype.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

DataFusion extension methods for Arrow [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) and [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)

This trait is implemented for both [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) and [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) and
provides convenience methods for efficiently working with both types.

For [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542), the methods will attempt to unwrap the `Arc`
to avoid unnecessary cloning when possible.

<a id="op-3a9584aa902f415a1e7c01ff"></a>
## into_fixed_size_list

`function` · `datafusion_common::datatype::FieldExt::into_fixed_size_list` · datafusion-common 55.1.0

```rust
fn into_fixed_size_list(self, list_size: i32) -> Self
```

Source: `src/datatype.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a new Field representing this Field as the item type of a
[`DataType::FixedSizeList`](../operations/arrow_schema.datatype.DataType.md#op-69f9b11fdd4d79bf23810b3a)

For example if input represents an `Int32`, the return value will
represent a `FixedSizeList<Int32, size>`.

Example:
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::datatype::FieldExt;
// Int32 field
let int_field = Field::new("my_int", DataType::Int32, true);
// convert to a FixedSizeList field of size 3
let fixed_size_list_field = int_field.into_fixed_size_list(3);
// FixedSizeList<Int32, 3>
// Note that the item field name has been renamed to "item"
assert_eq!(
  fixed_size_list_field.data_type(),
  &DataType::FixedSizeList(Arc::new(
   Field::new("item", DataType::Int32, true)),
   3
));

<a id="op-8c6d8b6da6323acf8bb4218f"></a>
## into_list

`function` · `datafusion_common::datatype::FieldExt::into_list` · datafusion-common 55.1.0

```rust
fn into_list(self) -> Self
```

Source: `src/datatype.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns a new Field representing a List of this Field's DataType.

For example if input represents an `Int32`, the return value will
represent a `List<Int32>`.

Example:
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::datatype::FieldExt;
// Int32 field
let int_field = Field::new("my_int", DataType::Int32, true);
// convert to a List field
let list_field = int_field.into_list();
// List<Int32>
// Note that the item field name has been renamed to "item"
assert_eq!(list_field.data_type(), &DataType::List(Arc::new(
    Field::new("item", DataType::Int32, true)
)));

<a id="op-1752ffdf998978a73893cddf"></a>
## into_list_item

`function` · `datafusion_common::datatype::FieldExt::into_list_item` · datafusion-common 55.1.0

```rust
fn into_list_item(self) -> Self
```

Source: `src/datatype.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Update the field to have the default list field name ("item")

Lists are allowed to have an arbitrarily named field; however, a name
other than 'item' will cause it to fail an == check against a more
idiomatically created list in arrow-rs which causes issues.

For example, if input represents an `Int32` field named "my_int",
the return value will represent an `Int32` field named "item".

Example:
```
# use arrow::datatypes::Field;
# use datafusion_common::datatype::FieldExt;
let my_field = Field::new("my_int", arrow::datatypes::DataType::Int32, true);
let item_field = my_field.into_list_item();
assert_eq!(item_field.name(), Field::LIST_FIELD_DEFAULT_NAME);
assert_eq!(item_field.name(), "item");
```

<a id="op-3714ef82752897e8eeba6a11"></a>
## renamed

`function` · `datafusion_common::datatype::FieldExt::renamed` · datafusion-common 55.1.0

```rust
fn renamed(self, new_name: &str) -> Self
```

Source: `src/datatype.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Ensure the field is named `new_name`, returning the given field if the
name matches, and a new field if not.

This method avoids `clone`ing fields and names if the name is the same
as the field's existing name.

Example:
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::datatype::FieldExt;
let int_field = Field::new("my_int", DataType::Int32, true);
// rename to "your_int"
let renamed_field = int_field.renamed("your_int");
assert_eq!(renamed_field.name(), "your_int");
```

<a id="op-df15cbbe3333687f8d73d812"></a>
## retyped

`function` · `datafusion_common::datatype::FieldExt::retyped` · datafusion-common 55.1.0

```rust
fn retyped(self, new_data_type: DataType) -> Self
```

Source: `src/datatype.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Ensure the field has the given data type

Note this is different than simply calling [`Field::with_data_type`] as
it avoids copying if the data type is already the same.

Example:
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::datatype::FieldExt;
let int_field = Field::new("my_int", DataType::Int32, true);
// change to Float64
let retyped_field = int_field.retyped(DataType::Float64);
assert_eq!(retyped_field.data_type(), &DataType::Float64);
```

Unresolved upstream links (retained, not inferred): ``Field::with_data_type``.

<a id="op-fb69ee903a8b69fd6b90d6ff"></a>
## with_field_metadata

`function` · `datafusion_common::datatype::FieldExt::with_field_metadata` · datafusion-common 55.1.0

```rust
fn with_field_metadata(self, metadata: &FieldMetadata) -> Self
```

Source: `src/datatype.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add field metadata to the Field

<a id="op-963849f2b5a62afbd72c501c"></a>
## with_field_metadata_opt

`function` · `datafusion_common::datatype::FieldExt::with_field_metadata_opt` · datafusion-common 55.1.0

```rust
fn with_field_metadata_opt(self, metadata: Option<&FieldMetadata>) -> Self
```

Source: `src/datatype.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add optional field metadata,
