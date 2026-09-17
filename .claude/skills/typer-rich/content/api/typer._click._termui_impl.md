# `typer._click._termui_impl`

Distribution: `typer`

## AFTER_BAR

`typer._click._termui_impl.AFTER_BAR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
AFTER_BAR = '\n'
```

## BEFORE_BAR

`typer._click._termui_impl.BEFORE_BAR`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
BEFORE_BAR = '\r'
```

## V

`typer._click._termui_impl.V`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
V = TypeVar('V')
```

## ProgressBar

`typer._click._termui_impl.ProgressBar`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ProgressBar(Generic[V])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[V]`

**Declared members (40)**

- `autowidth: bool = width == 0`  _instance-attribute_
- `avg: list[float] = []`  _instance-attribute_
- `bar_template = bar_template`  _instance-attribute_
- `color = color`  _instance-attribute_
- `current_item: V | None = None`  _instance-attribute_
- `empty_char = empty_char`  _instance-attribute_
- `entered: bool = False`  _instance-attribute_
- `eta: float`  _property_
- `eta_known: bool = False`  _instance-attribute_
- `file = file`  _instance-attribute_
- `fill_char = fill_char`  _instance-attribute_
- `def finish(self) -> None`
- `finished: bool = False`  _instance-attribute_
- `def format_bar(self) -> str`
- `def format_eta(self) -> str`
- `def format_pct(self) -> str`
- `def format_pos(self) -> str`
- `def format_progress_line(self) -> str`
- `def generator(self) -> Iterator[V]`
  Return a generator which yields the items added to the bar during construction, and updates the progress bar *after* the yielded block returns.
- `hidden = hidden`  _instance-attribute_
- `info_sep = info_sep`  _instance-attribute_
- `item_show_func = item_show_func`  _instance-attribute_
- `iter: Iterable[V] = iter(iterable)`  _instance-attribute_
- `label: str = label or ''`  _instance-attribute_
- `last_eta: float = time.time()`  _instance-attribute_
- `length = length`  _instance-attribute_
- `def make_step(self, n_steps: int) -> None`
- `max_width: int | None = None`  _instance-attribute_
- `pct: float`  _property_
- `pos: int = 0`  _instance-attribute_
- `def render_finish(self) -> None`
- `def render_progress(self) -> None`
- `show_eta = show_eta`  _instance-attribute_
- `show_percent = show_percent`  _instance-attribute_
- `show_pos = show_pos`  _instance-attribute_
- `start: float = time.time()`  _instance-attribute_
- `time_per_iteration: float`  _property_
- `def update(self, n_steps: int) -> None`
  Update the progress bar by advancing a specified number of steps.
- `update_min_steps = update_min_steps`  _instance-attribute_
- `width: int = width`  _instance-attribute_

## _translate_ch_to_exc

`typer._click._termui_impl._translate_ch_to_exc`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _translate_ch_to_exc(ch: str) -> None
```

## getchar

`typer._click._termui_impl.getchar`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def getchar(echo: bool) -> str
```

## open_url

`typer._click._termui_impl.open_url`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def open_url(url: str, wait: bool = False, locate: bool = False) -> int
```

## raw_terminal

`typer._click._termui_impl.raw_terminal`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def raw_terminal() -> Iterator[int]
```

