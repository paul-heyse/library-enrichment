# `datafusion_spark::function::math::factorial::SparkFactorial`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.factorial.SparkFactorial.json).

<a id="op-e2edc64bfbac350bfbe61479"></a>
## SparkFactorial

`struct` · `datafusion_spark::function::math::factorial::SparkFactorial` · datafusion-spark 55.1.0

```rust
struct SparkFactorial
```

Source: `src/function/math/factorial.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#factorial>

<a id="op-32cefe51d1dbf14053ace7c9"></a>
## aliases

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [72, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/factorial.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2f98ef5c0b478f0ac1cc14e"></a>
## default

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/factorial.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c06b7b13a1a7c233ca16b6"></a>
## eq

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkFactorial) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 26], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/factorial.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5a1a46066949d2452d47fea"></a>
## fmt

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/factorial.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c509bc34b8fc3e3b437bc82"></a>
## hash

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 32], "end": [31, 36], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/factorial.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ca0886cb723ee08d69597e2"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [72, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/factorial.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f440ea3cc15b315776c484e9"></a>
## name

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [72, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/factorial.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-368add3e80be1b218e6a0039"></a>
## new

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [50, 2], "filename": "src/function/math/factorial.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/factorial.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f016b03282fb56e1aa8c4ba"></a>
## return_type

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [72, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/factorial.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38256cf17212d6f2458a4489"></a>
## signature

`function` · `datafusion_spark::function::math::factorial::SparkFactorial::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::factorial::SparkFactorial", "path": "SparkFactorial"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [72, 2], "filename": "src/function/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/factorial.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
