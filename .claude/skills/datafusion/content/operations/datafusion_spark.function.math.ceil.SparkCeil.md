# `datafusion_spark::function::math::ceil::SparkCeil`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.ceil.SparkCeil.json).

<a id="op-7ff732725f4c7127de0de44c"></a>
## SparkCeil

`struct` · `datafusion_spark::function::math::ceil::SparkCeil` · datafusion-spark 55.1.0

```rust
struct SparkCeil
```

Source: `src/function/math/ceil.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `ceil` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#ceil>

Differences with DataFusion ceil:
 - Spark's ceil returns Int64 for float inputs; DataFusion preserves
   the input type (Float32→Float32, Float64→Float64)
 - Spark's ceil on Decimal128(p, s) returns Decimal128(p−s+1, 0), reducing scale
   to 0; DataFusion preserves the original precision and scale
 - Spark only supports Decimal128; DataFusion also supports Decimal32/64/256
 - Spark does not check for decimal overflow; DataFusion errors on overflow

2-argument ceil(value, scale) is not yet implemented
<https://github.com/apache/datafusion/issues/21560>

<a id="op-c21e4c82919e3a05dbe0643a"></a>
## aliases

`function` · `datafusion_spark::function::math::ceil::SparkCeil::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [98, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/ceil.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a9c89c33c315abce935ee75"></a>
## default

`function` · `datafusion_spark::function::math::ceil::SparkCeil::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [51, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/ceil.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29992df6991707b62e66375b"></a>
## eq

`function` · `datafusion_spark::function::math::ceil::SparkCeil::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkCeil) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 26], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/ceil.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a8fc95bf011bfacfc4f0fe4"></a>
## fmt

`function` · `datafusion_spark::function::math::ceil::SparkCeil::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/ceil.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23dcdfce0f5b0dd083a81096"></a>
## hash

`function` · `datafusion_spark::function::math::ceil::SparkCeil::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 32], "end": [41, 36], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/ceil.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45372916cc92f34686071086"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::ceil::SparkCeil::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [98, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/ceil.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86058f8bcb4f5aa68f518c40"></a>
## name

`function` · `datafusion_spark::function::math::ceil::SparkCeil::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [98, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/ceil.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4978234b2284f8ecbc68c67"></a>
## new

`function` · `datafusion_spark::function::math::ceil::SparkCeil::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [60, 2], "filename": "src/function/math/ceil.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/ceil.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dd6c2862759623fb6a7013d"></a>
## return_type

`function` · `datafusion_spark::function::math::ceil::SparkCeil::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [98, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/ceil.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdb8d9aab83ee08aa28c0783"></a>
## signature

`function` · `datafusion_spark::function::math::ceil::SparkCeil::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::ceil::SparkCeil", "path": "SparkCeil"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [98, 2], "filename": "src/function/math/ceil.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/ceil.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
