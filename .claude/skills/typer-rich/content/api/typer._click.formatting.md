# `typer._click.formatting`

Distribution: `typer`

## FORCED_WIDTH

`typer._click.formatting.FORCED_WIDTH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
FORCED_WIDTH: int | None = None
```

## HelpFormatter

`typer._click.formatting.HelpFormatter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class HelpFormatter
```

**Also exported as** `typer._click.HelpFormatter`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (15)**

- `buffer: list[str] = []`  _instance-attribute_
- `current_indent: int = 0`  _instance-attribute_
- `def dedent(self) -> None`
  Decreases the indentation.
- `def getvalue(self) -> str`
  Returns the buffer contents.
- `def indent(self) -> None`
  Increases the indentation.
- `indent_increment = indent_increment`  _instance-attribute_
- `def indentation(self) -> Iterator[None]`
  A context manager that increases the indentation.
- `def section(self, name: str) -> Iterator[None]`
  Helpful context manager that writes a paragraph, a heading, and the indents.
- `width = width`  _instance-attribute_
- `def write(self, string: str) -> None`
  Writes a unicode string into the internal buffer.
- `def write_dl(self, rows: Sequence[tuple[str, str]], col_max: int = 30, col_spacing: int = 2) -> None`
  Writes a definition list into the buffer.  This is how options and commands are usually formatted.
- `def write_heading(self, heading: str) -> None`
  Writes a heading into the buffer.
- `def write_paragraph(self) -> None`
  Writes a paragraph into the buffer.
- `def write_text(self, text: str) -> None`
  Writes re-indented text into the buffer.  This rewraps and preserves paragraphs.
- `def write_usage(self, prog: str, args: str = '', prefix: str | None = None) -> None`
  Writes a usage line into the buffer.

This class helps with formatting text-based help pages.  It's
usually just needed for very special internal cases, but it's also
exposed so that developers can write their own fancy outputs.

At present, it always writes into memory.


## iter_rows

`typer._click.formatting.iter_rows`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def iter_rows(rows: Iterable[tuple[str, str]], col_count: int) -> Iterator[tuple[str, ...]]
```

## join_options

`typer._click.formatting.join_options`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def join_options(options: Sequence[str]) -> tuple[str, bool]
```

Given a list of option strings this joins them in the most appropriate
way and returns them in the form ``(formatted_string,
any_prefix_is_slash)`` where the second item in the tuple is a flag that
indicates if any of the option prefixes was a slash.


## measure_table

`typer._click.formatting.measure_table`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def measure_table(rows: Iterable[tuple[str, str]]) -> tuple[int, ...]
```

## wrap_text

`typer._click.formatting.wrap_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def wrap_text(text: str, width: int = 78, initial_indent: str = '', subsequent_indent: str = '', preserve_paragraphs: bool = False) -> str
```

A helper function that intelligently wraps text.  By default, it
assumes that it operates on a single paragraph of text but if the
`preserve_paragraphs` parameter is provided it will intelligently
handle paragraphs (defined by two empty lines).

If paragraphs are handled, a paragraph can be prefixed with an empty
line containing the ``\b`` character (``\x08``) to indicate that
no rewrapping should happen in that block.


