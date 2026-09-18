# `arrow_schema::schema::SchemaBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.schema.SchemaBuilder.json).

<a id="op-431f34de9450be113f2a7fb1"></a>
## SchemaBuilder

`struct` · `arrow_schema::schema::SchemaBuilder` · arrow-schema 59.3.0

```rust
struct SchemaBuilder
```

Source: `src/schema.rs:29`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A builder to facilitate building a [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) from iteratively from [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)

<a id="op-a3b6cbddcfa3f09d39e4b1fc"></a>
## default

`function` · `arrow_schema::schema::SchemaBuilder::default` · arrow-schema 59.3.0

```rust
fn default() -> SchemaBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 17], "end": [28, 24], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/schema.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0593a9ce67b1dfcfcfb1d515"></a>
## extend

`function` · `arrow_schema::schema::SchemaBuilder::extend` · arrow-schema 59.3.0

```rust
fn extend<T: IntoIterator<Item = FieldRef>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [166, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/schema.rs:159`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9f8a3e8da15ab4f4b472161"></a>
## extend

`function` · `arrow_schema::schema::SchemaBuilder::extend` · arrow-schema 59.3.0

```rust
fn extend<T: IntoIterator<Item = Field>>(&mut self, iter: T)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [176, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}], "constraints": []}}, "id": "core::iter::traits::collect::Extend", "path": "Extend"}, "trait_path": "core::iter::traits::collect::Extend"}`

Source: `src/schema.rs:169`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10429e47e3e3d16c013b08bb"></a>
## field

`function` · `arrow_schema::schema::SchemaBuilder::field` · arrow-schema 59.3.0

```rust
fn field(&mut self, idx: usize) -> &FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference to the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) at index `idx`

# Panics

Panics if index out of bounds

<a id="op-cfbfb82f7f4455116dbb9602"></a>
## field_mut

`function` · `arrow_schema::schema::SchemaBuilder::field_mut` · arrow-schema 59.3.0

```rust
fn field_mut(&mut self, idx: usize) -> &mut FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a mutable reference to the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) at index `idx`

# Panics

Panics if index out of bounds

<a id="op-337cb70acc5e1154e26fa255"></a>
## finish

`function` · `arrow_schema::schema::SchemaBuilder::finish` · arrow-schema 59.3.0

```rust
fn finish(self) -> Schema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Consume this [`SchemaBuilder`](../operations/arrow_schema.schema.SchemaBuilder.md#op-431f34de9450be113f2a7fb1) yielding the final [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050)

<a id="op-0ceab9301d729f7559cb6196"></a>
## fmt

`function` · `arrow_schema::schema::SchemaBuilder::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [28, 10], "end": [28, 15], "filename": "src/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema.rs:28`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-072e40ce4405731154160389"></a>
## from

`function` · `arrow_schema::schema::SchemaBuilder::from` · arrow-schema 59.3.0

```rust
fn from(value: Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [149, 1], "end": [156, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:150`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bf1282fb495fe6d80d8cb4a"></a>
## from

`function` · `arrow_schema::schema::SchemaBuilder::from` · arrow-schema 59.3.0

```rust
fn from(value: &Schema) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [147, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8556496a0c25790e9af3f630"></a>
## from

`function` · `arrow_schema::schema::SchemaBuilder::from` · arrow-schema 59.3.0

```rust
fn from(value: Fields) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [141, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:135`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9af864332b4e6f7340c8252a"></a>
## from

`function` · `arrow_schema::schema::SchemaBuilder::from` · arrow-schema 59.3.0

```rust
fn from(value: &Fields) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [132, 2], "filename": "src/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::fields::Fields", "path": "Fields"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema.rs:126`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e448741cd4bc41d40e442e"></a>
## metadata

`function` · `arrow_schema::schema::SchemaBuilder::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&mut self) -> &HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns an immutable reference to the Map of custom metadata key-value pairs.

<a id="op-9ddaa80e78a9514accdcd87f"></a>
## metadata_mut

`function` · `arrow_schema::schema::SchemaBuilder::metadata_mut` · arrow-schema 59.3.0

```rust
fn metadata_mut(&mut self) -> &mut HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Returns a mutable reference to the Map of custom metadata key-value pairs.

<a id="op-8d4e3aace3dacae9c05b001e"></a>
## new

`function` · `arrow_schema::schema::SchemaBuilder::new` · arrow-schema 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new empty [`SchemaBuilder`](../operations/arrow_schema.schema.SchemaBuilder.md#op-431f34de9450be113f2a7fb1)

<a id="op-ddbca18bb6dd4b0ef80c93cc"></a>
## push

`function` · `arrow_schema::schema::SchemaBuilder::push` · arrow-schema 59.3.0

```rust
fn push(&mut self, field: impl Into<FieldRef>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:49`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Appends a [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) to this [`SchemaBuilder`](../operations/arrow_schema.schema.SchemaBuilder.md#op-431f34de9450be113f2a7fb1) without checking for collision

<a id="op-798eb327268d7178366b4f6f"></a>
## remove

`function` · `arrow_schema::schema::SchemaBuilder::remove` · arrow-schema 59.3.0

```rust
fn remove(&mut self, idx: usize) -> FieldRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Removes and returns the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) as index `idx`

# Panics

Panics if index out of bounds

<a id="op-42d08f88e114977a4efc8ff9"></a>
## reverse

`function` · `arrow_schema::schema::SchemaBuilder::reverse` · arrow-schema 59.3.0

```rust
fn reverse(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:91`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Reverse the fileds

<a id="op-05ab69ff24acc80fb989877f"></a>
## try_merge

`function` · `arrow_schema::schema::SchemaBuilder::try_merge` · arrow-schema 59.3.0

```rust
fn try_merge(&mut self, field: &FieldRef) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:98`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Appends a [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542) to this [`SchemaBuilder`](../operations/arrow_schema.schema.SchemaBuilder.md#op-431f34de9450be113f2a7fb1) checking for collision

If an existing field exists with the same name, calls [`Field::try_merge`](../operations/arrow_schema.field.Field.md#op-2e16bc6445276ebdc07db2f7)

<a id="op-562b5477bc74c6103bc7a7d4"></a>
## with_capacity

`function` · `arrow_schema::schema::SchemaBuilder::with_capacity` · arrow-schema 59.3.0

```rust
fn with_capacity(capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::schema::SchemaBuilder", "path": "SchemaBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [123, 2], "filename": "src/schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema.rs:41`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Creates a new empty [`SchemaBuilder`](../operations/arrow_schema.schema.SchemaBuilder.md#op-431f34de9450be113f2a7fb1) with space for `capacity` fields
