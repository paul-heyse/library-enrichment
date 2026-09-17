# `rich.bar`

Distribution: `rich`

## BEGIN_BLOCK_ELEMENTS

`rich.bar.BEGIN_BLOCK_ELEMENTS`

```python
BEGIN_BLOCK_ELEMENTS = ['█', '█', '█', '▐', '▐', '▐', '▕', '▕']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## END_BLOCK_ELEMENTS

`rich.bar.END_BLOCK_ELEMENTS`

```python
END_BLOCK_ELEMENTS = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## FULL_BLOCK

`rich.bar.FULL_BLOCK`

```python
FULL_BLOCK = '█'
```

**Inferred type** (`ty`, not declared in the source): `Literal["█"]`

## Bar

`rich.bar.Bar`

```python
class Bar(JupyterMixin)
```

**Bases** `JupyterMixin`

**Declared members (5)**

- `begin = max(begin, 0)`  _instance-attribute_
- `end = min(end, size)`  _instance-attribute_
- `size = size`  _instance-attribute_
- `style = Style(color=color, bgcolor=bgcolor)`  _instance-attribute_
- `width = width`  _instance-attribute_

Renders a solid block bar.

Args:
    size (float): Value for the end of the bar.
    begin (float): Begin point (between 0 and size, inclusive).
    end (float): End point (between 0 and size, inclusive).
    width (int, optional): Width of the bar, or ``None`` for maximum width. Defaults to None.
    color (Union[Color, str], optional): Color of the bar. Defaults to "default".
    bgcolor (Union[Color, str], optional): Color of bar background. Defaults to "default".


