# `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.hash_join.partitioned_hash_eval.SeededRandomState.json).

<a id="op-8b3607e24780dd522de7e632"></a>
## SeededRandomState

`struct` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState` · datafusion-physical-plan 55.1.0

```rust
struct SeededRandomState
```

Source: `src/joins/hash_join/partitioned_hash_eval.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

RandomState wrapper that preserves the seed used to create it.

This is needed because `RandomState` doesn't expose its seed after creation,
but we need them for serialization (e.g., protobuf serde).

<a id="op-f4f06e881f2f6ccb29e46174"></a>
## clone

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SeededRandomState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState", "path": "SeededRandomState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 10], "end": [43, 15], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-726e2efc376211eafd67e751"></a>
## fmt

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState", "path": "SeededRandomState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 17], "end": [43, 22], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9a5588348728afd3dd4c9da"></a>
## random_state

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState::random_state` · datafusion-physical-plan 55.1.0

```rust
fn random_state(&self) -> &RandomState
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState", "path": "SeededRandomState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [67, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the inner RandomState.

<a id="op-e8609ffbc19f3c049301662f"></a>
## seed

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState::seed` · datafusion-physical-plan 55.1.0

```rust
fn seed(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState", "path": "SeededRandomState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [67, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the seed used to create this RandomState.

<a id="op-ac095a213a21f6d7fd2e29f7"></a>
## with_seed

`function` · `datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState::with_seed` · datafusion-physical-plan 55.1.0

```rust
const fn with_seed(k: u64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::partitioned_hash_eval::SeededRandomState", "path": "SeededRandomState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [67, 2], "filename": "src/joins/hash_join/partitioned_hash_eval.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/partitioned_hash_eval.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new SeededRandomState with the given seed.
