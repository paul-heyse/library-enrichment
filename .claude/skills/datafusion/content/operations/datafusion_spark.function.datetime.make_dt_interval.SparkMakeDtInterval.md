# `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.datetime.make_dt_interval.SparkMakeDtInterval.json).

<a id="op-538b8c575c3e9b64f0a633dd"></a>
## SparkMakeDtInterval

`struct` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval` · datafusion-spark 55.1.0

```rust
struct SparkMakeDtInterval
```

Source: `src/function/datetime/make_dt_interval.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3bce6d4ba836def6bc0f567"></a>
## default

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [44, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/datetime/make_dt_interval.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d035dd7460cc302636832945"></a>
## eq

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkMakeDtInterval) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 17], "end": [35, 26], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/datetime/make_dt_interval.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e7a9e4e3af3e489af86733d"></a>
## fmt

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 10], "end": [35, 15], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/datetime/make_dt_interval.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db2bbb67615cd2bab23d6095"></a>
## hash

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 32], "end": [35, 36], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/datetime/make_dt_interval.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5353cd47909a771fa1ddeeee"></a>
## invoke_with_args

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [137, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_dt_interval.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cceaa2f47e90b4c8c2c8a692"></a>
## name

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [137, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_dt_interval.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-007272de1b719cdf15f90235"></a>
## new

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 1], "end": [81, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/datetime/make_dt_interval.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-710b118547925d298d486fcd"></a>
## return_field_from_args

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [137, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_dt_interval.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d41b6756f84554fb00a865"></a>
## return_type

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [137, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_dt_interval.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Note the return type is `DataType::Duration(TimeUnit::Microsecond)` and not `DataType::Interval(DayTime)` as you might expect.
This is because `DataType::Interval(DayTime)` has precision only to the millisecond, whilst Spark's `DayTimeIntervalType` has
precision to the microsecond. We use `DataType::Duration(TimeUnit::Microsecond)` in order to not lose any precision. See the
[Sail compatibility doc] for reference.

[Sail compatibility doc]: https://github.com/lakehq/sail/blob/dc5368daa24d40a7758a299e1ba8fc985cb29108/docs/guide/dataframe/data-types/compatibility.md?plain=1#L260

<a id="op-fab971b46c3ea3933c8d77d5"></a>
## signature

`function` · `datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::datetime::make_dt_interval::SparkMakeDtInterval", "path": "SparkMakeDtInterval"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [137, 2], "filename": "src/function/datetime/make_dt_interval.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/datetime/make_dt_interval.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
