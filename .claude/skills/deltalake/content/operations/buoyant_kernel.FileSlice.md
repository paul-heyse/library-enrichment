# `buoyant_kernel::FileSlice`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.FileSlice.json).

<a id="op-ed4eb1589d2ab2df8d211252"></a>
## FileSlice

`type_alias` · `buoyant_kernel::FileSlice` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type FileSlice = (url::Url, Option<std::ops::Range<FileIndex>>)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/lib.rs#L209).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/lib.rs:209`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A specification for a range of bytes to read from a file location
