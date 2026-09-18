# `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.time_trunc.SparkTimeTrunc.json).

<a id="op-cd1faf8e74254c455dfb340f"></a>
## SparkTimeTrunc

`struct` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc` · datafusion-spark 55.1.0

```rust
struct SparkTimeTrunc
```

Source: `src/function/datetime/time_trunc.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark time_trunc function only handles time inputs.
<https://spark.apache.org/docs/latest/api/sql/index.html#time_trunc>

<a id="op-6032391145e77fbca446b5b9"></a>
## default

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/time_trunc.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f16c6a3dc295d438ebc2833"></a>
## eq

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkTimeTrunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/time_trunc.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16cda88157282f44b2358622"></a>
## fmt

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/time_trunc.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-181e3bf7d4233c9a0fedb01f"></a>
## hash

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/time_trunc.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67d8f939b87e23f4ddf39c59"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-412e486e5e842c5affd00d5c"></a>
## name

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e76797ba33a9a380640476b"></a>
## new

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [55, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/time_trunc.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fd5753c1c3380f1cb56e40e"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02002679dfe7423f90be030e"></a>
## return_type

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd786ce41a66da293a141751"></a>
## signature

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d0da9362a8f1b84e05b3bc7"></a>
## simplify

`function` · `datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::time_trunc::SparkTimeTrunc", "path": "SparkTimeTrunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [117, 2], "filename": "src/function/datetime/time_trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/time_trunc.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
