# `buoyant_kernel::crc::state::SetTransactionState`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.crc.state.SetTransactionState.json).

<a id="op-813a79731eedededb5c6ef13"></a>
## SetTransactionState

`enum` · `buoyant_kernel::crc::state::SetTransactionState` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum SetTransactionState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L122).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:122`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The completeness state of cached set transactions in a CRC.

<a id="op-4d9454ca36d745c8496b8109"></a>
## Complete

`variant` · `buoyant_kernel::crc::state::SetTransactionState::Complete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Complete
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L126).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:126`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The CRC file had the `setTransactions` field (possibly as an empty array). The map is
the full set of active transactions at this version; an `app_id` not in the map has no
active transaction.

<a id="op-21d958c0b74a1475587f01ce"></a>
## Partial

`variant` · `buoyant_kernel::crc::state::SetTransactionState::Partial` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Partial
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L133).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:133`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The CRC file did not have the `setTransactions` field. The map starts empty and gets
populated by incremental CRC replay over the JSON commits after the latest stale CRC.
Hits are authoritative (definitely present at this version); misses are NOT
(the app_id may have a transaction in older commits), so callers must do a further log
scan.

<a id="op-e83730da028127000bf5bf5e"></a>
## clone

`function` · `buoyant_kernel::crc::state::SetTransactionState::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> SetTransactionState
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::SetTransactionState", "path": "SetTransactionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 17], "end": [121, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a8ecb5073f10eb515a42ae3"></a>
## default

`function` · `buoyant_kernel::crc::state::SetTransactionState::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L137).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::SetTransactionState", "path": "SetTransactionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [140, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:137`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82acdf8e3d4892b56da59750"></a>
## eq

`function` · `buoyant_kernel::crc::state::SetTransactionState::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &SetTransactionState) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::SetTransactionState", "path": "SetTransactionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 24], "end": [121, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91af11186a4c0bdd3a0aa38b"></a>
## fmt

`function` · `buoyant_kernel::crc::state::SetTransactionState::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/crc/state.rs#L121).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::crc::state::SetTransactionState", "path": "SetTransactionState"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/crc/state.rs:121`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
