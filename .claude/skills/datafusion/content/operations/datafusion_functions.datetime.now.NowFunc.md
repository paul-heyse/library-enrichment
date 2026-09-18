# `datafusion_functions::datetime::now::NowFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.now.NowFunc.json).

<a id="op-8b4cd8cb55b7bf4f5b221614"></a>
## NowFunc

`struct` · `datafusion_functions::datetime::now::NowFunc` · datafusion-functions 55.1.0

```rust
struct NowFunc
```

Source: `src/datetime/now.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7815abdd3d499c51993c0add"></a>
## aliases

`function` · `datafusion_functions::datetime::now::NowFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c624086315f52735a656f746"></a>
## default

`function` · `datafusion_functions::datetime::now::NowFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [69, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/now.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-150f68e87d087b1f751135e9"></a>
## documentation

`function` · `datafusion_functions::datetime::now::NowFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-791cc9b50130aac6ab4dceca"></a>
## eq

`function` · `datafusion_functions::datetime::now::NowFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &NowFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 26], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/now.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1e672b0aaafac742a7817a0"></a>
## fmt

`function` · `datafusion_functions::datetime::now::NowFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/now.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3430bb360f7c72494d41c995"></a>
## hash

`function` · `datafusion_functions::datetime::now::NowFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 32], "end": [58, 36], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/now.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c6e5de42f97631fc34e594"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::now::NowFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15fe1c07f7434a024113ed3d"></a>
## name

`function` · `datafusion_functions::datetime::now::NowFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0c68ebaf3b97f76e557729f"></a>
## new

`function` · `datafusion_functions::datetime::now::NowFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [93, 2], "filename": "src/datetime/now.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/now.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Deprecated constructor retained for backwards compatibility.

Prefer [`NowFunc::new_with_config`](../operations/datafusion_functions.datetime.now.NowFunc.md#op-110de58f55839878b51b764b) which allows specifying the
timezone via [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4). This helper now mirrors the
canonical default offset (None) provided by `ConfigOptions::default()`.

<a id="op-110de58f55839878b51b764b"></a>
## new_with_config

`function` · `datafusion_functions::datetime::now::NowFunc::new_with_config` · datafusion-functions 55.1.0

```rust
fn new_with_config(config: &ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [93, 2], "filename": "src/datetime/now.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/now.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-287bbfb47d56ccfb465b282f"></a>
## return_field_from_args

`function` · `datafusion_functions::datetime::now::NowFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5935f0fca9e82531689d80e6"></a>
## return_type

`function` · `datafusion_functions::datetime::now::NowFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af8c10608c3da5cefede8499"></a>
## signature

`function` · `datafusion_functions::datetime::now::NowFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9134a9424df02c6c1c0dfaae"></a>
## simplify

`function` · `datafusion_functions::datetime::now::NowFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62bab328c3615863d88aee81"></a>
## with_updated_config

`function` · `datafusion_functions::datetime::now::NowFunc::with_updated_config` · datafusion-functions 55.1.0

```rust
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::now::NowFunc", "path": "NowFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [156, 2], "filename": "src/datetime/now.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/now.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
