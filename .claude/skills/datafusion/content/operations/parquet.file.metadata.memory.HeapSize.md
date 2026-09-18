# `parquet::file::metadata::memory::HeapSize`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.metadata.memory.HeapSize.json).

<a id="op-1bf35532564796dd61830cfe"></a>
## HeapSize

`trait` · `parquet::file::metadata::memory::HeapSize` · parquet 59.3.0

```rust
trait HeapSize
```

Source: `src/file/metadata/memory.rs:36`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Trait for calculating the size of various containers

<a id="op-f390f7458aa2b72353d5a017"></a>
## heap_size

`function` · `parquet::file::metadata::memory::HeapSize::heap_size` · parquet 59.3.0

```rust
fn heap_size(&self) -> usize
```

Source: `src/file/metadata/memory.rs:42`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Return the size of any bytes allocated on the heap by this object,
including heap memory in those structures

Note that the size of the type itself is not included in the result --
instead, that size is added by the caller (e.g. container).
