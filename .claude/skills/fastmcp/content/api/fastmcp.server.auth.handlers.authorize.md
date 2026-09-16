# `fastmcp.server.auth.handlers.authorize`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.handlers.authorize.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthorizationHandler

Import as `fastmcp.server.auth.oauth_proxy.proxy.AuthorizationHandler`  ·  defined at `fastmcp.server.auth.handlers.authorize.AuthorizationHandler`

```python
class AuthorizationHandler(SDKAuthorizationHandler)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `SDKAuthorizationHandler`

**Declared members (1)**

- `async def handle(self, request: Request) -> Response`  _async_
  Handle authorization request with enhanced error responses.

Authorization handler with enhanced error responses for unregistered clients.

This handler extends the MCP SDK's AuthorizationHandler to provide better UX
when clients attempt to authorize without being registered. It implements
content negotiation to return:

- HTML error pages for browser requests
- Enhanced JSON with registration hints for API clients
- Link headers pointing to registration endpoints

This maintains OAuth 2.1 compliance (returns 400 for invalid client_id)
while providing actionable guidance to fix the error.


## create_unregistered_client_html

`fastmcp.server.auth.handlers.authorize.create_unregistered_client_html`

```python
def create_unregistered_client_html(client_id: str, registration_endpoint: str, discovery_endpoint: str, server_name: str | None = None, server_icon_url: str | None = None, title: str = 'Client Not Registered') -> str
```

Create styled HTML error page for unregistered client attempts.

Args:
    client_id: The unregistered client ID that was provided
    registration_endpoint: URL of the registration endpoint
    discovery_endpoint: URL of the OAuth metadata discovery endpoint
    server_name: Optional server name for branding
    server_icon_url: Optional server icon URL
    title: Page title

Returns:
    HTML string for the error page


