# `datafusion_datasource_json::utils`

Crate `datafusion-datasource-json` · 2 public items · structured records in [`model/datafusion_datasource_json.utils.json`](../model/datafusion_datasource_json.utils.json)

## ChannelReader

`struct` · `datafusion_datasource_json::utils::ChannelReader`

```rust
struct ChannelReader
```

**Implements**: `alloc::io::read::Read`

**Methods** (1)

```rust
fn new(rx: tokio::sync::mpsc::Receiver<Bytes>) -> Self
```

**via `alloc::io::read::Read`**

```rust
fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
```

A synchronous `Read` implementation that receives bytes from an async channel.

This enables true streaming between async and sync contexts without
loading the entire file into memory. Uses `tokio::sync::mpsc::Receiver`
with `blocking_recv()` so the async producer never blocks a tokio worker
thread, while the sync consumer (running in `spawn_blocking`) safely blocks.

---

## JsonArrayToNdjsonReader

`struct` · `datafusion_datasource_json::utils::JsonArrayToNdjsonReader`

```rust
struct JsonArrayToNdjsonReader<R: Read>
```

**Implements**: `alloc::io::buf_read::BufRead`, `alloc::io::read::Read`

**Methods** (3)

```rust
fn new(reader: R) -> Self
fn validate_complete(&self) -> std::io::Result<()>
fn with_capacity(reader: R, capacity: usize) -> Self
```

**via `alloc::io::buf_read::BufRead`**

```rust
fn consume(&mut self, amt: usize)
fn fill_buf(&mut self) -> std::io::Result<&[u8]>
```

**via `alloc::io::read::Read`**

```rust
fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
```

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

---
