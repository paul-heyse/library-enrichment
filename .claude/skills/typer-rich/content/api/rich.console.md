# `rich.console`

Distribution: `rich`

## COLOR_SYSTEMS

`rich.console.COLOR_SYSTEMS`

```python
COLOR_SYSTEMS = {'standard': ColorSystem.STANDARD, '256': ColorSystem.EIGHT_BIT, 'truecolor': ColorSystem.TRUECOLOR, 'windows': ColorSystem.WINDOWS}
```

**Inferred type** (`ty`, not declared in the source): `dict[str, ColorSystem]`

## HighlighterType

`rich.console.HighlighterType`

```python
HighlighterType = Callable[[Union[str, 'Text']], 'Text']
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(str | Text, /) -> Text'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## JUPYTER_DEFAULT_COLUMNS

`rich.console.JUPYTER_DEFAULT_COLUMNS`

```python
JUPYTER_DEFAULT_COLUMNS = 115
```

**Inferred type** (`ty`, not declared in the source): `Literal[115]`

## JUPYTER_DEFAULT_LINES

`rich.console.JUPYTER_DEFAULT_LINES`

```python
JUPYTER_DEFAULT_LINES = 100
```

**Inferred type** (`ty`, not declared in the source): `Literal[100]`

## JustifyMethod

`rich.console.JustifyMethod`

```python
JustifyMethod = Literal['default', 'left', 'center', 'right', 'full']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["default", "left", "center", "right", "full"]'> ````

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## NO_CHANGE

`rich.console.NO_CHANGE`

```python
NO_CHANGE = NoChange()
```

**Inferred type** (`ty`, not declared in the source): `NoChange`

## OverflowMethod

`rich.console.OverflowMethod`

