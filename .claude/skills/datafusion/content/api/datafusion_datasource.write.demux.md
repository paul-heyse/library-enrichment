# `datafusion_datasource::write::demux`

Crate `datafusion-datasource` · 1 public items · structured records in [`model/datafusion_datasource.write.demux.json`](../model/datafusion_datasource.write.demux.json)

## DemuxedStreamReceiver

`type_alias` · `datafusion_datasource::write::demux::DemuxedStreamReceiver`

```rust
type DemuxedStreamReceiver = tokio::sync::mpsc::UnboundedReceiver<(object_store::path::Path, tokio::sync::mpsc::Receiver<arrow::array::RecordBatch>)>
```

[Full member, field, variant and typed contracts](../operations/datafusion_datasource.write.demux.DemuxedStreamReceiver.md).


---
