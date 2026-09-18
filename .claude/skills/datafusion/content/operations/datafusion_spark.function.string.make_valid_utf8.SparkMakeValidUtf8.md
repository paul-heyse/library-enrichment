# `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.make_valid_utf8.SparkMakeValidUtf8.json).

<a id="op-059f3104ea4a1b87d61b0159"></a>
## SparkMakeValidUtf8

`struct` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8` · datafusion-spark 55.1.0

```rust
struct SparkMakeValidUtf8
```

Source: `src/function/string/make_valid_utf8.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `make_valid_utf8` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#make_valid_utf8>

<a id="op-3f4ec1e7454b94e9e5be949f"></a>
## default

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [43, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/make_valid_utf8.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb608958a5a443a3470fd2e"></a>
## eq

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkMakeValidUtf8) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 26], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/make_valid_utf8.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c5b0379920118ccc34acf9"></a>
## fmt

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/make_valid_utf8.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70f2c24189dc47441f5ed0a7"></a>
## hash

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 32], "end": [34, 36], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/make_valid_utf8.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-826e9c9ae4720f0a1a906b53"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/make_valid_utf8.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a51b746e66208d4cdfeb5cf"></a>
## name

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/make_valid_utf8.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01d0ba37c97dec4acd60399"></a>
## new

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 1], "end": [62, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/make_valid_utf8.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b9826af18c221f82c7384e1"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/make_valid_utf8.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90af89e1ae8a2fafbb185275"></a>
## return_type

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/make_valid_utf8.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83380c730983d689170b678f"></a>
## signature

`function` · `datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::make_valid_utf8::SparkMakeValidUtf8", "path": "SparkMakeValidUtf8"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [97, 2], "filename": "src/function/string/make_valid_utf8.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/make_valid_utf8.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
