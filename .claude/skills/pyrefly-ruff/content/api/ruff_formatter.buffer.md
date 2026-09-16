# `ruff_formatter::buffer`

Crate `ruff_formatter` · 8 public items · structured records in [`model/ruff_formatter.buffer.json`](../model/ruff_formatter.buffer.json)

## BufferSnapshot

`enum` · `ruff_formatter::buffer::BufferSnapshot`

Also reachable as `ruff_formatter::BufferSnapshot`

```rust
enum BufferSnapshot
```

**Variants**: `Position`, `Any`

**Derives**: Debug

Snapshot of a buffer state that can be restored at a later point.

Used in cases where the formatting of an object fails but a parent formatter knows an alternative
strategy on how to format the object that might succeed.

---

## Inspect

`struct` · `ruff_formatter::buffer::Inspect`

Also reachable as `ruff_formatter::Inspect`

```rust
struct Inspect<'inner, Context, Inspector>
```

**Implements**: `ruff_formatter::buffer::Buffer`

**via `ruff_formatter::buffer::Buffer`**

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
```

Buffer that allows you inspecting elements as they get written to the formatter.

---

## Recorded

`struct` · `ruff_formatter::buffer::Recorded`

```rust
struct Recorded<'a>
```

---

## Recording

`struct` · `ruff_formatter::buffer::Recording`

```rust
struct Recording<'buf, Buffer>
```

---

## RemoveSoftLinesBuffer

`struct` · `ruff_formatter::buffer::RemoveSoftLinesBuffer`

Also reachable as `ruff_formatter::RemoveSoftLinesBuffer`

```rust
struct RemoveSoftLinesBuffer<'a, Context>
```

**Implements**: `ruff_formatter::buffer::Buffer`

**Methods** (1)

```rust
fn new(inner: &'a mut dyn Buffer<Context = Context>) -> Self
```

**via `ruff_formatter::buffer::Buffer`**

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
```

A Buffer that removes any soft line breaks or [`if_group_breaks`](crate::builders::if_group_breaks) elements.

- Removes [`lines`](FormatElement::Line) with the mode [`Soft`](LineMode::Soft).
- Replaces [`lines`](FormatElement::Line) with the mode [`Soft`](LineMode::SoftOrSpace) with a [`Space`](FormatElement::Space)
- Removes [`if_group_breaks`](crate::builders::if_group_breaks) and all its content.
- Unwraps the content of [`if_group_fits_on_line`](crate::builders::if_group_fits_on_line) elements (but retains it).

# Examples

```
use ruff_formatter::prelude::*;
use ruff_formatter::{format, write};

# fn main() -> FormatResult<()> {
use ruff_formatter::{RemoveSoftLinesBuffer, SimpleFormatContext, VecBuffer};
use ruff_formatter::prelude::format_with;
let formatted = format!(
    SimpleFormatContext::default(),
    [format_with(|f| {
        let mut buffer = RemoveSoftLinesBuffer::new(f);

        write!(
            buffer,
            [
                token("The next soft line or space gets replaced by a space"),
                soft_line_break_or_space(),
                token("and the line here"),
                soft_line_break(),
                token("is removed entirely.")
            ]
        )
    })]
)?;

assert_eq!(
    formatted.document().as_ref(),
    &[
        FormatElement::Token { text: "The next soft line or space gets replaced by a space" },
        FormatElement::Space,
        FormatElement::Token { text: "and the line here" },
        FormatElement::Token { text: "is removed entirely." }
    ]
);

# Ok(())
# }
```

---

## VecBuffer

`struct` · `ruff_formatter::buffer::VecBuffer`

Also reachable as `ruff_formatter::VecBuffer`

```rust
struct VecBuffer<'a, Context>
```

**Implements**: `core::ops::deref::Deref`, `core::ops::deref::DerefMut`, `ruff_formatter::buffer::Buffer`

**Derives**: Debug

**Methods** (3)

```rust
fn into_vec(self) -> Vec<FormatElement>
fn new(state: &'a mut FormatState<Context>) -> Self
fn take_vec(&mut self) -> Vec<FormatElement>
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

**via `core::ops::deref::DerefMut`**

```rust
fn deref_mut(&mut self) -> &mut Self::Target
```

**via `ruff_formatter::buffer::Buffer`**

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
```

Vector backed [`Buffer`] implementation.

The buffer writes all elements into the internal elements buffer.

---

## Buffer

`trait` · `ruff_formatter::buffer::Buffer`

Also reachable as `ruff_formatter::Buffer`

```rust
trait Buffer
```

**Implementors** (4)

- `ruff_formatter::buffer::Inspect`
- `ruff_formatter::buffer::RemoveSoftLinesBuffer`
- `ruff_formatter::buffer::VecBuffer`
- `ruff_formatter::formatter::Formatter`

**Methods** (6)

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
fn write_fmt(&mut self, arguments: Arguments<'_, Self::Context>) -> FormatResult<()>
```

A trait for writing or formatting into [`FormatElement`]-accepting buffers or streams.

---

## BufferExtensions

`trait` · `ruff_formatter::buffer::BufferExtensions`

Also reachable as `ruff_formatter::BufferExtensions`, `ruff_formatter::prelude::BufferExtensions`

```rust
trait BufferExtensions: Buffer + Sized
```

**Methods** (3)

```rust
fn inspect<F>(&mut self, inspector: F) -> Inspect<'_, Self::Context, F> where F: FnMut(&FormatElement)
fn start_recording(&mut self) -> Recording<'_, Self>
fn write_elements<I>(&mut self, elements: I) where I: IntoIterator<Item = FormatElement>
```

---
