# `datafusion_spark::function::math::expm1::SparkExpm1`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.expm1.SparkExpm1.json).

<a id="op-6efd5b646338a464cac56dcd"></a>
## SparkExpm1

`struct` · `datafusion_spark::function::math::expm1::SparkExpm1` · datafusion-spark 55.1.0

```rust
struct SparkExpm1
```

Source: `src/function/math/expm1.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#expm1>

<a id="op-35b943223b2d47b68fff2a14"></a>
## default

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 1], "end": [38, 2], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/expm1.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d70363ca2c0c3d1da6a3a080"></a>
## eq

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkExpm1) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 26], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/expm1.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9f06b4da19e50182eb2ce5c"></a>
## fmt

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/expm1.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb9cbeb013abc3dc6334149e"></a>
## hash

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 32], "end": [29, 36], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/expm1.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30170f4d2f8ef1b93834e93"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/expm1.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d264322a6fc2362f09a3337"></a>
## name

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/expm1.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dddc2e93e5da080860c601b3"></a>
## new

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [46, 2], "filename": "src/function/math/expm1.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/expm1.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d30182837194ba2b3321543c"></a>
## return_type

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/expm1.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2af583aea88db6f069df299f"></a>
## signature

`function` · `datafusion_spark::function::math::expm1::SparkExpm1::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::expm1::SparkExpm1", "path": "SparkExpm1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [87, 2], "filename": "src/function/math/expm1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/expm1.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
