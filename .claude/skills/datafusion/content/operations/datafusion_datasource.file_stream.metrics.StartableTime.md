# `datafusion_datasource::file_stream::metrics::StartableTime`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.file_stream.metrics.StartableTime.json).

<a id="op-5ff61b8b4fa1cb6c1a7297ad"></a>
## StartableTime

`struct` · `datafusion_datasource::file_stream::metrics::StartableTime` · datafusion-datasource 55.1.0

```rust
struct StartableTime
```

Source: `src/file_stream/metrics.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A timer that can be started and stopped.

<a id="op-119dc93b85e088485b96a184"></a>
## metrics

`struct_field` · `datafusion_datasource::file_stream::metrics::StartableTime::metrics` · datafusion-datasource 55.1.0

```rust
metrics: datafusion_physical_plan::metrics::Time
```

Source: `src/file_stream/metrics.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de307c16c9a96a1b08e9d917"></a>
## start

`struct_field` · `datafusion_datasource::file_stream::metrics::StartableTime::start` · datafusion-datasource 55.1.0

```rust
start: Option<datafusion_common::instant::Instant>
```

Source: `src/file_stream/metrics.rs:27`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8609d4de1a2bf0cfeac00ea"></a>
## start

`function` · `datafusion_datasource::file_stream::metrics::StartableTime::start` · datafusion-datasource 55.1.0

```rust
fn start(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::metrics::StartableTime", "path": "StartableTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [41, 2], "filename": "src/file_stream/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/metrics.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e93b5c74ec84bae81fcabf"></a>
## stop

`function` · `datafusion_datasource::file_stream::metrics::StartableTime::stop` · datafusion-datasource 55.1.0

```rust
fn stop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::file_stream::metrics::StartableTime", "path": "StartableTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [41, 2], "filename": "src/file_stream/metrics.rs"}, "trait": null, "trait_path": null}`

Source: `src/file_stream/metrics.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
