# `mcp.shared._compat`

Distribution: `mcp`

## __all__

`mcp.shared._compat.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['resync_tracer']
```

## resync_tracer

Import as `mcp.client.sse.resync_tracer`  ·  defined at `mcp.shared._compat.resync_tracer`

```python
async def resync_tracer() -> None
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resync coverage tracing after a cancelled task-group join.

A cancel delivered at a join resumes the awaiting coroutine chain via
`coro.throw()`; on CPython 3.11 (python/cpython#106749) that drops the
`'call'` trace events for the outer frames and desyncs coverage's CTracer
until the chain next suspends and resumes normally. Yielding once here
resumes via `.send()`, which re-stamps the missing events. Shielded so a
pending outer cancel is not re-delivered at this point; behaviorally a
no-op. Delete this module when Python 3.11 support ends (EOL 2027-10).


