# `datafusion_spark::function::string::concat_ws::SparkConcatWs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.concat_ws.SparkConcatWs.json).

<a id="op-e93baddb1f413d12a63a2992"></a>
## SparkConcatWs

`struct` · `datafusion_spark::function::string::concat_ws::SparkConcatWs` · datafusion-spark 55.1.0

```rust
struct SparkConcatWs
```

Source: `src/function/string/concat_ws.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-010a3bc59694990e6168ea2d"></a>
## coerce_types

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::coerce_types` · datafusion-spark 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [126, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat_ws.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-969f18bd39aef278bb37f186"></a>
## default

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/concat_ws.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd619aa84cd3a8e73e47b0a9"></a>
## eq

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkConcatWs) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 26], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/concat_ws.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f9b3abe4ef5eb5c7add18f5"></a>
## fmt

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/concat_ws.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-340fb8ec9ca2f39a7832ea81"></a>
## hash

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 32], "end": [46, 36], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/concat_ws.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a26b563dbf11cd3036dd5f5"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [126, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat_ws.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e8386feae4c9e4f2887e997"></a>
## name

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [126, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat_ws.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f540448ef01c2f9b39c91cc4"></a>
## new

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [63, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/concat_ws.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9021f9486c4d0a19ad0ba1e0"></a>
## return_type

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [126, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat_ws.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce213ad9742b64526777165b"></a>
## signature

`function` · `datafusion_spark::function::string::concat_ws::SparkConcatWs::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat_ws::SparkConcatWs", "path": "SparkConcatWs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [126, 2], "filename": "src/function/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat_ws.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
