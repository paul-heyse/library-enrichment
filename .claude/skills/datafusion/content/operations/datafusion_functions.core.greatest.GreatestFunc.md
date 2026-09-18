# `datafusion_functions::core::greatest::GreatestFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.greatest.GreatestFunc.json).

<a id="op-742a70c084ee01200fb70ebe"></a>
## GreatestFunc

`struct` · `datafusion_functions::core::greatest::GreatestFunc` · datafusion-functions 55.1.0

```rust
struct GreatestFunc
```

Source: `src/core/greatest.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d3473f05252d2e128f3ca51"></a>
## coerce_types

`function` · `datafusion_functions::core::greatest::GreatestFunc::coerce_types` · datafusion-functions 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32559dd47e295484ed487003"></a>
## default

`function` · `datafusion_functions::core::greatest::GreatestFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [64, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/greatest.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0656e3586d080a00c412b5"></a>
## documentation

`function` · `datafusion_functions::core::greatest::GreatestFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95861bd0bf8bc3ce1b923019"></a>
## eq

`function` · `datafusion_functions::core::greatest::GreatestFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &GreatestFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 26], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/greatest.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d16086a4f5326280c665a2d"></a>
## fmt

`function` · `datafusion_functions::core::greatest::GreatestFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/greatest.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e7896366902ebc3be2d034b"></a>
## hash

`function` · `datafusion_functions::core::greatest::GreatestFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 32], "end": [55, 36], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/greatest.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30537f6ff871138734a9338a"></a>
## invoke_with_args

`function` · `datafusion_functions::core::greatest::GreatestFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7245368cc25e914933fd4eb6"></a>
## name

`function` · `datafusion_functions::core::greatest::GreatestFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6365366c965b5e041069fe16"></a>
## new

`function` · `datafusion_functions::core::greatest::GreatestFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [72, 2], "filename": "src/core/greatest.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/greatest.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb020e1c71ba5cef3c86576f"></a>
## return_type

`function` · `datafusion_functions::core::greatest::GreatestFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38b40cbc58833fcb56c40d4b"></a>
## signature

`function` · `datafusion_functions::core::greatest::GreatestFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::greatest::GreatestFunc", "path": "GreatestFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [124, 1], "end": [151, 2], "filename": "src/core/greatest.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/greatest.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
