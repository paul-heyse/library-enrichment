# Previewing partial results

`preview_limit` sets how many rows; `preview_fn` decides how they are rendered into the `datafusion.preview` field. `preview_limit(0)` disables preview entirely, and is the default. `pretty_format_compact_batch(batch, width, min_col, max_col)` is the shipped formatter and draws a `|===|` header rule that `pretty_format_batches` does not.

## Entry points

| Item | Visibility | Methods | Reached via |
|---|---|---:|---|
| [`pretty_format_compact_batch`](../api/datafusion_tracing.preview_utils.md) | supported | 0 | — |

## Spans it emits

| Span | Target | Level | Fields | Evidence |
|---|---|---|---:|---|
| `InstrumentedExec` | `integration_utils` | INFO | 49 | confirmed |

## What this seam cannot tell you

- What a preview will look like for your data. `index/previews.tsv` holds 35 captured renders; the widths and wrapping are a function of the arguments you pass, not of the library.
- Any promise that a preview is complete. It is at most `preview_limit` rows of one batch.
- The writable name of `preview_fn`'s type. `PreviewFn` is a type alias in a private module and rustdoc expands it away; pass a closure and let inference do it.

## Read next

- [`catalogs/spans.md`](../catalogs/spans.md)
- `content/corpus/source/preview.rs` — upstream, verbatim
- `content/corpus/source/preview_utils.rs` — upstream, verbatim

## Before you call it done

- Is `preview_fn` set at all? Without it `preview_limit` records nothing readable.
- Does the formatting cost belong on the query path in production?
