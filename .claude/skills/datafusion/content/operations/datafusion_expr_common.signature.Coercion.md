# `datafusion_expr_common::signature::Coercion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.Coercion.json).

<a id="op-4db7bfb3816e2d62df923c32"></a>
## Coercion

`enum` · `datafusion_expr_common::signature::Coercion` · datafusion-expr-common 55.1.0

```rust
enum Coercion
```

Source: `src/signature.rs:1047`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Represents type coercion rules for function arguments, specifying both the desired type
and optional implicit coercion rules for source types.

# Examples

```
use datafusion_common::types::{logical_binary, logical_string, NativeType};
use datafusion_expr_common::signature::{Coercion, TypeSignatureClass};

// Exact coercion that only accepts timestamp types
let exact = Coercion::new_exact(TypeSignatureClass::Timestamp);

// Implicit coercion that accepts string types but can coerce from binary types
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Native(logical_string()),
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::String,
);
```

There are two variants:

* `Exact` - Only accepts arguments that exactly match the desired type
* `Implicit` - Accepts the desired type and can coerce from specified source types

<a id="op-6f6b20da258fcbec9c59ba99"></a>
## Exact

`variant` · `datafusion_expr_common::signature::Coercion::Exact` · datafusion-expr-common 55.1.0

```rust
Exact
```

Source: `src/signature.rs:1049`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion that only accepts arguments exactly matching the desired type.

<a id="op-175eb276c0bc2cc869f3c2f2"></a>
## Implicit

`variant` · `datafusion_expr_common::signature::Coercion::Implicit` · datafusion-expr-common 55.1.0

```rust
Implicit
```

Source: `src/signature.rs:1057`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Coercion that accepts the desired type and can implicitly coerce from other types.

<a id="op-2995e7edd331013a522ec2ac"></a>
## allowed_source_types

`function` · `datafusion_expr_common::signature::Coercion::allowed_source_types` · datafusion-expr-common 55.1.0

```rust
fn allowed_source_types(&self) -> &[TypeSignatureClass]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1151`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a75a3f7589ab4bf5858bdc1"></a>
## clone

`function` · `datafusion_expr_common::signature::Coercion::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Coercion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1046, 17], "end": [1046, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:1046`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-000c58ea97c5a98becb2b311"></a>
## default_casted_type

`function` · `datafusion_expr_common::signature::Coercion::default_casted_type` · datafusion-expr-common 55.1.0

```rust
fn default_casted_type(&self) -> Option<&NativeType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1864219b3d40ee3f3c8ec6c"></a>
## desired_type

`function` · `datafusion_expr_common::signature::Coercion::desired_type` · datafusion-expr-common 55.1.0

```rust
fn desired_type(&self) -> &TypeSignatureClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1169`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81d02742d67a36450b342d82"></a>
## encoding_preservation

`function` · `datafusion_expr_common::signature::Coercion::encoding_preservation` · datafusion-expr-common 55.1.0

```rust
fn encoding_preservation(&self) -> EncodingPreservation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1138`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e16705d1db4a6d9d03689aa"></a>
## eq

`function` · `datafusion_expr_common::signature::Coercion::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1192, 1], "end": [1198, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:1193`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e4b2514c29d8fa276ea634b"></a>
## fmt

`function` · `datafusion_expr_common::signature::Coercion::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1186, 1], "end": [1190, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:1187`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63b2200583b79f7ac26cc967"></a>
## fmt

`function` · `datafusion_expr_common::signature::Coercion::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1046, 10], "end": [1046, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:1046`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00dcc727c29d36207ad7c321"></a>
## hash

`function` · `datafusion_expr_common::signature::Coercion::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<H: std::hash::Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 1], "end": [1206, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:1201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9df1d92a7c65a57b7849102e"></a>
## implicit_coercion

`function` · `datafusion_expr_common::signature::Coercion::implicit_coercion` · datafusion-expr-common 55.1.0

```rust
fn implicit_coercion(&self) -> Option<&ImplicitCoercion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61ae047472ab39dd64487cd8"></a>
## new_exact

`function` · `datafusion_expr_common::signature::Coercion::new_exact` · datafusion-expr-common 55.1.0

```rust
fn new_exact(desired_type: TypeSignatureClass) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1095`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7f7300afc3adbf051ef5683"></a>
## new_implicit

`function` · `datafusion_expr_common::signature::Coercion::new_implicit` · datafusion-expr-common 55.1.0

```rust
fn new_implicit(desired_type: TypeSignatureClass, allowed_source_types: Vec<TypeSignatureClass>, default_casted_type: NativeType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Create a new coercion with implicit coercion rules.

`allowed_source_types` defines the possible types that can be coerced to `desired_type`.
`default_casted_type` is the default type to be used for coercion if we cast from other types via `allowed_source_types`.

<a id="op-94c52d0153f164307c4faea4"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::Coercion::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &Coercion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1046, 28], "end": [1046, 38], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:1046`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7079b666f1a03994f58ffe75"></a>
## with_encoding_preservation

`function` · `datafusion_expr_common::signature::Coercion::with_encoding_preservation` · datafusion-expr-common 55.1.0

```rust
fn with_encoding_preservation(self, encoding_preservation: EncodingPreservation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Coercion", "path": "Coercion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1094, 1], "end": [1184, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
