# `buoyant_kernel::transaction::CommitResult`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transaction.CommitResult.json).

<a id="op-ef52c5b0205d38fde45a2f8f"></a>
## CommitResult

`enum` · `buoyant_kernel::transaction::CommitResult` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitResult<S = ExistingTable>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1619).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1619`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The result of attempting to commit this transaction. If the commit was
successful/conflicted/retryable, the result is Ok(CommitResult), otherwise, if a nonrecoverable
error occurred, the result is Err(Error).

The commit result can be one of the following:
- [CommittedTransaction](../operations/buoyant_kernel.transaction.CommittedTransaction.md#op-bac54c281be967a3d8ea6377): the transaction was successfully committed. [PostCommitStats](../operations/buoyant_kernel.transaction.PostCommitStats.md#op-f3e0479e1d3f22611f0bcde0) and in
  the future a post-commit snapshot can be obtained from the committed transaction.
- [ConflictedTransaction](../operations/buoyant_kernel.transaction.ConflictedTransaction.md#op-c360e9ca80998f2b077a9ebd): the transaction conflicted with an existing version. This transcation
  must be rebased before retrying. (currently no rebase APIs exist, caller must create new txn)
- [RetryableTransaction](../operations/buoyant_kernel.transaction.RetryableTransaction.md#op-11bb850b110c291cb5eec2b7): an IO (retryable) error occurred during the commit. This transaction
  can be retried without rebasing.

<a id="op-5dda199748bef6244d10b129"></a>
## CommittedTransaction

`variant` · `buoyant_kernel::transaction::CommitResult::CommittedTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CommittedTransaction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1621).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1621`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The transaction was successfully committed.

<a id="op-0e420780b4f5270bfb45b714"></a>
## ConflictedTransaction

`variant` · `buoyant_kernel::transaction::CommitResult::ConflictedTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ConflictedTransaction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1628).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1628`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

This transaction conflicted with an existing version (see
[ConflictedTransaction::conflict_version](../operations/buoyant_kernel.transaction.ConflictedTransaction.md#op-e00d78628ce4c43282fbe36d)). The transaction
is returned so the caller can resolve the conflict (along with the version which
conflicted).

<a id="op-00b661d364ecd73ecd68aeff"></a>
## RetryableTransaction

`variant` · `buoyant_kernel::transaction::CommitResult::RetryableTransaction` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RetryableTransaction
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1630).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1630`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

An IO (retryable) error occurred during the commit.

<a id="op-73ed259b0d658ee07918edfa"></a>
## fmt

`function` · `buoyant_kernel::transaction::CommitResult::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1617).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::CommitResult", "path": "CommitResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1617, 10], "end": [1617, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1617`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-671c23b9a9e170ee98c23f75"></a>
## is_committed

`function` · `buoyant_kernel::transaction::CommitResult::is_committed` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_committed(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transaction/mod.rs#L1635).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "buoyant_kernel::transaction::CommitResult", "path": "CommitResult"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1633, 1], "end": [1638, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transaction/mod.rs:1635`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if the commit was successful.
