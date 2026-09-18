# `datafusion_datasource::write::demux::DemuxedStreamReceiver`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.demux.DemuxedStreamReceiver.json).

<a id="op-049aa73e438c2e90d34aa825"></a>
## DemuxedStreamReceiver

`type_alias` · `datafusion_datasource::write::demux::DemuxedStreamReceiver` · datafusion-datasource 55.1.0

```rust
type DemuxedStreamReceiver = tokio::sync::mpsc::UnboundedReceiver<(object_store::path::Path, tokio::sync::mpsc::Receiver<arrow::array::RecordBatch>)>
```

Source: `src/write/demux.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
