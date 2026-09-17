# `rich.live_render`

Distribution: `rich`

## VerticalOverflowMethod

`rich.live_render.VerticalOverflowMethod`

```python
VerticalOverflowMethod = Literal['crop', 'ellipsis', 'visible']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["crop", "ellipsis", "visible"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## LiveRender

`rich.live_render.LiveRender`

```python
class LiveRender
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `last_render_height: int`  _property_
  The number of lines in the last render (may be 0 if nothing was rendered).
- `def position_cursor(self) -> Control`
  Get control codes to move cursor to beginning of live render.
- `renderable = renderable`  _instance-attribute_
- `def restore_cursor(self) -> Control`
  Get control codes to clear the render and restore the cursor to its previous position.
- `def set_renderable(self, renderable: RenderableType) -> None`
  Set a new renderable.
- `style = style`  _instance-attribute_
- `vertical_overflow = vertical_overflow`  _instance-attribute_

Creates a renderable that may be updated.

Args:
    renderable (RenderableType): Any renderable object.
    style (StyleType, optional): An optional style to apply to the renderable. Defaults to "".


