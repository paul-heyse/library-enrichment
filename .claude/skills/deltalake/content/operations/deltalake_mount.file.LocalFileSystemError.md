# `deltalake_mount::file::LocalFileSystemError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_mount.file.LocalFileSystemError.json).

<a id="op-feb4e1993657391f172bc561"></a>
## LocalFileSystemError

`enum` · `deltalake_mount::file::LocalFileSystemError` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
enum LocalFileSystemError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L21).

Source: `crates/mount/src/file.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error raised by storage lock client

<a id="op-8e663f2d77773522eacc9663"></a>
## AlreadyExists

`variant` · `deltalake_mount::file::LocalFileSystemError::AlreadyExists` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
AlreadyExists
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L24).

Source: `crates/mount/src/file.rs:24`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Object exists already at path

<a id="op-5d8ef03b26a8f1b577450b37"></a>
## Generic

`variant` · `deltalake_mount::file::LocalFileSystemError::Generic` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
Generic
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L60).

Source: `crates/mount/src/file.rs:60`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Generic catch-all error for this store

<a id="op-95ef685efd3da01ec115acdf"></a>
## InvalidArgument

`variant` · `deltalake_mount::file::LocalFileSystemError::InvalidArgument` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
InvalidArgument
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L42).

Source: `crates/mount/src/file.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Invalid argument sent to OS call

<a id="op-9e7133aa49b1a9c58cac01a5"></a>
## NotFound

`variant` · `deltalake_mount::file::LocalFileSystemError::NotFound` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
NotFound
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L33).

Source: `crates/mount/src/file.rs:33`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Object not found at the given path

<a id="op-a1691a985388391107b4b692"></a>
## NullError

`variant` · `deltalake_mount::file::LocalFileSystemError::NullError` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
NullError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L51).

Source: `crates/mount/src/file.rs:51`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Null error for path for FFI

<a id="op-a15ae4c2ade083c4a26e2769"></a>
## Tokio

`variant` · `deltalake_mount::file::LocalFileSystemError::Tokio` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
Tokio
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L69).

Source: `crates/mount/src/file.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Errors from the Tokio runtime

<a id="op-4a53239b96f18c9c02665d19"></a>
## fmt

`function` · `deltalake_mount::file::LocalFileSystemError::fmt` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::file::LocalFileSystemError", "path": "LocalFileSystemError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 28], "end": [19, 33], "filename": "crates/mount/src/file.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/mount/src/file.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-698054714fae559188cfe66e"></a>
## fmt

`function` · `deltalake_mount::file::LocalFileSystemError::fmt` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::file::LocalFileSystemError", "path": "LocalFileSystemError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 10], "end": [19, 26], "filename": "crates/mount/src/file.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/mount/src/file.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20cee793e149ea04cc78870c"></a>
## source

`function` · `deltalake_mount::file::LocalFileSystemError::source` · deltalake-mount 1.0.0+58f07cd6

Access: **returned_inferred**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/mount/src/file.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_mount::file::LocalFileSystemError", "path": "LocalFileSystemError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [19, 10], "end": [19, 26], "filename": "crates/mount/src/file.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/mount/src/file.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
