# `datafusion_spark::function::math::pow::SparkPow`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.pow.SparkPow.json).

<a id="op-d4d93261fd12b869b3e68f1b"></a>
## SparkPow

`struct` · `datafusion_spark::function::math::pow::SparkPow` · datafusion-spark 55.1.0

```rust
struct SparkPow
```

Source: `src/function/math/pow.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible implementation of `pow` / `power`.

Behavioural difference from the DataFusion default:
- `pow(0, <negative>)` → `Infinity`  (IEEE 754 / Spark semantics)
  The default raises `"zero raised to a negative power is undefined"` to
  match PostgreSQL.

<a id="op-a49db958faab3bd249ab13e1"></a>
## aliases

`function` · `datafusion_spark::function::math::pow::SparkPow::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9aa7e295ede1414262b870b"></a>
## default

`function` · `datafusion_spark::function::math::pow::SparkPow::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [51, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/pow.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4013def9d869bdafd9c7921"></a>
## documentation

`function` · `datafusion_spark::function::math::pow::SparkPow::documentation` · datafusion-spark 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cae373ceb6100986608ec282"></a>
## eq

`function` · `datafusion_spark::function::math::pow::SparkPow::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkPow) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 26], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/pow.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4409dcd43d59290a0ee3010"></a>
## fmt

`function` · `datafusion_spark::function::math::pow::SparkPow::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/pow.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bd4efd46b0b2128fd740a35"></a>
## hash

`function` · `datafusion_spark::function::math::pow::SparkPow::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 32], "end": [41, 36], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/pow.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1183bf6a928cd9341d98b5b6"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::pow::SparkPow::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-960888b7e5d3c2e58c46f7f4"></a>
## name

`function` · `datafusion_spark::function::math::pow::SparkPow::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1e1a990892a2596a89c7a0f"></a>
## new

`function` · `datafusion_spark::function::math::pow::SparkPow::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [62, 2], "filename": "src/function/math/pow.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/pow.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38da5084e1f187f2f6861de6"></a>
## return_type

`function` · `datafusion_spark::function::math::pow::SparkPow::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d753e444504077f699f69d2"></a>
## signature

`function` · `datafusion_spark::function::math::pow::SparkPow::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::pow::SparkPow", "path": "SparkPow"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [152, 2], "filename": "src/function/math/pow.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/pow.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
