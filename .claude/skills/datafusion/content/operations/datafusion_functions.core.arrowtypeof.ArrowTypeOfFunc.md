# `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.arrowtypeof.ArrowTypeOfFunc.json).

<a id="op-77eb8019146114e861aa630a"></a>
## ArrowTypeOfFunc

`struct` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc` · datafusion-functions 55.1.0

```rust
struct ArrowTypeOfFunc
```

Source: `src/core/arrowtypeof.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-976481e71e9d9d5245b835e0"></a>
## default

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [51, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/arrowtypeof.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66857465f0a00ccc81be7934"></a>
## documentation

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [85, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrowtypeof.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-739d4ba1d736bce5d0ddce64"></a>
## eq

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ArrowTypeOfFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 26], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/arrowtypeof.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b7c4b69434bb9fe1c0c6048"></a>
## fmt

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/arrowtypeof.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e682de4df2328827c1c6552"></a>
## hash

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 32], "end": [42, 36], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/arrowtypeof.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b5fa1646a6405ec118f713a"></a>
## invoke_with_args

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [85, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrowtypeof.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce047dc5e6316ceb945772c9"></a>
## name

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [85, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrowtypeof.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b00ef1bf48ac085ef8f9d09a"></a>
## new

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [59, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/arrowtypeof.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5d6e5cf1c43893bc9370a68"></a>
## return_type

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [85, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrowtypeof.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65c8c9c85a4a59ce217246ba"></a>
## signature

`function` · `datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrowtypeof::ArrowTypeOfFunc", "path": "ArrowTypeOfFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [85, 2], "filename": "src/core/arrowtypeof.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrowtypeof.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
