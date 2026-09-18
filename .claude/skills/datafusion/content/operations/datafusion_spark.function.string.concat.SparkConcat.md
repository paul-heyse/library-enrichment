# `datafusion_spark::function::string::concat::SparkConcat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.concat.SparkConcat.json).

<a id="op-d141b9e865c1a6fd7e41b0d2"></a>
## SparkConcat

`struct` · `datafusion_spark::function::string::concat::SparkConcat` · datafusion-spark 55.1.0

```rust
struct SparkConcat
```

Source: `src/function/string/concat.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `concat` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#concat>

Concatenates multiple input strings into a single string.
Returns NULL if any input is NULL.

Differences with DataFusion concat:
- Support 0 arguments
- Return NULL if any input is NULL

<a id="op-7bdd68454364befc8b45376a"></a>
## coerce_types

`function` · `datafusion_spark::function::string::concat::SparkConcat::coerce_types` · datafusion-spark 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3fde22d35f68200a481122a"></a>
## default

`function` · `datafusion_spark::function::string::concat::SparkConcat::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [50, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/concat.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2af15d798503b8f599bd4eaa"></a>
## eq

`function` · `datafusion_spark::function::string::concat::SparkConcat::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkConcat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 26], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/concat.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d28232c131eff6c267dcd71"></a>
## fmt

`function` · `datafusion_spark::function::string::concat::SparkConcat::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/concat.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ff09494a2347a283a70b3fb"></a>
## hash

`function` · `datafusion_spark::function::string::concat::SparkConcat::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 32], "end": [41, 36], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/concat.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3792266e5927426380cbf566"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::concat::SparkConcat::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1f8c2fc4a644e4e0ca0d867"></a>
## name

`function` · `datafusion_spark::function::string::concat::SparkConcat::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5fdc4b433f4c5c3f792bd9d"></a>
## new

`function` · `datafusion_spark::function::string::concat::SparkConcat::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [58, 2], "filename": "src/function/string/concat.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/concat.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9c8ebfe6bb7d3dcb1e7ea56"></a>
## return_field_from_args

`function` · `datafusion_spark::function::string::concat::SparkConcat::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16917345ff325bcb833dffa7"></a>
## return_type

`function` · `datafusion_spark::function::string::concat::SparkConcat::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85dab79e413307061f46663e"></a>
## signature

`function` · `datafusion_spark::function::string::concat::SparkConcat::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::concat::SparkConcat", "path": "SparkConcat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [100, 2], "filename": "src/function/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/concat.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
