# `datafusion_functions::crypto::sha::SHAFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.crypto.sha.SHAFunc.json).

<a id="op-37967507139a1c37aee23bb5"></a>
## SHAFunc

`struct` · `datafusion_functions::crypto::sha::SHAFunc` · datafusion-functions 55.1.0

```rust
struct SHAFunc
```

Source: `src/crypto/sha.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3978fcb1e96180c9dc0f105"></a>
## documentation

`function` · `datafusion_functions::crypto::sha::SHAFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [170, 2], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/sha.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81f35167bd70ade3069cdb94"></a>
## eq

`function` · `datafusion_functions::crypto::sha::SHAFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &SHAFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 17], "end": [97, 26], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/crypto/sha.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-032f2ebc0e1819dbc4c27243"></a>
## fmt

`function` · `datafusion_functions::crypto::sha::SHAFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 10], "end": [97, 15], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/crypto/sha.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9ce8356d6de528f44c7c441"></a>
## hash

`function` · `datafusion_functions::crypto::sha::SHAFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 32], "end": [97, 36], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/crypto/sha.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fe12ec69c77b3d4007c937a"></a>
## invoke_with_args

`function` · `datafusion_functions::crypto::sha::SHAFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [170, 2], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/sha.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba4846759c62c3b384850613"></a>
## name

`function` · `datafusion_functions::crypto::sha::SHAFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [170, 2], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/sha.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e10ba09415ad1b5778c4282f"></a>
## return_type

`function` · `datafusion_functions::crypto::sha::SHAFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [170, 2], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/sha.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51731bbe5aed009dde4dc7a5"></a>
## sha224

`function` · `datafusion_functions::crypto::sha::SHAFunc::sha224` · datafusion-functions 55.1.0

```rust
fn sha224() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [138, 2], "filename": "src/crypto/sha.rs"}, "trait": null, "trait_path": null}`

Source: `src/crypto/sha.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7b72d71a261de35239da2e"></a>
## sha256

`function` · `datafusion_functions::crypto::sha::SHAFunc::sha256` · datafusion-functions 55.1.0

```rust
fn sha256() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [138, 2], "filename": "src/crypto/sha.rs"}, "trait": null, "trait_path": null}`

Source: `src/crypto/sha.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9daaaa2ae84517ec03f925cd"></a>
## sha384

`function` · `datafusion_functions::crypto::sha::SHAFunc::sha384` · datafusion-functions 55.1.0

```rust
fn sha384() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [138, 2], "filename": "src/crypto/sha.rs"}, "trait": null, "trait_path": null}`

Source: `src/crypto/sha.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-711529d66afaf65967bfb74f"></a>
## sha512

`function` · `datafusion_functions::crypto::sha::SHAFunc::sha512` · datafusion-functions 55.1.0

```rust
fn sha512() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [138, 2], "filename": "src/crypto/sha.rs"}, "trait": null, "trait_path": null}`

Source: `src/crypto/sha.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02b55613b8e87fcf5408e4e1"></a>
## signature

`function` · `datafusion_functions::crypto::sha::SHAFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::crypto::sha::SHAFunc", "path": "SHAFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [170, 2], "filename": "src/crypto/sha.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/crypto/sha.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
