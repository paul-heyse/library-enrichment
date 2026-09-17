# `rich.box`

Distribution: `rich`

## ASCII

`rich.box.ASCII`

```python
ASCII: Box = Box('+--+\n| ||\n|-+|\n| ||\n|-+|\n|-+|\n| ||\n+--+\n', ascii=True)
```

## ASCII2

`rich.box.ASCII2`

```python
ASCII2: Box = Box('+-++\n| ||\n+-++\n| ||\n+-++\n+-++\n| ||\n+-++\n', ascii=True)
```

## ASCII_DOUBLE_HEAD

`rich.box.ASCII_DOUBLE_HEAD`

```python
ASCII_DOUBLE_HEAD: Box = Box('+-++\n| ||\n+=++\n| ||\n+-++\n+-++\n| ||\n+-++\n', ascii=True)
```

## BOXES

`rich.box.BOXES`

```python
BOXES = ['ASCII', 'ASCII2', 'ASCII_DOUBLE_HEAD', 'SQUARE', 'SQUARE_DOUBLE_HEAD', 'MINIMAL', 'MINIMAL_HEAVY_HEAD', 'MINIMAL_DOUBLE_HEAD', 'SIMPLE', 'SIMPLE_HEAD', 'SIMPLE_HEAVY', 'HORIZONTALS', 'ROUNDED', 'HEAVY', 'HEAVY_EDGE', 'HEAVY_HEAD', 'DOUBLE', 'DOUBLE_EDGE', 'MARKDOWN']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## DOUBLE

`rich.box.DOUBLE`

```python
DOUBLE: Box = Box('╔═╦╗\n║ ║║\n╠═╬╣\n║ ║║\n╠═╬╣\n╠═╬╣\n║ ║║\n╚═╩╝\n')
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DOUBLE_EDGE

`rich.box.DOUBLE_EDGE`

```python
DOUBLE_EDGE: Box = Box('╔═╤╗\n║ │║\n╟─┼╢\n║ │║\n╟─┼╢\n╟─┼╢\n║ │║\n╚═╧╝\n')
```

## HEAVY

`rich.box.HEAVY`

```python
HEAVY: Box = Box('┏━┳┓\n┃ ┃┃\n┣━╋┫\n┃ ┃┃\n┣━╋┫\n┣━╋┫\n┃ ┃┃\n┗━┻┛\n')
```

## HEAVY_EDGE

`rich.box.HEAVY_EDGE`

```python
HEAVY_EDGE: Box = Box('┏━┯┓\n┃ │┃\n┠─┼┨\n┃ │┃\n┠─┼┨\n┠─┼┨\n┃ │┃\n┗━┷┛\n')
```

## HEAVY_HEAD

`rich.box.HEAVY_HEAD`

```python
HEAVY_HEAD: Box = Box('┏━┳┓\n┃ ┃┃\n┡━╇┩\n│ ││\n├─┼┤\n├─┼┤\n│ ││\n└─┴┘\n')
```

## HORIZONTALS

`rich.box.HORIZONTALS`

```python
HORIZONTALS: Box = Box(' ── \n    \n ── \n    \n ── \n ── \n    \n ── \n')
```

## LEGACY_WINDOWS_SUBSTITUTIONS

`rich.box.LEGACY_WINDOWS_SUBSTITUTIONS`

```python
LEGACY_WINDOWS_SUBSTITUTIONS = {ROUNDED: SQUARE, MINIMAL_HEAVY_HEAD: MINIMAL, SIMPLE_HEAVY: SIMPLE, HEAVY: SQUARE, HEAVY_EDGE: SQUARE, HEAVY_HEAD: SQUARE}
```

**Inferred type** (`ty`, not declared in the source): `dict[Box, Box]`

## MARKDOWN

`rich.box.MARKDOWN`

```python
MARKDOWN: Box = Box('    \n| ||\n|-||\n| ||\n|-||\n|-||\n| ||\n    \n', ascii=True)
```

## MINIMAL

`rich.box.MINIMAL`

```python
MINIMAL: Box = Box('  ╷ \n  │ \n╶─┼╴\n  │ \n╶─┼╴\n╶─┼╴\n  │ \n  ╵ \n')
```

