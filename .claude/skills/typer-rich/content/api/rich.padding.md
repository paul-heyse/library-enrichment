# `rich.padding`

Distribution: `rich`

## PaddingDimensions

`rich.padding.PaddingDimensions`

```python
PaddingDimensions = Union[int, Tuple[int], Tuple[int, int], Tuple[int, int, int, int]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'int | tuple[int] | tuple[int, int] | tuple[int, int, int, int]'> ````

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## Padding

`rich.padding.Padding`

```python
class Padding(JupyterMixin)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (5)**

- `expand = expand`  _instance-attribute_
- `def indent(cls, renderable: RenderableType, level: int) -> Padding`  _classmethod_
  Make padding instance to render an indent.
- `renderable = renderable`  _instance-attribute_
- `style = style`  _instance-attribute_
- `def unpack(pad: PaddingDimensions) -> Tuple[int, int, int, int]`  _staticmethod_
  Unpack padding specified in CSS style.

Draw space around content.

Example:
    >>> print(Padding("Hello", (2, 4), style="on blue"))

Args:
    renderable (RenderableType): String or other renderable.
    pad (Union[int, Tuple[int]]): Padding for top, right, bottom, and left borders.
        May be specified with 1, 2, or 4 integers (CSS style).
    style (Union[str, Style], optional): Style for padding characters. Defaults to "none".
    expand (bool, optional): Expand padding to fit available width. Defaults to True.


