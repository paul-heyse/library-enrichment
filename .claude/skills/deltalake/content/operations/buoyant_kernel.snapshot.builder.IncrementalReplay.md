# `buoyant_kernel::snapshot::builder::IncrementalReplay`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.builder.IncrementalReplay.json).

<a id="op-b10a0852126775f2b57b627c"></a>
## IncrementalReplay

`enum` · `buoyant_kernel::snapshot::builder::IncrementalReplay` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum IncrementalReplay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L63).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Controls whether kernel replays commits to advance a stale base CRC (the existing snapshot's
in-memory CRC, or an on-disk CRC) to the target snapshot version on load. A CRC already at the
target version is always used regardless of this setting; this only bounds the cost of
advancing a *stale* CRC.

A resolved CRC gives the snapshot precomputed file statistics (file count and sizes, useful
for query optimization and for writers producing a post-commit CRC) along with domain metadata
and set transactions (useful for writers), all without extra log replay.

<a id="op-851d89740356352b3ce74f0c"></a>
## Disabled

`variant` · `buoyant_kernel::snapshot::builder::IncrementalReplay::Disabled` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Disabled
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L66).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:66`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Never advance a stale CRC; fall back to normal log replay. `UpToCommits(0)` is equivalent.

<a id="op-a8d535877e4623c8a01a067b"></a>
## Unlimited

`variant` · `buoyant_kernel::snapshot::builder::IncrementalReplay::Unlimited` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unlimited
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L71).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:71`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Advance regardless of how stale the CRC is.

<a id="op-15665f8dbb33ea30edd3f8aa"></a>
## UpToCommits

`variant` · `buoyant_kernel::snapshot::builder::IncrementalReplay::UpToCommits` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UpToCommits
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L69).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:69`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Advance only when the CRC is within `n` commits of the target version, i.e.
`target_version - crc_version <= n`.

<a id="op-7317c0395511a0f13a0f24b6"></a>
## clone

`function` · `buoyant_kernel::snapshot::builder::IncrementalReplay::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> IncrementalReplay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::IncrementalReplay", "path": "IncrementalReplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb17dfbd8a6d270d397470d3"></a>
## default

`function` · `buoyant_kernel::snapshot::builder::IncrementalReplay::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> IncrementalReplay
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::IncrementalReplay", "path": "IncrementalReplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 30], "end": [62, 37], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0976567e8cfcf83f4efda29a"></a>
## eq

`function` · `buoyant_kernel::snapshot::builder::IncrementalReplay::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &IncrementalReplay) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::IncrementalReplay", "path": "IncrementalReplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 39], "end": [62, 48], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dde02f4429f0669270c45a1"></a>
## fmt

`function` · `buoyant_kernel::snapshot::builder::IncrementalReplay::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/builder.rs#L62).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::builder::IncrementalReplay", "path": "IncrementalReplay"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/builder.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
