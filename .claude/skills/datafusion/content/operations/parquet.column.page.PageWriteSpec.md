# `parquet::column::page::PageWriteSpec`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.PageWriteSpec.json).

<a id="op-0a5318f63bef53aa29d1bba2"></a>
## PageWriteSpec

`struct` · `parquet::column::page::PageWriteSpec` · parquet 59.3.0

```rust
struct PageWriteSpec
```

Source: `src/column/page.rs:311`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Contains page write metrics.

<a id="op-5ae44f9ec6ba80042e03f0fe"></a>
## bytes_written

`struct_field` · `parquet::column::page::PageWriteSpec::bytes_written` · parquet 59.3.0

```rust
bytes_written: u64
```

Source: `src/column/page.rs:323`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of bytes written to the underlying sink

<a id="op-d218a6bd7f0defacc9431f4a"></a>
## compressed_size

`struct_field` · `parquet::column::page::PageWriteSpec::compressed_size` · parquet 59.3.0

```rust
compressed_size: usize
```

Source: `src/column/page.rs:317`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The compressed size of the page

<a id="op-2bcbae578f6f17ab4ccb60d4"></a>
## default

`function` · `parquet::column::page::PageWriteSpec::default` · parquet 59.3.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::PageWriteSpec", "path": "PageWriteSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [326, 1], "end": [330, 2], "filename": "src/column/page.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/column/page.rs:327`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6f25b27155a17a1472927b8"></a>
## new

`function` · `parquet::column::page::PageWriteSpec::new` · parquet 59.3.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::PageWriteSpec", "path": "PageWriteSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [332, 1], "end": [344, 2], "filename": "src/column/page.rs"}, "trait": null, "trait_path": null}`

Source: `src/column/page.rs:334`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new spec with default page write metrics.

<a id="op-957d1c3a33c30240931b4415"></a>
## num_values

`struct_field` · `parquet::column::page::PageWriteSpec::num_values` · parquet 59.3.0

```rust
num_values: u32
```

Source: `src/column/page.rs:319`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of values in the page

<a id="op-4125c150d4247c28ea86be7e"></a>
## offset

`struct_field` · `parquet::column::page::PageWriteSpec::offset` · parquet 59.3.0

```rust
offset: u64
```

Source: `src/column/page.rs:321`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The offset of the page in the column chunk

<a id="op-33bff5f858e9f2efcc530843"></a>
## page_type

`struct_field` · `parquet::column::page::PageWriteSpec::page_type` · parquet 59.3.0

```rust
page_type: basic::PageType
```

Source: `src/column/page.rs:313`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The type of page being written

<a id="op-8c919b5575e69fd76f7da63a"></a>
## uncompressed_size

`struct_field` · `parquet::column::page::PageWriteSpec::uncompressed_size` · parquet 59.3.0

```rust
uncompressed_size: usize
```

Source: `src/column/page.rs:315`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The total size of the page, before compression
