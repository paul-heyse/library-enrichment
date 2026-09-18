# `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitmap.bitmap_bit_position.BitmapBitPosition.json).

<a id="op-7f07adbc682eabc45a770da6"></a>
## BitmapBitPosition

`struct` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition` · datafusion-spark 55.1.0

```rust
struct BitmapBitPosition
```

Source: `src/function/bitmap/bitmap_bit_position.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `bitmap_bit_position` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#bitmap_bit_position>

<a id="op-5deff9303e45580e9c881cb1"></a>
## default

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 1], "end": [41, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3668a9c65fbfbc16829a826a"></a>
## eq

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &BitmapBitPosition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 26], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c44b846ecb3cb506b7888a"></a>
## fmt

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f11e4c94d3b91a514351cb7"></a>
## hash

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 32], "end": [32, 36], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ab1c3822480bfc52b1bcb2"></a>
## invoke_with_args

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9225df1032ac264267ce0edf"></a>
## name

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ac56786d4b7ed829132e548"></a>
## new

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [57, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/bitmap/bitmap_bit_position.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a482d0180585f12b8940953a"></a>
## return_field_from_args

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbd91addb579cfc44b146527"></a>
## return_type

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7761946ac8867ad55490555a"></a>
## signature

`function` · `datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_bit_position::BitmapBitPosition", "path": "BitmapBitPosition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [86, 2], "filename": "src/function/bitmap/bitmap_bit_position.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_bit_position.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
