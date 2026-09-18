# `datafusion_functions::core::arrow_cast::ArrowCastFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.arrow_cast.ArrowCastFunc.json).

<a id="op-76e5875493f385f89e2d8eb7"></a>
## ArrowCastFunc

`struct` · `datafusion_functions::core::arrow_cast::ArrowCastFunc` · datafusion-functions 55.1.0

```rust
struct ArrowCastFunc
```

Source: `src/core/arrow_cast.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Implements casting to arbitrary arrow types (rather than SQL types)

Note that the `arrow_cast` function is somewhat special in that its
return depends only on the *value* of its second argument (not its type)

It is implemented by calling the same underlying arrow `cast` kernel as
normal SQL casts.

For example to cast to `int` using SQL  (which is then mapped to the arrow
type `Int32`)

```sql
select cast(column_x as int) ...
```

Use the `arrow_cast` function to cast to a specific arrow type

For example
```sql
select arrow_cast(column_x, 'Float64')
```

<a id="op-e2f99a13703e664c04dde363"></a>
## default

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [100, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/arrow_cast.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc94bbac676397980edc2ab5"></a>
## documentation

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afc6e7be1098275fbe4eecf6"></a>
## eq

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ArrowCastFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 17], "end": [91, 26], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/arrow_cast.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c88eb4760468180a10e9351"></a>
## fmt

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 10], "end": [91, 15], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/arrow_cast.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e5da007636a3ca3927515d9"></a>
## hash

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 32], "end": [91, 36], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/arrow_cast.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e07fd1eb5d39becca0c8cf90"></a>
## invoke_with_args

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1612718eb1dedc35a245fd0"></a>
## name

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f72a7772610faf7acff29024"></a>
## new

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [114, 2], "filename": "src/core/arrow_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/arrow_cast.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-918a54abfbe015a7230ace66"></a>
## return_field_from_args

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8a6f4745a9738d2ccf5adc5"></a>
## return_type

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e0e885ce23fdc6690237374"></a>
## signature

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58fe7d29d8e786ac1f7a2c2d"></a>
## simplify

`function` · `datafusion_functions::core::arrow_cast::ArrowCastFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_cast::ArrowCastFunc", "path": "ArrowCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [199, 2], "filename": "src/core/arrow_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_cast.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
