# `mcp.shared._context_streams`

Distribution: `mcp`

## T

`mcp.shared._context_streams.T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T = TypeVar('T')
```

## _Envelope

`mcp.shared._context_streams._Envelope`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Envelope = tuple[contextvars.Context, T]
```

## ContextReceiveStream

Import as `mcp.server.runner.ContextReceiveStream`  ·  defined at `mcp.shared._context_streams.ContextReceiveStream`

```python
class ContextReceiveStream(Generic[T])
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[T]`

**Declared members (5)**

- `async def aclose(self) -> None`  _async_
- `def clone(self) -> ContextReceiveStream[T]`
- `def close(self) -> None`
- `last_context: contextvars.Context | None = None`  _instance-attribute_
- `async def receive(self) -> T`  _async_

Receive-side wrapper that yields ``T`` and stores the sender's context in ``last_context``.


## ContextSendStream

Import as `mcp.server.sse.ContextSendStream`  ·  defined at `mcp.shared._context_streams.ContextSendStream`

```python
class ContextSendStream(Generic[T])
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Generic[T]`

**Declared members (4)**

- `async def aclose(self) -> None`  _async_
- `def clone(self) -> ContextSendStream[T]`
- `def close(self) -> None`
- `async def send(self, item: T) -> None`  _async_

Send-side wrapper that snapshots ``contextvars.copy_context()`` on every ``send()``.


## create_context_streams

Import as `mcp.client.sse.create_context_streams`  ·  defined at `mcp.shared._context_streams.create_context_streams`

```python
class create_context_streams(tuple[ContextSendStream[T], ContextReceiveStream[T]])
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `tuple[ContextSendStream[T], ContextReceiveStream[T]]`

Create context-aware memory object streams.

Supports ``create_context_streams[T](n)`` bracket syntax,
matching anyio's ``create_memory_object_stream`` API style.


