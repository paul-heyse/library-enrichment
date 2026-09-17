# `typer._click._winconsole`

Distribution: `typer`

## CommandLineToArgvW

`typer._click._winconsole.CommandLineToArgvW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
CommandLineToArgvW = WINFUNCTYPE(POINTER(LPWSTR), LPCWSTR, POINTER(c_int))(('CommandLineToArgvW', windll.shell32))
```

## EOF

`typer._click._winconsole.EOF`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
EOF = b'\x1a'
```

## ERROR_NOT_ENOUGH_MEMORY

`typer._click._winconsole.ERROR_NOT_ENOUGH_MEMORY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ERROR_NOT_ENOUGH_MEMORY = 8
```

## ERROR_OPERATION_ABORTED

`typer._click._winconsole.ERROR_OPERATION_ABORTED`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ERROR_OPERATION_ABORTED = 995
```

## ERROR_SUCCESS

`typer._click._winconsole.ERROR_SUCCESS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ERROR_SUCCESS = 0
```

## GetCommandLineW

`typer._click._winconsole.GetCommandLineW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
GetCommandLineW = WINFUNCTYPE(LPWSTR)(('GetCommandLineW', windll.kernel32))
```

## GetConsoleMode

`typer._click._winconsole.GetConsoleMode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
GetConsoleMode = kernel32.GetConsoleMode
```

## GetLastError

`typer._click._winconsole.GetLastError`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
GetLastError = kernel32.GetLastError
```

## GetStdHandle

`typer._click._winconsole.GetStdHandle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
GetStdHandle = kernel32.GetStdHandle
```

## LocalFree

`typer._click._winconsole.LocalFree`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
LocalFree = WINFUNCTYPE(c_void_p, c_void_p)(('LocalFree', windll.kernel32))
```

## MAX_BYTES_WRITTEN

`typer._click._winconsole.MAX_BYTES_WRITTEN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
MAX_BYTES_WRITTEN = 32767
```

## PyBUF_SIMPLE

`typer._click._winconsole.PyBUF_SIMPLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
PyBUF_SIMPLE = 0
```

## PyBUF_WRITABLE

`typer._click._winconsole.PyBUF_WRITABLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
PyBUF_WRITABLE = 1
```

## PyBuffer_Release

`typer._click._winconsole.PyBuffer_Release`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
PyBuffer_Release = pythonapi.PyBuffer_Release
```

## PyObject_GetBuffer

`typer._click._winconsole.PyObject_GetBuffer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
PyObject_GetBuffer = pythonapi.PyObject_GetBuffer
```

## ReadConsoleW

`typer._click._winconsole.ReadConsoleW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ReadConsoleW = kernel32.ReadConsoleW
```

## STDERR_FILENO

`typer._click._winconsole.STDERR_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDERR_FILENO = 2
```

## STDERR_HANDLE

`typer._click._winconsole.STDERR_HANDLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDERR_HANDLE = GetStdHandle(-12)
```

## STDIN_FILENO

`typer._click._winconsole.STDIN_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDIN_FILENO = 0
```

## STDIN_HANDLE

`typer._click._winconsole.STDIN_HANDLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDIN_HANDLE = GetStdHandle(-10)
```

## STDOUT_FILENO

`typer._click._winconsole.STDOUT_FILENO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDOUT_FILENO = 1
```

## STDOUT_HANDLE

`typer._click._winconsole.STDOUT_HANDLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
STDOUT_HANDLE = GetStdHandle(-11)
```

## WriteConsoleW

`typer._click._winconsole.WriteConsoleW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
WriteConsoleW = kernel32.WriteConsoleW
```

## _stream_factories

`typer._click._winconsole._stream_factories`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_stream_factories: Mapping[int, Callable[[BinaryIO], TextIO]] = {0: _get_text_stdin, 1: _get_text_stdout, 2: _get_text_stderr}
```

## c_ssize_p

`typer._click._winconsole.c_ssize_p`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
c_ssize_p = POINTER(c_ssize_t)
```

## kernel32

`typer._click._winconsole.kernel32`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
kernel32 = windll.kernel32
```

## ConsoleStream

`typer._click._winconsole.ConsoleStream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class ConsoleStream
```

**Declared members (5)**

- `buffer = byte_stream`  _instance-attribute_
- `def isatty(self) -> bool`
- `name: str`  _property_
- `def write(self, x: AnyStr) -> int`
- `def writelines(self, lines: Iterable[AnyStr]) -> None`

## Py_buffer

`typer._click._winconsole.Py_buffer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Py_buffer(Structure)
```

**Bases** `Structure`

## _WindowsConsoleRawIOBase

`typer._click._winconsole._WindowsConsoleRawIOBase`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _WindowsConsoleRawIOBase(io.RawIOBase)
```

**Bases** `io.RawIOBase`

**Declared members (2)**

- `handle = handle`  _instance-attribute_
- `def isatty(self) -> Literal[True]`

## _WindowsConsoleReader

`typer._click._winconsole._WindowsConsoleReader`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _WindowsConsoleReader(_WindowsConsoleRawIOBase)
```

**Bases** `_WindowsConsoleRawIOBase`

**Declared members (2)**

- `def readable(self) -> Literal[True]`
- `def readinto(self, b: Buffer) -> int`

**Inherited (2)**

- from `typer._click._winconsole._WindowsConsoleRawIOBase`: `handle`, `isatty`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _WindowsConsoleWriter

`typer._click._winconsole._WindowsConsoleWriter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _WindowsConsoleWriter(_WindowsConsoleRawIOBase)
```

**Bases** `_WindowsConsoleRawIOBase`

**Declared members (2)**

- `def writable(self) -> Literal[True]`
- `def write(self, b: Buffer) -> int`

**Inherited (2)**

- from `typer._click._winconsole._WindowsConsoleRawIOBase`: `handle`, `isatty`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _get_text_stderr

`typer._click._winconsole._get_text_stderr`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_text_stderr(buffer_stream: BinaryIO) -> TextIO
```

## _get_text_stdin

`typer._click._winconsole._get_text_stdin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_text_stdin(buffer_stream: BinaryIO) -> TextIO
```

## _get_text_stdout

`typer._click._winconsole._get_text_stdout`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_text_stdout(buffer_stream: BinaryIO) -> TextIO
```

## _get_windows_console_stream

`typer._click._winconsole._get_windows_console_stream`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_windows_console_stream(f: TextIO, encoding: str | None, errors: str | None) -> TextIO | None
```

## _is_console

`typer._click._winconsole._is_console`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_console(f: TextIO) -> bool
```

## get_buffer

`typer._click._winconsole.get_buffer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def get_buffer(obj: Buffer, writable: bool = False) -> Array[c_char]
```

