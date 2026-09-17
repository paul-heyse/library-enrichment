# `rich.table`

Distribution: `rich`

## console

`rich.table.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## highlight

`rich.table.highlight`

```python
highlight = ReprHighlighter()
```

**Inferred type** (`ty`, not declared in the source): `ReprHighlighter`

## table

`rich.table.table`

```python
table = Table(title='Star Wars Movies', caption='Rich example table', caption_justify='right')
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Column

`rich.table.Column`

```python
class Column
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (16)**

- `cells: Iterable[RenderableType]`  _property_
  Get all cells in the column, not including header.
- `flexible: bool`  _property_
  Check if this column is flexible.
- `footer: RenderableType = ''`  _class-attribute, instance-attribute_
  RenderableType: Renderable for the footer (typically a string)
- `footer_style: StyleType = ''`  _class-attribute, instance-attribute_
  StyleType: The style of the footer.
- `header: RenderableType = ''`  _class-attribute, instance-attribute_
  RenderableType: Renderable for the header (typically a string)
- `header_style: StyleType = ''`  _class-attribute, instance-attribute_
  StyleType: The style of the header.
- `highlight: bool = False`  _class-attribute, instance-attribute_
  bool: Apply highlighter to column. Defaults to ``False``.
- `justify: JustifyMethod = 'left'`  _class-attribute, instance-attribute_
  str: How to justify text within the column ("left", "center", "right", or "full")
- `max_width: Optional[int] = None`  _class-attribute, instance-attribute_
  Optional[int]: Maximum width of column, or ``None`` for no maximum. Defaults to None.
- `min_width: Optional[int] = None`  _class-attribute, instance-attribute_
  Optional[int]: Minimum width of column, or ``None`` for no minimum. Defaults to None.
- `no_wrap: bool = False`  _class-attribute, instance-attribute_
  bool: Prevent wrapping of text within the column. Defaults to ``False``.
- `overflow: OverflowMethod = 'ellipsis'`  _class-attribute, instance-attribute_
  str: Overflow method.
- `ratio: Optional[int] = None`  _class-attribute, instance-attribute_
  Optional[int]: Ratio to use when calculating column width, or ``None`` (default) to adapt to column contents.
- `style: StyleType = ''`  _class-attribute, instance-attribute_
  StyleType: The style of the column.
- `vertical: VerticalAlignMethod = 'top'`  _class-attribute, instance-attribute_
  str: How to vertically align content ("top", "middle", or "bottom")
- `width: Optional[int] = None`  _class-attribute, instance-attribute_
  Optional[int]: Width of the column, or ``None`` (default) to auto calculate width.

Defines a column within a ~Table.

Args:
    title (Union[str, Text], optional): The title of the table rendered at the top. Defaults to None.
    caption (Union[str, Text], optional): The table caption rendered below. Defaults to None.
    width (int, optional): The width in characters of the table, or ``None`` to automatically fit. Defaults to None.
    min_width (Optional[int], optional): The minimum width of the table, or ``None`` for no minimum. Defaults to None.
    box (box.Box, optional): One of the constants in box.py used to draw the edges (see :ref:`appendix_box`), or ``None`` for no box lines. Defaults to box.HEAVY_HEAD.
    safe_box (Optional[bool], optional): Disable box characters that don't display on windows legacy terminal with *raster* fonts. Defaults to True.
    padding (PaddingDimensions, optional): Padding for cells (top, right, bottom, left). Defaults to (0, 1).
    collapse_padding (bool, optional): Enable collapsing of padding around cells. Defaults to False.
    pad_edge (bool, optional): Enable padding of edge cells. Defaults to True.
    show_header (bool, optional): Show a header row. Defaults to True.
    show_footer (bool, optional): Show a footer row. Defaults to False.
    show_edge (bool, optional): Draw a box around the outside of the table. Defaults to True.
    show_lines (bool, optional): Draw lines between every row. Defaults to False.
    leading (int, optional): Number of blank lines between rows (precludes ``show_lines``). Defaults to 0.
    style (Union[str, Style], optional): Default style for the table. Defaults to "none".
    row_styles (List[Union, str], optional): Optional list of row styles, if more than one style is given then the styles will alternate. Defaults to None.
    header_style (Union[str, Style], optional): Style of the header. Defaults to "table.header".
    footer_style (Union[str, Style], optional): Style of the footer. Defaults to "table.footer".
    border_style (Union[str, Style], optional): Style of the border. Defaults to None.
    title_style (Union[str, Style], optional): Style of the title. Defaults to None.
    caption_style (Union[str, Style], optional): Style of the caption. Defaults to None.
    title_justify (str, optional): Justify method for title. Defaults to "center".
    caption_justify (str, optional): Justify method for caption. Defaults to "center".
    highlight (bool, optional): Highlight cell contents (if str). Defaults to False.


## Row

`rich.table.Row`

```python
class Row
```

**Declared members (2)**

- `end_section: bool = False`  _class-attribute, instance-attribute_
  Indicated end of section, which will force a line beneath the row.
- `style: Optional[StyleType] = None`  _class-attribute, instance-attribute_
  Style to apply to row.

Information regarding a row.


## Table

`rich.table.Table`

```python
class Table(JupyterMixin)
```

_16 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (33)**

- `def add_column(self, header: RenderableType = '', footer: RenderableType = '', header_style: Optional[StyleType] = None, highlight: Optional[bool] = None, footer_style: Optional[StyleType] = None, style: Optional[StyleType] = None, justify: JustifyMethod = 'left', vertical: VerticalAlignMethod = 'top', overflow: OverflowMethod = 'ellipsis', width: Optional[int] = None, min_width: Optional[int] = None, max_width: Optional[int] = None, ratio: Optional[int] = None, no_wrap: bool = False) -> None`
  Add a column to the table.
- `def add_row(self, renderables: Optional[RenderableType] = (), style: Optional[StyleType] = None, end_section: bool = False) -> None`
  Add a row of renderables.
- `def add_section(self) -> None`
  Add a new section (draw a line after current row).
- `border_style = border_style`  _instance-attribute_
- `box = box`  _instance-attribute_
- `caption = caption`  _instance-attribute_
- `caption_justify: JustifyMethod = caption_justify`  _instance-attribute_
- `caption_style = caption_style`  _instance-attribute_
- `collapse_padding = collapse_padding`  _instance-attribute_
- `columns: List[Column] = []`  _instance-attribute_
- `expand: bool`  _property, writable_
  Setting a non-None self.width implies expand.
- `footer_style = footer_style or ''`  _instance-attribute_
- `def get_row_style(self, console: Console, index: int) -> StyleType`
  Get the current row style.
- `def grid(cls, headers: Union[Column, str] = (), padding: PaddingDimensions = 0, collapse_padding: bool = True, pad_edge: bool = False, expand: bool = False) -> Table`  _classmethod_
  Get a table with no lines, headers, or footer.
- `header_style = header_style or ''`  _instance-attribute_
- `highlight = highlight`  _instance-attribute_
- `leading = leading`  _instance-attribute_
- `min_width = min_width`  _instance-attribute_
- `pad_edge = pad_edge`  _instance-attribute_
- `padding: Tuple[int, int, int, int]`  _property, writable_
  Get cell padding.
- `row_count: int`  _property_
  Get the current number of rows.
- `row_styles: Sequence[StyleType] = list(row_styles or [])`  _instance-attribute_
- `rows: List[Row] = []`  _instance-attribute_
- `safe_box = safe_box`  _instance-attribute_
- `show_edge = show_edge`  _instance-attribute_
- `show_footer = show_footer`  _instance-attribute_
- `show_header = show_header`  _instance-attribute_
- `show_lines = show_lines`  _instance-attribute_
- `style = style`  _instance-attribute_
- `title = title`  _instance-attribute_
- `title_justify: JustifyMethod = title_justify`  _instance-attribute_
- `title_style = title_style`  _instance-attribute_
- `width = width`  _instance-attribute_

A console renderable to draw a table.

Args:
    *headers (Union[Column, str]): Column headers, either as a string, or :class:`~rich.table.Column` instance.
    title (Union[str, Text], optional): The title of the table rendered at the top. Defaults to None.
    caption (Union[str, Text], optional): The table caption rendered below. Defaults to None.
    width (int, optional): The width in characters of the table, or ``None`` to automatically fit. Defaults to None.
    min_width (Optional[int], optional): The minimum width of the table, or ``None`` for no minimum. Defaults to None.
    box (box.Box, optional): One of the constants in box.py used to draw the edges (see :ref:`appendix_box`), or ``None`` for no box lines. Defaults to box.HEAVY_HEAD.
    safe_box (Optional[bool], optional): Disable box characters that don't display on windows legacy terminal with *raster* fonts. Defaults to True.
    padding (PaddingDimensions, optional): Padding for cells (top, right, bottom, left). Defaults to (0, 1).
    collapse_padding (bool, optional): Enable collapsing of padding around cells. Defaults to False.
    pad_edge (bool, optional): Enable padding of edge cells. Defaults to True.
    expand (bool, optional): Expand the table to fit the available space if ``True``, otherwise the table width will be auto-calculated. Defaults to False.
    show_header (bool, optional): Show a header row. Defaults to True.
    show_footer (bool, optional): Show a footer row. Defaults to False.
    show_edge (bool, optional): Draw a box around the outside of the table. Defaults to True.
    show_lines (bool, optional): Draw lines between every row. Defaults to False.
    leading (int, optional): Number of blank lines between rows (precludes ``show_lines``). Defaults to 0.
    style (Union[str, Style], optional): Default style for the table. Defaults to "none".
    row_styles (List[Union, str], optional): Optional list of row styles, if more than one style is given then the styles will alternate. Defaults to None.
    header_style (Union[str, Style], optional): Style of the header. Defaults to "table.header".
    footer_style (Union[str, Style], optional): Style of the footer. Defaults to "table.footer".
    border_style (Union[str, Style], optional): Style of the border. Defaults to None.
    title_style (Union[str, Style], optional): Style of the title. Defaults to None.
    caption_style (Union[str, Style], optional): Style of the caption. Defaults to None.
    title_justify (str, optional): Justify method for title. Defaults to "center".
    caption_justify (str, optional): Justify method for caption. Defaults to "center".
    highlight (bool, optional): Highlight cell contents (if str). Defaults to False.


## _Cell

`rich.table._Cell`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Cell(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (3)**

- `renderable: RenderableType`  _instance-attribute_
  Cell renderable.
- `style: StyleType`  _instance-attribute_
  Style to apply to cell.
- `vertical: VerticalAlignMethod`  _instance-attribute_
  Cell vertical alignment.

A single cell in a table.


## header

`rich.table.header`

```python
def header(text: str) -> None
```

