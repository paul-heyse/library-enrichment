# `datafusion_spark::function::hash::xxhash64::SparkXxhash64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.xxhash64.SparkXxhash64.json).

<a id="op-bd5666a796213b64d02f6f4a"></a>
## SparkXxhash64

`struct` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64` · datafusion-spark 55.1.0

```rust
struct SparkXxhash64
```

Source: `src/function/hash/xxhash64.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible xxhash64 function.
<https://spark.apache.org/docs/latest/api/sql/index.html#xxhash64>

<a id="op-cf3f88cae73b6191289bc2e8"></a>
## default

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/hash/xxhash64.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d66ea4d3d5c3fee7b9c284c"></a>
## eq

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkXxhash64) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 26], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/hash/xxhash64.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02357e59260fc44255c48753"></a>
## fmt

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/hash/xxhash64.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b125a4d40679c0a092c87d2a"></a>
## hash

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 32], "end": [39, 36], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/hash/xxhash64.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b0bbab983a5d37baf8af63"></a>
## invoke_with_args

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [97, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/xxhash64.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb704d94e7e19fc4f20df259"></a>
## name

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [97, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/xxhash64.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fa79c73585fcf5738eeef8e"></a>
## new

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [56, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/hash/xxhash64.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed729b92f709634221b9ac96"></a>
## return_field_from_args

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, _args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [97, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/xxhash64.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d04cf174c82648c8d17ee16"></a>
## return_type

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [97, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/xxhash64.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c0862cebda17bae8c0406ad"></a>
## signature

`function` · `datafusion_spark::function::hash::xxhash64::SparkXxhash64::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::xxhash64::SparkXxhash64", "path": "SparkXxhash64"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [97, 2], "filename": "src/function/hash/xxhash64.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/xxhash64.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
