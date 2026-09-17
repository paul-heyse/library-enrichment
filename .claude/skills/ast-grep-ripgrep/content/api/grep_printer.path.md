# `grep_printer::path`

Crate `grep-printer` · 2 public items · structured records in [`model/grep_printer.path.json`](../model/grep_printer.path.json)

## PathPrinter

`struct` · `grep_printer::path::PathPrinter`

Also reachable as `grep_printer::PathPrinter`

```rust
struct PathPrinter<W>
```

**Derives**: Debug

**Methods** (1)

```rust
fn write(&mut self, path: &Path) -> io::Result<()>
```

A printer file paths, with optional color and hyperlink support.

This printer is very similar to [`Summary`](crate::Summary) in that it
principally only emits file paths. The main difference is that this printer
doesn't actually execute any search via a `Sink` implementation, and instead
just provides a way for the caller to print paths.

A caller could just print the paths themselves, but this printer handles
a few details:

* It can normalize path separators.
* It permits configuring the terminator.
* It allows setting the color configuration in a way that is consistent
with the other printers in this crate.
* It allows setting the hyperlink format in a way that is consistent
with the other printers in this crate.

---

## PathPrinterBuilder

`struct` · `grep_printer::path::PathPrinterBuilder`

Also reachable as `grep_printer::PathPrinterBuilder`

```rust
struct PathPrinterBuilder
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn build<W: WriteColor>(&self, wtr: W) -> PathPrinter<W>
fn color_specs(&mut self, specs: ColorSpecs) -> &mut PathPrinterBuilder
fn hyperlink(&mut self, config: HyperlinkConfig) -> &mut PathPrinterBuilder
fn new() -> PathPrinterBuilder
fn separator(&mut self, sep: Option<u8>) -> &mut PathPrinterBuilder
fn terminator(&mut self, terminator: u8) -> &mut PathPrinterBuilder
```

A builder for a printer that emits file paths.

---
