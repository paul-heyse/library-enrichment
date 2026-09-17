# `rich.logging`

Distribution: `rich`

## FORMAT

`rich.logging.FORMAT`

```python
FORMAT = '%(message)s'
```

**Inferred type** (`ty`, not declared in the source): `Literal["%(message)s"]`

## log

`rich.logging.log`

```python
log = logging.getLogger('rich')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## RichHandler

`rich.logging.RichHandler`

```python
class RichHandler(Handler)
```

**Bases** `Handler`

**Declared members (22)**

- `HIGHLIGHTER_CLASS: Type[Highlighter] = ReprHighlighter`  _class-attribute_
- `KEYWORDS: Optional[List[str]] = ['GET', 'POST', 'HEAD', 'PUT', 'DELETE', 'OPTIONS', 'TRACE', 'PATCH']`  _class-attribute_
- `console = console or get_console()`  _instance-attribute_
- `def emit(self, record: LogRecord) -> None`
  Invoked by logging.
- `enable_link_path = enable_link_path`  _instance-attribute_
- `def get_level_text(self, record: LogRecord) -> Text`
  Get the level name from the record.
- `highlighter = highlighter or self.HIGHLIGHTER_CLASS()`  _instance-attribute_
- `keywords = keywords`  _instance-attribute_
- `locals_max_length = locals_max_length`  _instance-attribute_
- `locals_max_string = locals_max_string`  _instance-attribute_
- `markup = markup`  _instance-attribute_
- `def render(self, record: LogRecord, traceback: Optional[Traceback], message_renderable: ConsoleRenderable) -> ConsoleRenderable`
  Render log for display.
- `def render_message(self, record: LogRecord, message: str) -> ConsoleRenderable`
  Render message text in to Text.
- `rich_tracebacks = rich_tracebacks`  _instance-attribute_
- `tracebacks_code_width = tracebacks_code_width`  _instance-attribute_
- `tracebacks_extra_lines = tracebacks_extra_lines`  _instance-attribute_
- `tracebacks_max_frames = tracebacks_max_frames`  _instance-attribute_
- `tracebacks_show_locals = tracebacks_show_locals`  _instance-attribute_
- `tracebacks_suppress = tracebacks_suppress`  _instance-attribute_
- `tracebacks_theme = tracebacks_theme`  _instance-attribute_
- `tracebacks_width = tracebacks_width`  _instance-attribute_
- `tracebacks_word_wrap = tracebacks_word_wrap`  _instance-attribute_

A logging handler that renders output with Rich. The time / level / message and file are displayed in columns.
The level is color coded, and the message is syntax highlighted.

Note:
    Be careful when enabling console markup in log messages if you have configured logging for libraries not
    under your control. If a dependency writes messages containing square brackets, it may not produce the intended output.

Args:
    level (Union[int, str], optional): Log level. Defaults to logging.NOTSET.
    console (:class:`~rich.console.Console`, optional): Optional console instance to write logs.
        Default will use a global console instance writing to stdout.
    show_time (bool, optional): Show a column for the time. Defaults to True.
    omit_repeated_times (bool, optional): Omit repetition of the same time. Defaults to True.
    show_level (bool, optional): Show a column for the level. Defaults to True.
    show_path (bool, optional): Show the path to the original log call. Defaults to True.
    enable_link_path (bool, optional): Enable terminal link of path column to file. Defaults to True.
    highlighter (Highlighter, optional): Highlighter to style log messages, or None to use ReprHighlighter. Defaults to None.
    markup (bool, optional): Enable console markup in log messages. Defaults to False.
    rich_tracebacks (bool, optional): Enable rich tracebacks with syntax highlighting and formatting. Defaults to False.
    tracebacks_width (Optional[int], optional): Number of characters used to render tracebacks, or None for full width. Defaults to None.
    tracebacks_code_width (int, optional): Number of code characters used to render tracebacks, or None for full width. Defaults to 88.
    tracebacks_extra_lines (int, optional): Additional lines of code to render tracebacks, or None for full width. Defaults to None.
    tracebacks_theme (str, optional): Override pygments theme used in traceback.
    tracebacks_word_wrap (bool, optional): Enable word wrapping of long tracebacks lines. Defaults to True.
    tracebacks_show_locals (bool, optional): Enable display of locals in tracebacks. Defaults to False.
    tracebacks_suppress (Sequence[Union[str, ModuleType]]): Optional sequence of modules or paths to exclude from traceback.
    tracebacks_max_frames (int, optional): Optional maximum number of frames returned by traceback.
    locals_max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to 10.
    locals_max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to 80.
    log_time_format (Union[str, TimeFormatterCallable], optional): If ``log_time`` is enabled, either string for strftime or callable that formats the time. Defaults to "[%x %X] ".
    keywords (List[str], optional): List of words to highlight instead of ``RichHandler.KEYWORDS``.


## divide

`rich.logging.divide`

```python
def divide() -> None
```

