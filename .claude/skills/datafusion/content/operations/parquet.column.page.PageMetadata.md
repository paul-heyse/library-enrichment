# `parquet::column::page::PageMetadata`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.column.page.PageMetadata.json).

<a id="op-8f9282b164cfc23334df7466"></a>
## PageMetadata

`struct` · `parquet::column::page::PageMetadata` · parquet 59.3.0

```rust
struct PageMetadata
```

Source: `src/column/page.rs:348`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Contains metadata for a page

<a id="op-00614dbd39f967c360ef9069"></a>
## clone

`function` · `parquet::column::page::PageMetadata::clone` · parquet 59.3.0

```rust
fn clone(&self) -> PageMetadata
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::column::page::PageMetadata", "path": "PageMetadata"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 10], "end": [347, 15], "filename": "src/column/page.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/column/page.rs:347`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b32fac16ee9071b72020aa2"></a>
## is_dict

`struct_field` · `parquet::column::page::PageMetadata::is_dict` · parquet 59.3.0

```rust
is_dict: bool
```

Source: `src/column/page.rs:354`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if the page is a dictionary page

<a id="op-cdf77d4611c4b66db153b3a5"></a>
## num_levels

`struct_field` · `parquet::column::page::PageMetadata::num_levels` · parquet 59.3.0

```rust
num_levels: Option<usize>
```

Source: `src/column/page.rs:352`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of levels within the page if known

<a id="op-926a331907f30df539dea839"></a>
## num_rows

`struct_field` · `parquet::column::page::PageMetadata::num_rows` · parquet 59.3.0

```rust
num_rows: Option<usize>
```

Source: `src/column/page.rs:350`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of rows within the page if known
