# `datafusion_physical_plan::coalesce::PushBatchStatus`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coalesce.PushBatchStatus.json).

<a id="op-87a6520c4e83cf094817a302"></a>
## PushBatchStatus

`enum` · `datafusion_physical_plan::coalesce::PushBatchStatus` · datafusion-physical-plan 55.1.0

```rust
enum PushBatchStatus
```

Source: `src/coalesce/mod.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Status returned by [`LimitedBatchCoalescer::push_batch`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-a99d2c42cb02132ba137f798)

<a id="op-99f58d86ebf0ee9c894a7a47"></a>
## Continue

`variant` · `datafusion_physical_plan::coalesce::PushBatchStatus::Continue` · datafusion-physical-plan 55.1.0

```rust
Continue
```

Source: `src/coalesce/mod.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The limit has **not** been reached, and more batches can be pushed

<a id="op-6dd7d3b1f7b8bcff05ad63d6"></a>
## LimitReached

`variant` · `datafusion_physical_plan::coalesce::PushBatchStatus::LimitReached` · datafusion-physical-plan 55.1.0

```rust
LimitReached
```

Source: `src/coalesce/mod.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The limit **has** been reached after processing this batch
The caller should call [`LimitedBatchCoalescer::finish`](../operations/datafusion_physical_plan.coalesce.LimitedBatchCoalescer.md#op-e7ce23d8851b2fd194c5d485)
to flush any buffered rows and stop pushing more batches.

<a id="op-ea4f511b5b9647b29607cb42"></a>
## clone

`function` · `datafusion_physical_plan::coalesce::PushBatchStatus::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> PushBatchStatus
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::PushBatchStatus", "path": "PushBatchStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/coalesce/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/coalesce/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0814f38337eff9964b45bbf"></a>
## eq

`function` · `datafusion_physical_plan::coalesce::PushBatchStatus::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &PushBatchStatus) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::PushBatchStatus", "path": "PushBatchStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 30], "end": [39, 39], "filename": "src/coalesce/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/coalesce/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b96813ad5d9f8173cd801033"></a>
## fmt

`function` · `datafusion_physical_plan::coalesce::PushBatchStatus::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::coalesce::PushBatchStatus", "path": "PushBatchStatus"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/coalesce/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/coalesce/mod.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
