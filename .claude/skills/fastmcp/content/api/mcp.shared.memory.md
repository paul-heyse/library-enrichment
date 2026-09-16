# `mcp.shared.memory`

Distribution: `mcp`

## MessageStream

`mcp.shared.memory.MessageStream`

```python
MessageStream = tuple[ContextReceiveStream[SessionMessage | Exception], ContextSendStream[SessionMessage | Exception]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'tuple[ContextReceiveStream[SessionMessage | Exception], ContextSendStream[SessionMessage | Exception]]'> ````

## create_client_server_memory_streams

Import as `fastmcp.client.transports.memory.create_client_server_memory_streams`  ·  defined at `mcp.shared.memory.create_client_server_memory_streams`

```python
async def create_client_server_memory_streams() -> AsyncGenerator[tuple[MessageStream, MessageStream], None]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Creates a pair of bidirectional memory streams for client-server communication.

Yields:
    A tuple of (client_streams, server_streams) where each is a tuple of
    (read_stream, write_stream)


