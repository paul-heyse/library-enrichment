# `datafusion_datasource::morsel`

Crate `datafusion-datasource` · 5 public items · structured records in [`model/datafusion_datasource.morsel.json`](../model/datafusion_datasource.morsel.json)

## MorselPlan

`struct` · `datafusion_datasource::morsel::MorselPlan`

```rust
struct MorselPlan
```

**Derives**: Default

**Methods** (9)

```rust
fn has_io_future(&self) -> bool
fn new() -> Self
fn set_pending_planner<F>(&mut self, io_future: F) where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
fn take_morsels(&mut self) -> Vec<Box<dyn Morsel>>
fn take_pending_planner(&mut self) -> Option<PendingMorselPlanner>
fn take_ready_planners(&mut self) -> Vec<Box<dyn MorselPlanner>>
fn with_morsels(self, morsels: Vec<Box<dyn Morsel>>) -> Self
fn with_pending_planner<F>(self, io_future: F) -> Self where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
fn with_planners(self, planners: Vec<Box<dyn MorselPlanner>>) -> Self
```

Return result of [`MorselPlanner::plan`].

# Logical Ordering

For plans where the output order of rows is maintained, the output order of
a [`MorselPlanner`] is logically defined as follows:
1. All morsels that are directly produced
2. Recursively, all morsels produced by the returned `planners`

---

## PendingMorselPlanner

`struct` · `datafusion_datasource::morsel::PendingMorselPlanner`

```rust
struct PendingMorselPlanner
```

**Implements**: `core::future::future::Future`

**Methods** (2)

```rust
fn into_future(self) -> BoxFuture<'static, Result<Box<dyn MorselPlanner>>>
fn new<F>(future: F) -> Self where F: Future<Output = Result<Box<dyn MorselPlanner>>> + Send + 'static
```

**via `core::future::future::Future`**

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

Wrapper for I/O that must complete before planning can continue.

---

## Morsel

`trait` · `datafusion_datasource::morsel::Morsel`

```rust
trait Morsel: Send + Debug
```

**Methods** (1)

```rust
fn into_stream(Box<self>) -> BoxStream<'static, Result<RecordBatch>>
```

A Morsel of work ready to resolve to a stream of [`RecordBatch`]es.

This represents a single morsel of work that is ready to be processed. It
has all data necessary (does not need any I/O) and is ready to be turned
into a stream of [`RecordBatch`]es for processing by the execution engine.

---

## MorselPlanner

`trait` · `datafusion_datasource::morsel::MorselPlanner`

```rust
trait MorselPlanner: Send + Debug
```

**Methods** (1)

```rust
fn plan(Box<self>) -> Result<Option<MorselPlan>>
```

A Morsel Planner is responsible for creating morsels for a given scan.

The [`MorselPlanner`] is the unit of I/O. There is only ever a single I/O
outstanding for a specific planner. DataFusion may run
multiple planners in parallel, which corresponds to multiple parallel
I/O requests.

It is not a Rust `Stream` so that it can explicitly separate CPU bound
work from I/O work.

The design is similar to `ParquetPushDecoder`: when `plan` is called, it
should do CPU work to produce the next morsels or discover the next I/O
phase.

Best practice is to spawn I/O in a Tokio task on a separate runtime to
ensure that CPU work doesn't block or slow down I/O work, but this is not
strictly required by the API.

---

## Morselizer

`trait` · `datafusion_datasource::morsel::Morselizer`

```rust
trait Morselizer: Send + Sync + Debug
```

**Methods** (1)

```rust
fn plan_file(&self, file: PartitionedFile) -> Result<Box<dyn MorselPlanner>>
```

A Morselizer takes a single [`PartitionedFile`] and creates the initial planner
for that file.

This is the entry point for morsel driven I/O.

---
