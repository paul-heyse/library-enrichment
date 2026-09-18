# `datafusion_functions::datetime::current_date::CurrentDateFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.current_date.CurrentDateFunc.json).

<a id="op-0409ab61f8d090ddf42d1006"></a>
## CurrentDateFunc

`struct` · `datafusion_functions::datetime::current_date::CurrentDateFunc` · datafusion-functions 55.1.0

```rust
struct CurrentDateFunc
```

Source: `src/datetime/current_date.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d52df8573fbd75a21af8810c"></a>
## aliases

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b235e4507a8d62553ddca53"></a>
## default

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [67, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/current_date.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-236deeaefbdc5b2ac00a901e"></a>
## documentation

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9678df650aa26f11c9ecaa35"></a>
## eq

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &CurrentDateFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 17], "end": [57, 26], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/current_date.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d2f9370a239e2bb3a07fac2"></a>
## fmt

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 15], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/current_date.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5bc870994f896679de57b64"></a>
## hash

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 32], "end": [57, 36], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/current_date.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e63c7cc74a4b4120f3c7cd3"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b4b5061cd818d13b438f0f6"></a>
## name

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dee0958392b48c0f9609629"></a>
## new

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [76, 2], "filename": "src/datetime/current_date.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/current_date.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbf2a15fadfb282b3c62b107"></a>
## return_type

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e5720f05249b13f940f409f"></a>
## signature

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71dd4c898372b1a54783430c"></a>
## simplify

`function` · `datafusion_functions::datetime::current_date::CurrentDateFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_date::CurrentDateFunc", "path": "CurrentDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [139, 2], "filename": "src/datetime/current_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_date.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
