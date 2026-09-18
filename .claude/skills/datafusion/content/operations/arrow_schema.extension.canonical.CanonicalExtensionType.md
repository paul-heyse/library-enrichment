# `arrow_schema::extension::canonical::CanonicalExtensionType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.extension.canonical.CanonicalExtensionType.json).

<a id="op-51f58c1d3b7910a19962940e"></a>
## CanonicalExtensionType

`enum` · `arrow_schema::extension::canonical::CanonicalExtensionType` · arrow-schema 59.3.0

```rust
enum CanonicalExtensionType
```

Source: `src/extension/canonical/mod.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Canonical extension types.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#format-canonical-extensions>

<a id="op-ce9e5002dc3a93533894e175"></a>
## Bool8

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::Bool8` · arrow-schema 59.3.0

```rust
Bool8
```

Source: `src/extension/canonical/mod.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `Bool8`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#bit-boolean>

<a id="op-2fb70f3f5932412924286553"></a>
## Error

`assoc_type` · `arrow_schema::extension::canonical::CanonicalExtensionType::Error` · arrow-schema 59.3.0

```rust
Error
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [124, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/extension/canonical/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c2d02618007cb1fd608c8e5"></a>
## FixedShapeTensor

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::FixedShapeTensor` · arrow-schema 59.3.0

```rust
FixedShapeTensor
```

Source: `src/extension/canonical/mod.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `FixedShapeTensor`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#fixed-shape-tensor>

<a id="op-5ae63d2a5aa282d25a7423d1"></a>
## Json

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::Json` · arrow-schema 59.3.0

```rust
Json
```

Source: `src/extension/canonical/mod.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for 'JSON'.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#json>

<a id="op-b3201a406abe598e46df1618"></a>
## Opaque

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::Opaque` · arrow-schema 59.3.0

```rust
Opaque
```

Source: `src/extension/canonical/mod.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `Opaque`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#opaque>

<a id="op-6a58bffcf5669e6be8608c94"></a>
## TimestampWithOffset

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::TimestampWithOffset` · arrow-schema 59.3.0

```rust
TimestampWithOffset
```

Source: `src/extension/canonical/mod.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `TimestampWithOffset`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#timestamp-with-offset>

<a id="op-f24ba93ad0ef71fccf4cc50f"></a>
## Uuid

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::Uuid` · arrow-schema 59.3.0

```rust
Uuid
```

Source: `src/extension/canonical/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `UUID`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#uuid>

<a id="op-43046f385a913b945e90f14c"></a>
## VariableShapeTensor

`variant` · `arrow_schema::extension::canonical::CanonicalExtensionType::VariableShapeTensor` · arrow-schema 59.3.0

```rust
VariableShapeTensor
```

Source: `src/extension/canonical/mod.rs:61`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

The extension type for `VariableShapeTensor`.

<https://arrow.apache.org/docs/format/CanonicalExtensions.html#variable-shape-tensor>

<a id="op-597be5432ed8c1e46e365fa1"></a>
## clone

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> CanonicalExtensionType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 22], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/extension/canonical/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac026e264afd4055a4c76aa9"></a>
## eq

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &CanonicalExtensionType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 24], "end": [51, 33], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/extension/canonical/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed22a420790cceb470a142c9"></a>
## fmt

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/extension/canonical/mod.rs:51`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f1d71482a55835849c7c40"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: VariableShapeTensor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [132, 1], "end": [136, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::variable_shape_tensor::VariableShapeTensor", "path": "VariableShapeTensor"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:133`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10acaf06db0a6cccf7590ee7"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: Json) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [138, 1], "end": [142, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::json::Json", "path": "Json"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:139`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12056bbda3c1d1a98dfe58ed"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: FixedShapeTensor) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [130, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::fixed_shape_tensor::FixedShapeTensor", "path": "FixedShapeTensor"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29348a63458d20308fdc0a7d"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: Uuid) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [148, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::uuid::Uuid", "path": "Uuid"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:145`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-410490a952496f75be1b4b38"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: Bool8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [160, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::bool8::Bool8", "path": "Bool8"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6ad1a3fb5c5bd28e144b708"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: TimestampWithOffset) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [162, 1], "end": [166, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::timestamp_with_offset::TimestampWithOffset", "path": "TimestampWithOffset"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:163`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c83fc9e0e5c39060e9ea8e31"></a>
## from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::from` · arrow-schema 59.3.0

```rust
fn from(value: Opaque) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [154, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::opaque::Opaque", "path": "Opaque"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/extension/canonical/mod.rs:151`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f981796b314baf7b6fbdbf00"></a>
## try_from

`function` · `arrow_schema::extension::canonical::CanonicalExtensionType::try_from` · arrow-schema 59.3.0

```rust
fn try_from(value: &Field) -> Result<Self, Self::Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::extension::canonical::CanonicalExtensionType", "path": "CanonicalExtensionType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [124, 2], "filename": "src/extension/canonical/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `src/extension/canonical/mod.rs:92`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
