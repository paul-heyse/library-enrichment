# `ruff_formatter::printer::printer_options`

Crate `ruff_formatter` · 4 public items · structured records in [`model/ruff_formatter.printer.printer_options.json`](../model/ruff_formatter.printer.printer_options.json)

## LineEnding

`enum` · `ruff_formatter::printer::printer_options::LineEnding`

Also reachable as `ruff_formatter::printer::LineEnding`

```rust
enum LineEnding
```

**Variants**: `LineFeed`, `CarriageReturnLineFeed`, `CarriageReturn`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn as_setting_str(&self) -> &'static str
const fn as_str(&self) -> &'static str
```

---

## SourceMapGeneration

`enum` · `ruff_formatter::printer::printer_options::SourceMapGeneration`

Also reachable as `ruff_formatter::printer::SourceMapGeneration`

```rust
enum SourceMapGeneration
```

**Variants**: `Disabled`, `Enabled`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
const fn is_disabled(self) -> bool
const fn is_enabled(self) -> bool
```

Configures whether the formatter and printer generate a source map that allows mapping
positions in the source document to positions in the formatted code.

---

## PrintWidth

`struct` · `ruff_formatter::printer::printer_options::PrintWidth`

Also reachable as `ruff_formatter::printer::PrintWidth`

```rust
struct PrintWidth
```

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new(width: u16) -> Self
```

**via `core::convert::From`**

```rust
fn from(width: LineWidth) -> Self
```

---

## PrinterOptions

`struct` · `ruff_formatter::printer::printer_options::PrinterOptions`

Also reachable as `ruff_formatter::prelude::PrinterOptions`, `ruff_formatter::printer::PrinterOptions`

```rust
struct PrinterOptions
```

**Fields**: `indent_width`, `indent_style`, `line_width`, `line_ending`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn with_indent(self, style: IndentStyle) -> Self
fn with_line_width(self, width: LineWidth) -> Self
fn with_tab_width(self, width: IndentWidth) -> Self
```

**via `core::convert::From`**

```rust
fn from(options: &'a O) -> Self
```

Options that affect how the [`crate::Printer`] prints the format tokens

---
