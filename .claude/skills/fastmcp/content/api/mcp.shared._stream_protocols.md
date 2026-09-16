# `mcp.shared._stream_protocols`

Distribution: `mcp`

## T_co

`mcp.shared._stream_protocols.T_co`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T_co = TypeVar('T_co', covariant=True)
```

## T_contra

`mcp.shared._stream_protocols.T_contra`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T_contra = TypeVar('T_contra', contravariant=True)
```

## ReadStream

Import as `mcp.server.runner.ReadStream`  ·  defined at `mcp.shared._stream_protocols.ReadStream`

```python
class ReadStream(Protocol[T_co])
```

**Also exported as** `mcp.client._transport.ReadStream`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol[T_co]`

**Declared members (2)**

- `async def aclose(self) -> None`  _async_
- `async def receive(self) -> T_co`  _async_

Protocol for reading items from a stream.

Consumers that need the sender's context should use
``getattr(stream, 'last_context', None)``.


## WriteStream

Import as `mcp.server.runner.WriteStream`  ·  defined at `mcp.shared._stream_protocols.WriteStream`

```python
class WriteStream(Protocol[T_contra])
```

**Also exported as** `mcp.client._transport.WriteStream`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Protocol[T_contra]`

**Declared members (2)**

- `async def aclose(self) -> None`  _async_
- `async def send(self, item: T_contra) -> None`  _async_

Protocol for writing items to a stream.


