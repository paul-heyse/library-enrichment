# `rich.segment`

Distribution: `rich`

## ControlCode

`rich.segment.ControlCode`

```python
ControlCode = Union[Tuple[ControlType], Tuple[ControlType, Union[int, str]], Tuple[ControlType, int, int]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'tuple[ControlType] | tuple[ControlType, int | str] | tuple[ControlType, int, int]'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## code

`rich.segment.code`

```python
code = 'from rich.console import Console\nconsole = Console()\ntext = Text.from_markup("Hello, [bold magenta]World[/]!")\nconsole.print(text)'
```

**Inferred type** (`ty`, not declared in the source): `Literal["from rich.console import Console\nconsole = Console()\ntext = Text.from_markup(\"Hello, [bold magenta]World[/]!\")\nconsole.print(text)"]`

## console

`rich.segment.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## fragments

`rich.segment.fragments`

```python
fragments = list(console.render(text))
```

**Inferred type** (`ty`, not declared in the source): `list[Segment]`

## text

`rich.segment.text`

```python
text = Text.from_markup('Hello, [bold magenta]World[/]!')
```

**Inferred type** (`ty`, not declared in the source): `Text`

## ControlType

`rich.segment.ControlType`

```python
class ControlType(IntEnum)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `IntEnum`

**Declared members (16)**

- `BELL = 1`  _class-attribute, instance-attribute_
- `CARRIAGE_RETURN = 2`  _class-attribute, instance-attribute_
- `CLEAR = 4`  _class-attribute, instance-attribute_
- `CURSOR_BACKWARD = 12`  _class-attribute, instance-attribute_
- `CURSOR_DOWN = 10`  _class-attribute, instance-attribute_
- `CURSOR_FORWARD = 11`  _class-attribute, instance-attribute_
- `CURSOR_MOVE_TO = 14`  _class-attribute, instance-attribute_
- `CURSOR_MOVE_TO_COLUMN = 13`  _class-attribute, instance-attribute_
- `CURSOR_UP = 9`  _class-attribute, instance-attribute_
- `DISABLE_ALT_SCREEN = 8`  _class-attribute, instance-attribute_
- `ENABLE_ALT_SCREEN = 7`  _class-attribute, instance-attribute_
- `ERASE_IN_LINE = 15`  _class-attribute, instance-attribute_
- `HIDE_CURSOR = 6`  _class-attribute, instance-attribute_
- `HOME = 3`  _class-attribute, instance-attribute_
- `SET_WINDOW_TITLE = 16`  _class-attribute, instance-attribute_
- `SHOW_CURSOR = 5`  _class-attribute, instance-attribute_

Non-printable control codes which typically translate to ANSI codes.


## Segment

`rich.segment.Segment`

```python
class Segment(NamedTuple)
```

_21 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (24)**

- `def adjust_line_length(cls, line: List[Segment], length: int, style: Optional[Style] = None, pad: bool = True) -> List[Segment]`  _classmethod_
  Adjust a line to a given width (cropping or padding as required).
- `def align_bottom(cls: Type[Segment], lines: List[List[Segment]], width: int, height: int, style: Style, new_lines: bool = False) -> List[List[Segment]]`  _classmethod_
  Aligns render to bottom (adds extra lines above as required).
- `def align_middle(cls: Type[Segment], lines: List[List[Segment]], width: int, height: int, style: Style, new_lines: bool = False) -> List[List[Segment]]`  _classmethod_
  Aligns lines to middle (adds extra lines to above and below as required).
- `def align_top(cls: Type[Segment], lines: List[List[Segment]], width: int, height: int, style: Style, new_lines: bool = False) -> List[List[Segment]]`  _classmethod_
  Aligns lines to top (adds extra lines to bottom as required).
- `def apply_style(cls, segments: Iterable[Segment], style: Optional[Style] = None, post_style: Optional[Style] = None) -> Iterable[Segment]`  _classmethod_
  Apply style(s) to an iterable of segments.
- `cell_length: int`  _property_
  The number of terminal cells required to display self.text.
- `control: Optional[Sequence[ControlCode]] = None`  _class-attribute, instance-attribute_
- `def divide(cls, segments: Iterable[Segment], cuts: Iterable[int]) -> Iterable[List[Segment]]`  _classmethod_
  Divides an iterable of segments in to portions.
- `def filter_control(cls, segments: Iterable[Segment], is_control: bool = False) -> Iterable[Segment]`  _classmethod_
  Filter segments by ``is_control`` attribute.
- `def get_line_length(cls, line: List[Segment]) -> int`  _classmethod_
  Get the length of list of segments.
- `def get_shape(cls, lines: List[List[Segment]]) -> Tuple[int, int]`  _classmethod_
  Get the shape (enclosing rectangle) of a list of lines.
- `is_control: bool`  _property_
  Check if the segment contains control codes.
- `def line(cls) -> Segment`  _classmethod_
  Make a new line segment.
- `def remove_color(cls, segments: Iterable[Segment]) -> Iterable[Segment]`  _classmethod_
  Remove all color from an iterable of segments.
- `def set_shape(cls, lines: List[List[Segment]], width: int, height: Optional[int] = None, style: Optional[Style] = None, new_lines: bool = False) -> List[List[Segment]]`  _classmethod_
  Set the shape of a list of lines (enclosing rectangle).
- `def simplify(cls, segments: Iterable[Segment]) -> Iterable[Segment]`  _classmethod_
  Simplify an iterable of segments by combining contiguous segments with the same style.
- `def split_and_crop_lines(cls, segments: Iterable[Segment], length: int, style: Optional[Style] = None, pad: bool = True, include_new_lines: bool = True) -> Iterable[List[Segment]]`  _classmethod_
  Split segments in to lines, and crop lines greater than a given length.
- `def split_cells(self, cut: int) -> Tuple[Segment, Segment]`
  Split segment in to two segments at the specified column.
- `def split_lines(cls, segments: Iterable[Segment]) -> Iterable[List[Segment]]`  _classmethod_
  Split a sequence of segments in to a list of lines.
- `def split_lines_terminator(cls, segments: Iterable[Segment]) -> Iterable[Tuple[List[Segment], bool]]`  _classmethod_
  Split a sequence of segments in to a list of lines and a boolean to indicate if there was a new line.
- `def strip_links(cls, segments: Iterable[Segment]) -> Iterable[Segment]`  _classmethod_
  Remove all links from an iterable of styles.
- `def strip_styles(cls, segments: Iterable[Segment]) -> Iterable[Segment]`  _classmethod_
  Remove all styles from an iterable of segments.
- `style: Optional[Style] = None`  _class-attribute, instance-attribute_
- `text: str`  _instance-attribute_

A piece of text with associated style. Segments are produced by the Console render process and
are ultimately converted in to strings to be written to the terminal.

Args:
    text (str): A piece of text.
    style (:class:`~rich.style.Style`, optional): An optional style to apply to the text.
    control (Tuple[ControlCode], optional): Optional sequence of control codes.

Attributes:
    cell_length (int): The cell length of this Segment.


## SegmentLines

`rich.segment.SegmentLines`

```python
class SegmentLines
```

**Declared members (2)**

- `lines = list(lines)`  _instance-attribute_
- `new_lines = new_lines`  _instance-attribute_

## Segments

`rich.segment.Segments`

```python
class Segments
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `new_lines = new_lines`  _instance-attribute_
- `segments = list(segments)`  _instance-attribute_

A simple renderable to render an iterable of segments. This class may be useful if
you want to print segments outside of a __rich_console__ method.

Args:
    segments (Iterable[Segment]): An iterable of segments.
    new_lines (bool, optional): Add new lines between segments. Defaults to False.


