# `buoyant_kernel::snapshot::ChecksumWriteResult`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.snapshot.ChecksumWriteResult.json).

<a id="op-a840d53ca7bc5e28f0c0d108"></a>
## ChecksumWriteResult

`enum` · `buoyant_kernel::snapshot::ChecksumWriteResult` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ChecksumWriteResult
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L47).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:47`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Result of attempting to write a version checksum (CRC) file.

<a id="op-e9be56cc16083403d709fbc4"></a>
## AlreadyExists

`variant` · `buoyant_kernel::snapshot::ChecksumWriteResult::AlreadyExists` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AlreadyExists
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L50).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:50`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A CRC file already exists at this version. Per the Delta protocol, writers MUST NOT
overwrite existing version checksum files.

<a id="op-bd972c2d7b172900b2c431a1"></a>
## Written

`variant` · `buoyant_kernel::snapshot::ChecksumWriteResult::Written` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Written
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L52).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The CRC file was successfully written to storage.

<a id="op-693959763acde803a8a34fdc"></a>
## clone

`function` · `buoyant_kernel::snapshot::ChecksumWriteResult::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ChecksumWriteResult
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::ChecksumWriteResult", "path": "ChecksumWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6777c17ac67a9c2362f7c92b"></a>
## eq

`function` · `buoyant_kernel::snapshot::ChecksumWriteResult::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ChecksumWriteResult) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::ChecksumWriteResult", "path": "ChecksumWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 30], "end": [46, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2103e14506aa48c148040507"></a>
## fmt

`function` · `buoyant_kernel::snapshot::ChecksumWriteResult::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/snapshot/mod.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::snapshot::ChecksumWriteResult", "path": "ChecksumWriteResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/snapshot/mod.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
