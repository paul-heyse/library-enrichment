# `datafusion_functions::core::getfield::GetFieldFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.getfield.GetFieldFunc.json).

<a id="op-94d7f57d746adb10588c4f2e"></a>
## GetFieldFunc

`struct` · `datafusion_functions::core::getfield::GetFieldFunc` · datafusion-functions 55.1.0

```rust
struct GetFieldFunc
```

Source: `src/core/getfield.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57a70cf91cd2b6cc61711b54"></a>
## coerce_types

`function` · `datafusion_functions::core::getfield::GetFieldFunc::coerce_types` · datafusion-functions 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:650`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac74d1655e443073dd3957b2"></a>
## default

`function` · `datafusion_functions::core::getfield::GetFieldFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [101, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/getfield.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ef6fe074802c58d7ecc7a8e"></a>
## display_name

`function` · `datafusion_functions::core::getfield::GetFieldFunc::display_name` · datafusion-functions 55.1.0

```rust
fn display_name(&self, args: &[Expr]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-692314c1e6cc39f5a39e3284"></a>
## documentation

`function` · `datafusion_functions::core::getfield::GetFieldFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:661`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0dbcdcf611fb680d7979e99"></a>
## eq

`function` · `datafusion_functions::core::getfield::GetFieldFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &GetFieldFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 17], "end": [92, 26], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/getfield.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e7de34528b8cfab5b0dddc5"></a>
## fmt

`function` · `datafusion_functions::core::getfield::GetFieldFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 10], "end": [92, 15], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/getfield.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8749d87f387a5be9f39eb41"></a>
## hash

`function` · `datafusion_functions::core::getfield::GetFieldFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 32], "end": [92, 36], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/getfield.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8e5050da81fe7886cac6e37"></a>
## invoke_with_args

`function` · `datafusion_functions::core::getfield::GetFieldFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d22939fe49517262905c309"></a>
## name

`function` · `datafusion_functions::core::getfield::GetFieldFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94903886ad29ee69d19e5a9d"></a>
## new

`function` · `datafusion_functions::core::getfield::GetFieldFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [369, 1], "end": [375, 2], "filename": "src/core/getfield.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/getfield.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b3a8c08afca8959546d8a88"></a>
## placement

`function` · `datafusion_functions::core::getfield::GetFieldFunc::placement` · datafusion-functions 55.1.0

```rust
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:665`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e217049143dac99ad7ad665f"></a>
## return_field_from_args

`function` · `datafusion_functions::core::getfield::GetFieldFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:435`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d18de803644af48e86ce91d"></a>
## return_type

`function` · `datafusion_functions::core::getfield::GetFieldFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2a17323f57d1b17c48f899b"></a>
## schema_name

`function` · `datafusion_functions::core::getfield::GetFieldFunc::schema_name` · datafusion-functions 55.1.0

```rust
fn schema_name(&self, args: &[Expr]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc80fbb7b9d3d62dcb169809"></a>
## signature

`function` · `datafusion_functions::core::getfield::GetFieldFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e90dac765b1800753c86cde"></a>
## simplify

`function` · `datafusion_functions::core::getfield::GetFieldFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &datafusion_expr::simplify::SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::getfield::GetFieldFunc", "path": "GetFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [378, 1], "end": [690, 2], "filename": "src/core/getfield.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/getfield.rs:587`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
