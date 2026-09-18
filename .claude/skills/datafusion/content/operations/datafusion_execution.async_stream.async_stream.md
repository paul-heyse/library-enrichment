# `datafusion_execution::async_stream::async_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.async_stream.async_stream.json).

<a id="op-134777deff59feac15c8b28e"></a>
## async_stream

`function` · `datafusion_execution::async_stream::async_stream` · datafusion-execution 55.1.0

```rust
fn async_stream<T, F: Future<Output = ()>>(generator: impl FnOnce(Emitter<T>) -> F) -> impl FusedStream<Item = T>
```

Source: `src/async_stream.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Creates a [`Stream`] from an async generator function.

The `generator` closure receives an [`Emitter<T>`](../operations/datafusion_execution.async_stream.Emitter.md#op-8937711056e5aea84f9c72af) and runs as an async
block. Each `emitter.emit(value).await` call suspends the generator and
produces the next item in the stream. The stream ends when the generator
future resolves.

# Example

```
use datafusion_execution::async_stream;
use futures::StreamExt;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let stream = async_stream(|mut emitter| async move {
    for i in 0_i32..3 {
        emitter.emit(i).await;
    }
});

let values: Vec<i32> = stream.collect().await;
assert_eq!(values, vec![0, 1, 2]);
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.
