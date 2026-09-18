# `datafusion_functions::string::ascii::AsciiFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.ascii.AsciiFunc.json).

<a id="op-879c94ff6587fbb57f8c5309"></a>
## AsciiFunc

`struct` · `datafusion_functions::string::ascii::AsciiFunc` · datafusion-functions 55.1.0

```rust
struct AsciiFunc
```

Source: `src/string/ascii.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed52e1ba0fbdef97a8ba2add"></a>
## default

`function` · `datafusion_functions::string::ascii::AsciiFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [63, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/ascii.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa73cc77e484d4746bc2deb9"></a>
## documentation

`function` · `datafusion_functions::string::ascii::AsciiFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [106, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/ascii.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c64af9c7a08e34b08ceafea0"></a>
## eq

`function` · `datafusion_functions::string::ascii::AsciiFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &AsciiFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 26], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/ascii.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80090f21eded22e7dd009bde"></a>
## fmt

`function` · `datafusion_functions::string::ascii::AsciiFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/ascii.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cce79c8fa69316350048951"></a>
## hash

`function` · `datafusion_functions::string::ascii::AsciiFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 32], "end": [54, 36], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/ascii.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c84ef8484ebd0c1bb042e21f"></a>
## invoke_with_args

`function` · `datafusion_functions::string::ascii::AsciiFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [106, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/ascii.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d9e739ef06cab32fbf5f6a9"></a>
## name

`function` · `datafusion_functions::string::ascii::AsciiFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [106, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/ascii.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea254c407f1f18dcc4cc2cfa"></a>
## new

`function` · `datafusion_functions::string::ascii::AsciiFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [77, 2], "filename": "src/string/ascii.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/ascii.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d4cb28fae87cd599fcf8922"></a>
## return_type

`function` · `datafusion_functions::string::ascii::AsciiFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [106, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/ascii.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca26249cbe7aa3b6f83bd1b4"></a>
## signature

`function` · `datafusion_functions::string::ascii::AsciiFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::ascii::AsciiFunc", "path": "AsciiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [106, 2], "filename": "src/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/ascii.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
