# `datafusion_spark::function::map::map_from_entries::MapFromEntries`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.map.map_from_entries.MapFromEntries.json).

<a id="op-a74555fb751487d2f6efb37f"></a>
## MapFromEntries

`struct` · `datafusion_spark::function::map::map_from_entries::MapFromEntries` · datafusion-spark 55.1.0

```rust
struct MapFromEntries
```

Source: `src/function/map/map_from_entries.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `map_from_entries` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#map_from_entries>

<a id="op-479af2450da23d621f53de41"></a>
## default

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/map/map_from_entries.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53dd1211744d2f83f3a7e99b"></a>
## eq

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &MapFromEntries) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/map/map_from_entries.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50a2be28b214b366c791910c"></a>
## fmt

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/map/map_from_entries.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d8ea75b8e3c468330e1617e"></a>
## hash

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/map/map_from_entries.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7702dc50d1fe41ffccc5e437"></a>
## invoke_with_args

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [112, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_entries.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75c19e9662c3732669a7ec1c"></a>
## name

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [112, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_entries.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7381cfaa1a5a93da427e590f"></a>
## new

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [55, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/map/map_from_entries.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-626d9c3be50678f0e824f2f7"></a>
## return_field_from_args

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [112, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_entries.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b03fa125841b65057db35198"></a>
## return_type

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [112, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_entries.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c57c7e9f3dac7811b055ba3f"></a>
## signature

`function` · `datafusion_spark::function::map::map_from_entries::MapFromEntries::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::map::map_from_entries::MapFromEntries", "path": "MapFromEntries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [112, 2], "filename": "src/function/map/map_from_entries.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/map/map_from_entries.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
