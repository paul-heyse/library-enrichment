# `datafusion_spark::function::datetime::unix::SparkUnixTimestamp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.unix.SparkUnixTimestamp.json).

<a id="op-3234e94ef87724775bf9c5cd"></a>
## SparkUnixTimestamp

`struct` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp` · datafusion-spark 55.1.0

```rust
struct SparkUnixTimestamp
```

Source: `src/function/datetime/unix.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3415c26723dc11b14006fb9"></a>
## eq

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkUnixTimestamp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 17], "end": [91, 26], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/unix.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2965ff5386e3477da5e505e8"></a>
## fmt

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 10], "end": [91, 15], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/unix.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-797d25a1cebab95e7267a539"></a>
## hash

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 32], "end": [91, 36], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/unix.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8dcb888fcd9297bb57d4f88e"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c51156058382bb44230e4e7"></a>
## microseconds

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::microseconds` · datafusion-spark 55.1.0

```rust
fn microseconds() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [127, 2], "filename": "src/function/datetime/unix.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/unix.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of microseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp.
<https://spark.apache.org/docs/latest/api/sql/index.html#unix_micros>

<a id="op-0778ec6f45f88131f0fc55c0"></a>
## milliseconds

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::milliseconds` · datafusion-spark 55.1.0

```rust
fn milliseconds() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [127, 2], "filename": "src/function/datetime/unix.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/unix.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of milliseconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp.
<https://spark.apache.org/docs/latest/api/sql/index.html#unix_millis>

<a id="op-c7e8e39e3107c51dc52c7285"></a>
## name

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b81b801973a182373b820f76"></a>
## new

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::new` · datafusion-spark 55.1.0

```rust
fn new(name: &'static str, time_unit: TimeUnit) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [127, 2], "filename": "src/function/datetime/unix.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/unix.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa16e7b28e07cabbf3606f4a"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a69938ea7dda3b89d17ce2"></a>
## return_type

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2665ceefd3f8c25e3926de35"></a>
## seconds

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::seconds` · datafusion-spark 55.1.0

```rust
fn seconds() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [127, 2], "filename": "src/function/datetime/unix.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/unix.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Returns the number of seconds since epoch (1970-01-01 00:00:00 UTC) for the given timestamp.
<https://spark.apache.org/docs/latest/api/sql/index.html#unix_seconds>

<a id="op-4bc06e3f77c0678a60413c20"></a>
## signature

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bf9fd91c647565396325faa"></a>
## simplify

`function` · `datafusion_spark::function::datetime::unix::SparkUnixTimestamp::simplify` · datafusion-spark 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::unix::SparkUnixTimestamp", "path": "SparkUnixTimestamp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [165, 2], "filename": "src/function/datetime/unix.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/unix.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
