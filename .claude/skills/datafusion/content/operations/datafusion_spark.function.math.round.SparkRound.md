# `datafusion_spark::function::math::round::SparkRound`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.round.SparkRound.json).

<a id="op-9222c4fe29e7d0ad6b995265"></a>
## SparkRound

`struct` · `datafusion_spark::function::math::round::SparkRound` · datafusion-spark 55.1.0

```rust
struct SparkRound
```

Source: `src/function/math/round.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `round` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#round>

Rounds the value of `expr` to `scale` decimal places using HALF_UP rounding mode.
Returns the same type as the input expression.

- `round(expr)` rounds to 0 decimal places (default scale = 0)
- `round(expr, scale)` rounds to `scale` decimal places
- For integer types with negative scale: `round(25, -1)` → `30`
- Uses HALF_UP rounding: 2.5 → 3, -2.5 → -3 (away from zero)

Supported types: Int8, Int16, Int32, Int64, UInt8, UInt16, UInt32, UInt64,
Float16, Float32, Float64, Decimal32, Decimal64, Decimal128, Decimal256

<a id="op-a1754159a370a3d92fdae288"></a>
## default

`function` · `datafusion_spark::function::math::round::SparkRound::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [57, 2], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/round.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f1ce6a83488e9451048696"></a>
## eq

`function` · `datafusion_spark::function::math::round::SparkRound::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkRound) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 17], "end": [48, 26], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/round.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-724e4f98fe9c2c0434337197"></a>
## fmt

`function` · `datafusion_spark::function::math::round::SparkRound::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 10], "end": [48, 15], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/round.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8590119c2bc845ec3aaca0c0"></a>
## hash

`function` · `datafusion_spark::function::math::round::SparkRound::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 32], "end": [48, 36], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/round.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57622e4117018c28388f0b31"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::round::SparkRound::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [125, 2], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/round.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8c6364ebd644a1648c6fe87"></a>
## name

`function` · `datafusion_spark::function::math::round::SparkRound::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [125, 2], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/round.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4734dda246087dd0e6d589b"></a>
## new

`function` · `datafusion_spark::function::math::round::SparkRound::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [107, 2], "filename": "src/function/math/round.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/round.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e204c587124d321e4597fb08"></a>
## return_type

`function` · `datafusion_spark::function::math::round::SparkRound::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [125, 2], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/round.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97c3018c51a04e2cf805c3b3"></a>
## signature

`function` · `datafusion_spark::function::math::round::SparkRound::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::round::SparkRound", "path": "SparkRound"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 1], "end": [125, 2], "filename": "src/function/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/round.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
