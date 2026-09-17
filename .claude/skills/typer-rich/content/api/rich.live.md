# `rich.live`

Distribution: `rich`

## console

`rich.live.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## examples

`rich.live.examples`

```python
examples = cycle(progress_renderables)
```

**Inferred type** (`ty`, not declared in the source): `cycle[str | Panel | Table | ... omitted 3 union elements]`

## exchange_rate_dict

`rich.live.exchange_rate_dict`

```python
exchange_rate_dict: Dict[Tuple[str, str], float] = {}
```

## exchanges

`rich.live.exchanges`

```python
exchanges = ['SGD', 'MYR', 'EUR', 'USD', 'AUD', 'JPY', 'CNH', 'HKD', 'CAD', 'INR', 'DKK', 'GBP', 'RUB', 'NZD', 'MXN', 'IDR', 'TWD', 'THB', 'VND']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## progress_renderables

`rich.live.progress_renderables`

```python
progress_renderables = ['You can make the terminal shorter and taller to see the live table hideText may be printed while the progress bars are rendering.', Panel('In fact, [i]any[/i] renderable will work'), 'Such as [magenta]tables[/]...', table, 'Pretty printed structures...', {'type': 'example', 'text': 'Pretty printed'}, 'Syntax...', syntax, Rule('Give it a try!')]
```

**Inferred type** (`ty`, not declared in the source): `list[str | Panel | Table | ... omitted 3 union elements]`

## select_exchange

`rich.live.select_exchange`

```python
select_exchange = exchanges[index % len(exchanges)]
```

**Inferred type** (`ty`, not declared in the source): `str`

## syntax

`rich.live.syntax`

```python
syntax = Syntax('def loop_last(values: Iterable[T]) -> Iterable[Tuple[bool, T]]:\n    """Iterate and generate a tuple with a flag for last value."""\n    iter_values = iter(values)\n    try:\n        previous_value = next(iter_values)\n    except StopIteration:\n        return\n    for value in iter_values:\n        yield False, previous_value\n        previous_value = value\n    yield True, previous_value', 'python', line_numbers=True)
```

**Inferred type** (`ty`, not declared in the source): `Syntax`

## table

`rich.live.table`

```python
table = Table(title='Exchange Rates')
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Live

`rich.live.Live`

```python
class Live(JupyterMixin, RenderHook)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`, `RenderHook`

**Declared members (14)**

- `auto_refresh = auto_refresh`  _instance-attribute_
- `console = console if console is not None else get_console()`  _instance-attribute_
- `def get_renderable(self) -> RenderableType`
- `ipy_widget: Optional[Any] = None`  _instance-attribute_
- `is_started: bool`  _property_
  Check if live display has been started.
- `def process_renderables(self, renderables: List[ConsoleRenderable]) -> List[ConsoleRenderable]`
  Process renderables to restore cursor and display progress.
- `def refresh(self) -> None`
  Update the display of the Live Render.
- `refresh_per_second = refresh_per_second`  _instance-attribute_
- `renderable: RenderableType`  _property_
  Get the renderable that is being displayed
- `def start(self, refresh: bool = False) -> None`
  Start live rendering display.
- `def stop(self) -> None`
  Stop live rendering display.
- `transient = True if screen else transient`  _instance-attribute_
- `def update(self, renderable: RenderableType, refresh: bool = False) -> None`
  Update the renderable that is being displayed
- `vertical_overflow = vertical_overflow`  _instance-attribute_

Renders an auto-updating live display of any given renderable.

Args:
    renderable (RenderableType, optional): The renderable to live display. Defaults to displaying nothing.
    console (Console, optional): Optional Console instance. Defaults to an internal Console instance writing to stdout.
    screen (bool, optional): Enable alternate screen mode. Defaults to False.
    auto_refresh (bool, optional): Enable auto refresh. If disabled, you will need to call `refresh()` or `update()` with refresh flag. Defaults to True
    refresh_per_second (float, optional): Number of times per second to refresh the live display. Defaults to 4.
    transient (bool, optional): Clear the renderable on exit (has no effect when screen=True). Defaults to False.
    redirect_stdout (bool, optional): Enable redirection of stdout, so ``print`` may be used. Defaults to True.
    redirect_stderr (bool, optional): Enable redirection of stderr. Defaults to True.
    vertical_overflow (VerticalOverflowMethod, optional): How to handle renderable when it is too tall for the console. Defaults to "ellipsis".
    get_renderable (Callable[[], RenderableType], optional): Optional callable to get renderable. Defaults to None.


## _RefreshThread

`rich.live._RefreshThread`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _RefreshThread(Thread)
```

**Bases** `Thread`

**Declared members (5)**

- `done = Event()`  _instance-attribute_
- `live = live`  _instance-attribute_
- `refresh_per_second = refresh_per_second`  _instance-attribute_
- `def run(self) -> None`
- `def stop(self) -> None`

A thread that calls refresh() at regular intervals.


