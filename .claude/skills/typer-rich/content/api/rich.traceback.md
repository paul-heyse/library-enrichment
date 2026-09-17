# `rich.traceback`

Distribution: `rich`

## LOCALS_MAX_LENGTH

`rich.traceback.LOCALS_MAX_LENGTH`

```python
LOCALS_MAX_LENGTH = 10
```

**Inferred type** (`ty`, not declared in the source): `Literal[10]`

## LOCALS_MAX_STRING

`rich.traceback.LOCALS_MAX_STRING`

```python
LOCALS_MAX_STRING = 80
```

**Inferred type** (`ty`, not declared in the source): `Literal[80]`

## WINDOWS

`rich.traceback.WINDOWS`

```python
WINDOWS = sys.platform == 'win32'
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## Frame

`rich.traceback.Frame`

```python
class Frame
```

**Declared members (6)**

- `filename: str`  _instance-attribute_
- `last_instruction: Optional[Tuple[Tuple[int, int], Tuple[int, int]]] = None`  _class-attribute, instance-attribute_
- `line: str = ''`  _class-attribute, instance-attribute_
- `lineno: int`  _instance-attribute_
- `locals: Optional[Dict[str, pretty.Node]] = None`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_

## PathHighlighter

`rich.traceback.PathHighlighter`

```python
class PathHighlighter(RegexHighlighter)
```

**Bases** `RegexHighlighter`

**Declared members (1)**

- `highlights = ['(?P<dim>.*/)(?P<bold>.+)']`  _class-attribute, instance-attribute_

**Inherited (2)**

- from `rich.highlighter.RegexHighlighter`: `base_style`, `highlight`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## Stack

`rich.traceback.Stack`

```python
class Stack
```

**Declared members (8)**

- `exc_type: str`  _instance-attribute_
- `exc_value: str`  _instance-attribute_
- `exceptions: List[Trace] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `frames: List[Frame] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `is_cause: bool = False`  _class-attribute, instance-attribute_
- `is_group: bool = False`  _class-attribute, instance-attribute_
- `notes: List[str] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `syntax_error: Optional[_SyntaxError] = None`  _class-attribute, instance-attribute_

## Trace

`rich.traceback.Trace`

```python
class Trace
```

**Declared members (1)**

- `stacks: List[Stack]`  _instance-attribute_

## Traceback

`rich.traceback.Traceback`

```python
class Traceback
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (19)**

- `LEXERS = {'': 'text', '.py': 'python', '.pxd': 'cython', '.pyx': 'cython', '.pxi': 'pyrex'}`  _class-attribute, instance-attribute_
- `code_width = code_width`  _instance-attribute_
- `extra_lines = extra_lines`  _instance-attribute_
- `def extract(cls, exc_type: Type[BaseException], exc_value: BaseException, traceback: Optional[TracebackType], show_locals: bool = False, locals_max_length: int = LOCALS_MAX_LENGTH, locals_max_string: int = LOCALS_MAX_STRING, locals_max_depth: Optional[int] = None, locals_hide_dunder: bool = True, locals_hide_sunder: bool = False, _visited_exceptions: Optional[Set[BaseException]] = None) -> Trace`  _classmethod_
  Extract traceback information.
- `def from_exception(cls, exc_type: Type[Any], exc_value: BaseException, traceback: Optional[TracebackType], width: Optional[int] = 100, code_width: Optional[int] = 88, extra_lines: int = 3, theme: Optional[str] = None, word_wrap: bool = False, show_locals: bool = False, locals_max_length: int = LOCALS_MAX_LENGTH, locals_max_string: int = LOCALS_MAX_STRING, locals_max_depth: Optional[int] = None, locals_hide_dunder: bool = True, locals_hide_sunder: bool = False, locals_overflow: Optional[OverflowMethod] = None, indent_guides: bool = True, suppress: Iterable[Union[str, ModuleType]] = (), max_frames: int = 100) -> Traceback`  _classmethod_
  Create a traceback from exception info
- `indent_guides = indent_guides`  _instance-attribute_
- `locals_hide_dunder = locals_hide_dunder`  _instance-attribute_
- `locals_hide_sunder = locals_hide_sunder`  _instance-attribute_
- `locals_max_depth = locals_max_depth`  _instance-attribute_
- `locals_max_length = locals_max_length`  _instance-attribute_
- `locals_max_string = locals_max_string`  _instance-attribute_
- `locals_overflow = locals_overlow`  _instance-attribute_
- `max_frames = max(4, max_frames) if max_frames > 0 else 0`  _instance-attribute_
- `show_locals = show_locals`  _instance-attribute_
- `suppress: Sequence[str] = []`  _instance-attribute_
- `theme = Syntax.get_theme(theme or 'ansi_dark')`  _instance-attribute_
- `trace = trace`  _instance-attribute_
- `width = width`  _instance-attribute_
- `word_wrap = word_wrap`  _instance-attribute_

A Console renderable that renders a traceback.

Args:
    trace (Trace, optional): A `Trace` object produced from `extract`. Defaults to None, which uses
        the last exception.
    width (Optional[int], optional): Number of characters used to traceback. Defaults to 100.
    code_width (Optional[int], optional): Number of code characters used to traceback. Defaults to 88.
    extra_lines (int, optional): Additional lines of code to render. Defaults to 3.
    theme (str, optional): Override pygments theme used in traceback.
    word_wrap (bool, optional): Enable word wrapping of long lines. Defaults to False.
    show_locals (bool, optional): Enable display of local variables. Defaults to False.
    indent_guides (bool, optional): Enable indent guides in code and locals. Defaults to True.
    locals_max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to 10.
    locals_max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to 80.
    locals_max_depth (int, optional): Maximum depths of locals before truncating, or None to disable. Defaults to None.
    locals_hide_dunder (bool, optional): Hide locals prefixed with double underscore. Defaults to True.
    locals_hide_sunder (bool, optional): Hide locals prefixed with single underscore. Defaults to False.
    locals_overflow (OverflowMethod, optional): How to handle overflowing locals, or None to disable. Defaults to None.
    suppress (Sequence[Union[str, ModuleType]]): Optional sequence of modules or paths to exclude from traceback.
    max_frames (int): Maximum number of frames to show in a traceback, 0 for no maximum. Defaults to 100.


## _SyntaxError

`rich.traceback._SyntaxError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _SyntaxError
```

**Declared members (6)**

- `filename: str`  _instance-attribute_
- `line: str`  _instance-attribute_
- `lineno: int`  _instance-attribute_
- `msg: str`  _instance-attribute_
- `notes: List[str] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `offset: int`  _instance-attribute_

