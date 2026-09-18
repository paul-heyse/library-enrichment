# `datafusion_spark::function::bitmap::bitmap_count::BitmapCount`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.bitmap.bitmap_count.BitmapCount.json).

<a id="op-57aa0843c6953b5eb277c8fb"></a>
## BitmapCount

`struct` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount` · datafusion-spark 55.1.0

```rust
struct BitmapCount
```

Source: `src/function/bitmap/bitmap_count.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9bad35521f03cda8f25b9a0"></a>
## default

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 1], "end": [46, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/bitmap/bitmap_count.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbe25b3695c272aa300860e2"></a>
## eq

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &BitmapCount) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 26], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/bitmap/bitmap_count.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76aab53106ef9d3504e1ee26"></a>
## fmt

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/bitmap/bitmap_count.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6a3a04b3c75ffa31d3e07d2"></a>
## hash

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 32], "end": [37, 36], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/bitmap/bitmap_count.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0009931d805cbc626921c62"></a>
## invoke_with_args

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [91, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_count.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-292f857aad50e042fa338c00"></a>
## name

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [91, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_count.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-154af94f2d5c2dcd183b28b7"></a>
## new

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [60, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/bitmap/bitmap_count.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26797a288e16c85375ac009b"></a>
## return_field_from_args

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::return_field_from_args` · datafusion-spark 55.1.0

```rust
fn return_field_from_args(&self, args: datafusion_expr::ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [91, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_count.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ec217436442b220dd50b612"></a>
## return_type

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [91, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_count.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c78c4c0d0146471d2982088"></a>
## signature

`function` · `datafusion_spark::function::bitmap::bitmap_count::BitmapCount::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::bitmap::bitmap_count::BitmapCount", "path": "BitmapCount"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [91, 2], "filename": "src/function/bitmap/bitmap_count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/bitmap/bitmap_count.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
