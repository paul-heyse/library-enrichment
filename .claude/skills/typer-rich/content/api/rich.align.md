# `rich.align`

Distribution: `rich`

## AlignMethod

`rich.align.AlignMethod`

```python
AlignMethod = Literal['left', 'center', 'right']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["left", "center", "right"]'> ````

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## VerticalAlignMethod

`rich.align.VerticalAlignMethod`

```python
VerticalAlignMethod = Literal['top', 'middle', 'bottom']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["top", "middle", "bottom"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## console

`rich.align.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## highlighter

`rich.align.highlighter`

```python
highlighter = ReprHighlighter()
```

**Inferred type** (`ty`, not declared in the source): `ReprHighlighter`

## panel

`rich.align.panel`

```python
panel = Panel(Group(Align.left(highlighter("align='left'")), Align.center(highlighter("align='center'")), Align.right(highlighter("align='right'"))), width=60, style='on dark_blue', title='Align')
```

**Inferred type** (`ty`, not declared in the source): `Panel`

## Align

`rich.align.Align`

```python
class Align(JupyterMixin)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (10)**

- `align = align`  _instance-attribute_
- `def center(cls, renderable: RenderableType, style: Optional[StyleType] = None, vertical: Optional[VerticalAlignMethod] = None, pad: bool = True, width: Optional[int] = None, height: Optional[int] = None) -> Align`  _classmethod_
  Align a renderable to the center.
- `height = height`  _instance-attribute_
- `def left(cls, renderable: RenderableType, style: Optional[StyleType] = None, vertical: Optional[VerticalAlignMethod] = None, pad: bool = True, width: Optional[int] = None, height: Optional[int] = None) -> Align`  _classmethod_
  Align a renderable to the left.
- `pad = pad`  _instance-attribute_
- `renderable = renderable`  _instance-attribute_
- `def right(cls, renderable: RenderableType, style: Optional[StyleType] = None, vertical: Optional[VerticalAlignMethod] = None, pad: bool = True, width: Optional[int] = None, height: Optional[int] = None) -> Align`  _classmethod_
  Align a renderable to the right.
- `style = style`  _instance-attribute_
- `vertical = vertical`  _instance-attribute_
- `width = width`  _instance-attribute_

Align a renderable by adding spaces if necessary.

Args:
    renderable (RenderableType): A console renderable.
    align (AlignMethod): One of "left", "center", or "right""
    style (StyleType, optional): An optional style to apply to the background.
    vertical (Optional[VerticalAlignMethod], optional): Optional vertical align, one of "top", "middle", or "bottom". Defaults to None.
    pad (bool, optional): Pad the right with spaces. Defaults to True.
    width (int, optional): Restrict contents to given width, or None to use default width. Defaults to None.
    height (int, optional): Set height of align renderable, or None to fit to contents. Defaults to None.

Raises:
    ValueError: if ``align`` is not one of the expected values.

Example:
    .. code-block:: python

        from rich.console import Console
        from rich.align import Align
        from rich.panel import Panel

        console = Console()
        # Create a panel 20 characters wide
        p = Panel("Hello, [b]World[/b]!", style="on green", width=20)

        # Renders the panel centered in the terminal
        console.print(Align(p, align="center"))


## VerticalCenter

`rich.align.VerticalCenter`

```python
class VerticalCenter(JupyterMixin)
```

**Bases** `JupyterMixin`

**Declared members (2)**

- `renderable = renderable`  _instance-attribute_
- `style = style`  _instance-attribute_

Vertically aligns a renderable.

Warn:
    This class is deprecated and may be removed in a future version. Use Align class with
    `vertical="middle"`.

Args:
    renderable (RenderableType): A renderable object.
    style (StyleType, optional): An optional style to apply to the background. Defaults to None.


