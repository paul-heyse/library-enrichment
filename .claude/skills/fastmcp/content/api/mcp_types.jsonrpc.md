# `mcp_types.jsonrpc`

Distribution: `mcp-types`

## CONNECTION_CLOSED

Import as `mcp_types.CONNECTION_CLOSED`  ·  defined at `mcp_types.jsonrpc.CONNECTION_CLOSED`

```python
CONNECTION_CLOSED = -32000
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32000]`

**Also exported as** `mcp_types.CONNECTION_CLOSED`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

SDK-only: the connection closed before a response arrived; never emitted on the wire.


## HEADER_MISMATCH

Import as `mcp_types.HEADER_MISMATCH`  ·  defined at `mcp_types.jsonrpc.HEADER_MISMATCH`

```python
HEADER_MISMATCH = -32020
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32020]`

**Also exported as** `mcp_types.HEADER_MISMATCH`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

HTTP headers do not match the request body, or required headers are missing/malformed (protocol 2026-07-28).


## INTERNAL_ERROR

Import as `mcp_types.INTERNAL_ERROR`  ·  defined at `mcp_types.jsonrpc.INTERNAL_ERROR`

```python
INTERNAL_ERROR = -32603
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32603]`

**Also exported as** `mcp_types.INTERNAL_ERROR`

_11 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Standard JSON-RPC: an internal error occurred on the receiver.

The SDK uses the generic `ErrorData` envelope; the schema's per-code wrapper types are not constructed.


## INVALID_PARAMS

Import as `mcp_types.INVALID_PARAMS`  ·  defined at `mcp_types.jsonrpc.INVALID_PARAMS`

```python
INVALID_PARAMS = -32602
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32602]`

**Also exported as** `mcp_types.INVALID_PARAMS`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Standard JSON-RPC: invalid method parameters.


## INVALID_REQUEST

Import as `mcp_types.INVALID_REQUEST`  ·  defined at `mcp_types.jsonrpc.INVALID_REQUEST`

```python
INVALID_REQUEST = -32600
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32600]`

**Also exported as** `mcp_types.INVALID_REQUEST`

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Standard JSON-RPC: the message is not a valid request object.


## JSONRPCMessage

Import as `mcp_types.JSONRPCMessage`  ·  defined at `mcp_types.jsonrpc.JSONRPCMessage`

```python
JSONRPCMessage = JSONRPCRequest | JSONRPCNotification | JSONRPCResponse | JSONRPCError
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'JSONRPCRequest | JSONRPCNotification | JSONRPCResponse | JSONRPCError'> ``` --- Any JSON-RPC envelope that can be decoded off the wire or encoded to be sent.`

**Also exported as** `mcp_types.JSONRPCMessage`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Any JSON-RPC envelope that can be decoded off the wire or encoded to be sent.


## JSONRPC_VERSION

Import as `mcp_types.JSONRPC_VERSION`  ·  defined at `mcp_types.jsonrpc.JSONRPC_VERSION`

```python
JSONRPC_VERSION: Final[Literal['2.0']] = '2.0'
```

**Also exported as** `mcp_types.JSONRPC_VERSION`

The JSON-RPC version string carried by every MCP message envelope.


## METHOD_NOT_FOUND

Import as `mcp_types.METHOD_NOT_FOUND`  ·  defined at `mcp_types.jsonrpc.METHOD_NOT_FOUND`

```python
METHOD_NOT_FOUND = -32601
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32601]`

**Also exported as** `mcp_types.METHOD_NOT_FOUND`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Standard JSON-RPC: the requested method does not exist or is not available.


## MISSING_REQUIRED_CLIENT_CAPABILITY

Import as `mcp_types.MISSING_REQUIRED_CLIENT_CAPABILITY`  ·  defined at `mcp_types.jsonrpc.MISSING_REQUIRED_CLIENT_CAPABILITY`

```python
MISSING_REQUIRED_CLIENT_CAPABILITY = -32021
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32021]`

**Also exported as** `mcp_types.MISSING_REQUIRED_CLIENT_CAPABILITY`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The server requires a client capability the request did not declare (protocol 2026-07-28).


## PARSE_ERROR

Import as `mcp_types.PARSE_ERROR`  ·  defined at `mcp_types.jsonrpc.PARSE_ERROR`

```python
PARSE_ERROR = -32700
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32700]`

**Also exported as** `mcp_types.PARSE_ERROR`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Standard JSON-RPC: invalid JSON was received.


## REQUEST_TIMEOUT

Import as `mcp_types.REQUEST_TIMEOUT`  ·  defined at `mcp_types.jsonrpc.REQUEST_TIMEOUT`

```python
REQUEST_TIMEOUT = -32001
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32001]`

**Also exported as** `mcp_types.REQUEST_TIMEOUT`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

SDK-only: a request timed out waiting for its response.


## RequestId

Import as `mcp_types.RequestId`  ·  defined at `mcp_types.jsonrpc.RequestId`

```python
RequestId = Annotated[int, Field(strict=True)] | str
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'int | str'> ``` --- The ID of a JSON-RPC request.`

