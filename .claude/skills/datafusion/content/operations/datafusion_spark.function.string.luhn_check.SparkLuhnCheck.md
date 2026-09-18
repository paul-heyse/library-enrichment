# `datafusion_spark::function::string::luhn_check::SparkLuhnCheck`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.luhn_check.SparkLuhnCheck.json).

<a id="op-7a4cbf82f1a63207ab0d3a40"></a>
## SparkLuhnCheck

`struct` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck` · datafusion-spark 55.1.0

```rust
struct SparkLuhnCheck
```

Source: `src/function/string/luhn_check.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `luhn_check` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#luhn_check>

<a id="op-44b46b0b786de741c1939623"></a>
## default

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/string/luhn_check.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1d421336e517582de73e54"></a>
## eq

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkLuhnCheck) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/luhn_check.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd2caebc64660bf77fac66e8"></a>
## fmt

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/luhn_check.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ce76553c4a4c06e24253f19"></a>
## hash

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/string/luhn_check.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5ebabeff52db32d1b04c51e"></a>
## invoke_with_args

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [119, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/luhn_check.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7348846a2b1ac38a0cb08c1e"></a>
## name

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [119, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/luhn_check.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1b5069ce1125ef219f25d52"></a>
## new

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [56, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/luhn_check.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f1dc36d77308eca845bfb96"></a>
## return_type

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [119, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/luhn_check.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29a7c44f03f72116e0b296c6"></a>
## signature

`function` · `datafusion_spark::function::string::luhn_check::SparkLuhnCheck::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::luhn_check::SparkLuhnCheck", "path": "SparkLuhnCheck"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [119, 2], "filename": "src/function/string/luhn_check.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/string/luhn_check.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
