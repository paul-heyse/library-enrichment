# `datafusion_functions::math::nans::IsNanFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.nans.IsNanFunc.json).

<a id="op-5feaf347ed75caabe9b4e85d"></a>
## IsNanFunc

`struct` · `datafusion_functions::math::nans::IsNanFunc` · datafusion-functions 55.1.0

```rust
struct IsNanFunc
```

Source: `src/math/nans.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ae7ab01aafd54586da4454"></a>
## default

`function` · `datafusion_functions::math::nans::IsNanFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [61, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/nans.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ffb330b70c3367e5be8cffe"></a>
## documentation

`function` · `datafusion_functions::math::nans::IsNanFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c94334d86fecfa07c6a09c83"></a>
## eq

`function` · `datafusion_functions::math::nans::IsNanFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &IsNanFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 26], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/nans.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7883a382a81ebc096af02427"></a>
## fmt

`function` · `datafusion_functions::math::nans::IsNanFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/nans.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e87246dfd2078e3722d3dff"></a>
## hash

`function` · `datafusion_functions::math::nans::IsNanFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 32], "end": [52, 36], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/nans.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-141c70118404c0226884f862"></a>
## invoke_with_args

`function` · `datafusion_functions::math::nans::IsNanFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aecacbf350e2e3e2d362128a"></a>
## is_strict

`function` · `datafusion_functions::math::nans::IsNanFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c10b1b4ba7d844c9119d2e1"></a>
## name

`function` · `datafusion_functions::math::nans::IsNanFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-068b457eb1289cb23b321ad2"></a>
## new

`function` · `datafusion_functions::math::nans::IsNanFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [71, 2], "filename": "src/math/nans.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/nans.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11a77d3e2e7f20b556a0421c"></a>
## return_type

`function` · `datafusion_functions::math::nans::IsNanFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb5d9d5cca563a7867ad2051"></a>
## signature

`function` · `datafusion_functions::math::nans::IsNanFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::nans::IsNanFunc", "path": "IsNanFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [213, 2], "filename": "src/math/nans.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/nans.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
