# `deltalake_lakefs`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.json).

<a id="op-0e9d00bae5fe5e73e1541e77"></a>
## deltalake_lakefs

`module` · `deltalake_lakefs` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod deltalake_lakefs
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/lib.rs#L1).

Source: `crates/lakefs/src/lib.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

LakeFS and similar tooling for delta-rs

This module also contains the [LakeFSLogStore] implementation for delta operations executed in transaction branches
where deltalake commits only happen when the branch can be safely merged.
