# `datafusion_common::metadata`

Crate `datafusion-common` · 4 public items · structured records in [`model/datafusion_common.metadata.json`](../model/datafusion_common.metadata.json)

## check_metadata_with_storage_equal

`function` · `datafusion_common::metadata::check_metadata_with_storage_equal`

```rust
fn check_metadata_with_storage_equal(actual: (&arrow::datatypes::DataType, Option<&std::collections::HashMap<String, String>>), expected: (&arrow::datatypes::DataType, Option<&std::collections::HashMap<String, String>>), what: &str, context: &str) -> Result<(), DataFusionError>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.metadata.check_metadata_with_storage_equal.md).


Assert equality of data types where one or both sides may have field metadata

This currently compares absent metadata (e.g., one side was a DataType) and
empty metadata (e.g., one side was a field where the field had no metadata)
as equal and uses byte-for-byte comparison for the keys and values of the
fields, even though this is potentially too strict for some cases (e.g.,
extension types where extension metadata is represented by JSON, or cases
where field metadata is orthogonal to the interpretation of the data type).

Returns a planning error with suitably formatted type representations if
actual and expected do not compare to equal.

---

## format_type_and_metadata

`function` · `datafusion_common::metadata::format_type_and_metadata`

```rust
fn format_type_and_metadata(data_type: &arrow::datatypes::DataType, metadata: Option<&std::collections::HashMap<String, String>>) -> String
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.metadata.format_type_and_metadata.md).


Given a data type represented by storage and optional metadata, generate
a user-facing string

This function exists to reduce the number of Field debug strings that are
used to communicate type information in error messages and plan explain
renderings.

---

## FieldMetadata

`struct` · `datafusion_common::metadata::FieldMetadata`

Also reachable as `datafusion_expr::expr::FieldMetadata`

```rust
struct FieldMetadata
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
fn add_to_field(&self, field: Field) -> Field
fn add_to_field_ref(&self, field_ref: FieldRef) -> FieldRef
fn extend(&mut self, other: Self)
fn inner(&self) -> &BTreeMap<String, String>
fn into_inner(self) -> Arc<BTreeMap<String, String>>
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn merge_options(m: Option<&FieldMetadata>, n: Option<&FieldMetadata>) -> Option<FieldMetadata>
fn new(inner: BTreeMap<String, String>) -> Self
fn new_empty() -> Self
fn new_from_field(field: &Field) -> Self
fn to_hashmap(&self) -> std::collections::HashMap<String, String>
```

**via `core::convert::From`**

```rust
fn from(field: &Field) -> Self
fn from(map: &std::collections::HashMap<String, String>) -> Self
fn from(map: HashMap<String, String>) -> Self
fn from(inner: BTreeMap<String, String>) -> Self
fn from(map: &HashMap<String, String>) -> Self
fn from(map: std::collections::HashMap<String, String>) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.metadata.FieldMetadata.md).


Literal metadata

Stores metadata associated with a literal expressions
and is designed to be fast to `clone`.

This structure is used to store metadata associated with a literal expression, and it
corresponds to the `metadata` field on [`Field`].

# Example: Create [`FieldMetadata`] from a [`Field`]
```
# use std::collections::HashMap;
# use datafusion_common::metadata::FieldMetadata;
# use arrow::datatypes::{Field, DataType};
# let field = Field::new("c1", DataType::Int32, true)
#  .with_metadata(HashMap::from([("foo".to_string(), "bar".to_string())]));
// Create a new `FieldMetadata` instance from a `Field`
let metadata = FieldMetadata::new_from_field(&field);
// There is also a `From` impl:
let metadata = FieldMetadata::from(&field);
```

# Example: Update a [`Field`] with [`FieldMetadata`]
```
# use datafusion_common::metadata::FieldMetadata;
# use arrow::datatypes::{Field, DataType};
# let field = Field::new("c1", DataType::Int32, true);
# let metadata = FieldMetadata::new_from_field(&field);
// Add any metadata from `FieldMetadata` to `Field`
let updated_field = metadata.add_to_field(field);
```

For more background, please also see the [Implementing User Defined Types and Custom Metadata in DataFusion blog]

[Implementing User Defined Types and Custom Metadata in DataFusion blog]: https://datafusion.apache.org/blog/2025/09/21/custom-types-using-metadata

---

## ScalarAndMetadata

`struct` · `datafusion_common::metadata::ScalarAndMetadata`

```rust
struct ScalarAndMetadata
```

**Fields**: `value`, `metadata`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn cast_storage_to(&self, target_type: &DataType) -> Result<Self, DataFusionError>
fn into_inner(self) -> (ScalarValue, Option<FieldMetadata>)
fn metadata(&self) -> Option<&FieldMetadata>
fn new(value: ScalarValue, metadata: Option<FieldMetadata>) -> Self
fn value(&self) -> &ScalarValue
```

**via `core::convert::From`**

```rust
fn from(value: ScalarValue) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.metadata.ScalarAndMetadata.md).


A [`ScalarValue`] with optional [`FieldMetadata`]

---
