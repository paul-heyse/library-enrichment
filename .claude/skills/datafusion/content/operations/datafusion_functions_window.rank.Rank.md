# `datafusion_functions_window::rank::Rank`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.Rank.json).

<a id="op-b0a9af7e1228beaf89985b0d"></a>
## Rank

`struct` · `datafusion_functions_window::rank::Rank` · datafusion-functions-window 55.1.0

```rust
struct Rank
```

Source: `src/rank.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Rank calculates the rank in the window function with order by

<a id="op-f759cfadd06ec33e68e91770"></a>
## basic

`function` · `datafusion_functions_window::rank::Rank::basic` · datafusion-functions-window 55.1.0

```rust
fn basic() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [99, 2], "filename": "src/rank.rs"}, "trait": null, "trait_path": null}`

Source: `src/rank.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a `rank` window function

<a id="op-52c0907680902986650bf48c"></a>
## dense_rank

`function` · `datafusion_functions_window::rank::Rank::dense_rank` · datafusion-functions-window 55.1.0

```rust
fn dense_rank() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [99, 2], "filename": "src/rank.rs"}, "trait": null, "trait_path": null}`

Source: `src/rank.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a `dense_rank` window function

<a id="op-8a5d86e77310389f3c7e946c"></a>
## documentation

`function` · `datafusion_functions_window::rank::Rank::documentation` · datafusion-functions-window 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7707bd80200e34910f5b9527"></a>
## eq

`function` · `datafusion_functions_window::rank::Rank::eq` · datafusion-functions-window 55.1.0

```rust
fn eq(&self, other: &Rank) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 26], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/rank.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-606422872f84ecd121814bb0"></a>
## field

`function` · `datafusion_functions_window::rank::Rank::field` · datafusion-functions-window 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f653f1386078ce8e75242a0"></a>
## fmt

`function` · `datafusion_functions_window::rank::Rank::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/rank.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b61cd2d0c3492672ea97872"></a>
## hash

`function` · `datafusion_functions_window::rank::Rank::hash` · datafusion-functions-window 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 32], "end": [68, 36], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/rank.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac697474a30d77ae8c52cca3"></a>
## limit_effect

`function` · `datafusion_functions_window::rank::Rank::limit_effect` · datafusion-functions-window 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53a55d7dd62106530489a931"></a>
## name

`function` · `datafusion_functions_window::rank::Rank::name` · datafusion-functions-window 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:199`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a35bcf92aaf1b06464c0f337"></a>
## new

`function` · `datafusion_functions_window::rank::Rank::new` · datafusion-functions-window 55.1.0

```rust
fn new(name: String, rank_type: RankType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [99, 2], "filename": "src/rank.rs"}, "trait": null, "trait_path": null}`

Source: `src/rank.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a new `rank` function with the specified name and rank type

<a id="op-91be87c90daad47594bdb3c1"></a>
## partition_evaluator

`function` · `datafusion_functions_window::rank::Rank::partition_evaluator` · datafusion-functions-window 55.1.0

```rust
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd010873f9e544ac0a1e542b"></a>
## percent_rank

`function` · `datafusion_functions_window::rank::Rank::percent_rank` · datafusion-functions-window 55.1.0

```rust
fn percent_rank() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [99, 2], "filename": "src/rank.rs"}, "trait": null, "trait_path": null}`

Source: `src/rank.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Create a `percent_rank` window function

<a id="op-876416cbc441d5665bc8e3a0"></a>
## signature

`function` · `datafusion_functions_window::rank::Rank::signature` · datafusion-functions-window 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00fc5c49611f95b5c9855137"></a>
## sort_options

`function` · `datafusion_functions_window::rank::Rank::sort_options` · datafusion-functions-window 55.1.0

```rust
fn sort_options(&self) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::Rank", "path": "Rank"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [249, 2], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/rank.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
