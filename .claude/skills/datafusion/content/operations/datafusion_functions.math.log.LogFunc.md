# `datafusion_functions::math::log::LogFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.log.LogFunc.json).

<a id="op-a60b7e17a08d102cbe100320"></a>
## LogFunc

`struct` · `datafusion_functions::math::log::LogFunc` · datafusion-functions 55.1.0

```rust
struct LogFunc
```

Source: `src/math/log.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db68f791a355f711f5d304c9"></a>
## default

`function` · `datafusion_functions::math::log::LogFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [70, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/log.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46bbd27a4df0c23ec431278a"></a>
## documentation

`function` · `datafusion_functions::math::log::LogFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09e4f9bfb8e8fed336433cc1"></a>
## eq

`function` · `datafusion_functions::math::log::LogFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &LogFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 17], "end": [61, 26], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/log.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b42233497c47f87de0393c6b"></a>
## fmt

`function` · `datafusion_functions::math::log::LogFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 10], "end": [61, 15], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/log.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f18e6c8f7baadc5c657e8bef"></a>
## hash

`function` · `datafusion_functions::math::log::LogFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 32], "end": [61, 36], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/log.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbd8d7dd7ac07fa720d1f0b9"></a>
## invoke_with_args

`function` · `datafusion_functions::math::log::LogFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83a50c0d49e18334734ddc01"></a>
## is_strict

`function` · `datafusion_functions::math::log::LogFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9783faf4a432a77f20d7afa4"></a>
## name

`function` · `datafusion_functions::math::log::LogFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-949d665aaa5223bd44e759d9"></a>
## new

`function` · `datafusion_functions::math::log::LogFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [101, 2], "filename": "src/math/log.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/log.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-090c13214c5d34569e136b40"></a>
## output_ordering

`function` · `datafusion_functions::math::log::LogFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b60962260257e6643127c65a"></a>
## return_type

`function` · `datafusion_functions::math::log::LogFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-419a52a0f3538b980870d942"></a>
## signature

`function` · `datafusion_functions::math::log::LogFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d4aa4d3d0d72c61f7841207"></a>
## simplify

`function` · `datafusion_functions::math::log::LogFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::log::LogFunc", "path": "LogFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [404, 2], "filename": "src/math/log.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/log.rs:319`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Simplify the `log` function by the relevant rules:
1. Log(a, 1) ===> 0
2. Log(a, Power(a, b)) ===> b
3. Log(a, a) ===> 1
