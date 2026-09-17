# `rich.control`

Distribution: `rich`

## CONTROL_CODES_FORMAT

`rich.control.CONTROL_CODES_FORMAT`

```python
CONTROL_CODES_FORMAT: Dict[int, Callable[..., str]] = {ControlType.BELL: lambda: '\x07', ControlType.CARRIAGE_RETURN: lambda: '\r', ControlType.HOME: lambda: '\x1b[H', ControlType.CLEAR: lambda: '\x1b[2J', ControlType.ENABLE_ALT_SCREEN: lambda: '\x1b[?1049h', ControlType.DISABLE_ALT_SCREEN: lambda: '\x1b[?1049l', ControlType.SHOW_CURSOR: lambda: '\x1b[?25h', ControlType.HIDE_CURSOR: lambda: '\x1b[?25l', ControlType.CURSOR_UP: lambda param: f'[{param}A', ControlType.CURSOR_DOWN: lambda param: f'[{param}B', ControlType.CURSOR_FORWARD: lambda param: f'[{param}C', ControlType.CURSOR_BACKWARD: lambda param: f'[{param}D', ControlType.CURSOR_MOVE_TO_COLUMN: lambda param: f'[{param + 1}G', ControlType.ERASE_IN_LINE: lambda param: f'[{param}K', ControlType.CURSOR_MOVE_TO: lambda x, y: f'[{y + 1};{x + 1}H', ControlType.SET_WINDOW_TITLE: lambda title: f']0;{title}'}
```

## CONTROL_ESCAPE

`rich.control.CONTROL_ESCAPE`

```python
CONTROL_ESCAPE: Final = {7: '\\a', 8: '\\b', 11: '\\v', 12: '\\f', 13: '\\r'}
```

## STRIP_CONTROL_CODES

`rich.control.STRIP_CONTROL_CODES`

```python
STRIP_CONTROL_CODES: Final = [7, 8, 11, 12, 13]
```

## _CONTROL_STRIP_TRANSLATE

`rich.control._CONTROL_STRIP_TRANSLATE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CONTROL_STRIP_TRANSLATE: Final = {_codepoint: None for _codepoint in STRIP_CONTROL_CODES}
```

## console

`rich.control.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Control

`rich.control.Control`

```python
class Control
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (10)**

- `def alt_screen(cls, enable: bool) -> Control`  _classmethod_
  Enable or disable alt screen.
- `def bell(cls) -> Control`  _classmethod_
  Ring the 'bell'.
- `def clear(cls) -> Control`  _classmethod_
  Clear the screen.
- `def home(cls) -> Control`  _classmethod_
  Move cursor to 'home' position.
- `def move(cls, x: int = 0, y: int = 0) -> Control`  _classmethod_
  Move cursor relative to current position.
- `def move_to(cls, x: int, y: int) -> Control`  _classmethod_
  Move cursor to absolute position.
- `def move_to_column(cls, x: int, y: int = 0) -> Control`  _classmethod_
  Move to the given column, optionally add offset to row.
- `segment = Segment(rendered_codes, None, control_codes)`  _instance-attribute_
- `def show_cursor(cls, show: bool) -> Control`  _classmethod_
  Show or hide the cursor.
- `def title(cls, title: str) -> Control`  _classmethod_
  Set the terminal window title

A renderable that inserts a control code (non printable but may move cursor).

Args:
    *codes (str): Positional arguments are either a :class:`~rich.segment.ControlType` enum or a
        tuple of ControlType and an integer parameter


## escape_control_codes

`rich.control.escape_control_codes`

```python
def escape_control_codes(text: str, _translate_table: Dict[int, str] = CONTROL_ESCAPE) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Replace control codes with their "escaped" equivalent in the given text.
(e.g. "" becomes "\b")

Args:
    text (str): A string possibly containing control codes.

Returns:
    str: String with control codes replaced with their escaped version.


## strip_control_codes

`rich.control.strip_control_codes`

```python
def strip_control_codes(text: str, _translate_table: Dict[int, None] = _CONTROL_STRIP_TRANSLATE) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Remove control codes from text.

Args:
    text (str): A string possibly contain control codes.

Returns:
    str: String with control codes removed.


