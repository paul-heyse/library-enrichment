# `datafusion_spark::function::string::ascii::SparkAscii`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.ascii.SparkAscii.json).

<a id="op-76be56f5789b0b081c9367d6"></a>
## SparkAscii

`struct` · `datafusion_spark::function::string::ascii::SparkAscii` · datafusion-spark 55.1.0

```rust
struct SparkAscii
```

Source: `src/function/string/ascii.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark compatible version of the [ascii] function. Differs from the [default ascii function]
in that it is more permissive of input types, for example casting numeric input to string
before executing the function (default version doesn't allow numeric input).

[ascii]: https://spark.apache.org/docs/latest/api/sql/index.html#ascii
[default ascii function]: datafusion_functions::string::ascii::AsciiFunc

<a id="op-1524edac9fbfb6742e0fa5b8"></a>
## default

`function` · `datafusion_spark::function::string::ascii::SparkAscii::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 1], "end": [45, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/ascii.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82e4edefef7475241c7151d8"></a>
## eq

`function` · `datafusion_spark::function::string::ascii::SparkAscii::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkAscii) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 17], "end": [36, 26], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/ascii.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eac8af563718004c62a01b5"></a>
## fmt

`function` · `datafusion_spark::function::string::ascii::SparkAscii::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 10], "end": [36, 15], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/ascii.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8564d01463247500c13575d"></a>
## hash

`function` · `datafusion_spark::function::string::ascii::SparkAscii::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 32], "end": [36, 36], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/ascii.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f5f6275aae92e97af61824a"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::ascii::SparkAscii::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [87, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ascii.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1581e405bfcd37d2cf7ae1b"></a>
## name

`function` · `datafusion_spark::function::string::ascii::SparkAscii::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [87, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ascii.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f622c6a7cabb45fcf64ec93"></a>
## new

`function` · `datafusion_spark::function::string::ascii::SparkAscii::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 1], "end": [62, 2], "filename": "src/function/string/ascii.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/ascii.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17714e697addf9208412f6ad"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::ascii::SparkAscii::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [87, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ascii.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37d6c12ca58a5787c6d30273"></a>
## return_type

`function` · `datafusion_spark::function::string::ascii::SparkAscii::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [87, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ascii.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e827563d341e75993746de6"></a>
## signature

`function` · `datafusion_spark::function::string::ascii::SparkAscii::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::ascii::SparkAscii", "path": "SparkAscii"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [87, 2], "filename": "src/function/string/ascii.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/ascii.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
