# `datafusion_spark::function::datetime::date_sub::SparkDateSub`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.date_sub.SparkDateSub.json).

<a id="op-7a98e60c62ef31b220083c3d"></a>
## SparkDateSub

`struct` · `datafusion_spark::function::datetime::date_sub::SparkDateSub` · datafusion-spark 55.1.0

```rust
struct SparkDateSub
```

Source: `src/function/datetime/date_sub.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5221812331a951b9a289977c"></a>
## default

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/date_sub.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e4a8003dea010d7eeb134cd"></a>
## eq

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkDateSub) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/date_sub.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e65d17674b29dd8ef8e6fcd"></a>
## fmt

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/date_sub.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f403cde68df06c17919ac8a"></a>
## hash

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 32], "end": [33, 36], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/date_sub.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ce2b4bd77014e261ef936a"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [84, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_sub.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85044030989393508c089507"></a>
## name

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [84, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_sub.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7276eba588e92d03ea0ee0d1"></a>
## new

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [57, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/date_sub.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dec1a0c839cccfc514aca89b"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [84, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_sub.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b3658d4135b427b63f1453e"></a>
## return_type

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [84, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_sub.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6087476ef07ab7ceda4f141d"></a>
## signature

`function` · `datafusion_spark::function::datetime::date_sub::SparkDateSub::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::date_sub::SparkDateSub", "path": "SparkDateSub"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [84, 2], "filename": "src/function/datetime/date_sub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/date_sub.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
