# `fastmcp.client.transports.sse`

Distribution: `fastmcp`

## SSETransport

Import as `fastmcp.client.SSETransport`  ·  defined at `fastmcp.client.transports.sse.SSETransport`

```python
class SSETransport(ClientTransport)
```

**Also exported as** `fastmcp.client.SSETransport`, `fastmcp.client.transports.SSETransport`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClientTransport`

**Declared members (7)**

- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _async_
- `headers = headers or {}`  _instance-attribute_
- `httpx_client_factory = httpx_client_factory`  _instance-attribute_
- `legacy_only = True`  _class-attribute, instance-attribute_
- `sse_read_timeout = normalize_timeout_to_timedelta(sse_read_timeout)`  _instance-attribute_
- `url: str = url`  _instance-attribute_
- `verify: ssl.SSLContext | bool | str | None = verify`  _instance-attribute_

**Inherited (2)**

- from `fastmcp.client.transports.base.ClientTransport`: `close`, `get_session_id`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport implementation that connects to an MCP server via Server-Sent Events.


