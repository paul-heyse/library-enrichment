# `ruff_formatter::formatter`

Crate `ruff_formatter` · 2 public items · structured records in [`model/ruff_formatter.formatter.json`](../model/ruff_formatter.formatter.json)

## Formatter

`struct` · `ruff_formatter::formatter::Formatter`

Also reachable as `ruff_formatter::prelude::Formatter`

```rust
struct Formatter<'buf, Context>
```

**Implements**: `ruff_formatter::buffer::Buffer`

**Methods** (10)

```rust
fn context(&self) -> &Context
fn context_mut(&mut self) -> &mut Context
fn fill<'a>(&'a mut self) -> FillBuilder<'a, 'buf, Context>
fn group_id(&self, debug_name: &'static str) -> GroupId
fn intern_vec(&mut self, elements: Vec<FormatElement>) -> Option<FormatElement>
fn join<'a>(&'a mut self) -> JoinBuilder<'a, 'buf, (), Context>
fn join_with<'a, Joiner>(&'a mut self, joiner: Joiner) -> JoinBuilder<'a, 'buf, Joiner, Context> where Joiner: Format<Context>
fn options(&self) -> &Context::Options where Context: FormatContext
fn restore_state_snapshot(&mut self, snapshot: FormatterSnapshot)
fn state_snapshot(&self) -> FormatterSnapshot
```

**via `ruff_formatter::buffer::Buffer`**

```rust
fn restore_snapshot(&mut self, snapshot: BufferSnapshot)
fn snapshot(&self) -> BufferSnapshot
fn state(&self) -> &FormatState<Self::Context>
fn state_mut(&mut self) -> &mut FormatState<Self::Context>
fn write_element(&mut self, element: FormatElement)
fn write_fmt(&mut self, arguments: Arguments<'_, Self::Context>) -> FormatResult<()>
```

Handles the formatting of a CST and stores the context how the CST should be formatted (user preferences).
The formatter is passed to the [Format] implementation of every node in the CST so that they
can use it to format their children.

---

## FormatterSnapshot

`struct` · `ruff_formatter::formatter::FormatterSnapshot`

```rust
struct FormatterSnapshot
```

Snapshot of the formatter state  used to handle backtracking if
errors are encountered in the formatting process and the formatter
has to fallback to printing raw tokens

In practice this only saves the set of printed tokens in debug
mode and compiled to nothing in release mode

---
