# `grep_printer::summary`

Crate `grep-printer` · 4 public items · structured records in [`model/grep_printer.summary.json`](../model/grep_printer.summary.json)

## SummaryKind

`enum` · `grep_printer::summary::SummaryKind`

Also reachable as `grep_printer::SummaryKind`

```rust
enum SummaryKind
```

**Variants**: `Count`, `CountMatches`, `PathWithMatch`, `PathWithoutMatch`, `QuietWithMatch`, `QuietWithoutMatch`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The type of summary output (if any) to print.

---

## Summary

`struct` · `grep_printer::summary::Summary`

Also reachable as `grep_printer::Summary`

```rust
struct Summary<W>
```

**Derives**: Clone, Debug

**Methods** (7)

```rust
fn get_mut(&mut self) -> &mut W
fn has_written(&self) -> bool
fn into_inner(self) -> W
fn new(wtr: W) -> Summary<W>
fn new_no_color(wtr: W) -> Summary<NoColor<W>>
fn sink<'s, M: Matcher>(&'s mut self, matcher: M) -> SummarySink<'static, 's, M, W>
fn sink_with_path<'p, 's, M, P>(&'s mut self, matcher: M, path: &'p P) -> SummarySink<'p, 's, M, W> where M: Matcher, P: ?Sized + AsRef<Path>
```

The summary printer, which emits aggregate results from a search.

Aggregate results generally correspond to file paths and/or the number of
matches found.

A default printer can be created with either of the `Summary::new` or
`Summary::new_no_color` constructors. However, there are a number of
options that configure this printer's output. Those options can be
configured using [`SummaryBuilder`].

This type is generic over `W`, which represents any implementation of
the `termcolor::WriteColor` trait.

---

## SummaryBuilder

`struct` · `grep_printer::summary::SummaryBuilder`

Also reachable as `grep_printer::SummaryBuilder`

```rust
struct SummaryBuilder
```

**Derives**: Clone, Debug

**Methods** (12)

```rust
fn build<W: WriteColor>(&self, wtr: W) -> Summary<W>
fn build_no_color<W: io::Write>(&self, wtr: W) -> Summary<NoColor<W>>
fn color_specs(&mut self, specs: ColorSpecs) -> &mut SummaryBuilder
fn exclude_zero(&mut self, yes: bool) -> &mut SummaryBuilder
fn hyperlink(&mut self, config: HyperlinkConfig) -> &mut SummaryBuilder
fn kind(&mut self, kind: SummaryKind) -> &mut SummaryBuilder
fn new() -> SummaryBuilder
fn path(&mut self, yes: bool) -> &mut SummaryBuilder
fn path_terminator(&mut self, terminator: Option<u8>) -> &mut SummaryBuilder
fn separator_field(&mut self, sep: Vec<u8>) -> &mut SummaryBuilder
fn separator_path(&mut self, sep: Option<u8>) -> &mut SummaryBuilder
fn stats(&mut self, yes: bool) -> &mut SummaryBuilder
```

A builder for summary printer.

The builder permits configuring how the printer behaves. The summary
printer has fewer configuration options than the standard printer because
it aims to produce aggregate output about a single search (typically just
one line) instead of output for each match.

Once a `Summary` printer is built, its configuration cannot be changed.

---

## SummarySink

`struct` · `grep_printer::summary::SummarySink`

Also reachable as `grep_printer::SummarySink`

```rust
struct SummarySink<'p, 's, M: Matcher, W>
```

**Implements**: `grep_searcher::sink::Sink`

**Derives**: Debug

**Methods** (3)

```rust
fn binary_byte_offset(&self) -> Option<u64>
fn has_match(&self) -> bool
fn stats(&self) -> Option<&Stats>
```

**via `grep_searcher::sink::Sink`**

```rust
fn begin(&mut self, _searcher: &Searcher) -> Result<bool, io::Error>
fn binary_data(&mut self, searcher: &Searcher, binary_byte_offset: u64) -> Result<bool, io::Error>
fn finish(&mut self, searcher: &Searcher, finish: &SinkFinish) -> Result<(), io::Error>
fn matched(&mut self, searcher: &Searcher, mat: &SinkMatch<'_>) -> Result<bool, io::Error>
```

An implementation of `Sink` associated with a matcher and an optional file
path for the summary printer.

This type is generic over a few type parameters:

* `'p` refers to the lifetime of the file path, if one is provided. When
no file path is given, then this is `'static`.
* `'s` refers to the lifetime of the [`Summary`] printer that this type
borrows.
* `M` refers to the type of matcher used by
`grep_searcher::Searcher` that is reporting results to this sink.
* `W` refers to the underlying writer that this printer is writing its
output to.

---
