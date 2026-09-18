# `datafusion_expr_common::signature::EncodingPreservation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.EncodingPreservation.json).

<a id="op-26c274da911a869101d3fb1c"></a>
## EncodingPreservation

`struct` · `datafusion_expr_common::signature::EncodingPreservation` · datafusion-expr-common 55.1.0

```rust
struct EncodingPreservation
```

Source: `src/signature.rs:1070`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Controls whether a [`Coercion`](../operations/datafusion_expr_common.signature.Coercion.md#op-4db7bfb3816e2d62df923c32) preserves an argument's physical encoding
(e.g. dictionary) instead of materializing it to the coerced value type.

<a id="op-59a578e3dfd7a40856473d0f"></a>
## clone

`function` · `datafusion_expr_common::signature::EncodingPreservation::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> EncodingPreservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 17], "end": [1069, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e190eda2c3745f7773f95ff"></a>
## default

`function` · `datafusion_expr_common::signature::EncodingPreservation::default` · datafusion-expr-common 55.1.0

```rust
fn default() -> EncodingPreservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 30], "end": [1069, 37], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ece44fe118231cde7ca5d02"></a>
## dictionary

`function` · `datafusion_expr_common::signature::EncodingPreservation::dictionary` · datafusion-expr-common 55.1.0

```rust
const fn dictionary() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1092, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1076`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Preserve dictionary encoding and coerce only the dictionary values.

<a id="op-59718e120d94121ad6153d22"></a>
## eq

`function` · `datafusion_expr_common::signature::EncodingPreservation::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &EncodingPreservation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 39], "end": [1069, 48], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5215c1365a9ccf6054f8b2d3"></a>
## fmt

`function` · `datafusion_expr_common::signature::EncodingPreservation::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 10], "end": [1069, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f4e324a8543b33039f32b93"></a>
## hash

`function` · `datafusion_expr_common::signature::EncodingPreservation::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 66], "end": [1069, 70], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24cd06ffc7f85639c3930bc5"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::EncodingPreservation::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &EncodingPreservation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1069, 54], "end": [1069, 64], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:1069`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e10a6dd2f5936b6fd38b9c2"></a>
## preserve_dictionary

`function` · `datafusion_expr_common::signature::EncodingPreservation::preserve_dictionary` · datafusion-expr-common 55.1.0

```rust
const fn preserve_dictionary(self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1092, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1089`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns whether dictionary encoding should be preserved.

<a id="op-cd479dee4e6e2483bcaae79b"></a>
## with_dictionary

`function` · `datafusion_expr_common::signature::EncodingPreservation::with_dictionary` · datafusion-expr-common 55.1.0

```rust
const fn with_dictionary(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::EncodingPreservation", "path": "EncodingPreservation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1074, 1], "end": [1092, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1083`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Preserve dictionary encoding and coerce only the dictionary values.
