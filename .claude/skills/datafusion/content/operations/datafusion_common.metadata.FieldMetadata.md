# `datafusion_common::metadata::FieldMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.metadata.FieldMetadata.json).

<a id="op-8afcac4d754acb196812c528"></a>
## FieldMetadata

`struct` · `datafusion_common::metadata::FieldMetadata` · datafusion-common 55.1.0

```rust
struct FieldMetadata
```

Source: `src/metadata.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Literal metadata

Stores metadata associated with a literal expressions
and is designed to be fast to `clone`.

This structure is used to store metadata associated with a literal expression, and it
corresponds to the `metadata` field on [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf).

# Example: Create [`FieldMetadata`](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528) from a [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)
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

# Example: Update a [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) with [`FieldMetadata`](../operations/datafusion_common.metadata.FieldMetadata.md#op-8afcac4d754acb196812c528)
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

<a id="op-8d234c9827304535a4419cbb"></a>
## add_to_field

`function` · `datafusion_common::metadata::FieldMetadata::add_to_field` · datafusion-common 55.1.0

```rust
fn add_to_field(&self, field: Field) -> Field
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Updates the metadata on the Field with this metadata, if it is not empty.

<a id="op-9b71f51ceb398a38b21fa907"></a>
## add_to_field_ref

`function` · `datafusion_common::metadata::FieldMetadata::add_to_field_ref` · datafusion-common 55.1.0

```rust
fn add_to_field_ref(&self, field_ref: FieldRef) -> FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Updates the metadata on the FieldRef with this metadata, if it is not empty.

<a id="op-17b06cdc98ba20449d0fa510"></a>
## clone

`function` · `datafusion_common::metadata::FieldMetadata::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> FieldMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 10], "end": [178, 15], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e94cba963831708f63fd3c6b"></a>
## default

`function` · `datafusion_common::metadata::FieldMetadata::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [192, 2], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/metadata.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfd090f1e501f4035e35f93d"></a>
## eq

`function` · `datafusion_common::metadata::FieldMetadata::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &FieldMetadata) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 17], "end": [178, 26], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e712bc307637a9e3bca08f78"></a>
## extend

`function` · `datafusion_common::metadata::FieldMetadata::extend` · datafusion-common 55.1.0

