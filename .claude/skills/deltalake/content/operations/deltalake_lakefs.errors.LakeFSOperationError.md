# `deltalake_lakefs::errors::LakeFSOperationError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.errors.LakeFSOperationError.json).

<a id="op-7c701ec998b2347ea7022c31"></a>
## LakeFSOperationError

`enum` · `deltalake_lakefs::errors::LakeFSOperationError` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum LakeFSOperationError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L23).

Source: `crates/lakefs/src/errors.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-add7e4691b0c2bd130228bd1"></a>
## CommitFailed

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::CommitFailed` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CommitFailed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L34).

Source: `crates/lakefs/src/errors.rs:34`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS commit has failed

<a id="op-a712df95fef3db6189dd5778"></a>
## CreateBranchFailed

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::CreateBranchFailed` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
CreateBranchFailed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L42).

Source: `crates/lakefs/src/errors.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS create branch has failed

<a id="op-087fe4ae4843194079a3d4d4"></a>
## DeleteBranchFailed

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::DeleteBranchFailed` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DeleteBranchFailed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L46).

Source: `crates/lakefs/src/errors.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS delete branch has failed

<a id="op-14ebb30e06ab9d5e7315988b"></a>
## HttpRequestFailed

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::HttpRequestFailed` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
HttpRequestFailed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L26).

Source: `crates/lakefs/src/errors.rs:26`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Failed to send http request to LakeFS

<a id="op-997d52654d4691220f2d6df4"></a>
## MergeFailed

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::MergeFailed` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
MergeFailed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L38).

Source: `crates/lakefs/src/errors.rs:38`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS merge has failed

<a id="op-fc9b738ed3c14567f15331ab"></a>
## TransactionIdNotFound

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::TransactionIdNotFound` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TransactionIdNotFound
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L50).

Source: `crates/lakefs/src/errors.rs:50`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS delete branch has failed

<a id="op-90b9e30053f03e8fac386ec8"></a>
## UnauthorizedAction

`variant` · `deltalake_lakefs::errors::LakeFSOperationError::UnauthorizedAction` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnauthorizedAction
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L30).

Source: `crates/lakefs/src/errors.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Missing authentication in LakeFS

<a id="op-c306f2308397ed9d830c9b4f"></a>
## fmt

`function` · `deltalake_lakefs::errors::LakeFSOperationError::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::errors::LakeFSOperationError", "path": "LakeFSOperationError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 26], "filename": "crates/lakefs/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/lakefs/src/errors.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6fff8740d22a49d4d8082fb"></a>
## fmt

`function` · `deltalake_lakefs::errors::LakeFSOperationError::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::errors::LakeFSOperationError", "path": "LakeFSOperationError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 28], "end": [22, 33], "filename": "crates/lakefs/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/errors.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6abfc711c9ac88dcc3e2dc5"></a>
## source

`function` · `deltalake_lakefs::errors::LakeFSOperationError::source` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::errors::LakeFSOperationError", "path": "LakeFSOperationError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 26], "filename": "crates/lakefs/src/errors.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/lakefs/src/errors.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
