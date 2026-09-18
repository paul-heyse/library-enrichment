# `parquet::file::serialized_reader::ReadGroupPredicate`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.serialized_reader.ReadGroupPredicate.json).

<a id="op-0199edf5d684474620e158f5"></a>
## ReadGroupPredicate

`type_alias` · `parquet::file::serialized_reader::ReadGroupPredicate` · parquet 59.3.0

```rust
type ReadGroupPredicate = Box<dyn FnMut(&RowGroupMetaData, usize) -> bool>
```

Source: `src/file/serialized_reader.rs:103`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A predicate for filtering row groups, invoked with the metadata and index
of each row group in the file. Only row groups for which the predicate
evaluates to `true` will be scanned
