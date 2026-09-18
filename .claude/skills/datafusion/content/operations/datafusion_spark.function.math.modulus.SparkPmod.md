# `datafusion_spark::function::math::modulus::SparkPmod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.modulus.SparkPmod.json).

<a id="op-a6af05237809e7422e71ef68"></a>
## SparkPmod

`struct` · `datafusion_spark::function::math::modulus::SparkPmod` · datafusion-spark 55.1.0

```rust
struct SparkPmod
```

Source: `src/function/math/modulus.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

SparkMod implements the Spark-compatible modulo function

<a id="op-547f508385bf0cbe38a5cf3e"></a>
## default

`function` · `datafusion_spark::function::math::modulus::SparkPmod::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [144, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/modulus.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0ea92186dab4fb1f7b04e23"></a>
## eq

`function` · `datafusion_spark::function::math::modulus::SparkPmod::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkPmod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 17], "end": [135, 26], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/modulus.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be3fa528c679a7bbc6f2934b"></a>
## fmt

`function` · `datafusion_spark::function::math::modulus::SparkPmod::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 10], "end": [135, 15], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/modulus.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61d4bb0a29dba4c492e63536"></a>
## hash

`function` · `datafusion_spark::function::math::modulus::SparkPmod::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [135, 32], "end": [135, 36], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/modulus.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e03c03705f1fd979cf40864a"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::modulus::SparkPmod::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [178, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89c72f1bae5c19bae82b8b6b"></a>
## name

`function` · `datafusion_spark::function::math::modulus::SparkPmod::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [178, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc17d708e6c703d93ec4505d"></a>
## new

`function` · `datafusion_spark::function::math::modulus::SparkPmod::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [152, 2], "filename": "src/function/math/modulus.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/modulus.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8e74629922b6248f5509081"></a>
## return_type

`function` · `datafusion_spark::function::math::modulus::SparkPmod::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [178, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac15effc6930dbca4ff9082a"></a>
## signature

`function` · `datafusion_spark::function::math::modulus::SparkPmod::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkPmod", "path": "SparkPmod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 1], "end": [178, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
