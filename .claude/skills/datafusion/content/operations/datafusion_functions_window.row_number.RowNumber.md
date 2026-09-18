# `datafusion_functions_window::row_number::RowNumber`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.row_number.RowNumber.json).

<a id="op-23e4fe67888ba149df4f4d72"></a>
## RowNumber

`struct` · `datafusion_functions_window::row_number::RowNumber` · datafusion-functions-window 55.1.0

```rust
struct RowNumber
```

Source: `src/row_number.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

row_number expression

<a id="op-f003761f8f2afb093d06bf63"></a>
## default

`function` · `datafusion_functions_window::row_number::RowNumber::default` · datafusion-functions-window 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [90, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/row_number.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de137af34c662b87b800faca"></a>
## documentation

`function` · `datafusion_functions_window::row_number::RowNumber::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d4dd8f2dcf20ef2aa1e9d63"></a>
## eq

`function` · `datafusion_functions_window::row_number::RowNumber::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &RowNumber) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 26], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/row_number.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ad5f697027e6d1e482b4351"></a>
## field

`function` · `datafusion_functions_window::row_number::RowNumber::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e7db611aa902830380934f0"></a>
## fmt

`function` · `datafusion_functions_window::row_number::RowNumber::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/row_number.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09d94f73f05058b5fee55a3f"></a>
## hash

`function` · `datafusion_functions_window::row_number::RowNumber::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 32], "end": [72, 36], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/row_number.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ac350ee00a0d8644abd07fa"></a>
## limit_effect

`function` · `datafusion_functions_window::row_number::RowNumber::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e6f1b0f849271745a3a0f53"></a>
## name

`function` · `datafusion_functions_window::row_number::RowNumber::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-367ce6842ca27c891c4589a2"></a>
## new

`function` · `datafusion_functions_window::row_number::RowNumber::new` · datafusion-functions-window 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [84, 2], "filename": "src/row_number.rs"}, "trait": null, "trait_path": null}`

Source: `src/row_number.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a new `row_number` function

<a id="op-60e74fe0e269de055d7319ea"></a>
## partition_evaluator

`function` · `datafusion_functions_window::row_number::RowNumber::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd0098a28d0c17a473d6b9ed"></a>
## signature

`function` · `datafusion_functions_window::row_number::RowNumber::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07cf896510bd9c32d1f9d398"></a>
## sort_options

`function` · `datafusion_functions_window::row_number::RowNumber::sort_options` · datafusion-functions-window 55.1.0

```rust
fn sort_options(&self) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::row_number::RowNumber", "path": "RowNumber"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [126, 2], "filename": "src/row_number.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/row_number.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
