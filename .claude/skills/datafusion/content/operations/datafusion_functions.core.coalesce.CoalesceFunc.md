# `datafusion_functions::core::coalesce::CoalesceFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.coalesce.CoalesceFunc.json).

<a id="op-d9d68100dd13733fb4da39cc"></a>
## CoalesceFunc

`struct` · `datafusion_functions::core::coalesce::CoalesceFunc` · datafusion-functions 55.1.0

```rust
struct CoalesceFunc
```

Source: `src/core/coalesce.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdbb963b9111a25a8c864f43"></a>
## coerce_types

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::coerce_types` · datafusion-functions 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d49e399157801c69c6242768"></a>
## conditional_arguments

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::conditional_arguments` · datafusion-functions 55.1.0

```rust
fn conditional_arguments<'a>(&self, args: &'a [Expr]) -> Option<(Vec<&'a Expr>, Vec<&'a Expr>)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5e108cab5cae8a582a2eb40"></a>
## default

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/coalesce.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03da52ca4408b53c7f19dbe7"></a>
## documentation

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d54dcd2944fda140e244ae89"></a>
## eq

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &CoalesceFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 26], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/coalesce.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40cd467f3799fd03611b0ac1"></a>
## fmt

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/coalesce.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8330ee3ffd5146af5f6e8f55"></a>
## hash

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 32], "end": [47, 36], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/coalesce.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8835bf4222dc1fb607a755cb"></a>
## invoke_with_args

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

coalesce evaluates to the first value which is not NULL

<a id="op-365a337fedec8761ba3f3b0f"></a>
## name

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8f747f3a44ca923862d94cb"></a>
## new

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [64, 2], "filename": "src/core/coalesce.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/coalesce.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b748446a826435c6376a4e2"></a>
## return_field_from_args

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6070af831886d81690c85c2"></a>
## return_type

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5014778b660220fcb2598368"></a>
## short_circuits

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::short_circuits` · datafusion-functions 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e773ca7cd436027c76b2ccd"></a>
## signature

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffe50652b78b94f4cf138a4f"></a>
## simplify

`function` · `datafusion_functions::core::coalesce::CoalesceFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::coalesce::CoalesceFunc", "path": "CoalesceFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [148, 2], "filename": "src/core/coalesce.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/coalesce.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
