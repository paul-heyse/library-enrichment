# `datafusion_functions_window::cume_dist::CumeDist`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.cume_dist.CumeDist.json).

<a id="op-22ef111a4cda582ddaed410d"></a>
## CumeDist

`struct` · `datafusion_functions_window::cume_dist::CumeDist` · datafusion-functions-window 55.1.0

```rust
struct CumeDist
```

Source: `src/cume_dist.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

CumeDist calculates the cume_dist in the window function with order by

<a id="op-59bf08daa89f797e965c22c9"></a>
## default

`function` · `datafusion_functions_window::cume_dist::CumeDist::default` · datafusion-functions-window 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 1], "end": [84, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/cume_dist.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55f894f9558b4524cbf639dd"></a>
## documentation

`function` · `datafusion_functions_window::cume_dist::CumeDist::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-159e3ec886dba96a38daba4f"></a>
## eq

`function` · `datafusion_functions_window::cume_dist::CumeDist::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &CumeDist) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 26], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/cume_dist.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1bb070419d4ea7847699176"></a>
## field

`function` · `datafusion_functions_window::cume_dist::CumeDist::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea4aef0bdd68c1f8dd9360fe"></a>
## fmt

`function` · `datafusion_functions_window::cume_dist::CumeDist::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cume_dist.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13843bbe0e8d41e99ebbfe4c"></a>
## hash

`function` · `datafusion_functions_window::cume_dist::CumeDist::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 32], "end": [67, 36], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/cume_dist.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1d6759c4905ba287b1b17dd"></a>
## limit_effect

`function` · `datafusion_functions_window::cume_dist::CumeDist::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db8e7283122bf5a5cccbc814"></a>
## name

`function` · `datafusion_functions_window::cume_dist::CumeDist::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f8c5a831d2c25d051cb5b23"></a>
## new

`function` · `datafusion_functions_window::cume_dist::CumeDist::new` · datafusion-functions-window 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [78, 2], "filename": "src/cume_dist.rs"}, "trait": null, "trait_path": null}`

Source: `src/cume_dist.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb6799b43ac16cf95fe16d2c"></a>
## partition_evaluator

`function` · `datafusion_functions_window::cume_dist::CumeDist::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be4e44109fc43e47860d4edc"></a>
## signature

`function` · `datafusion_functions_window::cume_dist::CumeDist::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::cume_dist::CumeDist", "path": "CumeDist"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [113, 2], "filename": "src/cume_dist.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/cume_dist.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
