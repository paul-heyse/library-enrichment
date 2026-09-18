# `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitmap.bitmap_bucket_number.BitmapBucketNumber.json).

<a id="op-3748b1e9fd54fd754b543837"></a>
## BitmapBucketNumber

`struct` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber` · datafusion-spark 55.1.0

```rust
struct BitmapBucketNumber
```

Source: `src/function/bitmap/bitmap_bucket_number.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `bitmap_bucket_number` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#bitmap_bucket_number>

<a id="op-f1915edd8019907216497153"></a>
## default

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae3de67a50c86ae538c8a095"></a>
## eq

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &BitmapBucketNumber) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efbfe92b508612e1f6f1863d"></a>
## fmt

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fe0a2a6862173f382c3009e"></a>
## hash

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d3d77be140aaa4086ba9ac2"></a>
## invoke_with_args

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4d4545a8be5b0ca7c52f6e3"></a>
## name

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b6385a8d0f70b02eb2b5f65"></a>
## new

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [57, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eac5b9d62e3b86467d2d857e"></a>
## return_field_from_args

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31e17fc132fa5531e18e9831"></a>
## return_type

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39b86b03918c2691c1866be3"></a>
## signature

`function` · `datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bucket_number::BitmapBucketNumber", "path": "BitmapBucketNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bucket_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bucket_number.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
