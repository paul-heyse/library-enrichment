# `datafusion_datasource_json::utils::ChannelReader`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource_json.utils.ChannelReader.json).

<a id="op-38f19578607140f071407521"></a>
## ChannelReader

`struct` · `datafusion_datasource_json::utils::ChannelReader` · datafusion-datasource-json 55.1.0

```rust
struct ChannelReader
```

Source: `src/utils.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

A synchronous `Read` implementation that receives bytes from an async channel.

This enables true streaming between async and sync contexts without
loading the entire file into memory. Uses `tokio::sync::mpsc::Receiver`
with `blocking_recv()` so the async producer never blocks a tokio worker
thread, while the sync consumer (running in `spawn_blocking`) safely blocks.

<a id="op-3db38d6c221c82ab6ec2a0c9"></a>
## new

`function` · `datafusion_datasource_json::utils::ChannelReader::new` · datafusion-datasource-json 55.1.0

```rust
fn new(rx: tokio::sync::mpsc::Receiver<Bytes>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::utils::ChannelReader", "path": "ChannelReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [425, 1], "end": [434, 2], "filename": "src/utils.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

Create a new ChannelReader from a tokio mpsc receiver.

<a id="op-3e7158e52fd9dbb0000f47b3"></a>
## read

`function` · `datafusion_datasource_json::utils::ChannelReader::read` · datafusion-datasource-json 55.1.0

```rust
fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource_json::utils::ChannelReader", "path": "ChannelReader"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [436, 1], "end": [461, 2], "filename": "src/utils.rs"}, "trait": {"args": null, "id": "alloc::io::read::Read", "path": "Read"}, "trait_path": "alloc::io::read::Read"}`

Source: `src/utils.rs:437`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource-json/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