```rust
fn extend(&mut self, other: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adds metadata from `other` into `self`, overwriting any existing keys.

<a id="op-d108fae9df8d542fe96c4710"></a>
## fmt

`function` · `datafusion_common::metadata::FieldMetadata::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 50], "end": [178, 55], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05af86e8e6c6dc32b0b413a8"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(field: &Field) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 1], "end": [343, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d642a8f72d327f7e47e2f00"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(map: std::collections::HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [355, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4887ce88dd8ee5f96d7eeab6"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(map: &std::collections::HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [358, 1], "end": [366, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a61a4d6bdccd4b5d52b0ebd"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(map: &HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [376, 1], "end": [384, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "hashbrown::map::HashMap", "path": "HashMap"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:377`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93ef419946b4fe52e03cf31d"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(inner: BTreeMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [349, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "alloc::collections::btree::map::BTreeMap", "path": "BTreeMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d20988042de853dbae08c96d"></a>
## from

`function` · `datafusion_common::metadata::FieldMetadata::from` · datafusion-common 55.1.0

```rust
fn from(map: HashMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [374, 2], "filename": "src/metadata.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}, {"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "hashbrown::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/metadata.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c92af09ebb60bb79ce768ddf"></a>
## hash

`function` · `datafusion_common::metadata::FieldMetadata::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 44], "end": [178, 48], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2281a0150e79498cb9d8dda3"></a>
## inner

`function` · `datafusion_common::metadata::FieldMetadata::inner` · datafusion-common 55.1.0

```rust
fn inner(&self) -> &BTreeMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get the inner metadata as a reference to a `BTreeMap`.

<a id="op-ab11594dee35790d920593d5"></a>
## into_inner

`function` · `datafusion_common::metadata::FieldMetadata::into_inner` · datafusion-common 55.1.0

```rust
fn into_inner(self) -> Arc<BTreeMap<String, String>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the inner metadata

<a id="op-5861be5d7be4cd46a9a4c8fe"></a>
## is_empty

`function` · `datafusion_common::metadata::FieldMetadata::is_empty` · datafusion-common 55.1.0

```rust
fn is_empty(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns true if the metadata is empty.

<a id="op-d192e9cc9bed02478edf183e"></a>
## len

`function` · `datafusion_common::metadata::FieldMetadata::len` · datafusion-common 55.1.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the number of key-value pairs in the metadata.

<a id="op-f5f1c356c0d04a456cbde9fa"></a>
## merge_options

`function` · `datafusion_common::metadata::FieldMetadata::merge_options` · datafusion-common 55.1.0

```rust
fn merge_options(m: Option<&FieldMetadata>, n: Option<&FieldMetadata>) -> Option<FieldMetadata>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Merges two optional `FieldMetadata` instances, overwriting any existing
keys in `m` with keys from `n` if present.

This function is commonly used in alias operations, particularly for literals
with metadata. When creating an alias expression, the metadata from the original
expression (such as a literal) is combined with any metadata specified on the alias.

# Arguments

* `m` - The first metadata (typically from the original expression like a literal)
* `n` - The second metadata (typically from the alias definition)

# Merge Strategy

- If both metadata instances exist, they are merged with `n` taking precedence
- Keys from `n` will overwrite keys from `m` if they have the same name
- If only one metadata instance exists, it is returned unchanged
- If neither exists, `None` is returned

# Example usage
```rust
use datafusion_common::metadata::FieldMetadata;
use std::collections::BTreeMap;

// Create metadata for a literal expression
let literal_metadata = Some(FieldMetadata::from(BTreeMap::from([
    ("source".to_string(), "constant".to_string()),
    ("type".to_string(), "int".to_string()),
])));

// Create metadata for an alias
let alias_metadata = Some(FieldMetadata::from(BTreeMap::from([
    ("description".to_string(), "answer".to_string()),
    ("source".to_string(), "user".to_string()), // This will override literal's "source"
])));

// Merge the metadata
let merged = FieldMetadata::merge_options(
    literal_metadata.as_ref(),
    alias_metadata.as_ref(),
);

// Result contains: {"source": "user", "type": "int", "description": "answer"}
assert!(merged.is_some());
```

<a id="op-8f6d33f635f83a9576e812a4"></a>
## new

`function` · `datafusion_common::metadata::FieldMetadata::new` · datafusion-common 55.1.0

```rust
fn new(inner: BTreeMap<String, String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new metadata instance from a map of string keys to string values.

<a id="op-48ccc5d23f1336cdb311a7fc"></a>
## new_empty

`function` · `datafusion_common::metadata::FieldMetadata::new_empty` · datafusion-common 55.1.0

```rust
fn new_empty() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new empty metadata instance.

<a id="op-b9cfaa3ed83ce4d2bbd3aa93"></a>
## new_from_field

`function` · `datafusion_common::metadata::FieldMetadata::new_from_field` · datafusion-common 55.1.0

```rust
fn new_from_field(field: &Field) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Create a new metadata instance from a `Field`'s metadata.

<a id="op-4872b18e2a407c8cdd84d174"></a>
## partial_cmp

`function` · `datafusion_common::metadata::FieldMetadata::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &FieldMetadata) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [178, 32], "end": [178, 42], "filename": "src/metadata.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/metadata.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a794f96dd1647a8da5e0375"></a>
## to_hashmap

`function` · `datafusion_common::metadata::FieldMetadata::to_hashmap` · datafusion-common 55.1.0

```rust
fn to_hashmap(&self) -> std::collections::HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::metadata::FieldMetadata", "path": "FieldMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [337, 2], "filename": "src/metadata.rs"}, "trait": null, "trait_path": null}`

Source: `src/metadata.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convert this `FieldMetadata` into a `HashMap<String, String>`
