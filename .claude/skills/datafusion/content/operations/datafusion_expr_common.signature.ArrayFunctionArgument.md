# `datafusion_expr_common::signature::ArrayFunctionArgument`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.ArrayFunctionArgument.json).

<a id="op-abc808258f8a9e359e7b4384"></a>
## ArrayFunctionArgument

`enum` · `datafusion_expr_common::signature::ArrayFunctionArgument` · datafusion-expr-common 55.1.0

```rust
enum ArrayFunctionArgument
```

Source: `src/signature.rs:566`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21a343470ac749f626161328"></a>
## Array

`variant` · `datafusion_expr_common::signature::ArrayFunctionArgument::Array` · datafusion-expr-common 55.1.0

```rust
Array
```

Source: `src/signature.rs:574`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An argument of type List/LargeList/FixedSizeList. All Array arguments must be coercible
to the same type.

<a id="op-55b93b51e102bbe78f9b851d"></a>
## Element

`variant` · `datafusion_expr_common::signature::ArrayFunctionArgument::Element` · datafusion-expr-common 55.1.0

```rust
Element
```

Source: `src/signature.rs:569`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A non-list or list argument. The list dimensions should be one less than the Array's list
dimensions.

<a id="op-bcd14faa5a4f09152fc106a6"></a>
## Index

`variant` · `datafusion_expr_common::signature::ArrayFunctionArgument::Index` · datafusion-expr-common 55.1.0

```rust
Index
```

Source: `src/signature.rs:571`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An Int64 index argument.

<a id="op-c4e6eb11d40fecb09e73c2b0"></a>
## String

`variant` · `datafusion_expr_common::signature::ArrayFunctionArgument::String` · datafusion-expr-common 55.1.0

```rust
String
```

Source: `src/signature.rs:576`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-643bed9a041b059610cd8d63"></a>
## clone

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ArrayFunctionArgument
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 17], "end": [565, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c52a5ab1e87ada66d80f163"></a>
## eq

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &ArrayFunctionArgument) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 24], "end": [565, 33], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2613e04f3ccb0aaea4a8076b"></a>
## fmt

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 10], "end": [565, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c65176be4f48ddbf6c4a0f0"></a>
## fmt

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [579, 1], "end": [596, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:580`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3486b0979a6abd6d3863082"></a>
## hash

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 51], "end": [565, 55], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e70c8e0e89c651e479b2aabc"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::ArrayFunctionArgument::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &ArrayFunctionArgument) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionArgument", "path": "ArrayFunctionArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [565, 39], "end": [565, 49], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
