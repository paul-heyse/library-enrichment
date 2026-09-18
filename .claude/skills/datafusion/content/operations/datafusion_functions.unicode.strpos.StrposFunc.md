# `datafusion_functions::unicode::strpos::StrposFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.strpos.StrposFunc.json).

<a id="op-08575654a5fffcfe2509cd82"></a>
## StrposFunc

`struct` · `datafusion_functions::unicode::strpos::StrposFunc` · datafusion-functions 55.1.0

```rust
struct StrposFunc
```

Source: `src/unicode/strpos.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f6a382286f237d93d60d1dc"></a>
## aliases

`function` · `datafusion_functions::unicode::strpos::StrposFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a74692eebea17b4d62aebc2e"></a>
## default

`function` · `datafusion_functions::unicode::strpos::StrposFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [62, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unicode/strpos.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e9d752e886757c4dbe4f4c5"></a>
## documentation

`function` · `datafusion_functions::unicode::strpos::StrposFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-556c36119aae370dfd2a5d3c"></a>
## eq

`function` · `datafusion_functions::unicode::strpos::StrposFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &StrposFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 26], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unicode/strpos.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-128839002910bc373a9724d0"></a>
## fmt

`function` · `datafusion_functions::unicode::strpos::StrposFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unicode/strpos.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebbe6f0ec3481b2d32acd71f"></a>
## hash

`function` · `datafusion_functions::unicode::strpos::StrposFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 32], "end": [52, 36], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unicode/strpos.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7beae35aa601054721081937"></a>
## invoke_with_args

`function` · `datafusion_functions::unicode::strpos::StrposFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9566f532a875d6a7d498cd8"></a>
## name

`function` · `datafusion_functions::unicode::strpos::StrposFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82f8740db679386e331391a1"></a>
## new

`function` · `datafusion_functions::unicode::strpos::StrposFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [77, 2], "filename": "src/unicode/strpos.rs"}, "trait": null, "trait_path": null}`

Source: `src/unicode/strpos.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ebf5d3397ef773198fb93ae"></a>
## return_field_from_args

`function` · `datafusion_functions::unicode::strpos::StrposFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-874dbff6c62477855902673c"></a>
## return_type

`function` · `datafusion_functions::unicode::strpos::StrposFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d89706f2149a1af1c33f9df2"></a>
## signature

`function` · `datafusion_functions::unicode::strpos::StrposFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::strpos::StrposFunc", "path": "StrposFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [127, 2], "filename": "src/unicode/strpos.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/strpos.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
