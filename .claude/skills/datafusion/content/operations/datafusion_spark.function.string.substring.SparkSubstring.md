# `datafusion_spark::function::string::substring::SparkSubstring`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.substring.SparkSubstring.json).

<a id="op-b2f10f2520ce37241410cfc9"></a>
## SparkSubstring

`struct` · `datafusion_spark::function::string::substring::SparkSubstring` · datafusion-spark 55.1.0

```rust
struct SparkSubstring
```

Source: `src/function/string/substring.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `substring` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#substring>

Returns the substring from string starting at position pos with length len.
Position is 1-indexed. If pos is negative, it counts from the end of the string.
Returns NULL if any input is NULL.

<a id="op-1410c61c3b9e9233015a4b30"></a>
## aliases

`function` · `datafusion_spark::function::string::substring::SparkSubstring::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe36048aeadf24963f78df50"></a>
## default

`function` · `datafusion_spark::function::string::substring::SparkSubstring::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/substring.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b2f86e8498b255ed15c3d90"></a>
## eq

`function` · `datafusion_spark::function::string::substring::SparkSubstring::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSubstring) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 26], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/substring.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efe49d26ab95f6dc29624829"></a>
## fmt

`function` · `datafusion_spark::function::string::substring::SparkSubstring::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/substring.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecc616f2a2ed6f4df44dfb1b"></a>
## hash

`function` · `datafusion_spark::function::string::substring::SparkSubstring::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 32], "end": [45, 36], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/substring.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18fb83dede7544df49c74db5"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::substring::SparkSubstring::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0831102abcc405ae1ee5167"></a>
## name

`function` · `datafusion_spark::function::string::substring::SparkSubstring::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea5a24221904cb843c0e8dc4"></a>
## new

`function` · `datafusion_spark::function::string::substring::SparkSubstring::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [93, 2], "filename": "src/function/string/substring.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/substring.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-427899a7ee227f87a4910e5c"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::substring::SparkSubstring::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-967ca1821705510b30507bfa"></a>
## return_type

`function` · `datafusion_spark::function::string::substring::SparkSubstring::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-414c4fafc81290471449bc82"></a>
## signature

`function` · `datafusion_spark::function::string::substring::SparkSubstring::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::substring::SparkSubstring", "path": "SparkSubstring"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [128, 2], "filename": "src/function/string/substring.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/substring.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
