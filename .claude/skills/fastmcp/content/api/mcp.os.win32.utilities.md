# `mcp.os.win32.utilities`

Distribution: `mcp`

## ServerProcess

Import as `mcp.client.stdio.ServerProcess`  ·  defined at `mcp.os.win32.utilities.ServerProcess`

```python
ServerProcess: TypeAlias = Process | FallbackProcess
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _EXIT_POLL_INTERVAL

`mcp.os.win32.utilities._EXIT_POLL_INTERVAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EXIT_POLL_INTERVAL = 0.01
```

## _process_jobs

`mcp.os.win32.utilities._process_jobs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_process_jobs: weakref.WeakKeyDictionary[Process | FallbackProcess, object] = weakref.WeakKeyDictionary()
```

## logger

`mcp.os.win32.utilities.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FallbackProcess

`mcp.os.win32.utilities.FallbackProcess`

```python
class FallbackProcess
```

**Declared members (8)**

- `def kill(self) -> None`
  Kills the subprocess (on Windows the same hard kill as terminate).
- `pid: int`  _property_
  Returns the process ID.
- `popen: subprocess.Popen[bytes] = popen_obj`  _instance-attribute_
- `returncode: int | None`  _property_
  The exit code, or None while the process is still running.
- `stdin = FileWriteStream(cast(BinaryIO, stdin)) if stdin else None`  _instance-attribute_
- `stdout = FileReadStream(cast(BinaryIO, stdout)) if stdout else None`  _instance-attribute_
- `def terminate(self) -> None`
  Terminates the subprocess.
- `async def wait(self) -> int`  _async_
  Waits for exit by polling the Popen.

Async wrapper around subprocess.Popen for SelectorEventLoop.

Windows event loops without async subprocess support get this Popen-backed
fallback, with anyio file streams wrapping the pipes.


## _close_job_handle

`mcp.os.win32.utilities._close_job_handle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _close_job_handle(job: object) -> None
```

Closes a Job Object handle, tolerating one that is already closed.


## _create_job_object

`mcp.os.win32.utilities._create_job_object`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _create_job_object() -> object | None
```

Creates a Windows Job Object configured to terminate all its processes when closed.


## _create_windows_fallback_process

`mcp.os.win32.utilities._create_windows_fallback_process`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _create_windows_fallback_process(command: str, args: list[str], env: dict[str, str] | None = None, errlog: TextIO | None = sys.stderr, cwd: Path | str | None = None) -> FallbackProcess
```

Spawns via subprocess.Popen and wraps it in FallbackProcess.


## _maybe_assign_process_to_job

`mcp.os.win32.utilities._maybe_assign_process_to_job`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _maybe_assign_process_to_job(process: Process | FallbackProcess, job: object | None) -> None
```

Assigns the process to the job and records it for tree termination.

On any failure the job handle is closed instead.


## close_process_job

Import as `mcp.client.stdio.close_process_job`  ·  defined at `mcp.os.win32.utilities.close_process_job`

```python
def close_process_job(process: Process | FallbackProcess) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Closes the process's Job Object handle, if it still has one.

KILL_ON_JOB_CLOSE makes the close also kill any members still alive,
deterministically rather than at GC time; a deliberate divergence from
POSIX, where a graceful server's children are left alive.


## create_windows_process

Import as `mcp.client.stdio.create_windows_process`  ·  defined at `mcp.os.win32.utilities.create_windows_process`

```python
async def create_windows_process(command: str, args: list[str], env: dict[str, str] | None = None, errlog: TextIO | None = sys.stderr, cwd: Path | str | None = None) -> Process | FallbackProcess
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Creates a subprocess with Job Object support for tree termination.

Spawns via anyio's open_process; event loops without async subprocess
support (notably the SelectorEventLoop) raise NotImplementedError, in which
case the spawn falls back to a Popen-backed FallbackProcess. Either way the
process is then assigned to a Job Object so its children can be terminated
with it; children spawned before the assignment completes are not captured
(see the inline note below).

Returns:
    Process | FallbackProcess: The spawned process with async stdin/stdout streams.


## get_windows_executable_command

Import as `mcp.client.stdio.get_windows_executable_command`  ·  defined at `mcp.os.win32.utilities.get_windows_executable_command`

```python
def get_windows_executable_command(command: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolves the command to a Windows executable path.

Tries the bare name first, then the common script extensions (.cmd, .bat,
.exe, .ps1).


## rebind_std_handle_to_fd

Import as `mcp.server.stdio.rebind_std_handle_to_fd`  ·  defined at `mcp.os.win32.utilities.rebind_std_handle_to_fd`

```python
def rebind_std_handle_to_fd(fd: int) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Points the Win32 standard-handle slot for fd 0, 1, or 2 at fd's current OS handle.

os.dup2 updates only the CRT descriptor table; subprocess handle inheritance
reads the Win32 slot, so it must be repointed too.

Raises:
    OSError: The slot could not be set.


## terminate_windows_process_tree

Import as `mcp.client.stdio.terminate_windows_process_tree`  ·  defined at `mcp.os.win32.utilities.terminate_windows_process_tree`

```python
async def terminate_windows_process_tree(process: Process | FallbackProcess) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Terminates the process's job, or just the process if it has no job.

Job termination is an immediate hard kill of every member. Windows has no
tree-wide SIGTERM; the stdin-close grace period is the server's chance to
exit cleanly.


