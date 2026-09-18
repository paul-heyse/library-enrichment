# `arrow_schema::extension::canonical::json::Json`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.json.Json.json).

<a id="op-3263368fd37d8190e1022007"></a>
## Json

`struct` · `arrow_schema::extension::canonical::json::Json` · arrow-schema 59.3.0

```rust
struct Json
```

Source: `src/extension/canonical/json.rs:45`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `JSON`.

Extension name: `arrow.json`.

The storage type of this extension is `String` or `LargeString` or
`StringView`. Only UTF-8 encoded JSON as specified in [rfc8259](https://datatracker.ietf.org/doc/html/rfc8259)
is supported.

This type does not have any parameters.

Metadata is either an empty string or a JSON string with an empty
object. In the future, additional fields may be added, but they are not
required to interpret the array.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#json>

<a id="op-3b2c83c3c22896d8569481be"></a>
## Metadata

`assoc_type` · `arrow_schema::extension::canonical::json::Json::Metadata` · arrow-schema 59.3.0

```rust
Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-043f16402c0e24ef0e8838f4"></a>
## NAME

`assoc_const` · `arrow_schema::extension::canonical::json::Json::NAME` · arrow-schema 59.3.0

```rust
NAME
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:125`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a153cf95e693059cfd5c372c"></a>
## clone

`function` · `arrow_schema::extension::canonical::json::Json::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> Json
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 22], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/json.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6421ba20449b8b16138edb71"></a>
## default

`function` · `arrow_schema::extension::canonical::json::Json::default` · arrow-schema 59.3.0

```rust
fn default() -> Json
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 24], "end": [44, 31], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/extension/canonical/json.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b480953c8acaa3668acab782"></a>
## deserialize_metadata

`function` · `arrow_schema::extension::canonical::json::Json::deserialize_metadata` · arrow-schema 59.3.0

```rust
fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:144`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6d1ea687f6d050cb6216c59"></a>
## eq

`function` · `arrow_schema::extension::canonical::json::Json::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &Json) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 33], "end": [44, 42], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/json.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5e29af8bffae8606748225c"></a>
## fmt

`function` · `arrow_schema::extension::canonical::json::Json::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/json.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d864b372dc63feb29353c4d"></a>
## metadata

`function` · `arrow_schema::extension::canonical::json::Json::metadata` · arrow-schema 59.3.0

```rust
fn metadata(&self) -> &Self::Metadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:129`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31c7bab0793125237a5613d7"></a>
## serialize_metadata

`function` · `arrow_schema::extension::canonical::json::Json::serialize_metadata` · arrow-schema 59.3.0

```rust
fn serialize_metadata(&self) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d48d96819cb5f829abc8426"></a>
## supports_data_type

`function` · `arrow_schema::extension::canonical::json::Json::supports_data_type` · arrow-schema 59.3.0

```rust
fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:162`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af8d019b64365deee0657e98"></a>
## try_new

`function` · `arrow_schema::extension::canonical::json::Json::try_new` · arrow-schema 59.3.0

```rust
fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:171`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b0dec03d4c29c4aed8a6ce"></a>
## validate

`function` · `arrow_schema::extension::canonical::json::Json::validate` · arrow-schema 59.3.0

```rust
fn validate(data_type: &DataType, _metadata: Self::Metadata) -> Result<(), ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [180, 2], "filename": "src/extension/canonical/json.rs"}, "trait": {"args": null, "id": "arrow_schema::extension::ExtensionType", "path": "ExtensionType"}, "trait_path": "arrow_schema::extension::ExtensionType"}`

Source: `src/extension/canonical/json.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
