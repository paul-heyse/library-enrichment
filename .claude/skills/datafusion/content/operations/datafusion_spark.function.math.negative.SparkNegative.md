# `datafusion_spark::function::math::negative::SparkNegative`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.negative.SparkNegative.json).

<a id="op-faf6b817d7c4b0248d57a74e"></a>
## SparkNegative

`struct` · `datafusion_spark::function::math::negative::SparkNegative` · datafusion-spark 55.1.0

```rust
struct SparkNegative
```

Source: `src/function/math/negative.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `negative` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#negative>

Returns the negation of input (equivalent to unary minus)
Returns NULL if input is NULL, returns NaN if input is NaN.

ANSI mode support:
 - When ANSI mode is disabled (`spark.sql.ansi.enabled=false`), negating the minimal
   value of a signed integer wraps around. For example: negative(i32::MIN) returns
   i32::MIN (wraps instead of error).
 - When ANSI mode is enabled (`spark.sql.ansi.enabled=true`), overflow conditions
   throw an ARITHMETIC_OVERFLOW error instead of wrapping.


<a id="op-b2fb5119cb9f5d96ca4ddee9"></a>
## default

`function` · `datafusion_spark::function::math::negative::SparkNegative::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [52, 2], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/negative.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c14c508e4b73f66d535398c"></a>
## eq

`function` · `datafusion_spark::function::math::negative::SparkNegative::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkNegative) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 26], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/negative.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1d04cba2ac1ecc81bba1488"></a>
## fmt

`function` · `datafusion_spark::function::math::negative::SparkNegative::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/negative.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0519691d74fb4a806eb40b0a"></a>
## hash

`function` · `datafusion_spark::function::math::negative::SparkNegative::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 32], "end": [43, 36], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/negative.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e54f6f9a62dc3025383ce115"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::negative::SparkNegative::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [94, 2], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/negative.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbdf379c6b1874ee36f534da"></a>
## name

`function` · `datafusion_spark::function::math::negative::SparkNegative::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [94, 2], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/negative.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4772bd5e7b7655c9b3321f80"></a>
## new

`function` · `datafusion_spark::function::math::negative::SparkNegative::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 1], "end": [76, 2], "filename": "src/function/math/negative.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/negative.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-545a3b60cbb6898ed54b8e83"></a>
## return_type

`function` · `datafusion_spark::function::math::negative::SparkNegative::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [94, 2], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/negative.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aca3242f070e2ad2a586a7aa"></a>
## signature

`function` · `datafusion_spark::function::math::negative::SparkNegative::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::negative::SparkNegative", "path": "SparkNegative"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [94, 2], "filename": "src/function/math/negative.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/negative.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
