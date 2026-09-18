# `datafusion_spark::function::math::abs::SparkAbs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.abs.SparkAbs.json).

<a id="op-0c4d5ee2428899838ab3791d"></a>
## SparkAbs

`struct` · `datafusion_spark::function::math::abs::SparkAbs` · datafusion-spark 55.1.0

```rust
struct SparkAbs
```

Source: `src/function/math/abs.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `abs` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#abs>

Returns the absolute value of input
Returns NULL if input is NULL, returns NaN if input is NaN.

Differences with DataFusion abs:
 - Spark's ANSI-compliant dialect, when off (i.e. `spark.sql.ansi.enabled=false`), taking absolute value on the minimal value of a signed integer returns the value as is. DataFusion's abs throws "DataFusion error: Arrow error: Compute error" on arithmetic overflow

TODOs:
 - Spark's abs also supports ANSI interval types: YearMonthIntervalType and DayTimeIntervalType. DataFusion's abs doesn't.


<a id="op-9b6b2af966e83efe034618c6"></a>
## default

`function` · `datafusion_spark::function::math::abs::SparkAbs::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [53, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/abs.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c0aaf9f629351e810a95407"></a>
## eq

`function` · `datafusion_spark::function::math::abs::SparkAbs::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkAbs) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 26], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/abs.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b096a34e4d14651f0c666ca8"></a>
## fmt

`function` · `datafusion_spark::function::math::abs::SparkAbs::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/abs.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-312d3406820353986f2800ea"></a>
## hash

`function` · `datafusion_spark::function::math::abs::SparkAbs::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 32], "end": [44, 36], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/abs.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d1c26025e4a64d8b40aff42"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::abs::SparkAbs::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [89, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/abs.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9bf63a1e5cc1ab5e3f02d14"></a>
## name

`function` · `datafusion_spark::function::math::abs::SparkAbs::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [89, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/abs.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a057cf016e40d79d34efddc2"></a>
## new

`function` · `datafusion_spark::function::math::abs::SparkAbs::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [61, 2], "filename": "src/function/math/abs.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/abs.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90ba8fb4469f9a5ad3ced4d1"></a>
## return_field_from_args

`function` · `datafusion_spark::function::math::abs::SparkAbs::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [89, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/abs.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7931f1921db06f2fb6dfef9"></a>
## return_type

`function` · `datafusion_spark::function::math::abs::SparkAbs::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [89, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/abs.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f57493360721b3beafe48a1"></a>
## signature

`function` · `datafusion_spark::function::math::abs::SparkAbs::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::abs::SparkAbs", "path": "SparkAbs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [89, 2], "filename": "src/function/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/abs.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
