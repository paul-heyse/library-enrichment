# `deltalake_core::protocol::DeltaOperation::Merge`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.Merge.json).

<a id="op-c7d9520db494f7991a769cd5"></a>
## matched_predicates

`struct_field` · `deltalake_core::protocol::DeltaOperation::Merge::matched_predicates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
matched_predicates: Vec<MergePredicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L305).

Source: `crates/core/src/protocol/mod.rs:305`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Match operations performed

<a id="op-2e7085edfec54285fbfdd871"></a>
## merge_predicate

`struct_field` · `deltalake_core::protocol::DeltaOperation::Merge::merge_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
merge_predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L302).

Source: `crates/core/src/protocol/mod.rs:302`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The original merge predicate

<a id="op-7cac94496c492da2f3535adc"></a>
## not_matched_by_source_predicates

`struct_field` · `deltalake_core::protocol::DeltaOperation::Merge::not_matched_by_source_predicates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
not_matched_by_source_predicates: Vec<MergePredicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L311).

Source: `crates/core/src/protocol/mod.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Not Match by Source operations performed

<a id="op-e62501e90d411bdd6e4d6d7f"></a>
## not_matched_predicates

`struct_field` · `deltalake_core::protocol::DeltaOperation::Merge::not_matched_predicates` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
not_matched_predicates: Vec<MergePredicate>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L308).

Source: `crates/core/src/protocol/mod.rs:308`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Not Match operations performed

<a id="op-e6284454c9e62b2789682d6b"></a>
## predicate

`struct_field` · `deltalake_core::protocol::DeltaOperation::Merge::predicate` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
predicate: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L299).

Source: `crates/core/src/protocol/mod.rs:299`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Cleaned merge predicate for conflict checks
