# `datafusion_spark::function::map::map_from_arrays::MapFromArrays`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.map.map_from_arrays.MapFromArrays.json).

<a id="op-c59b599766e42f6e8016836e"></a>
## MapFromArrays

`struct` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays` · datafusion-spark 55.1.0

```rust
struct MapFromArrays
```

Source: `src/function/map/map_from_arrays.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `map_from_arrays` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#map_from_arrays>

<a id="op-e7f6cb0cfe957d4bb063a83a"></a>
## default

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/map/map_from_arrays.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3d14794d76fcd3730659ec2"></a>
## eq

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &MapFromArrays) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 26], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/map/map_from_arrays.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4f3dd52eacfb00de51ff3c5"></a>
## fmt

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/map/map_from_arrays.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529f6eca47179e17cdeca52e"></a>
## hash

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 32], "end": [37, 36], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/map/map_from_arrays.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f734e74a0f4a0d980bac3968"></a>
## invoke_with_args

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [92, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_arrays.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6ff0e74da2d3c94f256228d"></a>
## name

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [92, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_arrays.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ab4b15b128d9cb4b18c3b5f"></a>
## new

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [54, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/map/map_from_arrays.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f52f4bf0e9062b8c1265578"></a>
## return_field_from_args

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [92, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_arrays.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18da2be857255a599a33f04b"></a>
## return_type

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [92, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_arrays.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac096ac8ffb8eda528b91cc4"></a>
## signature

`function` · `datafusion_spark::function::map::map_from_arrays::MapFromArrays::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_arrays::MapFromArrays", "path": "MapFromArrays"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [92, 2], "filename": "src/function/map/map_from_arrays.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_arrays.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
