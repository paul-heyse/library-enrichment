# `mcp.client._transport`

Distribution: `mcp`

## TransportStreams

Import as `mcp.client.stdio.TransportStreams`  ·  defined at `mcp.client._transport.TransportStreams`

```python
TransportStreams = tuple[ReadStream[SessionMessage | Exception], WriteStream[SessionMessage]]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## __all__

`mcp.client._transport.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ReadStream', 'WriteStream', 'Transport', 'TransportStreams']
```

## Transport

Import as `mcp.client.Transport`  ·  defined at `mcp.client._transport.Transport`

```python
class Transport(AbstractAsyncContextManager[TransportStreams], Protocol)
```

**Also exported as** `mcp.client.Transport`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `AbstractAsyncContextManager[TransportStreams]`, `Protocol`

Protocol for MCP transports.

A transport is an async context manager that yields read and write streams
for bidirectional communication with an MCP server.


