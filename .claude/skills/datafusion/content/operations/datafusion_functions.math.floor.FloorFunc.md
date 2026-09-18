# `datafusion_functions::math::floor::FloorFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.floor.FloorFunc.json).

<a id="op-520dfd099adbb2d4d5160ec9"></a>
## FloorFunc

`struct` · `datafusion_functions::math::floor::FloorFunc` · datafusion-functions 55.1.0

```rust
struct FloorFunc
```

Source: `src/math/floor.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-302d6d916030ecddf2f158ea"></a>
## default

`function` · `datafusion_functions::math::floor::FloorFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [63, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/floor.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d33e88bd8107e1d6318bc2a7"></a>
## documentation

`function` · `datafusion_functions::math::floor::FloorFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-15443734aafe3f33234189b5"></a>
## eq

`function` · `datafusion_functions::math::floor::FloorFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &FloorFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 26], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/floor.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb2499f81eb9c61e599d979e"></a>
## evaluate_bounds

`function` · `datafusion_functions::math::floor::FloorFunc::evaluate_bounds` · datafusion-functions 55.1.0

```rust
fn evaluate_bounds(&self, inputs: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d2a45f4807a6e64ddf0a840"></a>
## fmt

`function` · `datafusion_functions::math::floor::FloorFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/floor.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90f6b41e3a271ce0350d5987"></a>
## hash

`function` · `datafusion_functions::math::floor::FloorFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 32], "end": [54, 36], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/floor.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fad63a521d16f894edcced68"></a>
## invoke_with_args

`function` · `datafusion_functions::math::floor::FloorFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c0961a10ee994aaa65cb2ba"></a>
## is_strict

`function` · `datafusion_functions::math::floor::FloorFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ba3f6f634ab770296ba811"></a>
## name

`function` · `datafusion_functions::math::floor::FloorFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23e8bf168a03e899d57a5133"></a>
## new

`function` · `datafusion_functions::math::floor::FloorFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [78, 2], "filename": "src/math/floor.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/floor.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcf8c90dfd19f61781f3210b"></a>
## output_ordering

`function` · `datafusion_functions::math::floor::FloorFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83543bf115b1ff903c50d5ef"></a>
## preimage

`function` · `datafusion_functions::math::floor::FloorFunc::preimage` · datafusion-functions 55.1.0

```rust
fn preimage(&self, args: &[Expr], lit_expr: &Expr, _info: &SimplifyContext) -> Result<PreimageResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Compute the preimage for floor function.

For `floor(x) = N`, the preimage is `x >= N AND x < N + 1`
because floor(x) = N for all x in [N, N+1).

This enables predicate pushdown optimizations, transforming:
`floor(col) = 100` into `col >= 100 AND col < 101`

<a id="op-27444661b15989155da53390"></a>
## return_type

`function` · `datafusion_functions::math::floor::FloorFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-931f3c924788799967ce8f51"></a>
## signature

`function` · `datafusion_functions::math::floor::FloorFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::floor::FloorFunc", "path": "FloorFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [311, 2], "filename": "src/math/floor.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/floor.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
