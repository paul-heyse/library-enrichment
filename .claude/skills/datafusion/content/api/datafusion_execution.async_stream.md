# `datafusion_execution::async_stream`

Crate `datafusion-execution` · 4 public items · structured records in [`model/datafusion_execution.async_stream.json`](../model/datafusion_execution.async_stream.json)

## async_stream

`function` · `datafusion_execution::async_stream::async_stream`

Also reachable as `datafusion::execution::async_stream`, `datafusion_execution::async_stream`

```rust
fn async_stream<T, F: Future<Output = ()>>(generator: impl FnOnce(Emitter<T>) -> F) -> impl FusedStream<Item = T>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.async_stream.async_stream.md).


Creates a [`Stream`] from an async generator function.

The `generator` closure receives an [`Emitter<T>`] and runs as an async
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

---

## async_try_stream

`function` · `datafusion_execution::async_stream::async_try_stream`

Also reachable as `datafusion::execution::async_try_stream`, `datafusion_execution::async_try_stream`

```rust
fn async_try_stream<T, E, F: Future<Output = Result<(), E>>>(generator: impl FnOnce(TryEmitter<T, E>) -> F) -> impl FusedStream<Item = Result<T, E>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.async_stream.async_try_stream.md).


Creates a fallible [`Stream`] from an async generator function.

The `generator` closure receives a [`TryEmitter<T, E>`] and runs as an
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

---

## Emitter

`struct` · `datafusion_execution::async_stream::Emitter`

Also reachable as `datafusion::execution::Emitter`, `datafusion_execution::Emitter`

```rust
struct Emitter<T>
```

**Methods** (1)

```rust
fn emit(&mut self, value: T) -> impl FusedFuture<Output = ()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.async_stream.Emitter.md).


A handle for emitting values from an [`async_stream`] generator.

The generator closure receives an `Emitter<T>` as its argument.

---

## TryEmitter

`struct` · `datafusion_execution::async_stream::TryEmitter`

Also reachable as `datafusion::execution::TryEmitter`, `datafusion_execution::TryEmitter`

```rust
struct TryEmitter<T, E>
```

**Methods** (1)

```rust
fn emit(&mut self, value: T) -> impl FusedFuture<Output = ()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_execution.async_stream.TryEmitter.md).


A handle for emitting values from an [`async_try_stream`] generator.

The generator closure receives a `TryEmitter<T, E>` as its argument.

---
