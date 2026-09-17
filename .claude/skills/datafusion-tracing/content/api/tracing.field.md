# `tracing::field`

Crate `tracing` · 1 public items · structured records in [`model/tracing.field.json`](../model/tracing.field.json)

## AsField

`trait` · `tracing::field::AsField`

```rust
trait AsField: sealed::Sealed
```

**Implementors** (1)

- `tracing_core::field::Field`

**Methods** (1)

```rust
fn as_field(&self, metadata: &Metadata<'_>) -> Option<Field>
```

Trait implemented to allow a type to be used as a field key.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: Although this is implemented for both the
<a href="./struct.Field.html"><code>Field</code></a> type <em>and</em> any
type that can be borrowed as an <code>&str</code>, only <code>Field</code>
allows <em>O</em>(1) access.
Indexing a field with a string results in an iterative search that performs
string comparisons. Thus, if possible, once the key for a field is known, it
should be used whenever possible.
</pre>

---
