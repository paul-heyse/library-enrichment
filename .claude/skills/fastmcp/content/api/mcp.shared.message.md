# `mcp.shared.message`

Distribution: `mcp`

## CloseSSEStreamCallback

Import as `mcp.server.context.CloseSSEStreamCallback`  ·  defined at `mcp.shared.message.CloseSSEStreamCallback`

```python
CloseSSEStreamCallback = Callable[[], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '() -> Awaitable[None]'> ````

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## MessageMetadata

Import as `mcp.server.runner.MessageMetadata`  ·  defined at `mcp.shared.message.MessageMetadata`

```python
MessageMetadata = ClientMessageMetadata | ServerMessageMetadata | None
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'ClientMessageMetadata | ServerMessageMetadata | None'> ````

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ResumptionToken

`mcp.shared.message.ResumptionToken`

```python
ResumptionToken = str
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'str'> ````

## ResumptionTokenUpdateCallback

`mcp.shared.message.ResumptionTokenUpdateCallback`

```python
ResumptionTokenUpdateCallback = Callable[[ResumptionToken], Awaitable[None]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(str, /) -> Awaitable[None]'> ````

## ClientMessageMetadata

Import as `mcp.client.session.ClientMessageMetadata`  ·  defined at `mcp.shared.message.ClientMessageMetadata`

```python
class ClientMessageMetadata
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `headers: dict[str, str] | None = None`  _class-attribute, instance-attribute_
- `on_resumption_token_update: Callable[[ResumptionToken], Awaitable[None]] | None = None`  _class-attribute, instance-attribute_
- `resumption_token: ResumptionToken | None = None`  _class-attribute, instance-attribute_

Metadata specific to client messages.


## ServerMessageMetadata

Import as `mcp.server.sse.ServerMessageMetadata`  ·  defined at `mcp.shared.message.ServerMessageMetadata`

```python
class ServerMessageMetadata
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `can_send_request: bool = True`  _class-attribute, instance-attribute_
- `close_sse_stream: CloseSSEStreamCallback | None = None`  _class-attribute, instance-attribute_
- `close_standalone_sse_stream: CloseSSEStreamCallback | None = None`  _class-attribute, instance-attribute_
- `on_request_unanswered: Callable[[], Awaitable[None]] | None = None`  _class-attribute, instance-attribute_
- `related_request_id: RequestId | None = None`  _class-attribute, instance-attribute_
- `request_context: Any = None`  _class-attribute, instance-attribute_

Metadata specific to server messages.


## SessionMessage

Import as `mcp.client.sse.SessionMessage`  ·  defined at `mcp.shared.message.SessionMessage`

```python
class SessionMessage
```

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `message: JSONRPCMessage`  _instance-attribute_
- `metadata: MessageMetadata = None`  _class-attribute, instance-attribute_

A message with specific metadata for transport-specific features.


