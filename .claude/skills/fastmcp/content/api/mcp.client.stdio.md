# `mcp.client.stdio`

Distribution: `mcp`

## DEFAULT_INHERITED_ENV_VARS

`mcp.client.stdio.DEFAULT_INHERITED_ENV_VARS`

```python
DEFAULT_INHERITED_ENV_VARS = ['APPDATA', 'HOMEDRIVE', 'HOMEPATH', 'LOCALAPPDATA', 'PATH', 'PATHEXT', 'PROCESSOR_ARCHITECTURE', 'SYSTEMDRIVE', 'SYSTEMROOT', 'TEMP', 'USERNAME', 'USERPROFILE'] if sys.platform == 'win32' else ['HOME', 'LOGNAME', 'PATH', 'SHELL', 'TERM', 'USER']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## FORCE_KILL_TIMEOUT

`mcp.client.stdio.FORCE_KILL_TIMEOUT`

```python
FORCE_KILL_TIMEOUT = 2.0
```

**Inferred type** (`ty`, not declared in the source): `float*`

## PROCESS_TERMINATION_TIMEOUT

`mcp.client.stdio.PROCESS_TERMINATION_TIMEOUT`

```python
PROCESS_TERMINATION_TIMEOUT = 2.0
```

**Inferred type** (`ty`, not declared in the source): `float*`

## _EXIT_POLL_INTERVAL

`mcp.client.stdio._EXIT_POLL_INTERVAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EXIT_POLL_INTERVAL = 0.01
```

## _KILL_REAP_TIMEOUT

`mcp.client.stdio._KILL_REAP_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_KILL_REAP_TIMEOUT = 2.0
```

## _WRITER_FLUSH_TIMEOUT

`mcp.client.stdio._WRITER_FLUSH_TIMEOUT`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_WRITER_FLUSH_TIMEOUT = 0.5
```

## logger

`mcp.client.stdio.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## StdioServerParameters

Import as `mcp.StdioServerParameters`  ·  defined at `mcp.client.stdio.StdioServerParameters`

```python
class StdioServerParameters(BaseModel)
```

**Also exported as** `mcp.StdioServerParameters`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (6)**

- `args: list[str] = Field(default_factory=list)`  _class-attribute, instance-attribute_
  Command line arguments to pass to the executable.
- `command: str`  _instance-attribute_
  The executable to run to start the server.
- `cwd: str | Path | None = None`  _class-attribute, instance-attribute_
  The working directory to use when spawning the process.
- `encoding: str = 'utf-8'`  _class-attribute, instance-attribute_
  Text encoding for messages to and from the server.
- `encoding_error_handler: Literal['strict', 'ignore', 'replace'] = 'strict'`  _class-attribute, instance-attribute_
  Encoding error handler; see https://docs.python.org/3/library/codecs.html#error-handlers.
- `env: dict[str, str] | None = None`  _class-attribute, instance-attribute_
  Extra environment variables, merged over get_default_environment().

## _aclose_all

`mcp.client.stdio._aclose_all`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _aclose_all(streams: AsyncResource = ()) -> None
```

Closes every given stream.


## _close_pipe

`mcp.client.stdio._close_pipe`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _close_pipe(stream: AsyncResource) -> None
```

Closes a pipe stream, tolerating one already closed, broken, or contended.


## _close_subprocess_transport

`mcp.client.stdio._close_subprocess_transport`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _close_subprocess_transport(process: ServerProcess) -> None
```

Closes the asyncio subprocess transport, if there is one.

The transport otherwise stays open (and warns at GC) while a surviving
descendant holds a pipe end; nothing public exposes it, hence the attribute
walk. No-op on trio and the Windows fallback.


## _create_platform_compatible_process

`mcp.client.stdio._create_platform_compatible_process`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _create_platform_compatible_process(command: str, args: list[str], env: dict[str, str] | None = None, errlog: TextIO = sys.stderr, cwd: Path | str | None = None) -> ServerProcess
```

Spawns the server in its own kill scope.

A new session/process group on POSIX, a Job Object on Windows.


## _drain_stdout

`mcp.client.stdio._drain_stdout`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _drain_stdout(process: ServerProcess) -> None
```

Consumes and discards the server's remaining stdout.

Keeps a server flushing buffered output from blocking on a full pipe and
missing its chance to exit; shielded, raw bytes, ends when shutdown closes
the pipe.


## _get_executable_command

`mcp.client.stdio._get_executable_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_executable_command(command: str) -> str
```

Normalizes the command for the current platform.


## _parse_line

`mcp.client.stdio._parse_line`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_line(line: str) -> SessionMessage | Exception
```

Parses one stdout line, returning parse errors as values for the session to surface.


## _stop_server_process

`mcp.client.stdio._stop_server_process`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _stop_server_process(process: ServerProcess) -> None
```

Closes stdin, waits out the grace period, then kills the whole tree.

The escalation order is spec text; timeouts and tree-wide scope are SDK policy:
https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle#shutdown


## _terminate_process_tree

`mcp.client.stdio._terminate_process_tree`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _terminate_process_tree(process: ServerProcess) -> None
```

Kills the process and all its descendants.

POSIX: SIGTERM to the process group, SIGKILL after FORCE_KILL_TIMEOUT.
Windows: immediate Job Object termination (already a hard kill).


## _wait_for_process_exit

`mcp.client.stdio._wait_for_process_exit`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _wait_for_process_exit(process: ServerProcess, timeout: float) -> bool
```

Returns whether the process died within the timeout, by polling returncode.

Not process.wait(): on asyncio 3.11+ it also waits for pipe EOF, and a
child that inherited the pipes makes an exited server look hung.


## get_default_environment

`mcp.client.stdio.get_default_environment`

```python
def get_default_environment() -> dict[str, str]
```

Returns only the environment variables that are safe to inherit.


## stdio_client

Import as `mcp.stdio_client`  ·  defined at `mcp.client.stdio.stdio_client`

```python
async def stdio_client(server: StdioServerParameters, errlog: TextIO = sys.stderr) -> AsyncGenerator[TransportStreams, None]
```

**Also exported as** `mcp.stdio_client`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Spawns an MCP server subprocess and connects to it over stdin/stdout.

Raises:
    OSError: If the server process cannot be spawned.
    ValueError: If the spawn parameters are invalid (embedded NUL bytes).


