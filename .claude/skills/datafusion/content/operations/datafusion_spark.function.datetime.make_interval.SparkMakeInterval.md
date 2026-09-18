# `datafusion_spark::function::datetime::make_interval::SparkMakeInterval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.make_interval.SparkMakeInterval.json).

<a id="op-bc56f5fad24aaa19a30861e1"></a>
## SparkMakeInterval

`struct` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval` · datafusion-spark 55.1.0

```rust
struct SparkMakeInterval
```

Source: `src/function/datetime/make_interval.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ac49447af85b94a0ba198aa"></a>
## default

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/make_interval.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa13ff36ddb214abe1c1bc3"></a>
## eq

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkMakeInterval) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/make_interval.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a66f3dbe76ce10bf48e7bba"></a>
## fmt

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/make_interval.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa46a218152bf3aeff80fbbc"></a>
## hash

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/make_interval.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a2421e5c84a0fd6a9a402d1"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [128, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_interval.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d62953270b0e93db4379cf7"></a>
## name

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [128, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_interval.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75f8c7bfe15aedd04b548f94"></a>
## new

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [105, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/make_interval.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2689fec46571ca052c071b91"></a>
## return_type

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [128, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_interval.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1e3ecc5145ce9d330600ba1"></a>
## signature

`function` · `datafusion_spark::function::datetime::make_interval::SparkMakeInterval::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_interval::SparkMakeInterval", "path": "SparkMakeInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [128, 2], "filename": "src/function/datetime/make_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_interval.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
