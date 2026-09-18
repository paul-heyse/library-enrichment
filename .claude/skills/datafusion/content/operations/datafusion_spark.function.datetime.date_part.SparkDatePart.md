# `datafusion_spark::function::datetime::date_part::SparkDatePart`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.date_part.SparkDatePart.json).

<a id="op-31140e23a9849ed978e34d2e"></a>
## SparkDatePart

`struct` · `datafusion_spark::function::datetime::date_part::SparkDatePart` · datafusion-spark 55.1.0

```rust
struct SparkDatePart
```

Source: `src/function/datetime/date_part.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Wrapper around datafusion date_part function to handle
Spark behavior returning day of the week 1-indexed instead of 0-indexed and different part aliases.
<https://spark.apache.org/docs/latest/api/sql/index.html#date_part>

<a id="op-b34649a1b3869ad8fe7efe45"></a>
## aliases

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-061f9e899e973f35f0eabf98"></a>
## default

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [44, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/date_part.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e251ce40440f5817c559d51"></a>
## eq

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkDatePart) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/date_part.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c3a8fca7209d764192404f"></a>
## fmt

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/date_part.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-709d51fbf6f3c385adaf02b2"></a>
## hash

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 36], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/date_part.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a70c469a0095042641f5051a"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c69ecf986948c304ee8404"></a>
## name

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df35eda54ee60918fee742b1"></a>
## new

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [65, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/date_part.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4db568113f2183e8e11f968"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f051fb14b8d7cb084f30e64b"></a>
## return_type

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e34b175fe48daddb2cb979"></a>
## signature

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b67591cd4941a73312609686"></a>
## simplify

`function` · `datafusion_spark::function::datetime::date_part::SparkDatePart::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_part::SparkDatePart", "path": "SparkDatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [138, 2], "filename": "src/function/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_part.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
