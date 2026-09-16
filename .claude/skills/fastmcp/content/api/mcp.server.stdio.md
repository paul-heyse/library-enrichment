# `mcp.server.stdio`

Distribution: `mcp`

## _claims

`mcp.server.stdio._claims`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_claims: dict[int, _StreamClaim] = {}
```

## _claims_lock

`mcp.server.stdio._claims_lock`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_claims_lock = threading.Lock()
```

## _StreamClaim

`mcp.server.stdio._StreamClaim`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _StreamClaim
```

**Declared members (2)**

- `fd: int`  _instance-attribute_
- `private_fd: int | None = None`  _class-attribute, instance-attribute_

## _UnownedTextWrapper

`mcp.server.stdio._UnownedTextWrapper`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _UnownedTextWrapper(TextIOWrapper)
```

**Bases** `TextIOWrapper`

**Declared members (1)**

- `def close(self) -> None`

Text layer whose close never closes the underlying buffer.

The buffer is not the transport's to close: in the in-place paths it is the
sys stream's own buffer, and closing it at garbage collection destroyed
sys.stdout for the rest of the process (issue #1933).


## _claim_fd

`mcp.server.stdio._claim_fd`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _claim_fd(fd: int, stream: TextIO, mode: Literal['rb', 'wb'], open_diversion: Callable[[], int]) -> tuple[BinaryIO, Callable[[], None] | None]
```

Claim a standard stream: divert fd and serve the wire from a private duplicate.

Best-effort: when descriptors cannot be duplicated or diverted, serves the
sys stream's buffer in place, exactly as v1 did, with the claim held.

Raises:
    RuntimeError: fd is already claimed by another transport in this process.


## _dup_above_std

`mcp.server.stdio._dup_above_std`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _dup_above_std(fd: int) -> int
```

Duplicate fd onto a descriptor that cannot land in the standard range.


## _is_backed_by_fd

`mcp.server.stdio._is_backed_by_fd`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_backed_by_fd(stream: TextIO, fd: int) -> bool
```

## _open_stdin_diversion

`mcp.server.stdio._open_stdin_diversion`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _open_stdin_diversion() -> int
```

## _open_stdout_diversion

`mcp.server.stdio._open_stdout_diversion`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _open_stdout_diversion() -> int
```

## _restore_fd

`mcp.server.stdio._restore_fd`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _restore_fd(fd: int, private_fd: int) -> bool
```

Point fd back at the wire; the Windows handle rebind never affects the outcome.


## stdio_server

Import as `mcp.stdio_server`  ·  defined at `mcp.server.stdio.stdio_server`

```python
async def stdio_server(stdin: anyio.AsyncFile[str] | None = None, stdout: anyio.AsyncFile[str] | None = None)
```

**Also exported as** `mcp.stdio_server`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Serve MCP over the process's stdin and stdout.

While serving, fd 0 points at the null device and fd 1 at stderr, so handlers
and children read EOF and their stray output misses the wire; both descriptors
are restored on exit. Explicit streams skip the claim, and a second concurrent
stdio_server() raises RuntimeError.


