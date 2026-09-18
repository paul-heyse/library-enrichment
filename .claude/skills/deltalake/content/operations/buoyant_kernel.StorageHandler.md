# `buoyant_kernel::StorageHandler`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.StorageHandler.json).

<a id="op-925ea854f2a3b385f850512b"></a>
## StorageHandler

`trait` · `buoyant_kernel::StorageHandler` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait StorageHandler: AsAny
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L598).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:598`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Provides file system related functionalities to Delta Kernel.

Delta Kernel uses this handler whenever it needs to access the underlying
file system where the Delta table is present. Connector implementation of
this trait can hide filesystem specific details from Delta Kernel.

<a id="op-d5e9b911d48ad913bbe5af4e"></a>
## copy_atomic

`function` · `buoyant_kernel::StorageHandler::copy_atomic` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn copy_atomic(&self, src: &Url, dest: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L624).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:624`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Copy a file atomically from source to destination. If the destination file already exists,
it must return Err(Error::FileAlreadyExists).

<a id="op-a93888071198e8c996c66977"></a>
## delete

`function` · `buoyant_kernel::StorageHandler::delete` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn delete(&self, path: &Url) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L641).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:641`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Delete the file at the given path.

This operation is idempotent: deleting a path that does not exist should return `Ok(())`.
For any other error, this must propagate the corresponding error.

<a id="op-f3841fae6720ff56eefbd5b5"></a>
## head

`function` · `buoyant_kernel::StorageHandler::head` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn head(&self, path: &Url) -> DeltaResult<FileMeta>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L635).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:635`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Perform a HEAD request for the given file at a Url, returning the file metadata.

If the file does not exist, this must return an `Err` with [`Error::FileNotFound`](../operations/buoyant_kernel.error.Error.md#op-f58dcfa1d09ba8ac4cbba335).

<a id="op-91a4be26c460491b26f14d54"></a>
## list_from

`function` · `buoyant_kernel::StorageHandler::list_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn list_from(&self, path: &Url) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<FileMeta>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L613).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:613`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively list files whose full path is lexicographically greater than (UTF-8 sorting)
the given `path`, restricted to descendants of `path`'s parent directory. The result must
be sorted by the full path (UTF-8 byte order).

The listing is **recursive**: files in nested subdirectories are included, not just files
directly under the parent. For example, listing from `dir/0001.json` may return
`dir/0002.json`, `dir/sub/0003.json`, and `dir/sub/nested/0004.json`, all interleaved
in lexicographic order.

The parent directory is derived from `path`:
- If `path` is directory-like (ends with `/`), the parent is `path` itself and the result
  contains all files at or below that directory.
- Otherwise, the parent is the directory containing `path`, and only files (at any depth
  under that parent) whose full path sorts strictly greater than `path` are returned.

<a id="op-6c6ca264811061df3b99e27e"></a>
## put

`function` · `buoyant_kernel::StorageHandler::put` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn put(&self, path: &Url, data: Bytes, overwrite: bool) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L630).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:630`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Write data to the specified path.

If `overwrite` is false and the file already exists, this must return
`Err(Error::FileAlreadyExists)`.

<a id="op-249028a0cd068b3df3727340"></a>
## read_files

`function` · `buoyant_kernel::StorageHandler::read_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_files(&self, files: Vec<FileSlice>) -> DeltaResult<Box<dyn Iterator<Item = DeltaResult<Bytes>>>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L617).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:617`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Read data specified by the start and end offset from the file.
