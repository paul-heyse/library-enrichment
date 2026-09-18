# `datafusion_spark::function::array::shuffle::SparkShuffle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.shuffle.SparkShuffle.json).

<a id="op-78a35eec45a9fa79d13937dc"></a>
## SparkShuffle

`struct` · `datafusion_spark::function::array::shuffle::SparkShuffle` · datafusion-spark 55.1.0

```rust
struct SparkShuffle
```

Source: `src/function/array/shuffle.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f03ece94ce33b0c042a428d"></a>
## default

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [50, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/array/shuffle.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05b2761938e7e3e538a34c94"></a>
## eq

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkShuffle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 26], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/array/shuffle.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf511c816d44155f199365de"></a>
## fmt

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/array/shuffle.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0880e38c1a847988f3f364c4"></a>
## hash

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 32], "end": [41, 36], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/array/shuffle.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b7b4ba71bda250108c814d3"></a>
## invoke_with_args

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [115, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/shuffle.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e43f4004c744053f8b0a2dc0"></a>
## name

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [115, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/shuffle.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-867e21d34def4564552fcfc3"></a>
## new

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [76, 2], "filename": "src/function/array/shuffle.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/array/shuffle.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0aad21968f228ca8006d136"></a>
## return_field_from_args

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [115, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/shuffle.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-946919378a50ac84352fb26a"></a>
## return_type

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [115, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/shuffle.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ee745e904d2a497f3f449c6"></a>
## signature

`function` · `datafusion_spark::function::array::shuffle::SparkShuffle::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::shuffle::SparkShuffle", "path": "SparkShuffle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [115, 2], "filename": "src/function/array/shuffle.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/shuffle.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
