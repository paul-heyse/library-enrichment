# `arrow_json::reader::value_iter::ValueIter`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.value_iter.ValueIter.json).

<a id="op-76dbd4a69cbea3362640787a"></a>
## ValueIter

`struct` · `arrow_json::reader::value_iter::ValueIter` · arrow-json 59.3.0

```rust
struct ValueIter<R: BufRead>
```

Source: `src/reader/value_iter.rs:40`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

JSON file reader that produces a serde_json::Value iterator from a Read trait

# Example

```
use std::fs::File;
use std::io::BufReader;
use arrow_json::reader::ValueIter;

let mut reader =
    BufReader::new(File::open("test/data/mixed_arrays.json").unwrap());
let mut value_reader = ValueIter::new(&mut reader, None);
for value in value_reader {
    println!("JSON value: {}", value.unwrap());
}
```

<a id="op-dde7d7d985a9fa163605a487"></a>
## Item

`assoc_type` · `arrow_json::reader::value_iter::ValueIter::Item` · arrow-json 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::value_iter::ValueIter", "path": "ValueIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [103, 2], "filename": "src/reader/value_iter.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/value_iter.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc10dc008dfb11e3c1092a62"></a>
## fmt

`function` · `arrow_json::reader::value_iter::ValueIter::fmt` · arrow-json 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::value_iter::ValueIter", "path": "ValueIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/reader/value_iter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/reader/value_iter.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-225f09707fc9517ea973c50c"></a>
## new

`function` · `arrow_json::reader::value_iter::ValueIter::new` · arrow-json 59.3.0

```rust
fn new(reader: R, max_read_records: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::value_iter::ValueIter", "path": "ValueIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [63, 2], "filename": "src/reader/value_iter.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/value_iter.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Creates a new `ValueIter`

<a id="op-f6a9f11b657b6762bd321dee"></a>
## next

`function` · `arrow_json::reader::value_iter::ValueIter::next` · arrow-json 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::value_iter::ValueIter", "path": "ValueIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [103, 2], "filename": "src/reader/value_iter.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/reader/value_iter.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ee1e5aa1620bc33a7688daa"></a>
## record_count

`function` · `arrow_json::reader::value_iter::ValueIter::record_count` · arrow-json 59.3.0

```rust
fn record_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "arrow_json::reader::value_iter::ValueIter", "path": "ValueIter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [48, 1], "end": [63, 2], "filename": "src/reader/value_iter.rs"}, "trait": null, "trait_path": null}`

Source: `src/reader/value_iter.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

Returns the number of records this iterator has consumed