**Also exported as** `mcp_types.RequestId`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The ID of a JSON-RPC request.


## UNSUPPORTED_PROTOCOL_VERSION

Import as `mcp_types.UNSUPPORTED_PROTOCOL_VERSION`  ·  defined at `mcp_types.jsonrpc.UNSUPPORTED_PROTOCOL_VERSION`

```python
UNSUPPORTED_PROTOCOL_VERSION = -32022
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32022]`

**Also exported as** `mcp_types.UNSUPPORTED_PROTOCOL_VERSION`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The request's protocol version is not supported by the server (protocol 2026-07-28).


## URL_ELICITATION_REQUIRED

Import as `mcp_types.URL_ELICITATION_REQUIRED`  ·  defined at `mcp_types.jsonrpc.URL_ELICITATION_REQUIRED`

```python
URL_ELICITATION_REQUIRED = -32042
```

**Inferred type** (`ty`, not declared in the source): `Literal[-32042]`

**Also exported as** `mcp_types.URL_ELICITATION_REQUIRED`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A URL-mode elicitation is required before the request can be processed (protocol 2025-11-25 only).


## __all__

`mcp_types.jsonrpc.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['CONNECTION_CLOSED', 'HEADER_MISMATCH', 'INTERNAL_ERROR', 'INVALID_PARAMS', 'INVALID_REQUEST', 'JSONRPC_VERSION', 'METHOD_NOT_FOUND', 'MISSING_REQUIRED_CLIENT_CAPABILITY', 'PARSE_ERROR', 'REQUEST_TIMEOUT', 'UNSUPPORTED_PROTOCOL_VERSION', 'URL_ELICITATION_REQUIRED', 'ErrorData', 'JSONRPCError', 'JSONRPCMessage', 'JSONRPCNotification', 'JSONRPCRequest', 'JSONRPCResponse', 'RequestId', 'jsonrpc_message_adapter']
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## jsonrpc_message_adapter

Import as `mcp_types.jsonrpc_message_adapter`  ·  defined at `mcp_types.jsonrpc.jsonrpc_message_adapter`

```python
jsonrpc_message_adapter: TypeAdapter[JSONRPCMessage] = TypeAdapter(JSONRPCMessage)
```

**Also exported as** `mcp_types.jsonrpc_message_adapter`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ErrorData

Import as `mcp_types.ErrorData`  ·  defined at `mcp_types.jsonrpc.ErrorData`

```python
class ErrorData(BaseModel)
```

**Also exported as** `mcp.ErrorData`, `mcp_types.ErrorData`

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `code: int`  _instance-attribute_
  The error type that occurred.
- `data: Any = None`  _class-attribute, instance-attribute_
  Additional information about the error.
- `message: str`  _instance-attribute_
  A short description of the error.

Error information for JSON-RPC error responses.


## JSONRPCError

Import as `mcp_types.JSONRPCError`  ·  defined at `mcp_types.jsonrpc.JSONRPCError`

```python
class JSONRPCError(BaseModel)
```

**Also exported as** `mcp.JSONRPCError`, `mcp_types.JSONRPCError`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `error: ErrorData`  _instance-attribute_
- `id: RequestId | None`  _instance-attribute_
  The id of the request this error responds to.
- `jsonrpc: Literal['2.0']`  _instance-attribute_

A response to a request that indicates an error occurred.


## JSONRPCNotification

Import as `mcp_types.JSONRPCNotification`  ·  defined at `mcp_types.jsonrpc.JSONRPCNotification`

```python
class JSONRPCNotification(BaseModel)
```

**Also exported as** `mcp_types.JSONRPCNotification`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

A JSON-RPC notification which does not expect a response.


## JSONRPCRequest

Import as `mcp_types.JSONRPCRequest`  ·  defined at `mcp_types.jsonrpc.JSONRPCRequest`

```python
class JSONRPCRequest(BaseModel)
```

**Also exported as** `mcp.JSONRPCRequest`, `mcp_types.JSONRPCRequest`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (4)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `method: str`  _instance-attribute_
- `params: dict[str, Any] | None = None`  _class-attribute, instance-attribute_

A JSON-RPC request that expects a response.


## JSONRPCResponse

Import as `mcp_types.JSONRPCResponse`  ·  defined at `mcp_types.jsonrpc.JSONRPCResponse`

```python
class JSONRPCResponse(BaseModel)
```

**Also exported as** `mcp.JSONRPCResponse`, `mcp_types.JSONRPCResponse`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `id: RequestId`  _instance-attribute_
- `jsonrpc: Literal['2.0']`  _instance-attribute_
- `result: dict[str, Any]`  _instance-attribute_

A successful (non-error) response to a request.

Named `JSONRPCResultResponse` in the 2025-11-25+ schemas; the SDK keeps the original name.


