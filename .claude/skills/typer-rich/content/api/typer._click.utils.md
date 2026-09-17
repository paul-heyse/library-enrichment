# `typer._click.utils`

Distribution: `typer`

## P

`typer._click.utils.P`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
P = ParamSpec('P')
```

## R

`typer._click.utils.R`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
R = TypeVar('R')
```

## LazyFile

`typer._click.utils.LazyFile`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class LazyFile
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (9)**

- `atomic = atomic`  _instance-attribute_
- `def close(self) -> None`
  Closes the underlying file, no matter what.
- `def close_intelligently(self) -> None`
  This function only closes the file if it was opened by the lazy file wrapper.  For instance this will never close stdin.
- `encoding = encoding`  _instance-attribute_
- `errors = errors`  _instance-attribute_
- `mode = mode`  _instance-attribute_
- `name: str = os.fspath(filename)`  _instance-attribute_
- `def open(self) -> IO[Any]`
  Opens the file if it's not yet open.  This call might fail with a `FileError`.  Not handling this error will produce an error that Click shows.
- `should_close: bool`  _instance-attribute_

A lazy file works like a regular file but it does not fully open
the file but it does perform some basic checks early to see if the
filename parameter does make sense.  This is useful for safely opening
files for writing.


## PacifyFlushWrapper

`typer._click.utils.PacifyFlushWrapper`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class PacifyFlushWrapper
```

**Declared members (2)**

- `def flush(self) -> None`
- `wrapped = wrapped`  _instance-attribute_

This wrapper is used to catch and suppress BrokenPipeErrors resulting
from ``.flush()`` being called on broken pipe during the shutdown/final-GC
of the Python interpreter. Notably ``.flush()`` is always called on
``sys.stdout`` and ``sys.stderr``. So as to have minimal impact on any
other cleanup code, and the case where the underlying file is not a broken
pipe, all calls and attributes are proxied.


## _detect_program_name

`typer._click.utils._detect_program_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _detect_program_name(path: str | None = None, _main: ModuleType | None = None) -> str
```

Determine the command used to run the program, for use in help
text. If a file or entry point was executed, the file name is
returned. If ``python -m`` was used to execute a module or package,
``python -m name`` is returned.

This doesn't try to be too precise, the goal is to give a concise
name for help text. Files are only shown as their name without the
path. ``python`` is only shown for modules, and the full path to
``sys.executable`` is not shown.


## _expand_args

`typer._click.utils._expand_args`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _expand_args(args: Iterable[str], user: bool = True, env: bool = True, glob_recursive: bool = True) -> list[str]
```

Simulate Unix shell expansion with Python functions.


## _posixify

`typer._click.utils._posixify`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _posixify(name: str) -> str
```

## echo

Import as `typer.echo`  ·  defined at `typer._click.utils.echo`

```python
def echo(message: Any | None = None, file: IO[Any] | None = None, nl: bool = True, err: bool = False, color: bool | None = None) -> None
```

**Also exported as** `typer._click.echo`, `typer.echo`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Print a message and newline to stdout or a file. This should be
used instead of `print` because it provides better support
for different data, files, and environments.

Compared to `print`, this does the following:

-   Ensures that the output encoding is not misconfigured on Linux.
-   Supports Unicode in the Windows console.
-   Supports writing to binary outputs, and supports writing bytes
    to text outputs.
-   Supports colors and styles on Windows.
-   Removes ANSI color and style codes if the output does not look
    like an interactive terminal.
-   Always flushes the output.


## format_filename

Import as `typer.format_filename`  ·  defined at `typer._click.utils.format_filename`

```python
def format_filename(filename: str | bytes | os.PathLike[str] | os.PathLike[bytes], shorten: bool = False) -> str
```

**Also exported as** `typer.format_filename`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Format a filename as a string for display. Ensures the filename can be
displayed by replacing any invalid bytes or surrogate escapes in the name
with the replacement character ``�``.

Invalid bytes or surrogate escapes will raise an error when written to a
stream with ``errors="strict"``. This will typically happen with ``stdout``
when the locale is something like ``en_GB.UTF-8``.

Many scenarios *are* safe to write surrogates though, due to PEP 538 and
PEP 540, including:

-   Writing to ``stderr``, which uses ``errors="backslashreplace"``.
-   The system has ``LANG=C.UTF-8``, ``C``, or ``POSIX``. Python opens
    stdout and stderr with ``errors="surrogateescape"``.
-   None of ``LANG/LC_*`` are set. Python assumes ``LANG=C.UTF-8``.
-   Python is started in UTF-8 mode  with  ``PYTHONUTF8=1`` or ``-X utf8``.
    Python opens stdout and stderr with ``errors="surrogateescape"``.


## get_app_dir

Import as `typer.get_app_dir`  ·  defined at `typer._click.utils.get_app_dir`

```python
def get_app_dir(app_name: str, roaming: bool = True, force_posix: bool = False) -> str
```

**Also exported as** `typer.get_app_dir`

Returns the config folder for the application.  The default behavior
is to return whatever is most appropriate for the operating system.

To give you an idea, for an app called ``"Foo Bar"``, something like
the following folders could be returned:

Mac OS X:
  ``~/Library/Application Support/Foo Bar``
Mac OS X (POSIX):
  ``~/.foo-bar``
Unix:
  ``~/.config/foo-bar``
Unix (POSIX):
  ``~/.foo-bar``
Windows (roaming):
  ``C:\Users\<user>\AppData\Roaming\Foo Bar``
Windows (not roaming):
  ``C:\Users\<user>\AppData\Local\Foo Bar``


## get_binary_stream

Import as `typer.get_binary_stream`  ·  defined at `typer._click.utils.get_binary_stream`

```python
def get_binary_stream(name: Literal['stdin', 'stdout', 'stderr']) -> BinaryIO
```

**Also exported as** `typer.get_binary_stream`

Returns a system stream for byte processing.


## get_text_stream

Import as `typer.get_text_stream`  ·  defined at `typer._click.utils.get_text_stream`

```python
def get_text_stream(name: Literal['stdin', 'stdout', 'stderr'], encoding: str | None = None, errors: str | None = 'strict') -> TextIO
```

**Also exported as** `typer.get_text_stream`

Returns a system stream for text processing.  This usually returns
a wrapped stream around a binary stream returned from
`get_binary_stream` but it also can take shortcuts for already
correctly configured streams.


## make_default_short_help

`typer._click.utils.make_default_short_help`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def make_default_short_help(help: str, max_length: int = 45) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Returns a condensed version of help string.


## safecall

`typer._click.utils.safecall`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def safecall(func: Callable[P, R]) -> Callable[P, R | None]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Wraps a function so that it swallows exceptions.


