# `datafusion_functions_nested::array_compact::ArrayCompact`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.array_compact.ArrayCompact.json).

<a id="op-c3f148ea1c8d439ffebd4009"></a>
## ArrayCompact

`struct` · `datafusion_functions_nested::array_compact::ArrayCompact` · datafusion-functions-nested 55.1.0

```rust
struct ArrayCompact
```

Source: `src/array_compact.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82773116280b168410318e5"></a>
## aliases

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4095796acd5f5c9cffbf6fc8"></a>
## default

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [72, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/array_compact.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc65e23a48577a24d0e4016"></a>
## documentation

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8907756e071a760787bddac"></a>
## eq

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayCompact) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 26], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array_compact.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bc122e68dbfd22469b5f26d"></a>
## fmt

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array_compact.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d95d075b66f937d583f8e278"></a>
## hash

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 32], "end": [62, 36], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/array_compact.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d00f9771c6b3cbf0c57d4ba4"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f722ce697e6c1c1f18d9322"></a>
## name

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7097a33ae41e06f3d808a7f0"></a>
## new

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [81, 2], "filename": "src/array_compact.rs"}, "trait": null, "trait_path": null}`

Source: `src/array_compact.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a56444be8a34426aa8a1c60"></a>
## return_type

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d350594eec0fe3f43429e07b"></a>
## signature

`function` · `datafusion_functions_nested::array_compact::ArrayCompact::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::array_compact::ArrayCompact", "path": "ArrayCompact"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [107, 2], "filename": "src/array_compact.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/array_compact.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
