# `datafusion_functions::math::abs::AbsFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.abs.AbsFunc.json).

<a id="op-57d69ce510765ff2b29ee481"></a>
## AbsFunc

`struct` · `datafusion_functions::math::abs::AbsFunc` · datafusion-functions 55.1.0

```rust
struct AbsFunc
```

Source: `src/math/abs.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2aca02d31743d0b8499175f1"></a>
## default

`function` · `datafusion_functions::math::abs::AbsFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [138, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/abs.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51d06468209dfb8d6cab57eb"></a>
## documentation

`function` · `datafusion_functions::math::abs::AbsFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59078822c0082ea5e78dcf76"></a>
## eq

`function` · `datafusion_functions::math::abs::AbsFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &AbsFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 17], "end": [129, 26], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/abs.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afaaa802bff9a2191d9fb1ec"></a>
## fmt

`function` · `datafusion_functions::math::abs::AbsFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 10], "end": [129, 15], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/abs.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90e06a45c6eaaa4d15c35fb5"></a>
## hash

`function` · `datafusion_functions::math::abs::AbsFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 32], "end": [129, 36], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/abs.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd4efe872296f0740337a12"></a>
## invoke_with_args

`function` · `datafusion_functions::math::abs::AbsFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ddc4974052b20dcf7dfbf40"></a>
## is_strict

`function` · `datafusion_functions::math::abs::AbsFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d21673598824a1ca775ef9cf"></a>
## name

`function` · `datafusion_functions::math::abs::AbsFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97df23ce7f4e22a34f999e48"></a>
## new

`function` · `datafusion_functions::math::abs::AbsFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [146, 2], "filename": "src/math/abs.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/abs.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-878a7fe50b7600deca10a16e"></a>
## output_ordering

`function` · `datafusion_functions::math::abs::AbsFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d695362ff96cd3d102ffded3"></a>
## return_type

`function` · `datafusion_functions::math::abs::AbsFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d88c462a214e2b6331c7cb8f"></a>
## signature

`function` · `datafusion_functions::math::abs::AbsFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::abs::AbsFunc", "path": "AbsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [148, 1], "end": [193, 2], "filename": "src/math/abs.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/abs.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
