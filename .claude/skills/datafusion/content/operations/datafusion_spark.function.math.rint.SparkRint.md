# `datafusion_spark::function::math::rint::SparkRint`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.math.rint.SparkRint.json).

<a id="op-36f14dbc6884c6150bd63774"></a>
## SparkRint

`struct` · `datafusion_spark::function::math::rint::SparkRint` · datafusion-spark 55.1.0

```rust
struct SparkRint
```

Source: `src/function/math/rint.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93afe55f2120ba00a1e1d85f"></a>
## default

`function` · `datafusion_spark::function::math::rint::SparkRint::default` · datafusion-spark 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/function/math/rint.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-664bfc999ddd64768f4954ae"></a>
## eq

`function` · `datafusion_spark::function::math::rint::SparkRint::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &SparkRint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/math/rint.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd7ff0d38344c460db3c3473"></a>
## fmt

`function` · `datafusion_spark::function::math::rint::SparkRint::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/math/rint.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7374469b3d63a4d3636bc04"></a>
## hash

`function` · `datafusion_spark::function::math::rint::SparkRint::hash` · datafusion-spark 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 32], "end": [33, 36], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/function/math/rint.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-911a8def5f80c02aee66e602"></a>
## invoke_with_args

`function` · `datafusion_spark::function::math::rint::SparkRint::invoke_with_args` · datafusion-spark 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [78, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/rint.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-884b20c5e6f13c9164586851"></a>
## name

`function` · `datafusion_spark::function::math::rint::SparkRint::name` · datafusion-spark 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [78, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/rint.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c96334a1dc8daa3d57bae0d5"></a>
## new

`function` · `datafusion_spark::function::math::rint::SparkRint::new` · datafusion-spark 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/function/math/rint.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/math/rint.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e10ab008d52f99cb1c245d28"></a>
## output_ordering

`function` · `datafusion_spark::function::math::rint::SparkRint::output_ordering` · datafusion-spark 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [78, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/rint.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95521f9f65cfdfd0f39fc7a6"></a>
## return_type

`function` · `datafusion_spark::function::math::rint::SparkRint::return_type` · datafusion-spark 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [78, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/rint.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f192689573b53c661f91d195"></a>
## signature

`function` · `datafusion_spark::function::math::rint::SparkRint::signature` · datafusion-spark 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::math::rint::SparkRint", "path": "SparkRint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [78, 2], "filename": "src/function/math/rint.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/function/math/rint.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
