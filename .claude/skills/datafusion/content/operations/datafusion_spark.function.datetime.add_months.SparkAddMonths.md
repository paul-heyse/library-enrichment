# `datafusion_spark::function::datetime::add_months::SparkAddMonths`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.add_months.SparkAddMonths.json).

<a id="op-9595e5c3744b423761c56ea7"></a>
## SparkAddMonths

`struct` · `datafusion_spark::function::datetime::add_months::SparkAddMonths` · datafusion-spark 55.1.0

```rust
struct SparkAddMonths
```

Source: `src/function/datetime/add_months.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#add_months>

<a id="op-ecc88cb2baaa6d1987966eff"></a>
## default

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/add_months.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5e1b1c39525862385591e37"></a>
## eq

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkAddMonths) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 26], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/add_months.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5b4689050024227523cb776"></a>
## fmt

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/add_months.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0b2bbe4dcd26b428f9ce15b"></a>
## hash

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 32], "end": [31, 36], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/add_months.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e73cb93cea51d78b2514f43"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-183d3ae6f757d51d80328ab9"></a>
## name

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1c38945bf4b82065b2dc5b8"></a>
## new

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [51, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/add_months.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57bc9c27e8e09f296c997d4e"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89f6da06bb0bd050b2ade062"></a>
## return_type

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5fdfd34d97d0e7d6cb79b96"></a>
## signature

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fac8e37c517b15113d644b6"></a>
## simplify

`function` · `datafusion_spark::function::datetime::add_months::SparkAddMonths::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::add_months::SparkAddMonths", "path": "SparkAddMonths"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [90, 2], "filename": "src/function/datetime/add_months.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/add_months.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
