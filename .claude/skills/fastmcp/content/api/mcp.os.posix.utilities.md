# `mcp.os.posix.utilities`

Distribution: `mcp`

## _GROUP_POLL_INTERVAL

`mcp.os.posix.utilities._GROUP_POLL_INTERVAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GROUP_POLL_INTERVAL = 0.01
```

## logger

`mcp.os.posix.utilities.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _group_alive

`mcp.os.posix.utilities._group_alive`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _group_alive(pgid: int) -> bool
```

Probes the group with signal 0; only ESRCH proves it is gone.


## terminate_posix_process_tree

Import as `mcp.client.stdio.terminate_posix_process_tree`  ·  defined at `mcp.os.posix.utilities.terminate_posix_process_tree`

```python
async def terminate_posix_process_tree(process: Process, timeout_seconds: float = 2.0) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Terminates a process and all its descendants on POSIX.

SIGTERMs the process group, waits up to timeout_seconds for it to
disappear, then SIGKILLs whatever remains. killpg reaches every descendant
atomically, even ones whose parent already exited; daemonizers that left
the group escape by design. A group only disappears once every member is
dead and reaped, so a client running as PID 1 should reap orphans (e.g.
docker run --init) or the wait below runs its full timeout.


