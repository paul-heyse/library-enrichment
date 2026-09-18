# `datafusion_spark::function::array::array_contains::SparkArrayContains`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.array.array_contains.SparkArrayContains.json).

<a id="op-fc0e9151ef0aa6a7db672cee"></a>
## SparkArrayContains

`struct` · `datafusion_spark::function::array::array_contains::SparkArrayContains` · datafusion-spark 55.1.0

```rust
struct SparkArrayContains
```

Source: `src/function/array/array_contains.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `array_contains` function.

Calls DataFusion's `array_has` and then applies Spark's null semantics:
- If the result from `array_has` is `true`, return `true`.
- If the result is `false` and the input array row contains any null elements,
  return `null` (because the element might have been the null).
- If the result is `false` and the input array row has no null elements,
  return `false`.

<a id="op-dbba73194407b38541772f48"></a>
## default

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [47, 2], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/array/array_contains.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49fa0415d8df35c7719729a1"></a>
## eq

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkArrayContains) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 26], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/array/array_contains.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d92b5b775676583a4f74cf4"></a>
## fmt

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/array/array_contains.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03fcb191dcbd6193a4c6b24a"></a>
## hash

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 32], "end": [38, 36], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/array/array_contains.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ae9f3553e27e5e4594772b3"></a>
## invoke_with_args

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [78, 2], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/array_contains.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-164a2198fd7634f6fb24e171"></a>
## name

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [78, 2], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/array_contains.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e801ba741b12c2e20b226803"></a>
## new

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [55, 2], "filename": "src/function/array/array_contains.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/array/array_contains.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8c588a53c9635b2efbd9d17"></a>
## return_type

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [78, 2], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/array_contains.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30f692db1a97daeab3cd3fc0"></a>
## signature

`function` · `datafusion_spark::function::array::array_contains::SparkArrayContains::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::array::array_contains::SparkArrayContains", "path": "SparkArrayContains"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [78, 2], "filename": "src/function/array/array_contains.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/array/array_contains.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
