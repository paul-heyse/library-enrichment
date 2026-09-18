# `datafusion_spark::function::array::repeat::SparkArrayRepeat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.repeat.SparkArrayRepeat.json).

<a id="op-1d71fe718d3128203b6a2d3d"></a>
## SparkArrayRepeat

`struct` · `datafusion_spark::function::array::repeat::SparkArrayRepeat` · datafusion-spark 55.1.0

```rust
struct SparkArrayRepeat
```

Source: `src/function/array/repeat.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `array_repeat` expression. The difference with DataFusion's `array_repeat` is the handling of NULL count: in Spark if the count is NULL, the result is NULL.
<https://spark.apache.org/docs/latest/api/sql/index.html#array_repeat>

<a id="op-2d245581e29ae40d128abdb2"></a>
## coerce_types

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::coerce_types` · datafusion-spark 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [90, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/repeat.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43b04e0f0f4af01762f314dd"></a>
## default

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/array/repeat.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab00d602fefcc8e5382672b7"></a>
## eq

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkArrayRepeat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/array/repeat.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce18808204387c101a48715"></a>
## fmt

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/array/repeat.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06b3c039ccc6743fafcdfedf"></a>
## hash

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 32], "end": [33, 36], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/array/repeat.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4204029c62847e2f01e56f54"></a>
## invoke_with_args

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [90, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/repeat.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-095961f4adc7c84eeb3bbda7"></a>
## name

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [90, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/repeat.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66f29247f3ce9bdf29fed31b"></a>
## new

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/function/array/repeat.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/array/repeat.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be0b61544838b71bd45a54ac"></a>
## return_type

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [90, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/repeat.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88731805efafb7b2fa379f20"></a>
## signature

`function` · `datafusion_spark::function::array::repeat::SparkArrayRepeat::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::repeat::SparkArrayRepeat", "path": "SparkArrayRepeat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [90, 2], "filename": "src/function/array/repeat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/repeat.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
