# `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.try_cast_to_type.TryCastToTypeFunc.json).

<a id="op-19c31e6d67ab74dd65ac6ff5"></a>
## TryCastToTypeFunc

`struct` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc` · datafusion-functions 55.1.0

```rust
struct TryCastToTypeFunc
```

Source: `src/core/try_cast_to_type.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Like [`cast_to_type`](super::cast_to_type::CastToTypeFunc) but returns NULL
on cast failure instead of erroring.

This is implemented by simplifying `try_cast_to_type(expr, ref)` into
`Expr::TryCast` during optimization.

<a id="op-97a0506a7328c0668bb7b8a5"></a>
## default

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [68, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/try_cast_to_type.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2b1929f81b2a7316464075a"></a>
## documentation

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0479634dc29141fae5fc729"></a>
## eq

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &TryCastToTypeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 17], "end": [59, 26], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/try_cast_to_type.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7309124f45abc443ac9e74cd"></a>
## fmt

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/try_cast_to_type.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c715b6d992f4e9a41cbd0607"></a>
## hash

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 32], "end": [59, 36], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/try_cast_to_type.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f37c00ee415ef4a7c2730ddb"></a>
## invoke_with_args

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1141c864738878505d413088"></a>
## name

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dfd49f636b87fa7351d8f50"></a>
## new

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [82, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/try_cast_to_type.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0d0efdc17c7e8a5dd9b101e"></a>
## return_field_from_args

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-679d6e866eff24ad95f8dd6b"></a>
## return_type

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dfe392e9d7fea2f5d87505b"></a>
## signature

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d29cca7afaaf675e31ccca0"></a>
## simplify

`function` · `datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::try_cast_to_type::TryCastToTypeFunc", "path": "TryCastToTypeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [130, 2], "filename": "src/core/try_cast_to_type.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/try_cast_to_type.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
