# `tracing_core::event`

Crate `tracing-core` · 1 public items · structured records in [`model/tracing_core.event.json`](../model/tracing_core.event.json)

## Event

`struct` · `tracing_core::event::Event`

Also reachable as `tracing::Event`, `tracing::event::Event`, `tracing_core::Event`

```rust
struct Event<'a>
```

**Implements**: `tracing_subscriber::field::RecordFields`

**Derives**: Debug

**Methods** (10)

```rust
fn child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)
fn dispatch(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'_>)
fn fields(&self) -> field::Iter
fn is_contextual(&self) -> bool
fn is_root(&self) -> bool
fn metadata(&self) -> &'static Metadata<'static>
fn new(metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self
fn new_child_of(parent: impl Into<Option<Id>>, metadata: &'static Metadata<'static>, fields: &'a field::ValueSet<'a>) -> Self
fn parent(&self) -> Option<&Id>
fn record(&self, visitor: &mut dyn field::Visit)
```

`Event`s represent single points in time where something occurred during the
execution of a program.

An `Event` can be compared to a log record in unstructured logging, but with
two key differences:
- `Event`s exist _within the context of a [span]_. Unlike log lines, they
  may be located within the trace tree, allowing visibility into the
  _temporal_ context in which the event occurred, as well as the source
  code location.
- Like spans, `Event`s have structured key-value data known as _[fields]_,
  which may include textual message. In general, a majority of the data
  associated with an event should be in the event's fields rather than in
  the textual message, as the fields are more structured.

[span]: super::span
[fields]: super::field

---
