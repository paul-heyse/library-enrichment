# `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.arrow_try_cast.ArrowTryCastFunc.json).

<a id="op-69e4bad2d93d226f0792efd5"></a>
## ArrowTryCastFunc

`struct` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc` · datafusion-functions 55.1.0

```rust
struct ArrowTryCastFunc
```

Source: `src/core/arrow_try_cast.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Like [`arrow_cast`](super::arrow_cast::ArrowCastFunc) but returns NULL on cast failure instead of erroring.

This is implemented by simplifying `arrow_try_cast(expr, 'Type')` into
`Expr::TryCast` during optimization.

<a id="op-1a73c9df7b74974752fd4d45"></a>
## default

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [72, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/arrow_try_cast.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fed7ff49d196779f068c5e7"></a>
## documentation

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b30135fca5ddfb45199ae987"></a>
## eq

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ArrowTryCastFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 17], "end": [63, 26], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/arrow_try_cast.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3dd52c9f52de5a0ddc1848f"></a>
## fmt

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 10], "end": [63, 15], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/arrow_try_cast.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-126fbb405307b1cb586ea7ed"></a>
## hash

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 32], "end": [63, 36], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/arrow_try_cast.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-044f0f0a956289ced5bbc340"></a>
## invoke_with_args

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a159132725906ef88c86d4"></a>
## name

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45f93d36b8ea9b89418ab241"></a>
## new

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [86, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/arrow_try_cast.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88ff0838eab2666408cf5bc0"></a>
## return_field_from_args

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e9cbf9acdca9314150a1f0"></a>
## return_type

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f6fc1fbc54f385b74496bd4"></a>
## signature

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55b38561717569d6f8acd655"></a>
## simplify

`function` · `datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_try_cast::ArrowTryCastFunc", "path": "ArrowTryCastFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [169, 2], "filename": "src/core/arrow_try_cast.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_try_cast.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
