# `deltalake_core::kernel::transaction::PostCommitHookProperties`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.transaction.PostCommitHookProperties.json).

<a id="op-1f5df88fff9f952d4d9585f7"></a>
## PostCommitHookProperties

`struct` · `deltalake_core::kernel::transaction::PostCommitHookProperties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct PostCommitHookProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L541).

Source: `crates/core/src/kernel/transaction/mod.rs:541`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Properties for post commit hook.

<a id="op-d1543a3f663f6ef6ef84aead"></a>
## clone

`function` · `deltalake_core::kernel::transaction::PostCommitHookProperties::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> PostCommitHookProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L539).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::PostCommitHookProperties", "path": "PostCommitHookProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [539, 10], "end": [539, 15], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/transaction/mod.rs:539`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3b84e7f11362e03e5f3a2cc"></a>
## fmt

`function` · `deltalake_core::kernel::transaction::PostCommitHookProperties::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L539).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::transaction::PostCommitHookProperties", "path": "PostCommitHookProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [539, 17], "end": [539, 22], "filename": "crates/core/src/kernel/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/transaction/mod.rs:539`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61be7158650e19b5e15fe1a1"></a>
## cleanup_expired_logs

`struct_field` · `deltalake_core::kernel::transaction::PostCommitHookProperties::cleanup_expired_logs` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
cleanup_expired_logs: Option<bool>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L544).

Source: `crates/core/src/kernel/transaction/mod.rs:544`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Override the EnableExpiredLogCleanUp setting, if None config setting is used

<a id="op-3625ac2ccf8d0cb8f08ba080"></a>
## create_checkpoint

`struct_field` · `deltalake_core::kernel::transaction::PostCommitHookProperties::create_checkpoint` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
create_checkpoint: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/transaction/mod.rs#L542).

Source: `crates/core/src/kernel/transaction/mod.rs:542`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
