# `typer._click._compat`

Distribution: `typer`

## CYGWIN

`typer._click._compat.CYGWIN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
CYGWIN = sys.platform.startswith('cygwin')
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## WIN

`typer._click._compat.WIN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
WIN = sys.platform.startswith('win')
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _ansi_re

`typer._click._compat._ansi_re`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ansi_re = re.compile('\\033\\[[;?0-9]*[a-zA-Z]')
```

## _ansi_stream_wrappers

`typer._click._compat._ansi_stream_wrappers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ansi_stream_wrappers: MutableMapping[TextIO, TextIO] = WeakKeyDictionary()
```

## _default_text_stderr

`typer._click._compat._default_text_stderr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_default_text_stderr = _make_cached_stream_func(lambda: sys.stderr, get_text_stderr)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _default_text_stdin

`typer._click._compat._default_text_stdin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_default_text_stdin = _make_cached_stream_func(lambda: sys.stdin, get_text_stdin)
```

## _default_text_stdout

`typer._click._compat._default_text_stdout`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_default_text_stdout = _make_cached_stream_func(lambda: sys.stdout, get_text_stdout)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## binary_streams

`typer._click._compat.binary_streams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
binary_streams: Mapping[str, Callable[[], BinaryIO]] = {'stdin': get_binary_stdin, 'stdout': get_binary_stdout, 'stderr': get_binary_stderr}
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## text_streams

`typer._click._compat.text_streams`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
text_streams: Mapping[str, Callable[[str | None, str | None], TextIO]] = {'stdin': get_text_stdin, 'stdout': get_text_stdout, 'stderr': get_text_stderr}
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _AtomicFile

`typer._click._compat._AtomicFile`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _AtomicFile
```

**Declared members (3)**

- `def close(self, delete: bool = False) -> None`
- `closed = False`  _instance-attribute_
- `name: str`  _property_

## _FixupStream

`typer._click._compat._FixupStream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _FixupStream
```

**Declared members (4)**

- `def read1(self, size: int) -> bytes`
- `def readable(self) -> bool`
- `def seekable(self) -> bool`
- `def writable(self) -> bool`

The new io interface needs more from streams than streams
traditionally implement.  As such, this fix-up code is necessary in
some circumstances.


## _NonClosingTextIOWrapper

`typer._click._compat._NonClosingTextIOWrapper`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NonClosingTextIOWrapper(io.TextIOWrapper)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `io.TextIOWrapper`

**Declared members (1)**

- `def isatty(self) -> bool`

## _find_binary_reader

`typer._click._compat._find_binary_reader`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _find_binary_reader(stream: IO[Any]) -> BinaryIO | None
```

## _find_binary_writer

`typer._click._compat._find_binary_writer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _find_binary_writer(stream: IO[Any]) -> BinaryIO | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _force_correct_text_reader

`typer._click._compat._force_correct_text_reader`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _force_correct_text_reader(text_reader: IO[Any], encoding: str | None, errors: str | None) -> TextIO
```

## _force_correct_text_stream

`typer._click._compat._force_correct_text_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _force_correct_text_stream(text_stream: IO[Any], encoding: str | None, errors: str | None, is_binary: Callable[[IO[Any], bool], bool], find_binary: Callable[[IO[Any]], BinaryIO | None]) -> TextIO
```

## _force_correct_text_writer

`typer._click._compat._force_correct_text_writer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _force_correct_text_writer(text_writer: IO[Any], encoding: str | None, errors: str | None) -> TextIO
```

## _get_argv_encoding

`typer._click._compat._get_argv_encoding`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_argv_encoding() -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _get_windows_console_stream

`typer._click._compat._get_windows_console_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_windows_console_stream(f: TextIO, encoding: str | None, errors: str | None) -> TextIO | None
```

## _is_binary_reader

`typer._click._compat._is_binary_reader`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_binary_reader(stream: IO[Any], default: bool = False) -> bool
```

## _is_binary_writer

`typer._click._compat._is_binary_writer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_binary_writer(stream: IO[Any], default: bool = False) -> bool
```

## _is_compat_stream_attr

`typer._click._compat._is_compat_stream_attr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_compat_stream_attr(stream: TextIO, attr: str, value: str | None) -> bool
```

A stream attribute is compatible if it is equal to the
desired value or the desired value is unset and the attribute
has a value.


## _is_compatible_text_stream

`typer._click._compat._is_compatible_text_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_compatible_text_stream(stream: TextIO, encoding: str | None, errors: str | None) -> bool
```

Check if a stream's encoding and errors attributes are
compatible with the desired values.


## _is_jupyter_kernel_output

`typer._click._compat._is_jupyter_kernel_output`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_jupyter_kernel_output(stream: IO[Any]) -> bool
```

## _make_cached_stream_func

`typer._click._compat._make_cached_stream_func`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_cached_stream_func(src_func: Callable[[], TextIO], wrapper_func: Callable[[], TextIO]) -> Callable[[], TextIO]
```

## _make_text_stream

`typer._click._compat._make_text_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_text_stream(stream: BinaryIO, encoding: str | None, errors: str) -> TextIO
```

## _stream_is_misconfigured

`typer._click._compat._stream_is_misconfigured`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _stream_is_misconfigured(stream: TextIO) -> bool
```

A stream is misconfigured if its encoding is ASCII.


## _wrap_io_open

`typer._click._compat._wrap_io_open`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _wrap_io_open(file: str | os.PathLike[str] | int, mode: str, encoding: str | None, errors: str | None) -> IO[Any]
```

Handles not passing ``encoding`` and ``errors`` in binary mode.


## auto_wrap_for_ansi

`typer._click._compat.auto_wrap_for_ansi`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def auto_wrap_for_ansi(stream: TextIO, color: bool | None = None) -> TextIO
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Support ANSI color and style codes on Windows by wrapping a
stream with colorama.


## get_best_encoding

`typer._click._compat.get_best_encoding`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_best_encoding(stream: IO[Any]) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Returns the default stream encoding if not found.


## get_binary_stderr

`typer._click._compat.get_binary_stderr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_binary_stderr() -> BinaryIO
```

## get_binary_stdin

`typer._click._compat.get_binary_stdin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_binary_stdin() -> BinaryIO
```

## get_binary_stdout

`typer._click._compat.get_binary_stdout`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_binary_stdout() -> BinaryIO
```

## get_text_stderr

`typer._click._compat.get_text_stderr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_text_stderr(encoding: str | None = None, errors: str | None = None) -> TextIO
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## get_text_stdin

`typer._click._compat.get_text_stdin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_text_stdin(encoding: str | None = None, errors: str | None = None) -> TextIO
```

## get_text_stdout

`typer._click._compat.get_text_stdout`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_text_stdout(encoding: str | None = None, errors: str | None = None) -> TextIO
```

## is_ascii_encoding

`typer._click._compat.is_ascii_encoding`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def is_ascii_encoding(encoding: str) -> bool
```

Checks if a given encoding is ascii.


## isatty

`typer._click._compat.isatty`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def isatty(stream: IO[Any]) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## open_stream

`typer._click._compat.open_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def open_stream(filename: str | os.PathLike[str], mode: str = 'r', encoding: str | None = None, errors: str | None = 'strict', atomic: bool = False) -> tuple[IO[Any], bool]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## should_strip_ansi

`typer._click._compat.should_strip_ansi`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def should_strip_ansi(stream: IO[Any] | None = None, color: bool | None = None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## strip_ansi

`typer._click._compat.strip_ansi`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def strip_ansi(value: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## term_len

`typer._click._compat.term_len`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def term_len(x: str) -> int
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

