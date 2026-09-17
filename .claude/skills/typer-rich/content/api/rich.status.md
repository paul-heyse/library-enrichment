# `rich.status`

Distribution: `rich`

## console

`rich.status.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Status

`rich.status.Status`

```python
class Status(JupyterMixin)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (8)**

- `console: Console`  _property_
  Get the Console used by the Status objects.
- `renderable: Spinner`  _property_
- `speed = speed`  _instance-attribute_
- `spinner_style = spinner_style`  _instance-attribute_
- `def start(self) -> None`
  Start the status animation.
- `status = status`  _instance-attribute_
- `def stop(self) -> None`
  Stop the spinner animation.
- `def update(self, status: Optional[RenderableType] = None, spinner: Optional[str] = None, spinner_style: Optional[StyleType] = None, speed: Optional[float] = None) -> None`
  Update status.

Displays a status indicator with a 'spinner' animation.

Args:
    status (RenderableType): A status renderable (str or Text typically).
    console (Console, optional): Console instance to use, or None for global console. Defaults to None.
    spinner (str, optional): Name of spinner animation (see python -m rich.spinner). Defaults to "dots".
    spinner_style (StyleType, optional): Style of spinner. Defaults to "status.spinner".
    speed (float, optional): Speed factor for spinner animation. Defaults to 1.0.
    refresh_per_second (float, optional): Number of refreshes per second. Defaults to 12.5.


