# `buoyant_kernel::committer::commit_types::CommitResponse`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.committer.commit_types.CommitResponse.json).

<a id="op-3c4e22a7235055ae81fd6e9c"></a>
## CommitResponse

`enum` · `buoyant_kernel::committer::commit_types::CommitResponse` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitResponse
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L328).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:328`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

`CommitResponse` is the result of committing a transaction via a catalog. The committer uses
this type to indicate whether or not the commit was successful or conflicted. The kernel then
transforms the associated [`Transaction`] into the appropriate state.

If the commit was successful, the committer returns `CommitResponse::Committed` with the commit
version set. If the commit conflicted (e.g. another writer committed to the same version), the
Committer returns `CommitResponse::Conflict` with the version that was attempted.

[`Transaction`]: crate::transaction::Transaction

<a id="op-a19e5ce936654d29aaababd9"></a>
## Committed

`variant` · `buoyant_kernel::committer::commit_types::CommitResponse::Committed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Committed
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L329).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:329`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c889dfff243e0711aa0bae42"></a>
## Conflict

`variant` · `buoyant_kernel::committer::commit_types::CommitResponse::Conflict` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Conflict
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L330).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:330`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc8e349db7675fef23f2cbf1"></a>
## fmt

`function` · `buoyant_kernel::committer::commit_types::CommitResponse::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/committer/commit_types.rs#L327).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::committer::commit_types::CommitResponse", "path": "CommitResponse"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [327, 10], "end": [327, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/committer/commit_types.rs:327`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
