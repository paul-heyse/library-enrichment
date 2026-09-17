# `rich.screen`

Distribution: `rich`

## Screen

`rich.screen.Screen`

```python
class Screen
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `application_mode = application_mode`  _instance-attribute_
- `renderable: RenderableType = Group(*renderables)`  _instance-attribute_
- `style = style`  _instance-attribute_

A renderable that fills the terminal screen and crops excess.

Args:
    renderable (RenderableType): Child renderable.
    style (StyleType, optional): Optional background style. Defaults to None.


