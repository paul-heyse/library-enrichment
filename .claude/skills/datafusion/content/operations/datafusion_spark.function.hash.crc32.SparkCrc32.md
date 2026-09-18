# `datafusion_spark::function::hash::crc32::SparkCrc32`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.crc32.SparkCrc32.json).

<a id="op-e536fb63afce30e44fc4a4e4"></a>
## SparkCrc32

`struct` · `datafusion_spark::function::hash::crc32::SparkCrc32` · datafusion-spark 55.1.0

```rust
struct SparkCrc32
```

Source: `src/function/hash/crc32.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#crc32>

<a id="op-0a69b2325c5c5ab977ab528e"></a>
## default

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/hash/crc32.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d74cd076bdda5df03a282d92"></a>
## eq

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkCrc32) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 26], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/hash/crc32.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c911bbd10c4e72822c303748"></a>
## fmt

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/hash/crc32.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-668db1df7003f8235dc18e98"></a>
## hash

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 32], "end": [37, 36], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/hash/crc32.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da91cfe66c5490b71e76faae"></a>
## invoke_with_args

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [84, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/crc32.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850def2da7f948273ad8f09f"></a>
## name

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [84, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/crc32.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de3140e8aa3f81977efcc5f7"></a>
## new

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [61, 2], "filename": "src/function/hash/crc32.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/hash/crc32.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-238fe66480bfefbe95c506a4"></a>
## return_field_from_args

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [84, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/crc32.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ba9a35393a260d194955f8e"></a>
## return_type

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [84, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/crc32.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cc8d208f20ab96badd2d717"></a>
## signature

`function` · `datafusion_spark::function::hash::crc32::SparkCrc32::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::crc32::SparkCrc32", "path": "SparkCrc32"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [84, 2], "filename": "src/function/hash/crc32.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/crc32.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
