# `datafusion_spark::function::array::slice::SparkSlice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.slice.SparkSlice.json).

<a id="op-3650ae7f0228ff0ab2424791"></a>
## SparkSlice

`struct` · `datafusion_spark::function::array::slice::SparkSlice` · datafusion-spark 55.1.0

```rust
struct SparkSlice
```

Source: `src/function/array/slice.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark slice function implementation
Main difference from DataFusion's array_slice is that the third argument is the length of the slice and not the end index.
<https://spark.apache.org/docs/latest/api/sql/index.html#slice>

<a id="op-8ae989a2e5eb04a547ba1e6d"></a>
## default

`function` · `datafusion_spark::function::array::slice::SparkSlice::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [44, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/array/slice.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa80e7bfa00d6328b2fcad4d"></a>
## eq

`function` · `datafusion_spark::function::array::slice::SparkSlice::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSlice) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 26], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/array/slice.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f76608bfe2f237cf09428cb4"></a>
## fmt

`function` · `datafusion_spark::function::array::slice::SparkSlice::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/array/slice.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-250867a171ba7ddf945b08e8"></a>
## hash

`function` · `datafusion_spark::function::array::slice::SparkSlice::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 32], "end": [35, 36], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/array/slice.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae11fae9ae3bf32222580180"></a>
## invoke_with_args

`function` · `datafusion_spark::function::array::slice::SparkSlice::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, func_args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [137, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/slice.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5a3e0058c4b8a205070fc5a"></a>
## name

`function` · `datafusion_spark::function::array::slice::SparkSlice::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [137, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/slice.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39bcf25b67c105732cbea54f"></a>
## new

`function` · `datafusion_spark::function::array::slice::SparkSlice::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [65, 2], "filename": "src/function/array/slice.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/array/slice.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25540a2969086e52aea83022"></a>
## return_field_from_args

`function` · `datafusion_spark::function::array::slice::SparkSlice::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [137, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/slice.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61bf3291c83b3f9a6ee631d0"></a>
## return_type

`function` · `datafusion_spark::function::array::slice::SparkSlice::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [137, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/slice.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0347bae9007346030b443298"></a>
## signature

`function` · `datafusion_spark::function::array::slice::SparkSlice::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::slice::SparkSlice", "path": "SparkSlice"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [137, 2], "filename": "src/function/array/slice.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/slice.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