## MINIMAL_DOUBLE_HEAD

`rich.box.MINIMAL_DOUBLE_HEAD`

```python
MINIMAL_DOUBLE_HEAD: Box = Box('  ╷ \n  │ \n ═╪ \n  │ \n ─┼ \n ─┼ \n  │ \n  ╵ \n')
```

## MINIMAL_HEAVY_HEAD

`rich.box.MINIMAL_HEAVY_HEAD`

```python
MINIMAL_HEAVY_HEAD: Box = Box('  ╷ \n  │ \n╺━┿╸\n  │ \n╶─┼╴\n╶─┼╴\n  │ \n  ╵ \n')
```

## PLAIN_HEADED_SUBSTITUTIONS

`rich.box.PLAIN_HEADED_SUBSTITUTIONS`

```python
PLAIN_HEADED_SUBSTITUTIONS = {HEAVY_HEAD: SQUARE, SQUARE_DOUBLE_HEAD: SQUARE, MINIMAL_DOUBLE_HEAD: MINIMAL, MINIMAL_HEAVY_HEAD: MINIMAL, ASCII_DOUBLE_HEAD: ASCII2}
```

**Inferred type** (`ty`, not declared in the source): `dict[Box, Box]`

## ROUNDED

`rich.box.ROUNDED`

```python
ROUNDED: Box = Box('╭─┬╮\n│ ││\n├─┼┤\n│ ││\n├─┼┤\n├─┼┤\n│ ││\n╰─┴╯\n')
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## SIMPLE

`rich.box.SIMPLE`

```python
SIMPLE: Box = Box('    \n    \n ── \n    \n    \n ── \n    \n    \n')
```

## SIMPLE_HEAD

`rich.box.SIMPLE_HEAD`

```python
SIMPLE_HEAD: Box = Box('    \n    \n ── \n    \n    \n    \n    \n    \n')
```

## SIMPLE_HEAVY

`rich.box.SIMPLE_HEAVY`

```python
SIMPLE_HEAVY: Box = Box('    \n    \n ━━ \n    \n    \n ━━ \n    \n    \n')
```

## SQUARE

`rich.box.SQUARE`

```python
SQUARE: Box = Box('┌─┬┐\n│ ││\n├─┼┤\n│ ││\n├─┼┤\n├─┼┤\n│ ││\n└─┴┘\n')
```

## SQUARE_DOUBLE_HEAD

`rich.box.SQUARE_DOUBLE_HEAD`

```python
SQUARE_DOUBLE_HEAD: Box = Box('┌─┬┐\n│ ││\n╞═╪╡\n│ ││\n├─┼┤\n├─┼┤\n│ ││\n└─┴┘\n')
```

## columns

`rich.box.columns`

```python
columns = Columns(expand=True, padding=2)
```

**Inferred type** (`ty`, not declared in the source): `Columns`

## console

`rich.box.console`

```python
console = Console(record=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## table

`rich.box.table`

```python
table = Table(show_footer=True, style='dim', border_style='not dim', expand=True)
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Box

`rich.box.Box`

```python
class Box
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `ascii = ascii`  _instance-attribute_
- `def get_bottom(self, widths: Iterable[int]) -> str`
  Get the bottom of a simple box.
- `def get_plain_headed_box(self) -> Box`
  If this box uses special characters for the borders of the header, then return the equivalent box that does not.
- `def get_row(self, widths: Iterable[int], level: Literal['head', 'row', 'foot', 'mid'] = 'row', edge: bool = True) -> str`
  Get the top of a simple box.
- `def get_top(self, widths: Iterable[int]) -> str`
  Get the top of a simple box.
- `def substitute(self, options: ConsoleOptions, safe: bool = True) -> Box`
  Substitute this box for another if it won't render due to platform issues.

Defines characters to render boxes.

┌─┬┐ top
│ ││ head
├─┼┤ head_row
│ ││ mid
├─┼┤ row
├─┼┤ foot_row
│ ││ foot
└─┴┘ bottom

Args:
    box (str): Characters making up box.
    ascii (bool, optional): True if this box uses ascii characters only. Default is False.


