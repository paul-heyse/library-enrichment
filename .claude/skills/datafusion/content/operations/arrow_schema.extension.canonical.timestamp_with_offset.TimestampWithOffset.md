# `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.timestamp_with_offset.TimestampWithOffset.json).

<a id="op-9338a1b8a578c65b76007334"></a>
## TimestampWithOffset

`struct` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset` · arrow-schema 59.3.0

```rust
struct TimestampWithOffset
```

Source: `src/extension/canonical/timestamp_with_offset.rs:57`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `TimestampWithOffset`.

Extension name: `arrow.timestamp_with_offset`.

This type represents a timestamp column that stores potentially different timezone offsets per
value. The timestamp is stored in UTC alongside the original timezone offset in minutes. This
extension type is intended to be compatible with ANSI SQL's `TIMESTAMP WITH TIME ZONE`, which
is supported by multiple database engines.

The storage type of the extension is a `Struct` with 2 fields, in order: - `timestamp`: a
non-nullable `Timestamp(time_unit, "UTC")`, where `time_unit` is any Arrow `TimeUnit` (s, ms,
us or ns). - `offset_minutes`: a non-nullable signed 16-bit integer (`Int16`) representing the
offset in minutes from the UTC timezone. Negative offsets represent time zones west of UTC,
while positive offsets represent east. Offsets normally range from -779 (-12:59) to +780
(+13:00).

This type has no type parameters.

Metadata is either empty or an empty string.

It is also *permissible* for the `offset_minutes` field to be dictionary-encoded with a
preferred (*but not required*) index type of `int8`, or run-end-encoded with a preferred (*but
not required*) runs type of `int8`.

It's worth noting that the data source needs to resolve timezone strings such as `UTC` or
`Americas/Los_Angeles` into an offset in minutes in order to construct a `TimestampWithOffset`.
This makes `TimestampWithOffset` type "lossy" in the sense that any original "unresolved"
timezone string gets lost in this conversion. It's a tradeoff for optimizing the row
representation and simplifying the client code, which does not need to know how to convert from
timezone string to its corresponding offset in minutes.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#timestamp-with-offset>

<a id="op-0854307e0cb9dafc95ca8c6b"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:65`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9556ef56b701f2abb2890710"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ac4f77393bd0218bad5aea"></a>
## clone

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> TimestampWithOffset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 26], "end": [56, 31], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30ebcc9728d8b9ae3b6132c8"></a>
## default

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::default` · arrow-schema 59.3.0

```rust
fn default() -> TimestampWithOffset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 24], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e655c0074a2a2e74156d5f45"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:75`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21ab34b7ec66e8aed24b887e"></a>
## eq

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &TimestampWithOffset) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 39], "end": [56, 48], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7d43d008b09116f1d7118b4"></a>
## fmt

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78bbe566aa449f1d0c49d697"></a>
## metadata

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-133c6c8f19d9488efbd4e3b2"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21f44e22d1e5fb869aa90bff"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-485508d5fb178856fbf78699"></a>
## try_new

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, _metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d5276d554482d04fddcd263"></a>
## validate

`function` · `arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset::validate` · arrow-schema 59.3.0

```rust
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [146, 2], "filename": "src/extension/canonical/timestamp_with_offset.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/timestamp_with_offset.rs:143`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
