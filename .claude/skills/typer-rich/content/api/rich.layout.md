# `rich.layout`

Distribution: `rich`

## RegionMap

`rich.layout.RegionMap`

```python
RegionMap = Dict['Layout', Region]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'dict[Layout, Region]'> ````

## RenderMap

`rich.layout.RenderMap`

```python
RenderMap = Dict['Layout', LayoutRender]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'dict[Layout, LayoutRender]'> ````

## console

`rich.layout.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## layout

`rich.layout.layout`

```python
layout = Layout()
```

**Inferred type** (`ty`, not declared in the source): `Layout`

## ColumnSplitter

`rich.layout.ColumnSplitter`

```python
class ColumnSplitter(Splitter)
```

**Bases** `Splitter`

**Declared members (3)**

- `def divide(self, children: Sequence[Layout], region: Region) -> Iterable[Tuple[Layout, Region]]`
- `def get_tree_icon(self) -> str`
- `name = 'column'`  _class-attribute, instance-attribute_

Split a layout region in to columns.


## Layout

`rich.layout.Layout`

```python
class Layout
```

**Declared members (20)**

- `def add_split(self, layouts: Union[Layout, RenderableType] = ()) -> None`
  Add a new layout(s) to existing split.
- `children: List[Layout]`  _property_
  Gets (visible) layout children.
- `def get(self, name: str) -> Optional[Layout]`
  Get a named layout, or None if it doesn't exist.
- `map: RenderMap`  _property_
  Get a map of the last render.
- `minimum_size = minimum_size`  _instance-attribute_
- `name = name`  _instance-attribute_
- `ratio = ratio`  _instance-attribute_
- `def refresh_screen(self, console: Console, layout_name: str) -> None`
  Refresh a sub-layout.
- `def render(self, console: Console, options: ConsoleOptions) -> RenderMap`
  Render the sub_layouts.
- `renderable: RenderableType`  _property_
  Layout renderable.
- `size = size`  _instance-attribute_
- `def split(self, layouts: Union[Layout, RenderableType] = (), splitter: Union[Splitter, str] = 'column') -> None`
  Split the layout in to multiple sub-layouts.
- `def split_column(self, layouts: Union[Layout, RenderableType] = ()) -> None`
  Split the layout in to a column (layouts stacked on top of each other).
- `def split_row(self, layouts: Union[Layout, RenderableType] = ()) -> None`
  Split the layout in to a row (layouts side by side).
- `splitter: Splitter = self.splitters['column']()`  _instance-attribute_
- `splitters = {'row': RowSplitter, 'column': ColumnSplitter}`  _class-attribute, instance-attribute_
- `tree: Tree`  _property_
  Get a tree renderable to show layout structure.
- `def unsplit(self) -> None`
  Reset splits to initial state.
- `def update(self, renderable: RenderableType) -> None`
  Update renderable.
- `visible = visible`  _instance-attribute_

A renderable to divide a fixed height in to rows or columns.

Args:
    renderable (RenderableType, optional): Renderable content, or None for placeholder. Defaults to None.
    name (str, optional): Optional identifier for Layout. Defaults to None.
    size (int, optional): Optional fixed size of layout. Defaults to None.
    minimum_size (int, optional): Minimum size of layout. Defaults to 1.
    ratio (int, optional): Optional ratio for flexible layout. Defaults to 1.
    visible (bool, optional): Visibility of layout. Defaults to True.


## LayoutError

`rich.layout.LayoutError`

```python
class LayoutError(Exception)
```

**Bases** `Exception`

Layout related error.


## LayoutRender

`rich.layout.LayoutRender`

```python
class LayoutRender(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (2)**

- `region: Region`  _instance-attribute_
- `render: List[List[Segment]]`  _instance-attribute_

An individual layout render.


## NoSplitter

`rich.layout.NoSplitter`

```python
class NoSplitter(LayoutError)
```

**Bases** `LayoutError`

Requested splitter does not exist.


## RowSplitter

`rich.layout.RowSplitter`

```python
class RowSplitter(Splitter)
```

**Bases** `Splitter`

**Declared members (3)**

- `def divide(self, children: Sequence[Layout], region: Region) -> Iterable[Tuple[Layout, Region]]`
- `def get_tree_icon(self) -> str`
- `name = 'row'`  _class-attribute, instance-attribute_

Split a layout region in to rows.


## Splitter

`rich.layout.Splitter`

```python
class Splitter(ABC)
```

**Bases** `ABC`

**Declared members (3)**

- `def divide(self, children: Sequence[Layout], region: Region) -> Iterable[Tuple[Layout, Region]]`  _abstractmethod_
  Divide a region amongst several child layouts.
- `def get_tree_icon(self) -> str`  _abstractmethod_
  Get the icon (emoji) used in layout.tree
- `name: str = ''`  _class-attribute, instance-attribute_

Base class for a splitter.


## _Placeholder

`rich.layout._Placeholder`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Placeholder
```

**Declared members (3)**

- `highlighter = ReprHighlighter()`  _class-attribute, instance-attribute_
- `layout = layout`  _instance-attribute_
- `style = style`  _instance-attribute_

An internal renderable used as a Layout placeholder.


