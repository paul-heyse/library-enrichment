# `datafusion_expr_common::signature::ImplicitCoercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.ImplicitCoercion.json).

<a id="op-889364f3f1558f8aa716e5a3"></a>
## ImplicitCoercion

`struct` · `datafusion_expr_common::signature::ImplicitCoercion` · datafusion-expr-common 55.1.0

```rust
struct ImplicitCoercion
```

Source: `src/signature.rs:1230`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Defines rules for implicit type coercion, specifying which source types can be
coerced and the default type to use when coercing.

This is used by functions to specify which types they can accept via implicit
coercion in addition to their primary desired type.

# Examples

```
use arrow::datatypes::TimeUnit;

use datafusion_expr_common::signature::{Coercion, ImplicitCoercion, TypeSignatureClass};
use datafusion_common::types::{NativeType, logical_binary};

// Allow coercing from binary types to timestamp, coerce to specific timestamp unit and timezone
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Timestamp,
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::Timestamp(TimeUnit::Second, None),
);
```

<a id="op-e22398226a98b327d53edb8f"></a>
## clone

`function` · `datafusion_expr_common::signature::ImplicitCoercion::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ImplicitCoercion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1229, 17], "end": [1229, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43940214e3d58b4b2b49ba69"></a>
## eq

`function` · `datafusion_expr_common::signature::ImplicitCoercion::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1253, 1], "end": [1258, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:1254`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ba70d7b089890fca6b76258"></a>
## fmt

`function` · `datafusion_expr_common::signature::ImplicitCoercion::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1240, 1], "end": [1251, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:1241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e883f729c956f66af64bcf4a"></a>
## fmt

`function` · `datafusion_expr_common::signature::ImplicitCoercion::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1229, 10], "end": [1229, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a8ae2ad8ecad8ea5f8947e"></a>
## hash

`function` · `datafusion_expr_common::signature::ImplicitCoercion::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1260, 1], "end": [1265, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:1261`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3802bbe21a77dc0a5a0e5d8"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::ImplicitCoercion::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &ImplicitCoercion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ImplicitCoercion", "path": "ImplicitCoercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1229, 28], "end": [1229, 38], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
