# `datafusion_functions::core::cast_to_type::CastToTypeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.cast_to_type.CastToTypeFunc.json).

<a id="op-144d9df6381a93ccb6346c7a"></a>
## CastToTypeFunc

`struct` · `datafusion_functions::core::cast_to_type::CastToTypeFunc` · datafusion-functions 55.1.0

```rust
struct CastToTypeFunc
```

Source: `src/core/cast_to_type.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Casts the first argument to the data type of the second argument.

Only the type of the second argument is used; its value is ignored.
This is useful in macros or generic SQL where you need to preserve
or match types dynamically.

For example:
```sql
select cast_to_type('42', NULL::INTEGER);
```

<a id="op-6610172f25ad99258a1936df"></a>
## default

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [76, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/cast_to_type.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a1b3a3303e91b2c3852a118"></a>
## documentation

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26b3c1674e65f21a8af46ae2"></a>
## eq

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &CastToTypeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 26], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/cast_to_type.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29b6741a9133049e2c993fe8"></a>
## fmt

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/cast_to_type.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-815eac19f01efc8f9a0e757e"></a>
## hash

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 32], "end": [67, 36], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/cast_to_type.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae6dc63e05e075f17f55f595"></a>
## invoke_with_args

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-461dd7f46be9f4fcf244b2c8"></a>
## name

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f04a59875001583eb1d46b4"></a>
## new

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [90, 2], "filename": "src/core/cast_to_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/cast_to_type.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-466ee4b35b49e497c8abfe2d"></a>
## return_field_from_args

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cf1a8a017aff6862c6ac2be"></a>
## return_type

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da7a2489fd35334ded110f04"></a>
## signature

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf3e9dc9a54a9a4bec67452b"></a>
## simplify

`function` · `datafusion_functions::core::cast_to_type::CastToTypeFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::cast_to_type::CastToTypeFunc", "path": "CastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [146, 2], "filename": "src/core/cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/cast_to_type.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
