# `datafusion_spark::function::bitwise::bit_count::SparkBitCount`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitwise.bit_count.SparkBitCount.json).

<a id="op-7f61cd4932f7df088e8d5f62"></a>
## SparkBitCount

`struct` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount` · datafusion-spark 55.1.0

```rust
struct SparkBitCount
```

Source: `src/function/bitwise/bit_count.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea9dac7d7c0d338be6ee98a4"></a>
## default

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/bitwise/bit_count.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5f1edd5985bd375f1bdb5d8"></a>
## eq

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkBitCount) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/bitwise/bit_count.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d5c2a93741d52fefc6b0f7f"></a>
## fmt

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/bitwise/bit_count.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16d700af5e864e0d2bf28323"></a>
## hash

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 32], "end": [33, 36], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/bitwise/bit_count.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1d9533a2649a5435c232cad"></a>
## invoke_with_args

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [98, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitwise/bit_count.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-330cea0836c9a27149e277d8"></a>
## name

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [98, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitwise/bit_count.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24c5ebb32eda95133e22c871"></a>
## new

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [63, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/bitwise/bit_count.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54e191d5f7400a3815843b5a"></a>
## return_field_from_args

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [98, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitwise/bit_count.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b013df4cd977e44a4b4dcfc5"></a>
## return_type

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [98, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitwise/bit_count.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9c45a9f397bf79f4a9d0ec8"></a>
## signature

`function` · `datafusion_spark::function::bitwise::bit_count::SparkBitCount::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitwise::bit_count::SparkBitCount", "path": "SparkBitCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [98, 2], "filename": "src/function/bitwise/bit_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitwise/bit_count.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
