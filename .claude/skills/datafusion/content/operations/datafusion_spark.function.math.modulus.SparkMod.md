# `datafusion_spark::function::math::modulus::SparkMod`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.modulus.SparkMod.json).

<a id="op-8820627d2ac704fd440282cf"></a>
## SparkMod

`struct` · `datafusion_spark::function::math::modulus::SparkMod` · datafusion-spark 55.1.0

```rust
struct SparkMod
```

Source: `src/function/math/modulus.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

SparkMod implements the Spark-compatible modulo function

<a id="op-cbaa56737c4bf9ac58a5d5c1"></a>
## default

`function` · `datafusion_spark::function::math::modulus::SparkMod::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [94, 1], "end": [98, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/modulus.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bff5fc432b451fa6f256b454"></a>
## eq

`function` · `datafusion_spark::function::math::modulus::SparkMod::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkMod) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 17], "end": [89, 26], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/modulus.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef820d617cbb31a0dcd490f8"></a>
## fmt

`function` · `datafusion_spark::function::math::modulus::SparkMod::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 10], "end": [89, 15], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/modulus.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-091b8cd5923a4c0373c13734"></a>
## hash

`function` · `datafusion_spark::function::math::modulus::SparkMod::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 32], "end": [89, 36], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/modulus.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f02129715f1028b0f233c6a0"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::modulus::SparkMod::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [132, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8cb27edb5c0e5673de58d5a"></a>
## name

`function` · `datafusion_spark::function::math::modulus::SparkMod::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [132, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c59c24d493c07f1dac3e1d27"></a>
## new

`function` · `datafusion_spark::function::math::modulus::SparkMod::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [106, 2], "filename": "src/function/math/modulus.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/modulus.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f01cf083335f6aacf736502"></a>
## return_type

`function` · `datafusion_spark::function::math::modulus::SparkMod::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [132, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39571172e1f7e2292621bfc3"></a>
## signature

`function` · `datafusion_spark::function::math::modulus::SparkMod::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::modulus::SparkMod", "path": "SparkMod"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [132, 2], "filename": "src/function/math/modulus.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/modulus.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
