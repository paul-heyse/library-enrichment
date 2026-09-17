# `rich.spinner`

Distribution: `rich`

## all_spinners

`rich.spinner.all_spinners`

```python
all_spinners = Group(*[Spinner(spinner_name, text=Text(repr(spinner_name), style='green')) for spinner_name in sorted(SPINNERS.keys())])
```

**Inferred type** (`ty`, not declared in the source): `Group`

## Spinner

`rich.spinner.Spinner`

```python
class Spinner
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (10)**

- `frame_no_offset: float = 0.0`  _instance-attribute_
- `frames = cast(List[str], spinner['frames'])[:]`  _instance-attribute_
- `interval = cast(float, spinner['interval'])`  _instance-attribute_
- `name = name`  _instance-attribute_
- `def render(self, time: float) -> RenderableType`
  Render the spinner for a given time.
- `speed = speed`  _instance-attribute_
- `start_time: Optional[float] = None`  _instance-attribute_
- `style = style`  _instance-attribute_
- `text: Union[RenderableType, Text] = Text.from_markup(text) if isinstance(text, str) else text`  _instance-attribute_
- `def update(self, text: RenderableType = '', style: Optional[StyleType] = None, speed: Optional[float] = None) -> None`
  Updates attributes of a spinner after it has been started.

A spinner animation.

Args:
    name (str): Name of spinner (run python -m rich.spinner).
    text (RenderableType, optional): A renderable to display at the right of the spinner (str or Text typically). Defaults to "".
    style (StyleType, optional): Style for spinner animation. Defaults to None.
    speed (float, optional): Speed factor for animation. Defaults to 1.0.

Raises:
    KeyError: If name isn't one of the supported spinner animations.


