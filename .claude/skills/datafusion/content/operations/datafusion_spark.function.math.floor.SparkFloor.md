# `datafusion_spark::function::math::floor::SparkFloor`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.floor.SparkFloor.json).

<a id="op-2ddf3f0c809b25a5af5cea3b"></a>
## SparkFloor

`struct` · `datafusion_spark::function::math::floor::SparkFloor` · datafusion-spark 55.1.0

```rust
struct SparkFloor
```

Source: `src/function/math/floor.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `floor` function.

Differences from DataFusion's floor:
- Returns Int64 for float and integer inputs (while DataFusion preserves input type)
- For Decimal128(p, s), returns Decimal128(p-s+1, 0) with scale 0
  (DataFusion preserves original precision and scale)

<https://spark.apache.org/docs/latest/api/sql/index.html#floor>

<a id="op-5708d19799d37e49f8cfc3f5"></a>
## default

`function` · `datafusion_spark::function::math::floor::SparkFloor::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/floor.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9952a4639d3f8975b3e4994b"></a>
## eq

`function` · `datafusion_spark::function::math::floor::SparkFloor::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkFloor) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/floor.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fd5fcd86cfc3247acb06581"></a>
## fmt

`function` · `datafusion_spark::function::math::floor::SparkFloor::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/floor.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b85fd49417327f610e83c5c"></a>
## hash

`function` · `datafusion_spark::function::math::floor::SparkFloor::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/floor.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f25e5c24f03e97214ceca0a8"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::floor::SparkFloor::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> datafusion_common::Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [104, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/floor.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1207ea7923372fceb17fb04f"></a>
## name

`function` · `datafusion_spark::function::math::floor::SparkFloor::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [104, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/floor.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29f9b4360578dbaef4f499f5"></a>
## new

`function` · `datafusion_spark::function::math::floor::SparkFloor::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [55, 2], "filename": "src/function/math/floor.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/floor.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d0c2fddce8128e2a824d43"></a>
## return_field_from_args

`function` · `datafusion_spark::function::math::floor::SparkFloor::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> datafusion_common::Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [104, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/floor.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82190f03867e475667d9c25"></a>
## return_type

`function` · `datafusion_spark::function::math::floor::SparkFloor::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> datafusion_common::Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [104, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/floor.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edc823bcb8ddbcc2f13593ea"></a>
## signature

`function` · `datafusion_spark::function::math::floor::SparkFloor::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::floor::SparkFloor", "path": "SparkFloor"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [104, 2], "filename": "src/function/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/floor.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
