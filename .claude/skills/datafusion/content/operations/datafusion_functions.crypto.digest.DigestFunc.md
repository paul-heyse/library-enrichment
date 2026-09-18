# `datafusion_functions::crypto::digest::DigestFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.crypto.digest.DigestFunc.json).

<a id="op-3a36ee8857d4cefa3c60ef28"></a>
## DigestFunc

`struct` · `datafusion_functions::crypto::digest::DigestFunc` · datafusion-functions 55.1.0

```rust
struct DigestFunc
```

Source: `src/crypto/digest.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01d4a7c4d1f963fed9b82497"></a>
## default

`function` · `datafusion_functions::crypto::digest::DigestFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [68, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/crypto/digest.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6ad4e37913d4f81338ecb94"></a>
## documentation

`function` · `datafusion_functions::crypto::digest::DigestFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/digest.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9547d561366d07a2a03d8d4b"></a>
## eq

`function` · `datafusion_functions::crypto::digest::DigestFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &DigestFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 17], "end": [59, 26], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/crypto/digest.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3275169b21485acb40c49cb5"></a>
## fmt

`function` · `datafusion_functions::crypto::digest::DigestFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/crypto/digest.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-520f7b4c1c9c6744815e28fe"></a>
## hash

`function` · `datafusion_functions::crypto::digest::DigestFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 32], "end": [59, 36], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/crypto/digest.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b2543ab4044ba16f8e2bbca"></a>
## invoke_with_args

`function` · `datafusion_functions::crypto::digest::DigestFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/digest.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-477d58df4b13833ff3120efa"></a>
## name

`function` · `datafusion_functions::crypto::digest::DigestFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/digest.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1cacf2a1230e00fe59be53f"></a>
## new

`function` · `datafusion_functions::crypto::digest::DigestFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [88, 2], "filename": "src/crypto/digest.rs"}, "trait": null, "trait_path": null}`

Source: `src/crypto/digest.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de29b615649eb225e2c178cb"></a>
## return_type

`function` · `datafusion_functions::crypto::digest::DigestFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/digest.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89819066f54575154c661105"></a>
## signature

`function` · `datafusion_functions::crypto::digest::DigestFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::digest::DigestFunc", "path": "DigestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 1], "end": [111, 2], "filename": "src/crypto/digest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/digest.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
