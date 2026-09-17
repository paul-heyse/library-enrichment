# `typer.testing`

Distribution: `typer`

## BytesIOCopy

`typer.testing.BytesIOCopy`

```python
class BytesIOCopy(io.BytesIO)
```

**Bases** `io.BytesIO`

**Declared members (3)**

- `copy_to = copy_to`  _instance-attribute_
- `def flush(self) -> None`
- `def write(self, b: ReadableBuffer) -> int`

Patch ``io.BytesIO`` to let the written stream be copied to another.


## CliRunner

`typer.testing.CliRunner`

```python
class CliRunner
```

**Declared members (6)**

- `charset = charset`  _instance-attribute_
- `env: Mapping[str, str | None] = env or {}`  _instance-attribute_
- `def get_default_prog_name(self, cli: _click.Command) -> str`
  Return the default program name for a command. The default is the `name` attribute or ``"root"`` if not set.
- `def invoke(self, app: Typer, args: str | Sequence[str] | None = None, input: bytes | str | None = None, env: Mapping[str, str | None] | None = None, catch_exceptions: bool = True, color: bool = False, extra: Any = {}) -> Result`
- `def isolation(self, input: str | bytes | None = None, env: Mapping[str, str | None] | None = None, color: bool = False) -> Iterator[tuple[io.BytesIO, io.BytesIO, io.BytesIO]]`
  A context manager that sets up the isolation for invoking of a command line tool.  This sets up `<stdin>` with the given input data and `os.environ` with the overrides from the given dictionary.
- `def make_env(self, overrides: Mapping[str, str | None] | None = None) -> Mapping[str, str | None]`
  Returns the environment overrides for invoking a script.

The CLI runner provides functionality to invoke a command line
script for unittesting purposes in an isolated environment.  This only
works in single-threaded systems without any concurrency as it changes the
global interpreter state. Based on functionality from Click.


## Result

`typer.testing.Result`

```python
class Result
```

**Declared members (11)**

- `exc_info = exc_info`  _instance-attribute_
- `exception = exception`  _instance-attribute_
- `exit_code = exit_code`  _instance-attribute_
- `output: str`  _property_
  The terminal output as unicode string, as the user would see it.
- `output_bytes = output_bytes`  _instance-attribute_
- `return_value = return_value`  _instance-attribute_
- `runner = runner`  _instance-attribute_
- `stderr: str`  _property_
  The standard error as unicode string.
- `stderr_bytes = stderr_bytes`  _instance-attribute_
- `stdout: str`  _property_
  The standard output as unicode string.
- `stdout_bytes = stdout_bytes`  _instance-attribute_

Holds the captured result of an invoked CLI script.


## StreamMixer

`typer.testing.StreamMixer`

```python
class StreamMixer
```

**Declared members (3)**

- `output: io.BytesIO = io.BytesIO()`  _instance-attribute_
- `stderr: io.BytesIO = BytesIOCopy(copy_to=self.output)`  _instance-attribute_
- `stdout: io.BytesIO = BytesIOCopy(copy_to=self.output)`  _instance-attribute_

Mixes `<stdout>` and `<stderr>` streams.

The result is available in the ``output`` attribute.


## _NamedTextIOWrapper

`typer.testing._NamedTextIOWrapper`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _NamedTextIOWrapper(io.TextIOWrapper)
```

**Bases** `io.TextIOWrapper`

**Declared members (2)**

- `mode: str`  _property_
- `name: str`  _property_

## make_input_stream

`typer.testing.make_input_stream`

```python
def make_input_stream(input: str | bytes | None, charset: str) -> BinaryIO
```

