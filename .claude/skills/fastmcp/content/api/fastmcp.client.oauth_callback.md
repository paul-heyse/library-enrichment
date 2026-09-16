# `fastmcp.client.oauth_callback`

Distribution: `fastmcp`

## logger

`fastmcp.client.oauth_callback.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## port

`fastmcp.client.oauth_callback.port`

```python
port = find_available_port()
```

**Inferred type** (`ty`, not declared in the source): `int`

## server

`fastmcp.client.oauth_callback.server`

```python
server = create_oauth_callback_server(port=port, server_url='https://fastmcp-test-server.example.com')
```

**Inferred type** (`ty`, not declared in the source): `Server`

## CallbackResponse

`fastmcp.client.oauth_callback.CallbackResponse`

```python
class CallbackResponse
```

**Declared members (7)**

- `code: str | None = None`  _class-attribute, instance-attribute_
- `error: str | None = None`  _class-attribute, instance-attribute_
- `error_description: str | None = None`  _class-attribute, instance-attribute_
- `def from_dict(cls, data: dict[str, str]) -> CallbackResponse`  _classmethod_
- `iss: str | None = None`  _class-attribute, instance-attribute_
- `state: str | None = None`  _class-attribute, instance-attribute_
- `def to_dict(self) -> dict[str, str]`

## OAuthCallbackResult

Import as `fastmcp.client.auth.oauth.OAuthCallbackResult`  ·  defined at `fastmcp.client.oauth_callback.OAuthCallbackResult`

```python
class OAuthCallbackResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `code: str | None = None`  _class-attribute, instance-attribute_
- `error: Exception | None = None`  _class-attribute, instance-attribute_
- `iss: str | None = None`  _class-attribute, instance-attribute_
- `state: str | None = None`  _class-attribute, instance-attribute_

Container for OAuth callback results, used with anyio.Event for async coordination.


## create_callback_html

`fastmcp.client.oauth_callback.create_callback_html`

```python
def create_callback_html(message: str, is_success: bool = True, title: str = 'FastMCP OAuth', server_url: str | None = None) -> str
```

Create a styled HTML response for OAuth callbacks.


## create_oauth_callback_server

Import as `fastmcp.client.auth.oauth.create_oauth_callback_server`  ·  defined at `fastmcp.client.oauth_callback.create_oauth_callback_server`

```python
def create_oauth_callback_server(port: int, host: str = '127.0.0.1', callback_path: str = '/callback', server_url: str | None = None, result_container: OAuthCallbackResult | None = None, result_ready: anyio.Event | None = None) -> Server
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an OAuth callback server.

Args:
    port: The port to run the server on
    callback_path: The path to listen for OAuth redirects on
    server_url: Optional server URL to display in success messages
    result_container: Optional container to store callback results
    result_ready: Optional event to signal when callback is received

Returns:
    Configured uvicorn Server instance (not yet running)


