# `buoyant_kernel::log_segment_files`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.log_segment_files.json).

<a id="op-a91e04f036380ea92691ebc0"></a>
## log_segment_files

`module` · `buoyant_kernel::log_segment_files` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod log_segment_files
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/log_segment_files/mod.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/log_segment_files/mod.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`LogSegmentFiles`](../operations/buoyant_kernel.log_segment_files.LogSegmentFiles.md#op-65a62a8e62a765ce9da36e5d) is a struct holding the result of listing the delta log. Currently, it
exposes four APIs for listing:
1. [`list_commits`]: Lists all commit files between the provided start and end versions.
2. [`list`]: Lists all commit and checkpoint files between the provided start and end versions.
3. [`list_with_checkpoint_hint`]: Lists all commit and checkpoint files after the provided
   checkpoint hint.
4. [`list_with_backward_checkpoint_scan`]: Scans backward from an end version in 1000-version
   windows until a complete checkpoint is found or the log is exhausted.

After listing, one can leverage the [`LogSegmentFiles`](../operations/buoyant_kernel.log_segment_files.LogSegmentFiles.md#op-65a62a8e62a765ce9da36e5d) to construct a [`LogSegment`].

[`list_with_backward_checkpoint_scan`]: Self::list_with_backward_checkpoint_scan

[`list_commits`]: Self::list_commits
[`list`]: Self::list
[`list_with_checkpoint_hint`]: Self::list_with_checkpoint_hint
[`LogSegment`]: crate::log_segment::LogSegment
