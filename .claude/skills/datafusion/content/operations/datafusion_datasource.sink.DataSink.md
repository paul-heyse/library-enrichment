# `datafusion_datasource::sink::DataSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.sink.DataSink.json).

<a id="op-9a57c258a80fb4e5a004e0e1"></a>
## DataSink

`trait` · `datafusion_datasource::sink::DataSink` · datafusion-datasource 55.1.0

```rust
trait DataSink: Any + DisplayAs + Debug + Send + Sync
```

Source: `src/sink.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

`DataSink` implements writing streams of [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)es to
user defined destinations.

The `Display` impl is used to format the sink for explain plan
output.

<a id="op-d555d3b6f5b2dba91b1d7918"></a>
## metrics

`function` · `datafusion_datasource::sink::DataSink::metrics` · datafusion-datasource 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Source: `src/sink.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Return a snapshot of the [MetricsSet](../operations/datafusion_physical_expr_common.metrics.MetricsSet.md#op-c077c70588e76ca4a87a0b1b) for this
[DataSink](../operations/datafusion_datasource.sink.DataSink.md#op-9a57c258a80fb4e5a004e0e1).

See [ExecutionPlan::metrics()](../operations/datafusion_physical_plan.execution_plan.ExecutionPlan.md#op-f9e5333c240a14987dead676) for more details

<a id="op-8fb5791c285df9d468ef794c"></a>
## schema

`function` · `datafusion_datasource::sink::DataSink::schema` · datafusion-datasource 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Source: `src/sink.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Returns the sink schema

<a id="op-48b4423ff2ac81638461204a"></a>
## try_to_proto

`function` · `datafusion_datasource::sink::DataSink::try_to_proto` · datafusion-datasource 55.1.0

```rust
fn try_to_proto(&self, _exec: &DataSinkExec, _ctx: &datafusion_physical_plan::proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Source: `src/sink.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Serialize this sink into a full protobuf plan node, if it knows how.

Implementations can use `ctx` to encode the input plan, sink-specific
expressions, and [`DataSinkExec::encode_sort_order`](../operations/datafusion_datasource.sink.DataSinkExec.md#op-8e421a7f7046c4dfda0ddd00).

Returning `Ok(None)` lets the caller try its extension codec instead.

<a id="op-03153e06dfae84f047577032"></a>
## write_all

`function` · `datafusion_datasource::sink::DataSink::write_all` · datafusion-datasource 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, context: &Arc<TaskContext>) -> Result<u64>
```

Source: `src/sink.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Writes the data to the sink, returns the number of values written

This method will be called exactly once during each DML
statement. Thus prior to return, the sink should do any commit
or rollback required.
