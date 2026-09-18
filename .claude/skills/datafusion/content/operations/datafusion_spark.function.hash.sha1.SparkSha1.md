# `datafusion_spark::function::hash::sha1::SparkSha1`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.hash.sha1.SparkSha1.json).

<a id="op-1f3dcfa94847bce09a3a264c"></a>
## SparkSha1

`struct` · `datafusion_spark::function::hash::sha1::SparkSha1` · datafusion-spark 55.1.0

```rust
struct SparkSha1
```

Source: `src/function/hash/sha1.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

<https://spark.apache.org/docs/latest/api/sql/index.html#sha1>

<a id="op-834f0cffe247b2f06131263b"></a>
## aliases

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::aliases` · datafusion-spark 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38a455bd1f740514191d111c"></a>
## default

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/hash/sha1.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4386b09725ddc57e52ec3ab2"></a>
## eq

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkSha1) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/hash/sha1.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69fb0d2721eaf50d938f2bf6"></a>
## fmt

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/hash/sha1.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08751cf7925de24955464823"></a>
## hash

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/hash/sha1.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaf0aaa89f70dd92e91a0a97"></a>
## invoke_with_args

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8631f8a2a5bdc1e42c9e7bf"></a>
## name

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6bf1687c60150e1620522a0"></a>
## new

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [64, 2], "filename": "src/function/hash/sha1.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/hash/sha1.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-755341e37e9b91137e85a0e3"></a>
## return_field_from_args

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91d1477c82939837c54241dd"></a>
## return_type

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d5fb61aeb34557671d0bd7"></a>
## signature

`function` · `datafusion_spark::function::hash::sha1::SparkSha1::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::hash::sha1::SparkSha1", "path": "SparkSha1"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [91, 2], "filename": "src/function/hash/sha1.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/hash/sha1.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
