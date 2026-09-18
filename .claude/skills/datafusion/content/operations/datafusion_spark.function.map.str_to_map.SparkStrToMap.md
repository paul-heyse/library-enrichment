# `datafusion_spark::function::map::str_to_map::SparkStrToMap`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.map.str_to_map.SparkStrToMap.json).

<a id="op-45158632a3f575552b5f0da5"></a>
## SparkStrToMap

`struct` · `datafusion_spark::function::map::str_to_map::SparkStrToMap` · datafusion-spark 55.1.0

```rust
struct SparkStrToMap
```

Source: `src/function/map/str_to_map.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `str_to_map` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#str_to_map>

Creates a map from a string by splitting on delimiters.
str_to_map(text[, pairDelim[, keyValueDelim]]) -> Map<String, String>

- text: The input string
- pairDelim: Delimiter between key-value pairs (default: ',')
- keyValueDelim: Delimiter between key and value (default: ':')

# Duplicate Key Handling
Mirrors Spark's [`spark.sql.mapKeyDedupPolicy`](https://github.com/apache/spark/blob/v4.0.0/sql/catalyst/src/main/scala/org/apache/spark/sql/internal/SQLConf.scala#L4502-L4511),
wired through DataFusion's `datafusion.spark.map_key_dedup_policy`:
- `EXCEPTION` (default): error on duplicate keys.
- `LAST_WIN`: keep the last occurrence of each duplicate key.

<a id="op-31c91453b77f2cbd805d1dfb"></a>
## default

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [65, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/map/str_to_map.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec2403b957f35ff50844bffd"></a>
## eq

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkStrToMap) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 26], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/map/str_to_map.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d89d6ee083590dbef4c86b96"></a>
## fmt

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/map/str_to_map.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1db4915b316b8221bdebeb10"></a>
## hash

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 32], "end": [56, 36], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/map/str_to_map.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38c00ecc1e8d2206ba2c1d9b"></a>
## invoke_with_args

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [111, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/str_to_map.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f0a03821eada6c349ce39da"></a>
## name

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [111, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/str_to_map.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2076c35ce1423241dd74d25"></a>
## new

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [83, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/map/str_to_map.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7124ebed1bb02753ae5387f6"></a>
## return_field_from_args

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [111, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/str_to_map.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa176a20d902dab454e8aae9"></a>
## return_type

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [111, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/str_to_map.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70b6d2b55f488f673897b062"></a>
## signature

`function` · `datafusion_spark::function::map::str_to_map::SparkStrToMap::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::str_to_map::SparkStrToMap", "path": "SparkStrToMap"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [111, 2], "filename": "src/function/map/str_to_map.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/str_to_map.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
