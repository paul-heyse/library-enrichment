# `datafusion_functions_window::lead_lag::WindowShift`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.lead_lag.WindowShift.json).

<a id="op-d9911c2573d4fe29b74a0ebc"></a>
## WindowShift

`struct` · `datafusion_functions_window::lead_lag::WindowShift` · datafusion-functions-window 55.1.0

```rust
struct WindowShift
```

Source: `src/lead_lag.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

window shift expression

<a id="op-802318b1f8322165c3f9a424"></a>
## documentation

`function` · `datafusion_functions_window::lead_lag::WindowShift::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:306`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7ad7134363b9cee97028f53"></a>
## eq

`function` · `datafusion_functions_window::lead_lag::WindowShift::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &WindowShift) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 17], "end": [127, 26], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lead_lag.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64dba94b46f0aeffa5d208aa"></a>
## expressions

`function` · `datafusion_functions_window::lead_lag::WindowShift::expressions` · datafusion-functions-window 55.1.0

```rust
fn expressions(&self, expr_args: ExpressionArgs<'_>) -> Vec<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Handles the case where `NULL` expression is passed as an
argument to `lead`/`lag`. The type is refined depending
on the default value argument.

For more details see: <https://github.com/apache/datafusion/issues/12717>

<a id="op-cec60c198f2a71800e85c24f"></a>
## field

`function` · `datafusion_functions_window::lead_lag::WindowShift::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47717c8f95e0f81a2b69f67b"></a>
## fmt

`function` · `datafusion_functions_window::lead_lag::WindowShift::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 10], "end": [127, 15], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lead_lag.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89045d8a6ce370daebe0bdb0"></a>
## hash

`function` · `datafusion_functions_window::lead_lag::WindowShift::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 32], "end": [127, 36], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lead_lag.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b7cdbcbb0ee16989cccb9c6"></a>
## kind

`function` · `datafusion_functions_window::lead_lag::WindowShift::kind` · datafusion-functions-window 55.1.0

```rust
fn kind(&self) -> &WindowShiftKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [165, 2], "filename": "src/lead_lag.rs"}, "trait": null, "trait_path": null}`

Source: `src/lead_lag.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65020821614b71ae474756ae"></a>
## lag

`function` · `datafusion_functions_window::lead_lag::WindowShift::lag` · datafusion-functions-window 55.1.0

```rust
fn lag() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [165, 2], "filename": "src/lead_lag.rs"}, "trait": null, "trait_path": null}`

Source: `src/lead_lag.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9861911e78469b3e74e85ca4"></a>
## lead

`function` · `datafusion_functions_window::lead_lag::WindowShift::lead` · datafusion-functions-window 55.1.0

```rust
fn lead() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [133, 1], "end": [165, 2], "filename": "src/lead_lag.rs"}, "trait": null, "trait_path": null}`

Source: `src/lead_lag.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc7f2876597a95638eeb0840"></a>
## limit_effect

`function` · `datafusion_functions_window::lead_lag::WindowShift::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90cd36b6c5c03fa38c658e12"></a>
## name

`function` · `datafusion_functions_window::lead_lag::WindowShift::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07100bc307e708a03ffba335"></a>
## partition_evaluator

`function` · `datafusion_functions_window::lead_lag::WindowShift::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3a7ebe7b98faf2ec602948d"></a>
## reverse_expr

`function` · `datafusion_functions_window::lead_lag::WindowShift::reverse_expr` · datafusion-functions-window 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDWF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7033649366d77a035c204463"></a>
## signature

`function` · `datafusion_functions_window::lead_lag::WindowShift::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::lead_lag::WindowShift", "path": "WindowShift"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [331, 2], "filename": "src/lead_lag.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/lead_lag.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
