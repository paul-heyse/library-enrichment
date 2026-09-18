# `datafusion_functions_window::rank::RankState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window.rank.RankState.json).

<a id="op-201d35f40d45549908c16804"></a>
## RankState

`struct` · `datafusion_functions_window::rank::RankState` · datafusion-functions-window 55.1.0

```rust
struct RankState
```

Source: `src/rank.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

State for the RANK(rank) built-in window function.

<a id="op-231752c04c5b76413201c7da"></a>
## clone

`function` · `datafusion_functions_window::rank::RankState::clone` · datafusion-functions-window 55.1.0

```rust
fn clone(&self) -> RankState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::RankState", "path": "RankState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 17], "end": [252, 22], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/rank.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e2d69e6f71b548affbdb89d"></a>
## current_group_count

`struct_field` · `datafusion_functions_window::rank::RankState::current_group_count` · datafusion-functions-window 55.1.0

```rust
current_group_count: usize
```

Source: `src/rank.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Keep the number of entries in current rank

<a id="op-0018f6b576813cc835391e5c"></a>
## default

`function` · `datafusion_functions_window::rank::RankState::default` · datafusion-functions-window 55.1.0

```rust
fn default() -> RankState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::RankState", "path": "RankState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 24], "end": [252, 31], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/rank.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c7d3ae8658a6512aa4f11f4"></a>
## fmt

`function` · `datafusion_functions_window::rank::RankState::fmt` · datafusion-functions-window 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_window::rank::RankState", "path": "RankState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [252, 10], "end": [252, 15], "filename": "src/rank.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/rank.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-845248af01c6ab28fc2ff13d"></a>
## last_rank_boundary

`struct_field` · `datafusion_functions_window::rank::RankState::last_rank_boundary` · datafusion-functions-window 55.1.0

```rust
last_rank_boundary: usize
```

Source: `src/rank.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

The index where last_rank_boundary is started

<a id="op-cd84ae9e3bdcd3fd07a97a60"></a>
## last_rank_data

`struct_field` · `datafusion_functions_window::rank::RankState::last_rank_data` · datafusion-functions-window 55.1.0

```rust
last_rank_data: Option<Vec<datafusion_common::ScalarValue>>
```

Source: `src/rank.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

The last values for rank as these values change, we increase n_rank

<a id="op-593814c5e10db895177c2f6e"></a>
## n_rank

`struct_field` · `datafusion_functions_window::rank::RankState::n_rank` · datafusion-functions-window 55.1.0

```rust
n_rank: usize
```

Source: `src/rank.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window/55.1.0/json).

Rank number kept from the start
