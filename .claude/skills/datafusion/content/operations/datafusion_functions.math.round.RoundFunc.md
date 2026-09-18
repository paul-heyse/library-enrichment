# `datafusion_functions::math::round::RoundFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.round.RoundFunc.json).

<a id="op-d7d52bfadb3a41427622eb44"></a>
## RoundFunc

`struct` · `datafusion_functions::math::round::RoundFunc` · datafusion-functions 55.1.0

```rust
struct RoundFunc
```

Source: `src/math/round.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6217b9c6d0bf69834d8eda"></a>
## default

`function` · `datafusion_functions::math::round::RoundFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [181, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/round.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93a5864cde36b1247223d7a9"></a>
## documentation

`function` · `datafusion_functions::math::round::RoundFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:471`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e2dcd6a427c0e016860f63c"></a>
## eq

`function` · `datafusion_functions::math::round::RoundFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &RoundFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 17], "end": [172, 26], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/round.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36f9f4d35627788c0724e398"></a>
## fmt

`function` · `datafusion_functions::math::round::RoundFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 10], "end": [172, 15], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/round.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e249d957aeab1dd0ea192bc3"></a>
## hash

`function` · `datafusion_functions::math::round::RoundFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 32], "end": [172, 36], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/round.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d8f062f7a1cad4bb0c12b00"></a>
## invoke_with_args

`function` · `datafusion_functions::math::round::RoundFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6ed8fb3c7459ec56709a3f4"></a>
## is_strict

`function` · `datafusion_functions::math::round::RoundFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77beb1af4c105d0b493fe8d"></a>
## name

`function` · `datafusion_functions::math::round::RoundFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac327d6347e2d6e831ea7ca9"></a>
## new

`function` · `datafusion_functions::math::round::RoundFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 1], "end": [223, 2], "filename": "src/math/round.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/round.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0b7f8d91e70ab3c632c02d9"></a>
## output_ordering

`function` · `datafusion_functions::math::round::RoundFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42bba08ebf9cb0bb9b8ff2ab"></a>
## return_field_from_args

`function` · `datafusion_functions::math::round::RoundFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c570c756f1d66f88dd3288fa"></a>
## return_type

`function` · `datafusion_functions::math::round::RoundFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7b953e0182274ed0bc3ef20"></a>
## signature

`function` · `datafusion_functions::math::round::RoundFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::round::RoundFunc", "path": "RoundFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [225, 1], "end": [474, 2], "filename": "src/math/round.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/round.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
