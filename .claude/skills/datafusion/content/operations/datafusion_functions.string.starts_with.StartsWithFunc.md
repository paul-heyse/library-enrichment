# `datafusion_functions::string::starts_with::StartsWithFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.starts_with.StartsWithFunc.json).

<a id="op-de968773caaa3c1a5449d43c"></a>
## StartsWithFunc

`struct` · `datafusion_functions::string::starts_with::StartsWithFunc` · datafusion-functions 55.1.0

```rust
struct StartsWithFunc
```

Source: `src/string/starts_with.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e68ac56fe15f8f2c0e22069"></a>
## default

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [60, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/starts_with.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26fa3b7b4cf968b42eee5dc4"></a>
## documentation

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1896b829c8ab76f9835f41c"></a>
## eq

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &StartsWithFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 17], "end": [51, 26], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/starts_with.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb83b1b54596362e20fb49a9"></a>
## fmt

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 10], "end": [51, 15], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/starts_with.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b1ce2f8dd512872c8c71f62"></a>
## hash

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 32], "end": [51, 36], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/starts_with.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d1423b1467fb06746e3005b"></a>
## invoke_with_args

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7543c76ebf102d62179dff8"></a>
## name

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39c4b74c794c8ab7cc16b8e9"></a>
## new

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [74, 2], "filename": "src/string/starts_with.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/starts_with.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcad8e5455acaa0f9be38a55"></a>
## return_type

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bbe3f963f573d2de560bd4f"></a>
## signature

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe0c5384159ba3f98db25efd"></a>
## simplify

`function` · `datafusion_functions::string::starts_with::StartsWithFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::starts_with::StartsWithFunc", "path": "StartsWithFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [219, 2], "filename": "src/string/starts_with.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/starts_with.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
