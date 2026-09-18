# `datafusion_expr_common::signature::ArrayFunctionSignature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.ArrayFunctionSignature.json).

<a id="op-de5878d4c45db0c90f1ffaf5"></a>
## ArrayFunctionSignature

`enum` · `datafusion_expr_common::signature::ArrayFunctionSignature` · datafusion-expr-common 55.1.0

```rust
enum ArrayFunctionSignature
```

Source: `src/signature.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b5bd256b049dec52ada012f"></a>
## Array

`variant` · `datafusion_expr_common::signature::ArrayFunctionSignature::Array` · datafusion-expr-common 55.1.0

```rust
Array
```

Source: `src/signature.rs:529`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A function takes at least one List/LargeList/FixedSizeList argument.

<a id="op-f390cea468f031ff00eac226"></a>
## MapArray

`variant` · `datafusion_expr_common::signature::ArrayFunctionSignature::MapArray` · datafusion-expr-common 55.1.0

```rust
MapArray
```

Source: `src/signature.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized Signature for MapArray
The function takes a single argument that must be a MapArray

<a id="op-7758b772dfab672cf4f1d4fc"></a>
## RecursiveArray

`variant` · `datafusion_expr_common::signature::ArrayFunctionSignature::RecursiveArray` · datafusion-expr-common 55.1.0

```rust
RecursiveArray
```

Source: `src/signature.rs:537`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A function takes a single argument that must be a List/LargeList/FixedSizeList
which gets coerced to List, with element type recursively coerced to List too if it is list-like.

<a id="op-0f5e28097bb101da923fa5c1"></a>
## clone

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ArrayFunctionSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [526, 17], "end": [526, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04bf080004ccf59f0acf7e15"></a>
## eq

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &ArrayFunctionSignature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [526, 24], "end": [526, 33], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34e62ad5a51f25b6c216f33c"></a>
## fmt

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [526, 10], "end": [526, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fea77e56cb09dc10873f0332"></a>
## fmt

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [543, 1], "end": [563, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:544`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1473cffb28954e6e52249310"></a>
## hash

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [526, 51], "end": [526, 55], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a0bc4340b115e139fc57cd5"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::ArrayFunctionSignature::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &ArrayFunctionSignature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::ArrayFunctionSignature", "path": "ArrayFunctionSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [526, 39], "end": [526, 49], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:526`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
