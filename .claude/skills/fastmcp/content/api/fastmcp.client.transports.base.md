# `fastmcp.client.transports.base`

Distribution: `fastmcp`

## ClientTransportT

Import as `fastmcp.client.client.ClientTransportT`  ·  defined at `fastmcp.client.transports.base.ClientTransportT`

```python
ClientTransportT = TypeVar('ClientTransportT', bound='ClientTransport')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## SessionKwargs

Import as `fastmcp.client.client.SessionKwargs`  ·  defined at `fastmcp.client.transports.base.SessionKwargs`

```python
SessionKwargs = ClientSessionKwargs
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'ClientSessionKwargs'> ````

**Also exported as** `fastmcp.client.client.SessionKwargs`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ClientSessionKwargs

Import as `fastmcp.client.transports.base.SessionKwargs`  ·  defined at `fastmcp.client.transports.base.ClientSessionKwargs`

```python
class ClientSessionKwargs(TypedDict)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TypedDict`

**Declared members (12)**

- `client_info: mcp_types.Implementation | None`  _instance-attribute_
- `elicitation_callback: ElicitationFnT | None`  _instance-attribute_
- `extensions: dict[str, dict[str, Any]] | None`  _instance-attribute_
- `list_roots_callback: ListRootsFnT | None`  _instance-attribute_
- `log_level: mcp_types.LoggingLevel | None`  _instance-attribute_
- `logging_callback: LoggingFnT | None`  _instance-attribute_
- `message_handler: MessageHandlerFnT | None`  _instance-attribute_
- `notification_bindings: Sequence[NotificationBinding[Any]] | None`  _instance-attribute_
- `read_timeout_seconds: float | None`  _instance-attribute_
- `result_claims: Mapping[str, Sequence[ResultClaim[Any]]] | None`  _instance-attribute_
- `sampling_callback: SamplingFnT | None`  _instance-attribute_
- `sampling_capabilities: mcp_types.SamplingCapability | None`  _instance-attribute_

Keyword arguments for the MCP ClientSession constructor.


## ClientTransport

Import as `fastmcp.client.ClientTransport`  ·  defined at `fastmcp.client.transports.base.ClientTransport`

```python
class ClientTransport(abc.ABC)
```

**Also exported as** `fastmcp.client.ClientTransport`, `fastmcp.client.transports.ClientTransport`

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `abc.ABC`

**Declared members (4)**

- `async def close(self)`  _async_
  Close the transport.
- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _abstractmethod, async_
  Establishes a connection and yields an active ClientSession.
- `def get_session_id(self) -> str | None`
  Get the session ID for this transport, if available.
- `legacy_only: bool = False`  _class-attribute, instance-attribute_

Abstract base class for different MCP client transport mechanisms.

A Transport is responsible for establishing and managing connections
to an MCP server, and providing a ClientSession within an async context.


## TransportOptions

Import as `fastmcp.client.client.TransportOptions`  ·  defined at `fastmcp.client.transports.base.TransportOptions`

```python
class TransportOptions
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `backend_mode: str | None = None`  _class-attribute, instance-attribute_
- `forward_incoming_headers: bool = False`  _class-attribute, instance-attribute_
- `session_class: type[ClientSession] = ClientSession`  _class-attribute, instance-attribute_

How one client wants its connection built.

These belong to the client rather than to the transport, so a transport
shared between clients doesn't leak one client's settings to another.
Different client layers may own different fields; a layer that adds its
settings must preserve the existing options rather than replace the bundle.

Attributes:
    session_class: The ClientSession class to instantiate. Proxies supply a
        session that skips output-schema validation, since they relay
        results rather than consume them.
    forward_incoming_headers: Whether to forward eligible inbound HTTP
        headers upstream, including authorization. Hop-specific HTTP headers
        and MCP transport, routing, and event-stream state are excluded
        because each backend connection owns that state. Only appropriate
        for proxies; honored by the HTTP and SSE transports and ignored by
        the others.
    backend_mode: The connect `mode` to give backend clients that a wrapping
        transport builds on this client's behalf, so a chain of connections
        speaks one protocol era end to end. `None` leaves each backend
        client at its own default. Honored by `MCPConfigTransport`, whose
        multi-server form mounts a proxy per configured server and resolves
        one shared era for the aggregate; ignored by transports that connect
        to a single backend directly, since those carry the connecting
        client's own session and era.


