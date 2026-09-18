# `datafusion_spark::function::math::hypot::SparkHypot`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.hypot.SparkHypot.json).

<a id="op-4dced8dfaca625e9d956b961"></a>
## SparkHypot

`struct` · `datafusion_spark::function::math::hypot::SparkHypot` · datafusion-spark 55.1.0

```rust
struct SparkHypot
```

Source: `src/function/math/hypot.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `hypot` function.

<https://spark.apache.org/docs/latest/api/sql/index.html#hypot>

Returns `sqrt(expr1^2 + expr2^2)` computed without intermediate overflow or
underflow, matching Spark's use of `java.lang.Math.hypot`.

<a id="op-3ff164b846cc1006a5250e14"></a>
## default

`function` · `datafusion_spark::function::math::hypot::SparkHypot::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [45, 2], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/hypot.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be25c70d6b266b4d80204208"></a>
## eq

`function` · `datafusion_spark::function::math::hypot::SparkHypot::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkHypot) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 26], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/hypot.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f7c07c4cf4af1b135b810e2"></a>
## fmt

`function` · `datafusion_spark::function::math::hypot::SparkHypot::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/hypot.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4577139f8889ba1bc3bb33ad"></a>
## hash

`function` · `datafusion_spark::function::math::hypot::SparkHypot::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 32], "end": [36, 36], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/hypot.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b23926b4f3a8eb1f8c4423e6"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::hypot::SparkHypot::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/hypot.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a97869b18abc80fa77f2b53"></a>
## name

`function` · `datafusion_spark::function::math::hypot::SparkHypot::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/hypot.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8fb3436b95e69051f928fd9"></a>
## new

`function` · `datafusion_spark::function::math::hypot::SparkHypot::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [57, 2], "filename": "src/function/math/hypot.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/hypot.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9da606e7c8753beb06ea105a"></a>
## return_type

`function` · `datafusion_spark::function::math::hypot::SparkHypot::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/hypot.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74835ee3b914c1bc9493f4b9"></a>
## signature

`function` · `datafusion_spark::function::math::hypot::SparkHypot::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::hypot::SparkHypot", "path": "SparkHypot"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [75, 2], "filename": "src/function/math/hypot.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/hypot.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
