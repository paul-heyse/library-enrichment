# `mcp.server.transport_security`

Distribution: `mcp`

## DEFAULT_MAX_REQUEST_BODY_SIZE

Import as `mcp.server.sse.DEFAULT_MAX_REQUEST_BODY_SIZE`  ·  defined at `mcp.server.transport_security.DEFAULT_MAX_REQUEST_BODY_SIZE`

```python
DEFAULT_MAX_REQUEST_BODY_SIZE: Final = 4 * 1024 * 1024
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Default maximum HTTP request body size in bytes (4 MiB).


## logger

`mcp.server.transport_security.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## RequestBodyLimitMiddleware

Import as `mcp.server.sse.RequestBodyLimitMiddleware`  ·  defined at `mcp.server.transport_security.RequestBodyLimitMiddleware`

```python
class RequestBodyLimitMiddleware
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `app = app`  _instance-attribute_
- `max_body_size = max_body_size`  _instance-attribute_

Reject oversized HTTP request bodies before invoking an ASGI application.


## TransportSecurityMiddleware

Import as `mcp.server.sse.TransportSecurityMiddleware`  ·  defined at `mcp.server.transport_security.TransportSecurityMiddleware`

```python
class TransportSecurityMiddleware
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `settings = settings or TransportSecuritySettings(enable_dns_rebinding_protection=False)`  _instance-attribute_
- `async def validate_request(self, request: Request, is_post: bool = False) -> Response | None`  _async_
  Validate request headers for DNS rebinding protection.

Middleware to enforce DNS rebinding protection for MCP transport endpoints.


## TransportSecuritySettings

Import as `mcp.server.sse.TransportSecuritySettings`  ·  defined at `mcp.server.transport_security.TransportSecuritySettings`

```python
class TransportSecuritySettings(BaseModel)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `allowed_hosts: list[str] = Field(default_factory=list)`  _class-attribute, instance-attribute_
  List of allowed Host header values.
- `allowed_origins: list[str] = Field(default_factory=list)`  _class-attribute, instance-attribute_
  List of allowed Origin header values.
- `enable_dns_rebinding_protection: bool = True`  _class-attribute, instance-attribute_
  Enable DNS rebinding protection (recommended for production).

Settings for MCP transport security features.

These settings help protect against DNS rebinding attacks by validating incoming request headers.


