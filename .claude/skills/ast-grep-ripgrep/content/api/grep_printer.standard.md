# `grep_printer::standard`

Crate `grep-printer` · 3 public items · structured records in [`model/grep_printer.standard.json`](../model/grep_printer.standard.json)

## Standard

`struct` · `grep_printer::standard::Standard`

Also reachable as `grep_printer::Standard`

```rust
struct Standard<W>
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn get_mut(&mut self) -> &mut W
fn has_written(&self) -> bool
fn into_inner(self) -> W
fn new(wtr: W) -> Standard<W>
fn new_no_color(wtr: W) -> Standard<NoColor<W>>
fn sink<'s, M: Matcher>(&'s mut self, matcher: M) -> StandardSink<'static, 's, M, W>
fn sink_with_path<'p, 's, M, P>(&'s mut self, matcher: M, path: &'p P) -> StandardSink<'p, 's, M, W> where M: Matcher, P: ?Sized + AsRef<Path>
```

The standard printer, which implements grep-like formatting, including
color support.

A default printer can be created with either of the `Standard::new` or
`Standard::new_no_color` constructors. However, there are a considerable
number of options that configure this printer's output. Those options can
be configured using [`StandardBuilder`].

This type is generic over `W`, which represents any implementation
of the `termcolor::WriteColor` trait. If colors are not desired,
then the `new_no_color` constructor can be used, or, alternatively,
the `termcolor::NoColor` adapter can be used to wrap any `io::Write`
implementation without enabling any colors.

---

## StandardBuilder

`struct` · `grep_printer::standard::StandardBuilder`

Also reachable as `grep_printer::StandardBuilder`

```rust
struct StandardBuilder
```

**Derives**: Clone, Debug

**Methods** (23)

```rust
fn build<W: WriteColor>(&self, wtr: W) -> Standard<W>
fn build_no_color<W: io::Write>(&self, wtr: W) -> Standard<NoColor<W>>
fn byte_offset(&mut self, yes: bool) -> &mut StandardBuilder
fn color_specs(&mut self, specs: ColorSpecs) -> &mut StandardBuilder
fn column(&mut self, yes: bool) -> &mut StandardBuilder
fn heading(&mut self, yes: bool) -> &mut StandardBuilder
fn hyperlink(&mut self, config: HyperlinkConfig) -> &mut StandardBuilder
fn max_columns(&mut self, limit: Option<u64>) -> &mut StandardBuilder
fn max_columns_preview(&mut self, yes: bool) -> &mut StandardBuilder
fn new() -> StandardBuilder
fn only_matching(&mut self, yes: bool) -> &mut StandardBuilder
fn path(&mut self, yes: bool) -> &mut StandardBuilder
fn path_terminator(&mut self, terminator: Option<u8>) -> &mut StandardBuilder
fn per_match(&mut self, yes: bool) -> &mut StandardBuilder
fn per_match_one_line(&mut self, yes: bool) -> &mut StandardBuilder
fn replacement(&mut self, replacement: Option<Vec<u8>>) -> &mut StandardBuilder
fn separator_context(&mut self, sep: Option<Vec<u8>>) -> &mut StandardBuilder
fn separator_field_context(&mut self, sep: Vec<u8>) -> &mut StandardBuilder
fn separator_field_match(&mut self, sep: Vec<u8>) -> &mut StandardBuilder
fn separator_path(&mut self, sep: Option<u8>) -> &mut StandardBuilder
fn separator_search(&mut self, sep: Option<Vec<u8>>) -> &mut StandardBuilder
fn stats(&mut self, yes: bool) -> &mut StandardBuilder
fn trim_ascii(&mut self, yes: bool) -> &mut StandardBuilder
```

A builder for the "standard" grep-like printer.

The builder permits configuring how the printer behaves. Configurable
behavior includes, but is not limited to, limiting the number of matches,
tweaking separators, executing pattern replacements, recording statistics
and setting colors.

Some configuration options, such as the display of line numbers or
contextual lines, are drawn directly from the
`grep_searcher::Searcher`'s configuration.

Once a `Standard` printer is built, its configuration cannot be changed.

---

## StandardSink

`struct` · `grep_printer::standard::StandardSink`

Also reachable as `grep_printer::StandardSink`

```rust
struct StandardSink<'p, 's, M: Matcher, W>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Debug

**Methods** (4)

```rust
fn binary_byte_offset(&self) -> Option<u64>
fn has_match(&self) -> bool
fn match_count(&self) -> u64
fn stats(&self) -> Option<&Stats>
```

**via `grep_searcher::sink::Sink`**

```rust
fn begin(&mut self, _searcher: &Searcher) -> Result<bool, io::Error>
fn binary_data(&mut self, searcher: &Searcher, binary_byte_offset: u64) -> Result<bool, io::Error>
fn context(&mut self, searcher: &Searcher, ctx: &SinkContext<'_>) -> Result<bool, io::Error>
fn context_break(&mut self, searcher: &Searcher) -> Result<bool, io::Error>
fn finish(&mut self, searcher: &Searcher, finish: &SinkFinish) -> Result<(), io::Error>
fn matched(&mut self, searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

An implementation of `Sink` associated with a matcher and an optional file
path for the standard printer.

A `Sink` can be created via the [`Standard::sink`] or
[`Standard::sink_with_path`] methods, depending on whether you want to
include a file path in the printer's output.

Building a `StandardSink` is cheap, and callers should create a new one
for each thing that is searched. After a search has completed, callers may
query this sink for information such as whether a match occurred or whether
binary data was found (and if so, the offset at which it occurred).

This type is generic over a few type parameters:

* `'p` refers to the lifetime of the file path, if one is provided. When
no file path is given, then this is `'static`.
* `'s` refers to the lifetime of the [`Standard`] printer that this type
borrows.
* `M` refers to the type of matcher used by
`grep_searcher::Searcher` that is reporting results to this sink.
* `W` refers to the underlying writer that this printer is writing its
output to.

---
