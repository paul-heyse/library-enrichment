# `datafusion_functions::math::trunc::TruncFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.trunc.TruncFunc.json).

<a id="op-008a3bab272144f82a5871e4"></a>
## TruncFunc

`struct` · `datafusion_functions::math::trunc::TruncFunc` · datafusion-functions 55.1.0

```rust
struct TruncFunc
```

Source: `src/math/trunc.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d8240693d2dea38422fe307"></a>
## default

`function` · `datafusion_functions::math::trunc::TruncFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [77, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/trunc.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0992e98e7d1b0a5190a6d5dc"></a>
## documentation

`function` · `datafusion_functions::math::trunc::TruncFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2cde2b695be387a8e160b3c"></a>
## eq

`function` · `datafusion_functions::math::trunc::TruncFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &TruncFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 26], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/trunc.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3ead9d5d9694c34d353e04b"></a>
## fmt

`function` · `datafusion_functions::math::trunc::TruncFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/trunc.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-450eeca6c7ddcfd3c1e73fd0"></a>
## hash

`function` · `datafusion_functions::math::trunc::TruncFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 32], "end": [68, 36], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/trunc.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-150858d9bb2a566e5da04e3a"></a>
## invoke_with_args

`function` · `datafusion_functions::math::trunc::TruncFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a4c32a2199065795e3eaeca"></a>
## is_strict

`function` · `datafusion_functions::math::trunc::TruncFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-640a712f10f902aeeea0ea6e"></a>
## name

`function` · `datafusion_functions::math::trunc::TruncFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa54fdc358b3a045ee413245"></a>
## new

`function` · `datafusion_functions::math::trunc::TruncFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [118, 2], "filename": "src/math/trunc.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/trunc.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78b8dd9e5f4e8f79d169b030"></a>
## output_ordering

`function` · `datafusion_functions::math::trunc::TruncFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:268`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62f815be095c68a59a8c978b"></a>
## return_type

`function` · `datafusion_functions::math::trunc::TruncFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a9c38b01c56d87ea77061c6"></a>
## signature

`function` · `datafusion_functions::math::trunc::TruncFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::trunc::TruncFunc", "path": "TruncFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [120, 1], "end": [286, 2], "filename": "src/math/trunc.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/trunc.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
