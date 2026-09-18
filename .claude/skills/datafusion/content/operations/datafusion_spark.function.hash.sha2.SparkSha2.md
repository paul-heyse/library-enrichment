# `datafusion_spark::function::hash::sha2::SparkSha2`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.sha2.SparkSha2.json).

<a id="op-86c48baf8d463c3480b93c9d"></a>
## SparkSha2

`struct` · `datafusion_spark::function::hash::sha2::SparkSha2` · datafusion-spark 55.1.0

```rust
struct SparkSha2
```

Source: `src/function/hash/sha2.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Differs from DataFusion version in allowing array input for bit lengths, and
also hex encoding the output.

<https://spark.apache.org/docs/latest/api/sql/index.html#sha2>

<a id="op-07b465b8428bd8b6164f154f"></a>
## default

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/hash/sha2.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ac53204bdeb0ca85af2d245"></a>
## eq

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSha2) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/hash/sha2.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76ce9b6fbc506d44f3527826"></a>
## fmt

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/hash/sha2.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f8ffcc7a3e095e6889e7f76"></a>
## hash

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/hash/sha2.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3842a14a7034255d308d012"></a>
## invoke_with_args

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha2.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-190255a09173bdf89688268d"></a>
## name

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha2.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbf5dfa0ecf95b032af447df"></a>
## new

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [69, 2], "filename": "src/function/hash/sha2.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/hash/sha2.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd72f4f7afb636c68b11d071"></a>
## return_type

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha2.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b365e168b226be16d09a97a"></a>
## signature

`function` · `datafusion_spark::function::hash::sha2::SparkSha2::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha2::SparkSha2", "path": "SparkSha2"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/function/hash/sha2.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha2.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