## _iter_syntax_lines

`rich.traceback._iter_syntax_lines`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _iter_syntax_lines(start: SyntaxPosition, end: SyntaxPosition) -> Iterable[Tuple[int, int, int]]
```

Yield start and end positions per line.

Args:
    start: Start position.
    end: End position.

Returns:
    Iterable of (LINE, COLUMN1, COLUMN2).


## bar

`rich.traceback.bar`

```python
def bar(a: Any) -> None
```

## error

`rich.traceback.error`

```python
def error() -> None
```

## foo

`rich.traceback.foo`

```python
def foo(a: Any) -> None
```

## install

`rich.traceback.install`

```python
def install(console: Optional[Console] = None, width: Optional[int] = 100, code_width: Optional[int] = 88, extra_lines: int = 3, theme: Optional[str] = None, word_wrap: bool = False, show_locals: bool = False, locals_max_length: int = LOCALS_MAX_LENGTH, locals_max_string: int = LOCALS_MAX_STRING, locals_max_depth: Optional[int] = None, locals_hide_dunder: bool = True, locals_hide_sunder: Optional[bool] = None, locals_overflow: Optional[OverflowMethod] = None, indent_guides: bool = True, suppress: Iterable[Union[str, ModuleType]] = (), max_frames: int = 100) -> Callable[[Type[BaseException], BaseException, Optional[TracebackType]], Any]
```

Install a rich traceback handler.

Once installed, any tracebacks will be printed with syntax highlighting and rich formatting.


Args:
    console (Optional[Console], optional): Console to write exception to. Default uses internal Console instance.
    width (Optional[int], optional): Width (in characters) of traceback. Defaults to 100.
    code_width (Optional[int], optional): Code width (in characters) of traceback. Defaults to 88.
    extra_lines (int, optional): Extra lines of code. Defaults to 3.
    theme (Optional[str], optional): Pygments theme to use in traceback. Defaults to ``None`` which will pick
        a theme appropriate for the platform.
    word_wrap (bool, optional): Enable word wrapping of long lines. Defaults to False.
    show_locals (bool, optional): Enable display of local variables. Defaults to False.
    locals_max_length (int, optional): Maximum length of containers before abbreviating, or None for no abbreviation.
        Defaults to 10.
    locals_max_string (int, optional): Maximum length of string before truncating, or None to disable. Defaults to 80.
    locals_max_depth (int, optional): Maximum depths of locals before truncating, or None to disable. Defaults to None.
    locals_hide_dunder (bool, optional): Hide locals prefixed with double underscore. Defaults to True.
    locals_hide_sunder (bool, optional): Hide locals prefixed with single underscore. Defaults to False.
    locals_overflow (OverflowMethod, optional): How to handle overflowing locals, or None to disable. Defaults to None.
    indent_guides (bool, optional): Enable indent guides in code and locals. Defaults to True.
    suppress (Sequence[Union[str, ModuleType]]): Optional sequence of modules or paths to exclude from traceback.

Returns:
    Callable: The previous exception handler that was replaced.


