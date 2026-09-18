# `datafusion_spark::function::string::ilike::SparkILike`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.ilike.SparkILike.json).

<a id="op-9da1fd30fe26087fbb0ea348"></a>
## SparkILike

`struct` · `datafusion_spark::function::string::ilike::SparkILike` · datafusion-spark 55.1.0

```rust
struct SparkILike
```

Source: `src/function/string/ilike.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

ILIKE function for case-insensitive pattern matching
<https://spark.apache.org/docs/latest/api/sql/index.html#ilike>

<a id="op-27c2c99e6e9e4d9954cdbfbb"></a>
## default

`function` · `datafusion_spark::function::string::ilike::SparkILike::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/ilike.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc1e4412ec58639dddd3467c"></a>
## eq

`function` · `datafusion_spark::function::string::ilike::SparkILike::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkILike) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 26], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/ilike.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7e856501cef85a2dc97860a"></a>
## fmt

`function` · `datafusion_spark::function::string::ilike::SparkILike::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/ilike.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a97f09fb90f8c8660cf5402"></a>
## hash

`function` · `datafusion_spark::function::string::ilike::SparkILike::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 32], "end": [31, 36], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/ilike.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7134d12861a54a20a3991504"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::ilike::SparkILike::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [73, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ilike.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e51b5a9b2915a767161f3629"></a>
## name

`function` · `datafusion_spark::function::string::ilike::SparkILike::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [73, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ilike.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cebb6da4de95a88497b35d8d"></a>
## new

`function` · `datafusion_spark::function::string::ilike::SparkILike::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [48, 2], "filename": "src/function/string/ilike.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/ilike.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7f55ec71d6daf55de569c5b"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::ilike::SparkILike::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<Arc<Field>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [73, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ilike.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-374470ab2ec347dc36a7cb1b"></a>
## return_type

`function` · `datafusion_spark::function::string::ilike::SparkILike::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [73, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ilike.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7181c1a506831efbdc7dfc14"></a>
## signature

`function` · `datafusion_spark::function::string::ilike::SparkILike::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ilike::SparkILike", "path": "SparkILike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [73, 2], "filename": "src/function/string/ilike.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ilike.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
