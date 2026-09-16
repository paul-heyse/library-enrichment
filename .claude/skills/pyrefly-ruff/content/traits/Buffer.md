# Buffer

`ruff_formatter::buffer::Buffer`

```rust
trait Buffer
```

Also reachable as `ruff_formatter::Buffer`

Prose: [`api/ruff_formatter.buffer.md`](../api/ruff_formatter.buffer.md#buffer) · records: [`model/ruff_formatter.buffer.json`](../model/ruff_formatter.buffer.json)

## Required

Every implementation must supply these.

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
```

## Provided

Defaulted, and this is where the capability hides. The default is the conservative answer -- no pushdown, no statistics, no specialization -- so an implementation that overrides none of these works correctly and performs badly.

```rust
fn write_fmt(&mut self, arguments: Arguments<'_, Self::Context>) -> FormatResult<()>
```

## Implementors (4)

Read one before writing your own.

- `ruff_formatter::buffer::Inspect`
- `ruff_formatter::buffer::RemoveSoftLinesBuffer`
- `ruff_formatter::buffer::VecBuffer`
- `ruff_formatter::formatter::Formatter`

## Documentation

A trait for writing or formatting into [`FormatElement`]-accepting buffers or streams.
