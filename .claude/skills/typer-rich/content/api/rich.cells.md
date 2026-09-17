# `rich.cells`

Distribution: `rich`

## CellSpan

`rich.cells.CellSpan`

```python
CellSpan = Tuple[int, int, int]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'tuple[int, int, int]'> ````

## _SINGLE_CELLS

`rich.cells._SINGLE_CELLS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SINGLE_CELLS = frozenset([character for _start, _end in _SINGLE_CELL_UNICODE_RANGES for character in map(chr, range(_start, _end + 1))])
```

## _SINGLE_CELL_UNICODE_RANGES

`rich.cells._SINGLE_CELL_UNICODE_RANGES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SINGLE_CELL_UNICODE_RANGES: list[tuple[int, int]] = [(32, 126), (160, 172), (174, 767), (880, 1154), (9472, 9724), (10240, 10495)]
```

## _is_single_cell_widths

`rich.cells._is_single_cell_widths`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_is_single_cell_widths: Callable[[str], bool] = _SINGLE_CELLS.issuperset
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _span_get_cell_len

`rich.cells._span_get_cell_len`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_span_get_cell_len = itemgetter(2)
```

## CellTable

`rich.cells.CellTable`

```python
class CellTable(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (3)**

- `narrow_to_wide: frozenset[str]`  _instance-attribute_
- `unicode_version: str`  _instance-attribute_
- `widths: Sequence[tuple[int, int, int]]`  _instance-attribute_

Contains unicode data required to measure the cell widths of glyphs.


## _cell_len

`rich.cells._cell_len`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _cell_len(text: str, unicode_version: str) -> int
```

Get the cell length of a string (length as it appears in the terminal).

Args:
    text: String to measure.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    Length of string in terminal cells.


## _split_text

`rich.cells._split_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _split_text(text: str, cell_position: int, unicode_version: str = 'auto') -> tuple[str, str]
```

Split text by cell position.

If the cell position falls within a double width character, it is converted to two spaces.

Args:
    text: Text to split.
    cell_position Offset in cells.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    Tuple to two split strings.


## cached_cell_len

`rich.cells.cached_cell_len`

```python
def cached_cell_len(text: str, unicode_version: str = 'auto') -> int
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the number of cells required to display text.

This method always caches, which may use up a lot of memory. It is recommended to use
`cell_len` over this method.

Args:
    text (str): Text to display.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    int: Get the number of cells required to display text.


## cell_len

`rich.cells.cell_len`

```python
def cell_len(text: str, unicode_version: str = 'auto') -> int
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the cell length of a string (length as it appears in the terminal).

Args:
    text: String to measure.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    Length of string in terminal cells.


## chop_cells

`rich.cells.chop_cells`

```python
def chop_cells(text: str, width: int, unicode_version: str = 'auto') -> list[str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Split text into lines such that each line fits within the available (cell) width.

Args:
    text: The text to fold such that it fits in the given width.
    width: The width available (number of cells).

Returns:
    A list of strings such that each string in the list has cell width
    less than or equal to the available width.


## get_character_cell_size

`rich.cells.get_character_cell_size`

```python
def get_character_cell_size(character: str, unicode_version: str = 'auto') -> int
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get the cell size of a character.

Args:
    character (str): A single character.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    int: Number of cells (0, 1 or 2) occupied by that character.


## set_cell_size

`rich.cells.set_cell_size`

```python
def set_cell_size(text: str, total: int, unicode_version: str = 'auto') -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Adjust a string by cropping or padding with spaces such that it fits within the given number of cells.

Args:
    text: String to adjust.
    total: Desired size in cells.
    unicode_version: Unicode version.

Returns:
    A string with cell size equal to total.


## split_graphemes

`rich.cells.split_graphemes`

```python
def split_graphemes(text: str, unicode_version: str = 'auto') -> 'tuple[list[CellSpan], int]'
```

Divide text into spans that define a single grapheme, and additionally return the cell length of the whole string.

The returned spans will cover every index in the string, with no gaps. It is possible for some graphemes to have a cell length of zero.
This can occur for nonsense strings like two zero width joiners, or for control codes that don't contribute to the grapheme size.

Args:
    text: String to split.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    A tuple of a list of *spans* and the cell length of the entire string. A span is a list of tuples
        of three values consisting of (<START>, <END>, <CELL LENGTH>), where START and END are string indices,
        and CELL LENGTH is the cell length of the single grapheme.


## split_text

`rich.cells.split_text`

```python
def split_text(text: str, cell_position: int, unicode_version: str = 'auto') -> tuple[str, str]
```

Split text by cell position.

If the cell position falls within a double width character, it is converted to two spaces.

Args:
    text: Text to split.
    cell_position Offset in cells.
    unicode_version: Unicode version, `"auto"` to auto detect, `"latest"` for the latest unicode version.

Returns:
    Tuple to two split strings.


