# `datafusion_common::scalar::struct_builder::ScalarStructBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.scalar.struct_builder.ScalarStructBuilder.json).

<a id="op-ab6c9d48ccd748e1372fa840"></a>
## ScalarStructBuilder

`struct` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder` · datafusion-common 55.1.0

```rust
struct ScalarStructBuilder
```

Source: `src/scalar/struct_builder.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Builder for [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809).

See examples on [`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b)

<a id="op-345f60775b8d3acfd19c694b"></a>
## build

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::build` · datafusion-common 55.1.0

```rust
fn build(self) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809) with the fields and values added so far

# Errors

If the [`StructArray`](../operations/arrow_array.array.struct_array.StructArray.md#op-994d82c95af72fa0e3aa6207) cannot be created (for example if there is a
mismatch between field types and arrays) or the arrays do not have
exactly one element.

<a id="op-9e8bec5dce1467b949145714"></a>
## default

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::default` · datafusion-common 55.1.0

```rust
fn default() -> ScalarStructBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 17], "end": [28, 24], "filename": "src/scalar/struct_builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/scalar/struct_builder.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-160e15d187bd5dfa5aa74400"></a>
## fmt

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 15], "filename": "src/scalar/struct_builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar/struct_builder.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b3b3d8c5c42f8ae57ac6512"></a>
## new

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::new` · datafusion-common 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new `ScalarStructBuilder`

<a id="op-da0da984aa3719e48e24d366"></a>
## new_null

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::new_null` · datafusion-common 55.1.0

```rust
fn new_null(fields: impl IntoFields) -> ScalarValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return a new [`ScalarValue::Struct`](../operations/datafusion_common.scalar.ScalarValue.md#op-71bed3d8a14df037fb466809) with a single `null` value.

Note this is different from a struct where each of the specified fields
are null (e.g. `{a: NULL}`)

# Example

```rust
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::scalar::ScalarStructBuilder;
let fields = vec![Field::new("a", DataType::Int32, false)];
let sv = ScalarStructBuilder::new_null(fields);
// Note this is `NULL`, not `{a: NULL}`
assert_eq!(format!("{sv}"), "NULL");
```

To create a struct where the *fields* are null, use `Self::new()` and
pass null values for each field:

```rust
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::scalar::{ScalarStructBuilder, ScalarValue};
// make a nullable field
let field = Field::new("a", DataType::Int32, true);
// add a null value for the "a" field
let sv = ScalarStructBuilder::new()
    .with_scalar(field, ScalarValue::Int32(None))
    .build()
    .unwrap();
// value is not null, but field is
assert_eq!(format!("{sv}"), "{a:}");
```

<a id="op-c7a1f774d955404a837a5e92"></a>
## with_array

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::with_array` · datafusion-common 55.1.0

```rust
fn with_array(self, field: impl IntoFieldRef, value: ArrayRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add the specified field and [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1) to the struct.

Note the array should have a single row.

<a id="op-6db2df5008306ef5fb5a0a02"></a>
## with_name_and_scalar

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::with_name_and_scalar` · datafusion-common 55.1.0

```rust
fn with_name_and_scalar(self, name: &str, value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add a field with the specified name and value to the struct.
the field is created with the specified data type and as non nullable

<a id="op-97ec0d6bb6390573b69a4073"></a>
## with_scalar

`function` · `datafusion_common::scalar::struct_builder::ScalarStructBuilder::with_scalar` · datafusion-common 55.1.0

```rust
fn with_scalar(self, field: impl IntoFieldRef, value: ScalarValue) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::scalar::struct_builder::ScalarStructBuilder", "path": "ScalarStructBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [114, 2], "filename": "src/scalar/struct_builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar/struct_builder.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Add the specified field and `ScalarValue` to the struct.
