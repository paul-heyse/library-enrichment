# `typer._click.termui`

Distribution: `typer`

## V

`typer._click.termui.V`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
V = TypeVar('V')
```

## _ansi_colors

`typer._click.termui._ansi_colors`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ansi_colors = {'black': 30, 'red': 31, 'green': 32, 'yellow': 33, 'blue': 34, 'magenta': 35, 'cyan': 36, 'white': 37, 'reset': 39, 'bright_black': 90, 'bright_red': 91, 'bright_green': 92, 'bright_yellow': 93, 'bright_blue': 94, 'bright_magenta': 95, 'bright_cyan': 96, 'bright_white': 97}
```

## _ansi_reset_all

`typer._click.termui._ansi_reset_all`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ansi_reset_all = '\x1b[0m'
```

## _getchar

`typer._click.termui._getchar`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_getchar: Callable[[bool], str] | None = None
```

## visible_prompt_func

`typer._click.termui.visible_prompt_func`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
visible_prompt_func: Callable[[str], str] = input
```

## _build_prompt

`typer._click.termui._build_prompt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_prompt(text: str, suffix: str, show_default: bool = False, default: Any | None = None, show_choices: bool = True, type: ParamType | None = None) -> str
```

## _format_default

`typer._click.termui._format_default`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_default(default: Any) -> Any
```

## _interpret_color

`typer._click.termui._interpret_color`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _interpret_color(color: int | tuple[int, int, int] | str, offset: int = 0) -> str
```

## confirm

Import as `typer.confirm`  ·  defined at `typer._click.termui.confirm`

```python
def confirm(text: str, default: bool | None = False, abort: bool = False, prompt_suffix: str = ': ', show_default: bool = True, err: bool = False) -> bool
```

**Also exported as** `typer.confirm`

Prompts for confirmation (yes/no question).

If the user aborts the input by sending a interrupt signal this
function will catch it and raise an `Abort` exception.


## getchar

Import as `typer.getchar`  ·  defined at `typer._click.termui.getchar`

```python
def getchar(echo: bool = False) -> str
```

**Also exported as** `typer.getchar`

Fetches a single character from the terminal and returns it.  This
will always return a unicode character and under certain rare
circumstances this might return more than one character.  The
situations which more than one character is returned is when for
whatever reason multiple characters end up in the terminal buffer or
standard input was not actually a terminal.

Note that this will always read from the terminal, even if something
is piped into the standard input.

Note for Windows: in rare cases when typing non-ASCII characters, this
function might wait for a second character and then return both at once.
This is because certain Unicode characters look like special-key markers.


## hidden_prompt_func

`typer._click.termui.hidden_prompt_func`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def hidden_prompt_func(prompt: str) -> str
```

## launch

`typer._click.termui.launch`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def launch(url: str, wait: bool = False, locate: bool = False) -> int
```

**Also exported as** `typer._click.launch`

This function launches the given URL (or filename) in the default
viewer application for this file type.  If this is an executable, it
might launch the executable in a new session.  The return value is
the exit code of the launched application.  Usually, ``0`` indicates
success.


## progressbar

Import as `typer.progressbar`  ·  defined at `typer._click.termui.progressbar`

```python
def progressbar(iterable: Iterable[V] | None = None, length: int | None = None, label: str | None = None, hidden: bool = False, show_eta: bool = True, show_percent: bool | None = None, show_pos: bool = False, item_show_func: Callable[[V | None], str | None] | None = None, fill_char: str = '#', empty_char: str = '-', bar_template: str = '%(label)s  [%(bar)s]  %(info)s', info_sep: str = '  ', width: int = 36, file: TextIO | None = None, color: bool | None = None, update_min_steps: int = 1) -> ProgressBar[V]
```

**Overloads** (the signature above is the runtime dispatcher):

- `def progressbar(length: int, label: str | None = None, hidden: bool = False, show_eta: bool = True, show_percent: bool | None = None, show_pos: bool = False, fill_char: str = '#', empty_char: str = '-', bar_template: str = '%(label)s  [%(bar)s]  %(info)s', info_sep: str = '  ', width: int = 36, file: TextIO | None = None, color: bool | None = None, update_min_steps: int = 1) -> ProgressBar[int]`
- `def progressbar(iterable: Iterable[V] | None = None, length: int | None = None, label: str | None = None, hidden: bool = False, show_eta: bool = True, show_percent: bool | None = None, show_pos: bool = False, item_show_func: Callable[[V | None], str | None] | None = None, fill_char: str = '#', empty_char: str = '-', bar_template: str = '%(label)s  [%(bar)s]  %(info)s', info_sep: str = '  ', width: int = 36, file: TextIO | None = None, color: bool | None = None, update_min_steps: int = 1) -> ProgressBar[V]`

**Also exported as** `typer.progressbar`

This function creates an iterable context manager that can be used
to iterate over something while showing a progress bar.  It will
either iterate over the `iterable` or `length` items (that are counted
up).  While iteration happens, this function will print a rendered
progress bar to the given `file` (defaults to stdout) and will attempt
to calculate remaining time and more.  By default, this progress bar
will not be rendered if the file is not a terminal.

The context manager creates the progress bar.  When the context
manager is entered the progress bar is already created.  With every
iteration over the progress bar, the iterable passed to the bar is
advanced and the bar is updated.  When the context manager exits,
a newline is printed and the progress bar is finalized on screen.

Note: The progress bar is currently designed for use cases where the
total progress can be expected to take at least several seconds.
Because of this, the ProgressBar class object won't display
progress that is considered too fast, and progress where the time
between steps is less than a second.

No printing must happen or the progress bar will be unintentionally
destroyed.


## prompt

Import as `typer.prompt`  ·  defined at `typer._click.termui.prompt`

```python
def prompt(text: str, default: Any | None = None, hide_input: bool = False, confirmation_prompt: bool | str = False, type: ParamType | Any | None = None, value_proc: Callable[[str], Any] | None = None, prompt_suffix: str = ': ', show_default: bool = True, err: bool = False, show_choices: bool = True) -> Any
```

**Also exported as** `typer.prompt`

Prompts a user for input.  This is a convenience function that can
be used to prompt a user for input later.

If the user aborts the input by sending an interrupt signal, this
function will catch it and raise an `Abort` exception.


## raw_terminal

`typer._click.termui.raw_terminal`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def raw_terminal() -> AbstractContextManager[int]
```

## secho

Import as `typer.secho`  ·  defined at `typer._click.termui.secho`

```python
def secho(message: Any | None = None, file: IO[AnyStr] | None = None, nl: bool = True, err: bool = False, color: bool | None = None, styles: Any = {}) -> None
```

**Also exported as** `typer.secho`

This function combines `echo` and `style` into one call.


## style

Import as `typer.style`  ·  defined at `typer._click.termui.style`

```python
def style(text: Any, fg: int | tuple[int, int, int] | str | None = None, bg: int | tuple[int, int, int] | str | None = None, bold: bool | None = None, dim: bool | None = None, underline: bool | None = None, overline: bool | None = None, italic: bool | None = None, blink: bool | None = None, reverse: bool | None = None, strikethrough: bool | None = None, reset: bool = True) -> str
```

**Also exported as** `typer.style`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Styles a text with ANSI styles and returns the new string.  By
default the styling is self contained which means that at the end
of the string a reset code is issued.  This can be prevented by
passing ``reset=False``.