```python
OverflowMethod = Literal['fold', 'crop', 'ellipsis', 'ignore']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["fold", "crop", "ellipsis", "ignore"]'> ````

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## RenderResult

`rich.console.RenderResult`

```python
RenderResult = Iterable[Union[RenderableType, Segment]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'Iterable[ConsoleRenderable | RichCast | str | Segment]'> ````

_24 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## RenderableType

`rich.console.RenderableType`

```python
RenderableType = Union[ConsoleRenderable, RichCast, str]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'ConsoleRenderable | RichCast | str'> ``` --- A string or any object that may be rendered by Rich.`

_22 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A string or any object that may be rendered by Rich.


## WINDOWS

`rich.console.WINDOWS`

```python
WINDOWS = sys.platform == 'win32'
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## _COLOR_SYSTEMS_NAMES

`rich.console._COLOR_SYSTEMS_NAMES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_COLOR_SYSTEMS_NAMES = {system: name for name, system in COLOR_SYSTEMS.items()}
```

## _STDERR_FILENO

`rich.console._STDERR_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STDERR_FILENO = sys.__stderr__.fileno()
```

## _STDIN_FILENO

`rich.console._STDIN_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STDIN_FILENO = sys.__stdin__.fileno()
```

## _STDOUT_FILENO

`rich.console._STDOUT_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STDOUT_FILENO = sys.__stdout__.fileno()
```

## _STD_STREAMS

`rich.console._STD_STREAMS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STD_STREAMS = (_STDIN_FILENO, _STDOUT_FILENO, _STDERR_FILENO)
```

## _STD_STREAMS_OUTPUT

`rich.console._STD_STREAMS_OUTPUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STD_STREAMS_OUTPUT = (_STDOUT_FILENO, _STDERR_FILENO)
```

## _TERM_COLORS

`rich.console._TERM_COLORS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TERM_COLORS = {'kitty': ColorSystem.EIGHT_BIT, '256color': ColorSystem.EIGHT_BIT, '16color': ColorSystem.STANDARD}
```

## _null_highlighter

`rich.console._null_highlighter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_null_highlighter = NullHighlighter()
```

## _windows_console_features

`rich.console._windows_console_features`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_windows_console_features: Optional[WindowsConsoleFeatures] = None
```

## console

`rich.console.console`

```python
console = Console(record=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Capture

`rich.console.Capture`

```python
class Capture
```

**Declared members (1)**

- `def get(self) -> str`
  Get the result of the capture.

Context manager to capture the result of printing to the console.
See :meth:`~rich.console.Console.capture` for how to use.

Args:
    console (Console): A console instance to capture output.


## CaptureError

`rich.console.CaptureError`

```python
class CaptureError(Exception)
```

**Bases** `Exception`

An error in the Capture context manager.


## Console

`rich.console.Console`

```python
class Console
```

_47 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (65)**

- `def begin_capture(self) -> None`
  Begin capturing console output. Call :meth:`end_capture` to exit capture mode and return output.
- `def bell(self) -> None`
  Play a 'bell' sound (if supported by the terminal).
- `def capture(self) -> Capture`
  A context manager to *capture* the result of print() or log() in a string, rather than writing it to the console.
- `def clear(self, home: bool = True) -> None`
  Clear the screen.
- `def clear_live(self) -> None`
  Clear the Live instance. Used by the Live context manager (no need to call directly).
- `color_system: Optional[str]`  _property_
  Get color system string.
- `def control(self, control: Control = ()) -> None`
  Insert non-printing control codes.
- `encoding: str`  _property_
  Get the encoding of the console file, e.g. ``"utf-8"``.
- `def end_capture(self) -> str`
  End capture mode and return captured string.
- `def export_html(self, theme: Optional[TerminalTheme] = None, clear: bool = True, code_format: Optional[str] = None, inline_styles: bool = False) -> str`
  Generate HTML from console contents (requires record=True argument in constructor).
- `def export_svg(self, title: str = 'Rich', theme: Optional[TerminalTheme] = None, clear: bool = True, code_format: str = CONSOLE_SVG_FORMAT, font_aspect_ratio: float = 0.61, unique_id: Optional[str] = None) -> str`
  Generate an SVG from the console contents (requires record=True in Console constructor).
- `def export_text(self, clear: bool = True, styles: bool = False) -> str`
  Generate text from console contents (requires record=True argument in constructor).
- `file: IO[str]`  _property, writable_
  Get the file object to write to.
- `get_datetime = get_datetime or datetime.now`  _instance-attribute_
- `def get_style(self, name: Union[str, Style], default: Optional[Union[Style, str]] = None) -> Style`
  Get a Style instance by its theme name or parse a definition.
- `get_time = get_time or monotonic`  _instance-attribute_
- `height: int`  _property, writable_
  Get the height of the console.
- `highlighter: HighlighterType = highlighter or _null_highlighter`  _instance-attribute_
- `def input(self, prompt: TextType = '', markup: bool = True, emoji: bool = True, password: bool = False, stream: Optional[TextIO] = None) -> str`
  Displays a prompt and waits for input from the user. The prompt may contain color / style.
- `is_alt_screen: bool`  _property_
  Check if the alt screen was enabled.
- `is_dumb_terminal: bool`  _property_
  Detect dumb terminal.
- `is_interactive = self.is_terminal and not self.is_dumb_terminal if force_interactive is None else force_interactive`  _instance-attribute_
- `is_jupyter = _is_jupyter() if force_jupyter is None else force_jupyter`  _instance-attribute_
- `is_terminal: bool`  _property_
  Check if the console is writing to a terminal.
- `legacy_windows: bool = detect_legacy_windows() and not self.is_jupyter if legacy_windows is None else legacy_windows`  _instance-attribute_
- `def line(self, count: int = 1) -> None`
  Write new line(s).
- `def log(self, objects: Any = (), sep: str = ' ', end: str = '\n', style: Optional[Union[str, Style]] = None, justify: Optional[JustifyMethod] = None, emoji: Optional[bool] = None, markup: Optional[bool] = None, highlight: Optional[bool] = None, log_locals: bool = False, _stack_offset: int = 1) -> None`
  Log rich content to the terminal.
- `def measure(self, renderable: RenderableType, options: Optional[ConsoleOptions] = None) -> Measurement`
  Measure a renderable. Returns a :class:`~rich.measure.Measurement` object which contains information regarding the number of characters required to print the renderable.
- `no_color = no_color if no_color is not None else self._environ.get('NO_COLOR', '') != ''`  _instance-attribute_
- `def on_broken_pipe(self) -> None`
  This function is called when a `BrokenPipeError` is raised.
- `options: ConsoleOptions`  _property_
  Get default console options.
- `def out(self, objects: Any = (), sep: str = ' ', end: str = '\n', style: Optional[Union[str, Style]] = None, highlight: Optional[bool] = None) -> None`
  Output to the terminal. This is a low-level way of writing to the terminal which unlike :meth:`~rich.console.Console.print` won't pretty print, wrap text, or apply markup, but will optionally apply highlighting and a basic style.
- `def pager(self, pager: Optional[Pager] = None, styles: bool = False, links: bool = False) -> PagerContext`
  A context manager to display anything printed within a "pager". The pager application is defined by the system and will typically support at least pressing a key to scroll.
- `def pop_render_hook(self) -> None`
  Pop the last renderhook from the stack.
- `def pop_theme(self) -> None`
  Remove theme from top of stack, restoring previous theme.
- `def print(self, objects: Any = (), sep: str = ' ', end: str = '\n', style: Optional[Union[str, Style]] = None, justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, no_wrap: Optional[bool] = None, emoji: Optional[bool] = None, markup: Optional[bool] = None, highlight: Optional[bool] = None, width: Optional[int] = None, height: Optional[int] = None, crop: bool = True, soft_wrap: Optional[bool] = None, new_line_start: bool = False) -> None`
  Print to the console.
- `def print_exception(self, width: Optional[int] = 100, extra_lines: int = 3, theme: Optional[str] = None, word_wrap: bool = False, show_locals: bool = False, suppress: Iterable[Union[str, ModuleType]] = (), max_frames: int = 100) -> None`
  Prints a rich render of the last exception and traceback.
- `def print_json(self, json: Optional[str] = None, data: Any = None, indent: Union[None, int, str] = 2, highlight: bool = True, skip_keys: bool = False, ensure_ascii: bool = False, check_circular: bool = True, allow_nan: bool = True, default: Optional[Callable[[Any], Any]] = None, sort_keys: bool = False) -> None`
  Pretty prints JSON. Output will be valid JSON.
- `def push_render_hook(self, hook: RenderHook) -> None`
  Add a new render hook to the stack.
- `def push_theme(self, theme: Theme, inherit: bool = True) -> None`
  Push a new theme on to the top of the stack, replacing the styles from the previous theme. Generally speaking, you should call :meth:`~rich.console.Console.use_theme` to get a context manager, rather than calling this method directly.
- `quiet = quiet`  _instance-attribute_
- `record = record`  _instance-attribute_
- `def render(self, renderable: RenderableType, options: Optional[ConsoleOptions] = None) -> Iterable[Segment]`
  Render an object in to an iterable of `Segment` instances.
- `def render_lines(self, renderable: RenderableType, options: Optional[ConsoleOptions] = None, style: Optional[Style] = None, pad: bool = True, new_lines: bool = False) -> List[List[Segment]]`
  Render objects in to a list of lines.
- `def render_str(self, text: str, style: Union[str, Style] = '', justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, emoji: Optional[bool] = None, markup: Optional[bool] = None, highlight: Optional[bool] = None, highlighter: Optional[HighlighterType] = None) -> Text`
  Convert a string to a Text instance. This is called automatically if you print or log a string.
- `def rule(self, title: TextType = '', characters: str = '─', style: Union[str, Style] = 'rule.line', align: AlignMethod = 'center') -> None`
  Draw a line with optional centered title.
- `safe_box = safe_box`  _instance-attribute_
- `def save_html(self, path: Union[str, PathLike[str]], theme: Optional[TerminalTheme] = None, clear: bool = True, code_format: str = CONSOLE_HTML_FORMAT, inline_styles: bool = False) -> None`
  Generate HTML from console contents and write to a file (requires record=True argument in constructor).
- `def save_svg(self, path: Union[str, PathLike[str]], title: str = 'Rich', theme: Optional[TerminalTheme] = None, clear: bool = True, code_format: str = CONSOLE_SVG_FORMAT, font_aspect_ratio: float = 0.61, unique_id: Optional[str] = None) -> None`
  Generate an SVG file from the console contents (requires record=True in Console constructor).
- `def save_text(self, path: Union[str, PathLike[str]], clear: bool = True, styles: bool = False) -> None`
  Generate text from console and save to a given location (requires record=True argument in constructor).
- `def screen(self, hide_cursor: bool = True, style: Optional[StyleType] = None) -> ScreenContext`
  Context manager to enable and disable 'alternative screen' mode.
- `def set_alt_screen(self, enable: bool = True) -> bool`
  Enables alternative screen mode.
- `def set_live(self, live: Live) -> bool`
  Set Live instance. Used by Live context manager (no need to call directly).
- `def set_window_title(self, title: str) -> bool`
  Set the title of the console terminal window.
- `def show_cursor(self, show: bool = True) -> bool`
  Show or hide the cursor.
- `size: ConsoleDimensions`  _property, writable_
  Get the size of the console.
- `soft_wrap = soft_wrap`  _instance-attribute_
- `def status(self, status: RenderableType, spinner: str = 'dots', spinner_style: StyleType = 'status.spinner', speed: float = 1.0, refresh_per_second: float = 12.5) -> Status`
  Display a status and spinner.
- `stderr = stderr`  _instance-attribute_
- `style = style`  _instance-attribute_
- … and 5 more, see `model/`

A high level console interface.

Args:
    color_system (str, optional): The color system supported by your terminal,
        either ``"standard"``, ``"256"`` or ``"truecolor"``. Leave as ``"auto"`` to autodetect.
    force_terminal (Optional[bool], optional): Enable/disable terminal control codes, or None to auto-detect terminal. Defaults to None.
    force_jupyter (Optional[bool], optional): Enable/disable Jupyter rendering, or None to auto-detect Jupyter. Defaults to None.
    force_interactive (Optional[bool], optional): Enable/disable interactive mode, or None to auto detect. Defaults to None.
    soft_wrap (Optional[bool], optional): Set soft wrap default on print method. Defaults to False.
    theme (Theme, optional): An optional style theme object, or ``None`` for default theme.
    stderr (bool, optional): Use stderr rather than stdout if ``file`` is not specified. Defaults to False.
    file (IO, optional): A file object where the console should write to. Defaults to stdout.
    quiet (bool, Optional): Boolean to suppress all output. Defaults to False.
    width (int, optional): The width of the terminal. Leave as default to auto-detect width.
    height (int, optional): The height of the terminal. Leave as default to auto-detect height.
    style (StyleType, optional): Style to apply to all output, or None for no style. Defaults to None.
    no_color (Optional[bool], optional): Enabled no color mode, or None to auto detect. Defaults to None.
    tab_size (int, optional): Number of spaces used to replace a tab character. Defaults to 8.
    record (bool, optional): Boolean to enable recording of terminal output,
        required to call :meth:`export_html`, :meth:`export_svg`, and :meth:`export_text`. Defaults to False.
    markup (bool, optional): Boolean to enable :ref:`console_markup`. Defaults to True.
    emoji (bool, optional): Enable emoji code. Defaults to True.
    emoji_variant (str, optional): Optional emoji variant, either "text" or "emoji". Defaults to None.
    highlight (bool, optional): Enable automatic highlighting. Defaults to True.
    log_time (bool, optional): Boolean to enable logging of time by :meth:`log` methods. Defaults to True.
    log_path (bool, optional): Boolean to enable the logging of the caller by :meth:`log`. Defaults to True.
    log_time_format (Union[str, TimeFormatterCallable], optional): If ``log_time`` is enabled, either string for strftime or callable that formats the time. Defaults to "[%X] ".
    highlighter (HighlighterType, optional): Default highlighter.
    legacy_windows (bool, optional): Enable legacy Windows mode, or ``None`` to auto detect. Defaults to ``None``.
    safe_box (bool, optional): Restrict box options that don't render on legacy Windows.
    get_datetime (Callable[[], datetime], optional): Callable that gets the current time as a datetime.datetime object (used by Console.log),
        or None for datetime.now.
    get_time (Callable[[], time], optional): Callable that gets the current time in seconds, default uses time.monotonic.


## ConsoleDimensions

`rich.console.ConsoleDimensions`

```python
class ConsoleDimensions(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (2)**

- `height: int`  _instance-attribute_
  The height of the console in lines.
- `width: int`  _instance-attribute_
  The width of the console in 'cells'.

Size of the terminal.


## ConsoleOptions

`rich.console.ConsoleOptions`

```python
class ConsoleOptions
```

_28 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (19)**

- `ascii_only: bool`  _property_
  Check if renderables should use ascii only.
- `encoding: str`  _instance-attribute_
  Encoding of terminal.
- `height: Optional[int] = None`  _class-attribute, instance-attribute_
- `highlight: Optional[bool] = None`  _class-attribute, instance-attribute_
  Highlight override for render_str.
- `is_terminal: bool`  _instance-attribute_
  True if the target is a terminal, otherwise False.
- `justify: Optional[JustifyMethod] = None`  _class-attribute, instance-attribute_
  Justify value override for renderable.
- `legacy_windows: bool`  _instance-attribute_
  legacy_windows: flag for legacy windows.
- `markup: Optional[bool] = None`  _class-attribute, instance-attribute_
  Enable markup when rendering strings.
- `max_height: int`  _instance-attribute_
  Height of container (starts as terminal)
- `max_width: int`  _instance-attribute_
  Maximum width of renderable.
- `min_width: int`  _instance-attribute_
  Minimum width of renderable.
- `no_wrap: Optional[bool] = False`  _class-attribute, instance-attribute_
  Disable wrapping for text.
- `overflow: Optional[OverflowMethod] = None`  _class-attribute, instance-attribute_
  Overflow value override for renderable.
- `def reset_height(self) -> ConsoleOptions`
  Return a copy of the options with height set to ``None``.
- `size: ConsoleDimensions`  _instance-attribute_
  Size of console.
- `def update(self, width: Union[int, NoChange] = NO_CHANGE, min_width: Union[int, NoChange] = NO_CHANGE, max_width: Union[int, NoChange] = NO_CHANGE, justify: Union[Optional[JustifyMethod], NoChange] = NO_CHANGE, overflow: Union[Optional[OverflowMethod], NoChange] = NO_CHANGE, no_wrap: Union[Optional[bool], NoChange] = NO_CHANGE, highlight: Union[Optional[bool], NoChange] = NO_CHANGE, markup: Union[Optional[bool], NoChange] = NO_CHANGE, height: Union[Optional[int], NoChange] = NO_CHANGE) -> ConsoleOptions`
  Update values, return a copy.
- `def update_dimensions(self, width: int, height: int) -> ConsoleOptions`
  Update the width and height, and return a copy.
- `def update_height(self, height: int) -> ConsoleOptions`
  Update the height, and return a copy.
- `def update_width(self, width: int) -> ConsoleOptions`
  Update just the width, return a copy.

Options for __rich_console__ method.


## ConsoleRenderable

`rich.console.ConsoleRenderable`

```python
class ConsoleRenderable(Protocol)
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol`

An object that supports the console protocol.


## ConsoleThreadLocals

`rich.console.ConsoleThreadLocals`

```python
class ConsoleThreadLocals(threading.local)
```

**Bases** `threading.local`

**Declared members (3)**

- `buffer: List[Segment] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `buffer_index: int = 0`  _class-attribute, instance-attribute_
- `theme_stack: ThemeStack`  _instance-attribute_

Thread local values for Console context.


## Group

`rich.console.Group`

```python
class Group
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `fit = fit`  _instance-attribute_
- `renderables: List[RenderableType]`  _property_

Takes a group of renderables and returns a renderable object that renders the group.

Args:
    renderables (Iterable[RenderableType]): An iterable of renderable objects.
    fit (bool, optional): Fit dimension of group to contents, or fill available space. Defaults to True.


## NewLine

`rich.console.NewLine`

```python
class NewLine
```

**Declared members (1)**

- `count = count`  _instance-attribute_

A renderable to generate new line(s)


## NoChange

`rich.console.NoChange`

```python
class NoChange
```

## PagerContext

`rich.console.PagerContext`

```python
class PagerContext
```

**Declared members (3)**

- `links = links`  _instance-attribute_
- `pager = SystemPager() if pager is None else pager`  _instance-attribute_
- `styles = styles`  _instance-attribute_

A context manager that 'pages' content. See :meth:`~rich.console.Console.pager` for usage.


## RenderHook

`rich.console.RenderHook`

```python
class RenderHook(ABC)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (1)**

- `def process_renderables(self, renderables: List[ConsoleRenderable]) -> List[ConsoleRenderable]`  _abstractmethod_
  Called with a list of objects to render.

Provides hooks in to the render process.


## RichCast

`rich.console.RichCast`

```python
class RichCast(Protocol)
```

**Bases** `Protocol`

An object that may be 'cast' to a console renderable.


## ScreenContext

`rich.console.ScreenContext`

```python
class ScreenContext
```

**Declared members (4)**

- `console = console`  _instance-attribute_
- `hide_cursor = hide_cursor`  _instance-attribute_
- `screen = Screen(style=style)`  _instance-attribute_
- `def update(self, renderables: RenderableType = (), style: Optional[StyleType] = None) -> None`
  Update the screen.

A context manager that enables an alternative screen. See :meth:`~rich.console.Console.screen` for usage.


## ScreenUpdate

`rich.console.ScreenUpdate`

```python
class ScreenUpdate
```

**Declared members (2)**

- `x = x`  _instance-attribute_
- `y = y`  _instance-attribute_

Render a list of lines at a given offset.


## ThemeContext

`rich.console.ThemeContext`

```python
class ThemeContext
```

**Declared members (3)**

- `console = console`  _instance-attribute_
- `inherit = inherit`  _instance-attribute_
- `theme = theme`  _instance-attribute_

A context manager to use a temporary theme. See :meth:`~rich.console.Console.use_theme` for usage.


## _is_jupyter

`rich.console._is_jupyter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_jupyter() -> bool
```

Check if we're running in a Jupyter notebook.


## detect_legacy_windows

`rich.console.detect_legacy_windows`

```python
def detect_legacy_windows() -> bool
```

Detect legacy Windows.


## get_windows_console_features

`rich.console.get_windows_console_features`

```python
def get_windows_console_features() -> WindowsConsoleFeatures
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## group

`rich.console.group`

```python
def group(fit: bool = True) -> Callable[..., Callable[..., Group]]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A decorator that turns an iterable of renderables in to a group.

Args:
    fit (bool, optional): Fit dimension of group to contents, or fill available space. Defaults to True.


