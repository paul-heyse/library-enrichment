# `datafusion_spark::function::datetime::unix::SparkUnixDate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.unix.SparkUnixDate.json).

<a id="op-3efe61175dfd020283b12d20"></a>
## SparkUnixDate

`struct` · `datafusion_spark::function::datetime::unix::SparkUnixDate` · datafusion-spark 55.1.0

```rust
struct SparkUnixDate
```

Source: `src/function/datetime/unix.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of days since epoch (1970-01-01) for the given date.
<https://spark.apache.org/docs/latest/api/sql/index.html#unix_date>

<a id="op-f8ddc29d92df121c8430b170"></a>
## default

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/unix.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1364f6bcdf8a07b1019696a2"></a>
## eq

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkUnixDate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/unix.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c72bdf55bd9acc58fde6765"></a>
## fmt

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/unix.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76ecc373d6f5bb82b12482eb"></a>
## hash

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/unix.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85864224d9fbadccd7171620"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25463fc79ecb737a52d79b42"></a>
## name

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a21bab6efcd0a322c262c48a"></a>
## new

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [54, 2], "filename": "src/function/datetime/unix.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/unix.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4071a81e17457720eb8841bc"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3769311b09532ad5d3a6c6f8"></a>
## return_type

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46c307e8c27f628897b9c565"></a>
## signature

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-973374af50d202fcea2c4a2a"></a>
## simplify

`function` · `datafusion_spark::function::datetime::unix::SparkUnixDate::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixDate", "path": "SparkUnixDate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [89, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
