# `datafusion_functions::string::octet_length::OctetLengthFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.octet_length.OctetLengthFunc.json).

<a id="op-8766abce617599e26566ef00"></a>
## OctetLengthFunc

`struct` · `datafusion_functions::string::octet_length::OctetLengthFunc` · datafusion-functions 55.1.0

```rust
struct OctetLengthFunc
```

Source: `src/string/octet_length.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48df405b977ace2be9ab0e81"></a>
## default

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/octet_length.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04dd381d91d4297a5fc01108"></a>
## documentation

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [99, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/octet_length.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5cdbd677c3d5ce247ccbeaa"></a>
## eq

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &OctetLengthFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 26], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/octet_length.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3310d9528a151f3481bb6254"></a>
## fmt

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/octet_length.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f062463fe85ea62ca9e7ff38"></a>
## hash

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 32], "end": [47, 36], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/octet_length.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbc388acd3ef1ea01bfe2d7d"></a>
## invoke_with_args

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [99, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/octet_length.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b6e2a556a91d43359c2574d"></a>
## name

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [99, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/octet_length.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64adf9eae8c94cc6c4c9d2a7"></a>
## new

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [70, 2], "filename": "src/string/octet_length.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/octet_length.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f09e30808367525b734fcf88"></a>
## return_type

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [99, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/octet_length.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05db371b4d425666404fff03"></a>
## signature

`function` · `datafusion_functions::string::octet_length::OctetLengthFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::octet_length::OctetLengthFunc", "path": "OctetLengthFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [99, 2], "filename": "src/string/octet_length.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/octet_length.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
