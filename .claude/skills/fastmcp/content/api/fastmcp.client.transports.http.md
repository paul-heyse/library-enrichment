# `fastmcp.client.transports.http`

Distribution: `fastmcp`

## StreamableHttpTransport

Import as `fastmcp.client.StreamableHttpTransport`  ·  defined at `fastmcp.client.transports.http.StreamableHttpTransport`

```python
class StreamableHttpTransport(ClientTransport)
```

**Also exported as** `fastmcp.client.StreamableHttpTransport`, `fastmcp.client.transports.StreamableHttpTransport`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ClientTransport`

**Declared members (7)**

- `async def close(self)`  _async_
- `async def connect_session(self, transport_options: TransportOptions | None = None, session_kwargs: Unpack[SessionKwargs] = {}) -> AsyncIterator[ClientSession]`  _async_
- `def get_session_id(self) -> str | None`
- `headers = headers or {}`  _instance-attribute_
- `httpx_client_factory = httpx_client_factory`  _instance-attribute_
- `url: str = url`  _instance-attribute_
- `verify: ssl.SSLContext | bool | str | None = verify`  _instance-attribute_

**Inherited (1)**

- from `fastmcp.client.transports.base.ClientTransport`: `legacy_only`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Transport implementation that connects to an MCP server via Streamable HTTP Requests.


