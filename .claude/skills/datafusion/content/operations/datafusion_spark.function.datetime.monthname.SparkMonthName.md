# `datafusion_spark::function::datetime::monthname::SparkMonthName`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.monthname.SparkMonthName.json).

<a id="op-ef7abfe117eb3e309b9dfdc7"></a>
## SparkMonthName

`struct` · `datafusion_spark::function::datetime::monthname::SparkMonthName` · datafusion-spark 55.1.0

```rust
struct SparkMonthName
```

Source: `src/function/datetime/monthname.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `monthname` expression.
Returns the three-letter abbreviated month name from a date or timestamp.

<https://spark.apache.org/docs/latest/api/sql/index.html#monthname>

<a id="op-ec5c9e44b4a13ac3471ef13d"></a>
## default

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/monthname.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd0af3163b34d079ac54be53"></a>
## eq

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkMonthName) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 26], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/monthname.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4406b1984b0b2399d4ebc4e"></a>
## fmt

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/monthname.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f671f25feb4e8ac67afd253a"></a>
## hash

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 32], "end": [43, 36], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/monthname.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2fbb7bf5ec712975c0d292e"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [115, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/monthname.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb7d7655de77a3cf8900a93d"></a>
## name

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [115, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/monthname.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-becaeee8e12ad86d782df00e"></a>
## new

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [67, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/monthname.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdcc622311c7b22213d8d028"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [115, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/monthname.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-321f79a18b64ef225e39253f"></a>
## return_type

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [115, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/monthname.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76ec6e440d91d21c96d25f12"></a>
## signature

`function` · `datafusion_spark::function::datetime::monthname::SparkMonthName::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::monthname::SparkMonthName", "path": "SparkMonthName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [115, 2], "filename": "src/function/datetime/monthname.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/monthname.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
