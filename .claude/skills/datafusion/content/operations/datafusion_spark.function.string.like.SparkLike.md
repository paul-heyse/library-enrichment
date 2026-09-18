# `datafusion_spark::function::string::like::SparkLike`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.like.SparkLike.json).

<a id="op-1aee6ece637309f14f2064c7"></a>
## SparkLike

`struct` · `datafusion_spark::function::string::like::SparkLike` · datafusion-spark 55.1.0

```rust
struct SparkLike
```

Source: `src/function/string/like.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

LIKE function for case-sensitive pattern matching
<https://spark.apache.org/docs/latest/api/sql/index.html#like>

<a id="op-318309f86ee5fd04c3758d27"></a>
## default

`function` · `datafusion_spark::function::string::like::SparkLike::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [40, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/like.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e44306a52f4f21a6d82d4030"></a>
## eq

`function` · `datafusion_spark::function::string::like::SparkLike::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkLike) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 17], "end": [31, 26], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/like.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72bab85036b4a961a0e4ed5f"></a>
## fmt

`function` · `datafusion_spark::function::string::like::SparkLike::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 10], "end": [31, 15], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/like.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76f469f7850dbfdef6e64f4c"></a>
## hash

`function` · `datafusion_spark::function::string::like::SparkLike::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [31, 32], "end": [31, 36], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/like.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3b5cf98f6322b2b0ea347de"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::like::SparkLike::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [75, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/like.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5044ef01c075fa3ecada7e4f"></a>
## name

`function` · `datafusion_spark::function::string::like::SparkLike::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [75, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/like.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-939610c07afb9daf799834cf"></a>
## new

`function` · `datafusion_spark::function::string::like::SparkLike::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [48, 2], "filename": "src/function/string/like.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/like.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cff1d7ccf70c53c22cbf0530"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::like::SparkLike::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [75, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/like.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59084aae48118967fbd7cf8b"></a>
## return_type

`function` · `datafusion_spark::function::string::like::SparkLike::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [75, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/like.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a375a834f7602b950eeae07"></a>
## signature

`function` · `datafusion_spark::function::string::like::SparkLike::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::like::SparkLike", "path": "SparkLike"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [75, 2], "filename": "src/function/string/like.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/like.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
