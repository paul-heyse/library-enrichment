# `datafusion_datasource_json::utils::JsonArrayToNdjsonReader`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.utils.JsonArrayToNdjsonReader.json).

<a id="op-ea73f471c3b41ccda1ae2623"></a>
## JsonArrayToNdjsonReader

`struct` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader` · datafusion-datasource-json 55.1.0

```rust
struct JsonArrayToNdjsonReader<R: Read>
```

Source: `src/utils.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

A streaming reader that converts JSON array format to NDJSON format.

This reader wraps an underlying reader containing JSON array data
`[{...}, {...}, ...]` and transforms it on-the-fly to newline-delimited
JSON format that Arrow's JSON reader can process.

Implements both `Read` and `BufRead` traits for compatibility with Arrow's
`ReaderBuilder::build()` which requires `BufRead`.

# Transformation Rules

- Skip leading `[` and whitespace before it
- Convert top-level `,` (between objects) to `\n`
- Skip whitespace at top level (between objects)
- Stop at trailing `]`
- Preserve everything inside objects (including nested `[`, `]`, `,`)
- Properly handle strings (ignore special chars inside quotes)

# Example

```text
Input:  [{"a":1}, {"b":[1,2]}, {"c":"x,y"}]
Output: {"a":1}
        {"b":[1,2]}
        {"c":"x,y"}
```

<a id="op-1ef6bfbdbffc8cff944772a8"></a>
## consume

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::consume` · datafusion-datasource-json 55.1.0

```rust
fn consume(&mut self, amt: usize)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [367, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}, "trait_path": "alloc::io::buf_read::BufRead"}`

Source: `src/utils.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1715b0fb4adab11c52681673"></a>
## fill_buf

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::fill_buf` · datafusion-datasource-json 55.1.0

```rust
fn fill_buf(&mut self) -> std::io::Result<&[u8]>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [356, 1], "end": [367, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "alloc::io::buf_read::BufRead", "path": "BufRead"}, "trait_path": "alloc::io::buf_read::BufRead"}`

Source: `src/utils.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e75d692ab459a87d709cc0"></a>
## new

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::new` · datafusion-datasource-json 55.1.0

```rust
fn new(reader: R) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [333, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Create a new streaming reader that converts JSON array to NDJSON.

<a id="op-95f0271498ed816b32e054ff"></a>
## read

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::read` · datafusion-datasource-json 55.1.0

```rust
fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [335, 1], "end": [354, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}, "trait_path": "alloc::io::read::Read"}`

Source: `src/utils.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc4c0cd2ab33370912bf56cc"></a>
## validate_complete

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::validate_complete` · datafusion-datasource-json 55.1.0

```rust
fn validate_complete(&self) -> std::io::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [333, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Check if the JSON array was properly terminated.

This should be called after all data has been read.

Returns an error if:
- Unbalanced braces/brackets (depth != 0)
- Unterminated string
- Missing closing `]`
- Unexpected trailing content after `]`

<a id="op-514abea30b581acdaf849b10"></a>
## with_capacity

`function` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader::with_capacity` · datafusion-datasource-json 55.1.0

```rust
fn with_capacity(reader: R, capacity: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "R"}}], "constraints": []}}, "id": "datafusion_datasource_json::utils::JsonArrayToNdjsonReader", "path": "JsonArrayToNdjsonReader"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}}}], "default": null, "is_synthetic": false}}, "name": "R"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 1], "end": [333, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Create a new streaming reader with custom buffer size.

Larger buffers improve throughput but use more memory.
Total memory usage is approximately 2 * capacity (input + output buffers).
