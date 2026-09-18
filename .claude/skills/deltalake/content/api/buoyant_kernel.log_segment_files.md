# `buoyant_kernel::log_segment_files`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.log_segment_files.json`](../model/buoyant_kernel.log_segment_files.json)

## LogSegmentFiles

`struct` · `buoyant_kernel::log_segment_files::LogSegmentFiles`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.log_segment_files.LogSegmentFiles.md)

Also reachable as `buoyant_kernel::log_segment::LogSegmentFiles`, `delta_kernel::log_segment_files::LogSegmentFiles`

```rust
struct LogSegmentFiles
```

**Fields**: `ascending_commit_files`, `ascending_compaction_files`, `checkpoint_parts`, `latest_crc_file`, `latest_commit_file`, `max_published_version`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

Represents the set of log files found during a listing operation in the Delta log directory.

- `ascending_commit_files`: All commit and staged commit files found, sorted by version. May
  contain gaps.
- `ascending_compaction_files`: All compaction commit files found, sorted by version.
- `checkpoint_parts`: All parts of the most recent complete checkpoint (all same version). Empty
  if no checkpoint found.
- `latest_crc_file`: The CRC file with the highest version, only if version >= checkpoint
  version.
- `latest_commit_file`: The commit file with the highest version, or `None` if no commits were
  found. This field may be present even when `ascending_commit_files` is empty, such as when a
  checkpoint subsumes all commits. In that case, it is retained because downstream code (e.g.
  In-Commit Timestamp reading) needs access to the commit file at the snapshot version.
- `max_published_version`: The highest published commit file version, or `None` if no published
  commits were found.

---
