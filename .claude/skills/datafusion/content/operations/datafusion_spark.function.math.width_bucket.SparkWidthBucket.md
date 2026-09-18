# `datafusion_spark::function::math::width_bucket::SparkWidthBucket`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.width_bucket.SparkWidthBucket.json).

<a id="op-7a2975d5515b28802f9df282"></a>
## SparkWidthBucket

`struct` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket` · datafusion-spark 55.1.0

```rust
struct SparkWidthBucket
```

Source: `src/function/math/width_bucket.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49428d341ffd461a266310b5"></a>
## default

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/width_bucket.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-606131e48f1a0d0c2e701d2c"></a>
## eq

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkWidthBucket) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 26], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/width_bucket.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4575ba85d028b355c11c22d3"></a>
## fmt

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/width_bucket.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ccf968ec5215b16115e84c4f"></a>
## hash

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 32], "end": [47, 36], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/width_bucket.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e77dc21c1d13e30b652235be"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [142, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/width_bucket.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba157b5c7d8130875a132c53"></a>
## name

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [142, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/width_bucket.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0435c62390642d24aae3eb9d"></a>
## new

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [115, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/width_bucket.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eabd0e16de921418e8960fac"></a>
## output_ordering

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::output_ordering` · datafusion-spark 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [142, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/width_bucket.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a467929be319366d132eb8a7"></a>
## return_type

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [142, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/width_bucket.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ce097a30c7c6c1dbdf8808e"></a>
## signature

`function` · `datafusion_spark::function::math::width_bucket::SparkWidthBucket::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::width_bucket::SparkWidthBucket", "path": "SparkWidthBucket"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [142, 2], "filename": "src/function/math/width_bucket.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/width_bucket.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
