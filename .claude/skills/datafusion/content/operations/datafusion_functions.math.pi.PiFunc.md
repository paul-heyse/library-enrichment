# `datafusion_functions::math::pi::PiFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.pi.PiFunc.json).

<a id="op-f4bacc3a3be245d9256b8c20"></a>
## PiFunc

`struct` · `datafusion_functions::math::pi::PiFunc` · datafusion-functions 55.1.0

```rust
struct PiFunc
```

Source: `src/math/pi.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-191e536825c1827cb31ca2f0"></a>
## default

`function` · `datafusion_functions::math::pi::PiFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [42, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/pi.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19cda3e29f43af956028f2b5"></a>
## documentation

`function` · `datafusion_functions::math::pi::PiFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb1b1955aec58c70cc84f714"></a>
## eq

`function` · `datafusion_functions::math::pi::PiFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &PiFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 17], "end": [33, 26], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/pi.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00b2382e85311a46040990d1"></a>
## fmt

`function` · `datafusion_functions::math::pi::PiFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 15], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/pi.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769a84577d5c0fafba26a8b6"></a>
## hash

`function` · `datafusion_functions::math::pi::PiFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 32], "end": [33, 36], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/pi.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1df4b7e996a7c9a7fbed4dea"></a>
## invoke_with_args

`function` · `datafusion_functions::math::pi::PiFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f09e9d71a5de06c3bd74afd"></a>
## name

`function` · `datafusion_functions::math::pi::PiFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80bffc9e769905eb16e5e1eb"></a>
## new

`function` · `datafusion_functions::math::pi::PiFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [50, 2], "filename": "src/math/pi.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/pi.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57c728639352be46b9220033"></a>
## output_ordering

`function` · `datafusion_functions::math::pi::PiFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, _input: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25cbcc8385d8d2bf8193bb3a"></a>
## return_type

`function` · `datafusion_functions::math::pi::PiFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113c9f26d9f8177ab7648255"></a>
## signature

`function` · `datafusion_functions::math::pi::PiFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::pi::PiFunc", "path": "PiFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [84, 2], "filename": "src/math/pi.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/pi.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
