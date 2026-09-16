# `mcp.server.auth.routes`

Distribution: `mcp`

## AUTHORIZATION_PATH

`mcp.server.auth.routes.AUTHORIZATION_PATH`

```python
AUTHORIZATION_PATH = '/authorize'
```

**Inferred type** (`ty`, not declared in the source): `Literal["/authorize"]`

## ID_JAG_GRANT_PROFILE

`mcp.server.auth.routes.ID_JAG_GRANT_PROFILE`

```python
ID_JAG_GRANT_PROFILE = 'urn:ietf:params:oauth:grant-profile:id-jag'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:grant-profile:id-jag"]`

## REGISTRATION_PATH

`mcp.server.auth.routes.REGISTRATION_PATH`

```python
REGISTRATION_PATH = '/register'
```

**Inferred type** (`ty`, not declared in the source): `Literal["/register"]`

## REVOCATION_PATH

`mcp.server.auth.routes.REVOCATION_PATH`

```python
REVOCATION_PATH = '/revoke'
```

**Inferred type** (`ty`, not declared in the source): `Literal["/revoke"]`

## TOKEN_PATH

`mcp.server.auth.routes.TOKEN_PATH`

```python
TOKEN_PATH = '/token'
```

**Inferred type** (`ty`, not declared in the source): `Literal["/token"]`

## _body_limited

`mcp.server.auth.routes._body_limited`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _body_limited(app: ASGIApp) -> ASGIApp
```

## _cors

`mcp.server.auth.routes._cors`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _cors(app: ASGIApp, allow_methods: list[str]) -> ASGIApp
```

## build_metadata

Import as `fastmcp.server.auth.auth.build_metadata`  ·  defined at `mcp.server.auth.routes.build_metadata`

```python
def build_metadata(issuer_url: AnyHttpUrl, service_documentation_url: AnyHttpUrl | None, client_registration_options: ClientRegistrationOptions, revocation_options: RevocationOptions, supports_identity_assertion: bool = False) -> OAuthMetadata
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## build_resource_metadata_url

Import as `mcp.server.lowlevel.server.build_resource_metadata_url`  ·  defined at `mcp.server.auth.routes.build_resource_metadata_url`

```python
def build_resource_metadata_url(resource_server_url: AnyHttpUrl) -> AnyHttpUrl
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build RFC 9728 compliant protected resource metadata URL.

Inserts /.well-known/oauth-protected-resource between host and resource path
as specified in RFC 9728 §3.1.

Args:
    resource_server_url: The resource server URL (e.g., https://example.com/mcp)

Returns:
    The metadata URL (e.g., https://example.com/.well-known/oauth-protected-resource/mcp)


## cors_middleware

Import as `fastmcp.server.auth.auth.cors_middleware`  ·  defined at `mcp.server.auth.routes.cors_middleware`

```python
def cors_middleware(handler: Callable[[Request], Response | Awaitable[Response]], allow_methods: list[str]) -> ASGIApp
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## create_auth_routes

Import as `mcp.server.lowlevel.server.create_auth_routes`  ·  defined at `mcp.server.auth.routes.create_auth_routes`

```python
def create_auth_routes(provider: OAuthAuthorizationServerProvider[Any, Any, Any], issuer_url: AnyHttpUrl, service_documentation_url: AnyHttpUrl | None = None, client_registration_options: ClientRegistrationOptions | None = None, revocation_options: RevocationOptions | None = None, identity_assertion_enabled: bool = False) -> list[Route]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## create_protected_resource_routes

Import as `mcp.server.lowlevel.server.create_protected_resource_routes`  ·  defined at `mcp.server.auth.routes.create_protected_resource_routes`

```python
def create_protected_resource_routes(resource_url: AnyHttpUrl, authorization_servers: list[AnyHttpUrl], scopes_supported: list[str] | None = None, resource_name: str | None = None, resource_documentation: AnyHttpUrl | None = None) -> list[Route]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create routes for OAuth 2.0 Protected Resource Metadata (RFC 9728).

Args:
    resource_url: The URL of this resource server
    authorization_servers: List of authorization servers that can issue tokens
    scopes_supported: Optional list of scopes supported by this resource
    resource_name: Optional human-readable name for this resource
    resource_documentation: Optional URL to documentation for this resource

Returns:
    List of Starlette routes for protected resource metadata


## validate_issuer_url

`mcp.server.auth.routes.validate_issuer_url`

```python
def validate_issuer_url(url: AnyHttpUrl)
```

**Inferred type** (`ty`, not declared in the source): `def validate_issuer_url(url: AnyHttpUrl) -> Unknown`

Validate that the issuer URL meets OAuth 2.0 requirements.

Args:
    url: The issuer URL to validate.

Raises:
    ValueError: If the issuer URL is invalid.


