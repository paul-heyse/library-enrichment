# `datafusion_functions::string::upper::UpperFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.upper.UpperFunc.json).

<a id="op-26f28e2e6a7c51b656e47d7c"></a>
## UpperFunc

`struct` · `datafusion_functions::string::upper::UpperFunc` · datafusion-functions 55.1.0

```rust
struct UpperFunc
```

Source: `src/string/upper.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8c8795ef80c86430243bfb3"></a>
## default

`function` · `datafusion_functions::string::upper::UpperFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [53, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/upper.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-412b1c5364500fc31182e829"></a>
## documentation

`function` · `datafusion_functions::string::upper::UpperFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [89, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/upper.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a95abb664dcba2c5b0458079"></a>
## eq

`function` · `datafusion_functions::string::upper::UpperFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &UpperFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 26], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/upper.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-944cd6356cec83a82a21c694"></a>
## fmt

`function` · `datafusion_functions::string::upper::UpperFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/upper.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c44d32f2c323a1157be6e92"></a>
## hash

`function` · `datafusion_functions::string::upper::UpperFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 32], "end": [44, 36], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/upper.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fff580b18bdd243d2112890"></a>
## invoke_with_args

`function` · `datafusion_functions::string::upper::UpperFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [89, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/upper.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6acd5feb8c0824447ac0766"></a>
## name

`function` · `datafusion_functions::string::upper::UpperFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [89, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/upper.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7730910ce089adb2ee789b9c"></a>
## new

`function` · `datafusion_functions::string::upper::UpperFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [67, 2], "filename": "src/string/upper.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/upper.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f33e43fbe1a5a50d4423613"></a>
## return_type

`function` · `datafusion_functions::string::upper::UpperFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [89, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/upper.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-874c4a2df3c919d1d2f52c29"></a>
## signature

`function` · `datafusion_functions::string::upper::UpperFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::upper::UpperFunc", "path": "UpperFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [89, 2], "filename": "src/string/upper.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/upper.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
