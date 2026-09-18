# `datafusion_common::error::SchemaError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.error.SchemaError.json).

<a id="op-55bc8f12497f26e3960ad2b0"></a>
## SchemaError

`enum` · `datafusion_common::error::SchemaError` · datafusion-common 55.1.0

```rust
enum SchemaError
```

Source: `src/error.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Schema-related errors

<a id="op-fe5f579b29aff552a3801538"></a>
## AmbiguousReference

`variant` · `datafusion_common::error::SchemaError::AmbiguousReference` · datafusion-common 55.1.0

```rust
AmbiguousReference
```

Source: `src/error.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Schema contains a (possibly) qualified and unqualified field with same unqualified name

<a id="op-829fc7d06831e843b93dca4f"></a>
## DuplicateQualifiedField

`variant` · `datafusion_common::error::SchemaError::DuplicateQualifiedField` · datafusion-common 55.1.0

```rust
DuplicateQualifiedField
```

Source: `src/error.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Schema contains duplicate qualified field name

<a id="op-1a6198cb6271181047a310a3"></a>
## DuplicateUnqualifiedField

`variant` · `datafusion_common::error::SchemaError::DuplicateUnqualifiedField` · datafusion-common 55.1.0

```rust
DuplicateUnqualifiedField
```

Source: `src/error.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Schema contains duplicate unqualified field name

<a id="op-c1b3db61dd512f5f414273f3"></a>
## FieldNotFound

`variant` · `datafusion_common::error::SchemaError::FieldNotFound` · datafusion-common 55.1.0

```rust
FieldNotFound
```

Source: `src/error.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No field with this name

<a id="op-4820ed50a864695f218a4ff2"></a>
## fmt

`function` · `datafusion_common::error::SchemaError::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::SchemaError", "path": "SchemaError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 10], "end": [183, 15], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/error.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-528d110c0332264a43b79cbb"></a>
## fmt

`function` · `datafusion_common::error::SchemaError::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::error::SchemaError", "path": "SchemaError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [346, 2], "filename": "src/error.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/error.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
