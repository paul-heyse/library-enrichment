# `datafusion_spark::function::string::space::SparkSpace`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.space.SparkSpace.json).

<a id="op-944a566031d392046a3faef9"></a>
## SparkSpace

`struct` · `datafusion_spark::function::string::space::SparkSpace` · datafusion-spark 55.1.0

```rust
struct SparkSpace
```

Source: `src/function/string/space.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `space` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#space>

<a id="op-ceeda5718845df593c0b5231"></a>
## default

`function` · `datafusion_spark::function::string::space::SparkSpace::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/space.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1d22ff82c47d442e31954e1"></a>
## eq

`function` · `datafusion_spark::function::string::space::SparkSpace::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSpace) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/space.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1b02588a62f9936ec32e431"></a>
## fmt

`function` · `datafusion_spark::function::string::space::SparkSpace::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/space.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9d808845cd2315b58a7c730"></a>
## hash

`function` · `datafusion_spark::function::string::space::SparkSpace::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/space.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d8fbcf11bb5588f12dd565c"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::space::SparkSpace::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [83, 2], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/space.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73a9c7730b034067ecb7107b"></a>
## name

`function` · `datafusion_spark::function::string::space::SparkSpace::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [83, 2], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/space.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bdda141185f8725619f68ae"></a>
## new

`function` · `datafusion_spark::function::string::space::SparkSpace::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [59, 2], "filename": "src/function/string/space.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/space.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39b4d8a9929d7732df237bc8"></a>
## return_type

`function` · `datafusion_spark::function::string::space::SparkSpace::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, args: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [83, 2], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/space.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f52cc5ad34f9083afb2ab358"></a>
## signature

`function` · `datafusion_spark::function::string::space::SparkSpace::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::space::SparkSpace", "path": "SparkSpace"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [83, 2], "filename": "src/function/string/space.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/space.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
