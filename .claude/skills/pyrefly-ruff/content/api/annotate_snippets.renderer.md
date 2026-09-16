# `annotate_snippets::renderer`

Crate `ruff_annotate_snippets` · 14 public items · structured records in [`model/annotate_snippets.renderer.json`](../model/annotate_snippets.renderer.json)

## DEFAULT_ADDITION_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_ADDITION_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_ADDITION_STYLE`

```rust
const DEFAULT_ADDITION_STYLE: Style = _
```

[`Renderer::addition`] applied by [`Renderer::styled`]

---

## DEFAULT_CONTEXT_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_CONTEXT_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_CONTEXT_STYLE`

```rust
const DEFAULT_CONTEXT_STYLE: Style = _
```

[`Renderer::context`] applied by [`Renderer::styled`]

---

## DEFAULT_EMPHASIS_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_EMPHASIS_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_EMPHASIS_STYLE`

```rust
const DEFAULT_EMPHASIS_STYLE: Style = _
```

[`Renderer::emphasis`] applied by [`Renderer::styled`]

---

## DEFAULT_ERROR_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_ERROR_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_ERROR_STYLE`

```rust
const DEFAULT_ERROR_STYLE: Style = _
```

[`Renderer::error`] applied by [`Renderer::styled`]

---

## DEFAULT_HELP_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_HELP_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_HELP_STYLE`

```rust
const DEFAULT_HELP_STYLE: Style = _
```

[`Renderer::help`] applied by [`Renderer::styled`]

---

## DEFAULT_INFO_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_INFO_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_INFO_STYLE`

```rust
const DEFAULT_INFO_STYLE: Style = _
```

[`Renderer::info`] applied by [`Renderer::styled`]

---

## DEFAULT_LINE_NUM_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_LINE_NUM_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_LINE_NUM_STYLE`

```rust
const DEFAULT_LINE_NUM_STYLE: Style = _
```

[`Renderer::line_num`] applied by [`Renderer::styled`]

---

## DEFAULT_NONE_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_NONE_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_NONE_STYLE`

```rust
const DEFAULT_NONE_STYLE: Style = _
```

[`Renderer::none`] applied by [`Renderer::styled`]

---

## DEFAULT_NOTE_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_NOTE_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_NOTE_STYLE`

```rust
const DEFAULT_NOTE_STYLE: Style = _
```

[`Renderer::note`] applied by [`Renderer::styled`]

---

## DEFAULT_REMOVAL_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_REMOVAL_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_REMOVAL_STYLE`

```rust
const DEFAULT_REMOVAL_STYLE: Style = _
```

[`Renderer::removal`] applied by [`Renderer::styled`]

---

## DEFAULT_TERM_WIDTH

`constant` · `annotate_snippets::renderer::DEFAULT_TERM_WIDTH`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_TERM_WIDTH`

```rust
const DEFAULT_TERM_WIDTH: usize = 140
```

See [`Renderer::term_width`]

---

## DEFAULT_WARNING_STYLE

`constant` · `annotate_snippets::renderer::DEFAULT_WARNING_STYLE`

Also reachable as `ruff_annotate_snippets::renderer::DEFAULT_WARNING_STYLE`

```rust
const DEFAULT_WARNING_STYLE: Style = _
```

[`Renderer::warning`] applied by [`Renderer::styled`]

---

## DecorStyle

`enum` · `annotate_snippets::renderer::DecorStyle`

Also reachable as `ruff_annotate_snippets::renderer::DecorStyle`

```rust
enum DecorStyle
```

**Variants**: `Ascii`, `Unicode`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

The character set for rendering for decor

---

## Renderer

`struct` · `annotate_snippets::renderer::Renderer`

Also reachable as `ruff_annotate_snippets::Renderer`, `ruff_annotate_snippets::renderer::Renderer`

```rust
struct Renderer
```

**Derives**: Clone, Debug

**Methods** (20)

```rust
const fn addition(self, style: Style) -> Self
const fn anonymized_line_numbers(self, anonymized_line_numbers: bool) -> Self
const fn context(self, style: Style) -> Self
const fn cut_indicator(self, cut: &'static str) -> Self
const fn decor_style(self, decor_style: DecorStyle) -> Self
const fn emphasis(self, style: Style) -> Self
const fn error(self, style: Style) -> Self
const fn help(self, style: Style) -> Self
const fn hyperlink(self, hyperlink: bool) -> Self
const fn info(self, style: Style) -> Self
const fn line_num(self, style: Style) -> Self
const fn none(self, style: Style) -> Self
const fn note(self, style: Style) -> Self
const fn plain() -> Self
const fn removal(self, style: Style) -> Self
fn render(&self, groups: Report<'_>) -> String
const fn short_message(self, short_message: bool) -> Self
const fn styled() -> Self
const fn term_width(self, term_width: usize) -> Self
const fn warning(self, style: Style) -> Self
```

The [Renderer] for a [`Report`]

The caller is expected to detect any relevant terminal features and configure the renderer,
including
- ANSI Escape code support (always outputted with [`Renderer::styled`])
- Terminal width ([`Renderer::term_width`])
- Unicode support ([`Renderer::decor_style`])

# Example

```
# use annotate_snippets::*;
# use annotate_snippets::renderer::*;
# use annotate_snippets::Level;
let report = // ...
# &[Group::with_title(
#     Level::ERROR
#         .primary_title("unresolved import `baz::zed`")
#         .id("E0432")
# )];

let renderer = Renderer::styled();
let output = renderer.render(report);
anstream::println!("{output}");
```

---
