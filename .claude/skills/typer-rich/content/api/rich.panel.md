# `rich.panel`

Distribution: `rich`

## c

`rich.panel.c`

```python
c = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## p

`rich.panel.p`

```python
p = Panel('Hello, World!', title='rich.Panel', style='white on blue', box=DOUBLE, padding=1)
```

**Inferred type** (`ty`, not declared in the source): `Panel`

## Panel

`rich.panel.Panel`

```python
class Panel(JupyterMixin)
```

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (15)**

- `border_style = border_style`  _instance-attribute_
- `box = box`  _instance-attribute_
- `expand = expand`  _instance-attribute_
- `def fit(cls, renderable: RenderableType, box: Box = ROUNDED, title: Optional[TextType] = None, title_align: AlignMethod = 'center', subtitle: Optional[TextType] = None, subtitle_align: AlignMethod = 'center', safe_box: Optional[bool] = None, style: StyleType = 'none', border_style: StyleType = 'none', width: Optional[int] = None, height: Optional[int] = None, padding: PaddingDimensions = (0, 1), highlight: bool = False) -> Panel`  _classmethod_
  An alternative constructor that sets expand=False.
- `height = height`  _instance-attribute_
- `highlight = highlight`  _instance-attribute_
- `padding = padding`  _instance-attribute_
- `renderable = renderable`  _instance-attribute_
- `safe_box = safe_box`  _instance-attribute_
- `style = style`  _instance-attribute_
- `subtitle = subtitle`  _instance-attribute_
- `subtitle_align = subtitle_align`  _instance-attribute_
- `title = title`  _instance-attribute_
- `title_align: AlignMethod = title_align`  _instance-attribute_
- `width = width`  _instance-attribute_

A console renderable that draws a border around its contents.

Example:
    >>> console.print(Panel("Hello, World!"))

Args:
    renderable (RenderableType): A console renderable object.
    box (Box): A Box instance that defines the look of the border (see :ref:`appendix_box`. Defaults to box.ROUNDED.
    title (Optional[TextType], optional): Optional title displayed in panel header. Defaults to None.
    title_align (AlignMethod, optional): Alignment of title. Defaults to "center".
    subtitle (Optional[TextType], optional): Optional subtitle displayed in panel footer. Defaults to None.
    subtitle_align (AlignMethod, optional): Alignment of subtitle. Defaults to "center".
    safe_box (bool, optional): Disable box characters that don't display on windows legacy terminal with *raster* fonts. Defaults to True.
    expand (bool, optional): If True the panel will stretch to fill the console width, otherwise it will be sized to fit the contents. Defaults to True.
    style (str, optional): The style of the panel (border and contents). Defaults to "none".
    border_style (str, optional): The style of the border. Defaults to "none".
    width (Optional[int], optional): Optional width of panel. Defaults to None to auto-detect.
    height (Optional[int], optional): Optional height of panel. Defaults to None to auto-detect.
    padding (Optional[PaddingDimensions]): Optional padding around renderable. Defaults to 0.
    highlight (bool, optional): Enable automatic highlighting of panel title (if str). Defaults to False.


