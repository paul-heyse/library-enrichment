# `datafusion_execution::async_stream::async_try_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.async_stream.async_try_stream.json).

<a id="op-d258c5b02971e9e9225dd32c"></a>
## async_try_stream

`function` · `datafusion_execution::async_stream::async_try_stream` · datafusion-execution 55.1.0

```rust
fn async_try_stream<T, E, F: Future<Output = Result<(), E>>>(generator: impl FnOnce(TryEmitter<T, E>) -> F) -> impl FusedStream<Item = Result<T, E>>
```

Source: `src/async_stream.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Creates a fallible [`Stream`] from an async generator function.

The `generator` closure receives a [`TryEmitter<T, E>`](../operations/datafusion_execution.async_stream.TryEmitter.md#op-a4a27f9f5c8ee8d9be3ec4b0) and runs as an
async block that returns `Result<(), E>`. Each `emitter.emit(value).await`
call suspends the generator and produces `Ok(value)` as the next stream
item. The `?` operator can be used inside the generator to short-circuit on
errors: the error is emitted as the final `Err(e)` item and the stream
ends. The stream also ends when the generator future resolves to `Ok(())`.

# Example

```
use datafusion_execution::async_try_stream;
use futures::StreamExt;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let stream = async_try_stream(|mut emitter| async move {
    emitter.emit(1_i32).await;
    emitter.emit(2_i32).await;
    Err::<(), _>("something went wrong")?;
    emitter.emit(3_i32).await; // never reached
    Ok(())
});

let values: Vec<Result<i32, &str>> = stream.collect().await;
assert_eq!(values, vec![Ok(1), Ok(2), Err("something went wrong")]);
# }
```

Unresolved upstream links (retained, not inferred): ``Stream``.
