# `datafusion_spark::function::string::soundex::SparkSoundex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.soundex.SparkSoundex.json).

<a id="op-b629f1c9f0736e972f5ff372"></a>
## SparkSoundex

`struct` · `datafusion_spark::function::string::soundex::SparkSoundex` · datafusion-spark 55.1.0

```rust
struct SparkSoundex
```

Source: `src/function/string/soundex.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `soundex` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#soundex>

<a id="op-fbec49fa866255ba9258d337"></a>
## default

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [39, 2], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/soundex.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cd32118da8fb4e0453ef503"></a>
## eq

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSoundex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 26], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/soundex.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f7621d31c132ee1f3f414d"></a>
## fmt

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/soundex.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c92c476a83dc0f9cf73c0aa"></a>
## hash

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 32], "end": [30, 36], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/soundex.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d89c11162653ddaa21e56826"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [68, 2], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/soundex.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-296aed02c564565a6ba4ec80"></a>
## name

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [68, 2], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/soundex.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b5be96247ae02aac65a7171"></a>
## new

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [47, 2], "filename": "src/function/string/soundex.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/soundex.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b1867d163789a3536e66671"></a>
## return_type

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [68, 2], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/soundex.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-435f74995c5f68b106c95e10"></a>
## signature

`function` · `datafusion_spark::function::string::soundex::SparkSoundex::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::soundex::SparkSoundex", "path": "SparkSoundex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [68, 2], "filename": "src/function/string/soundex.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/soundex.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
