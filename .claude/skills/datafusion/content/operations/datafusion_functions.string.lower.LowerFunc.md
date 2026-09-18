# `datafusion_functions::string::lower::LowerFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.lower.LowerFunc.json).

<a id="op-efc51c39d81d0a7c189edbaa"></a>
## LowerFunc

`struct` · `datafusion_functions::string::lower::LowerFunc` · datafusion-functions 55.1.0

```rust
struct LowerFunc
```

Source: `src/string/lower.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acf5ace87f09ac32a3a67cc6"></a>
## default

`function` · `datafusion_functions::string::lower::LowerFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/lower.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4edf9477f81a139d06fcd414"></a>
## documentation

`function` · `datafusion_functions::string::lower::LowerFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [90, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/lower.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9b2f45c847a70d7045eafcd"></a>
## eq

`function` · `datafusion_functions::string::lower::LowerFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &LowerFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 26], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/lower.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe8560c132229d884ff884b3"></a>
## fmt

`function` · `datafusion_functions::string::lower::LowerFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/lower.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31c7fc10f3356805a8f8595a"></a>
## hash

`function` · `datafusion_functions::string::lower::LowerFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 32], "end": [45, 36], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/lower.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ced22e3f8a2b6a048b6b91a1"></a>
## invoke_with_args

`function` · `datafusion_functions::string::lower::LowerFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [90, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/lower.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27c05feb2738756ab7bcf3e9"></a>
## name

`function` · `datafusion_functions::string::lower::LowerFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [90, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/lower.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e2329dbd8ebc9b19b20d3f5"></a>
## new

`function` · `datafusion_functions::string::lower::LowerFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [68, 2], "filename": "src/string/lower.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/lower.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc7f3bc3c8e972f4e0f76e6"></a>
## return_type

`function` · `datafusion_functions::string::lower::LowerFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [90, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/lower.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbd61d8cf63107cb24e4293f"></a>
## signature

`function` · `datafusion_functions::string::lower::LowerFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::lower::LowerFunc", "path": "LowerFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [90, 2], "filename": "src/string/lower.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/lower.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
