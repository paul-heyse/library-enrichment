# `datafusion_spark::function::math::atan2::SparkAtan2`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.atan2.SparkAtan2.json).

<a id="op-13ef3fd98b2ef0baa1375a2d"></a>
## SparkAtan2

`struct` · `datafusion_spark::function::math::atan2::SparkAtan2` · datafusion-spark 55.1.0

```rust
struct SparkAtan2
```

Source: `src/function/math/atan2.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `atan2` function.

<https://spark.apache.org/docs/latest/api/sql/index.html#atan2>

`atan2(exprY, exprX)` returns the angle in radians between the positive
x-axis and the point given by the coordinates (exprX, exprY).

<a id="op-d9309fb321bec51281015aa2"></a>
## default

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [45, 2], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/atan2.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da0996308e1a1ea10bade63f"></a>
## eq

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkAtan2) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 26], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/atan2.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b77683639d6e020aaeb13ac"></a>
## fmt

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/atan2.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ec6d74d98b6542d38e9e54a"></a>
## hash

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 32], "end": [36, 36], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/atan2.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c22dc8a3e296e253ac80e3fc"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/atan2.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4107d1e2f5e4d5ca40d6d4d4"></a>
## name

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/atan2.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e538fd0e61194bc8b8368436"></a>
## new

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [57, 2], "filename": "src/function/math/atan2.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/atan2.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-069306b94c1864c4d9f5e744"></a>
## return_type

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/atan2.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b65d810dadb5fd806725b7d7"></a>
## signature

`function` · `datafusion_spark::function::math::atan2::SparkAtan2::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::atan2::SparkAtan2", "path": "SparkAtan2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/atan2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/atan2.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
