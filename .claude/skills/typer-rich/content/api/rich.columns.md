# `rich.columns`

Distribution: `rich`

## columns

`rich.columns.columns`

```python
columns = Columns(files, padding=(0, 1), expand=False, equal=False)
```

**Inferred type** (`ty`, not declared in the source): `Columns`

## console

`rich.columns.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## files

`rich.columns.files`

```python
files = [f'{i} {s}' for i, s in enumerate(sorted(os.listdir()))]
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## Columns

`rich.columns.Columns`

```python
class Columns(JupyterMixin)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (10)**

- `def add_renderable(self, renderable: RenderableType) -> None`
  Add a renderable to the columns.
- `align: Optional[AlignMethod] = align`  _instance-attribute_
- `column_first = column_first`  _instance-attribute_
- `equal = equal`  _instance-attribute_
- `expand = expand`  _instance-attribute_
- `padding = padding`  _instance-attribute_
- `renderables = list(renderables or [])`  _instance-attribute_
- `right_to_left = right_to_left`  _instance-attribute_
- `title = title`  _instance-attribute_
- `width = width`  _instance-attribute_

Display renderables in neat columns.

Args:
    renderables (Iterable[RenderableType]): Any number of Rich renderables (including str).
    width (int, optional): The desired width of the columns, or None to auto detect. Defaults to None.
    padding (PaddingDimensions, optional): Optional padding around cells. Defaults to (0, 1).
    expand (bool, optional): Expand columns to full width. Defaults to False.
    equal (bool, optional): Arrange in to equal sized columns. Defaults to False.
    column_first (bool, optional): Align items from top to bottom (rather than left to right). Defaults to False.
    right_to_left (bool, optional): Start column from right hand side. Defaults to False.
    align (str, optional): Align value ("left", "right", or "center") or None for default. Defaults to None.
    title (TextType, optional): Optional title for Columns.


