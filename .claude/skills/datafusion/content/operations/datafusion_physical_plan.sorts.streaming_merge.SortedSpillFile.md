# `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.streaming_merge.SortedSpillFile.json).

<a id="op-b4fc1aca5ac96408a05dd560"></a>
## SortedSpillFile

`struct` · `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile` · datafusion-physical-plan 55.1.0

```rust
struct SortedSpillFile
```

Source: `src/sorts/streaming_merge.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a1a32c64f5e7545b787e6ce"></a>
## file

`struct_field` · `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile::file` · datafusion-physical-plan 55.1.0

```rust
file: std::sync::Arc<dyn SpillFile>
```

Source: `src/sorts/streaming_merge.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df71e12387c0a67069271cdd"></a>
## fmt

`function` · `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile", "path": "SortedSpillFile"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [85, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sorts/streaming_merge.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c82dace3b2b9635e88805f12"></a>
## max_record_batch_memory

`struct_field` · `datafusion_physical_plan::sorts::streaming_merge::SortedSpillFile::max_record_batch_memory` · datafusion-physical-plan 55.1.0

```rust
max_record_batch_memory: usize
```

Source: `src/sorts/streaming_merge.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

how much memory the largest memory batch is taking
