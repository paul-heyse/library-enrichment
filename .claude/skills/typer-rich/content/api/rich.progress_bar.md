# `rich.progress_bar`

Distribution: `rich`

## PULSE_SIZE

`rich.progress_bar.PULSE_SIZE`

```python
PULSE_SIZE = 20
```

**Inferred type** (`ty`, not declared in the source): `Literal[20]`

## bar

`rich.progress_bar.bar`

```python
bar = ProgressBar(width=50, total=100)
```

**Inferred type** (`ty`, not declared in the source): `ProgressBar`

## console

`rich.progress_bar.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## ProgressBar

`rich.progress_bar.ProgressBar`

```python
class ProgressBar(JupyterMixin)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (11)**

- `animation_time = animation_time`  _instance-attribute_
- `complete_style = complete_style`  _instance-attribute_
- `completed = completed`  _instance-attribute_
- `finished_style = finished_style`  _instance-attribute_
- `percentage_completed: Optional[float]`  _property_
  Calculate percentage complete.
- `pulse = pulse`  _instance-attribute_
- `pulse_style = pulse_style`  _instance-attribute_
- `style = style`  _instance-attribute_
- `total = total`  _instance-attribute_
- `def update(self, completed: float, total: Optional[float] = None) -> None`
  Update progress with new values.
- `width = width`  _instance-attribute_

Renders a (progress) bar. Used by rich.progress.

Args:
    total (float, optional): Number of steps in the bar. Defaults to 100. Set to None to render a pulsing animation.
    completed (float, optional): Number of steps completed. Defaults to 0.
    width (int, optional): Width of the bar, or ``None`` for maximum width. Defaults to None.
    pulse (bool, optional): Enable pulse effect. Defaults to False. Will pulse if a None total was passed.
    style (StyleType, optional): Style for the bar background. Defaults to "bar.back".
    complete_style (StyleType, optional): Style for the completed bar. Defaults to "bar.complete".
    finished_style (StyleType, optional): Style for a finished bar. Defaults to "bar.finished".
    pulse_style (StyleType, optional): Style for pulsing bars. Defaults to "bar.pulse".
    animation_time (Optional[float], optional): Time in seconds to use for animation, or None to use system time.


