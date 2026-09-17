# Instrumenting the object store

`instrument_object_store(store, name)` wraps an `ObjectStore` and is the whole API of the second crate. Register the wrapper rather than the store. Unlike the macro family, this crate calls `tracing` directly, so its spans carry its OWN target.

## Entry points

| Item | Visibility | Methods | Reached via |
|---|---|---:|---|
| [`instrument_object_store`](../api/instrumented_object_store.instrumented_object_store.md) | supported | 0 | — |

## Spans it emits

| Span | Target | Level | Fields | Evidence |
|---|---|---|---:|---|
| `get_opts` | `instrumented_object_store::instrumented_object_store` | INFO | 5 | recorded |
| `get_ranges` | `instrumented_object_store::instrumented_object_store` | INFO | 4 | recorded |

## What this seam cannot tell you

- Coverage of every `ObjectStore` method. Two span names were observed; the others are unobserved, and the crate is 11 KB of source you can read.
- Anything about `object_store` itself. That crate is not indexed here.
- A way to enable it from `InstrumentationOptions`. It is a separate crate and a separate call.

## Read next

- [`catalogs/spans.md`](../catalogs/spans.md)
- `content/corpus/source/instrumented-object-store/instrumented_object_store.rs` — upstream, verbatim

## Before you call it done

- Is the store passed through `instrument_object_store` BEFORE `register_object_store`?
- Does your subscriber filter admit `instrumented_object_store`, which differs from the target the macros use?
