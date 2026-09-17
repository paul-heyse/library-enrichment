# `grep_cli::wtr`

Crate `grep-cli` · 4 public items · structured records in [`model/grep_cli.wtr.json`](../model/grep_cli.wtr.json)

## stdout

`function` · `grep_cli::wtr::stdout`

Also reachable as `grep_cli::stdout`

```rust
fn stdout(color_choice: termcolor::ColorChoice) -> StandardStream
```

Returns a possibly buffered writer to stdout for the given color choice.

The writer returned is either line buffered or block buffered. The decision
between these two is made automatically based on whether a tty is attached
to stdout or not. If a tty is attached, then line buffering is used.
Otherwise, block buffering is used. In general, block buffering is more
efficient, but may increase the time it takes for the end user to see the
first bits of output.

If you need more fine grained control over the buffering mode, then use one
of `stdout_buffered_line` or `stdout_buffered_block`.

The color choice given is passed along to the underlying writer. To
completely disable colors in all cases, use `ColorChoice::Never`.

---

## stdout_buffered_block

`function` · `grep_cli::wtr::stdout_buffered_block`

Also reachable as `grep_cli::stdout_buffered_block`

```rust
fn stdout_buffered_block(color_choice: termcolor::ColorChoice) -> StandardStream
```

Returns a block buffered writer to stdout for the given color choice.

This writer is useful when printing results to a file since it amortizes
the cost of writing data. The downside of this approach is that it can
increase the latency of display output when writing to a tty.

You might consider using [`stdout`] instead, which chooses the buffering
strategy automatically based on whether stdout is connected to a tty.

---

## stdout_buffered_line

`function` · `grep_cli::wtr::stdout_buffered_line`

Also reachable as `grep_cli::stdout_buffered_line`

```rust
fn stdout_buffered_line(color_choice: termcolor::ColorChoice) -> StandardStream
```

Returns a line buffered writer to stdout for the given color choice.

This writer is useful when printing results directly to a tty such that
users see output as soon as it's written. The downside of this approach
is that it can be slower, especially when there is a lot of output.

You might consider using [`stdout`] instead, which chooses the buffering
strategy automatically based on whether stdout is connected to a tty.

---

## StandardStream

`struct` · `grep_cli::wtr::StandardStream`

Also reachable as `grep_cli::StandardStream`

```rust
struct StandardStream
```

**Implements**: `core::io::write::Write`, `termcolor::WriteColor`

**Derives**: Debug

**via `core::io::write::Write`**

```rust
fn flush(&mut self) -> io::Result<()>
fn write(&mut self, buf: &[u8]) -> io::Result<usize>
```

**via `termcolor::WriteColor`**

```rust
fn is_synchronous(&self) -> bool
fn reset(&mut self) -> io::Result<()>
fn set_color(&mut self, spec: &termcolor::ColorSpec) -> io::Result<()>
fn set_hyperlink(&mut self, link: &HyperlinkSpec<'_>) -> io::Result<()>
fn supports_color(&self) -> bool
fn supports_hyperlinks(&self) -> bool
```

A writer that supports coloring with either line or block buffering.

---
