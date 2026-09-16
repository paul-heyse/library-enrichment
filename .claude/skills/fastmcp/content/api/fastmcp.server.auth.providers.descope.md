# `fastmcp.server.auth.providers.descope`

Distribution: `fastmcp`

## _OAUTH_WK

`fastmcp.server.auth.providers.descope._OAUTH_WK`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OAUTH_WK = '/.well-known/oauth-authorization-server'
```

## _OPENID_WK

`fastmcp.server.auth.providers.descope._OPENID_WK`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OPENID_WK = '/.well-known/openid-configuration'
```

## logger

`fastmcp.server.auth.providers.descope.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DescopeProvider

`fastmcp.server.auth.providers.descope.DescopeProvider`

```python
class DescopeProvider(RemoteAuthProvider)
```

**Bases** `RemoteAuthProvider`

**Declared members (6)**

- `base_url = AnyHttpUrl(str(base_url).rstrip('/'))`  _instance-attribute_
- `descope_base_url = descope_base_url_str`  _instance-attribute_
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
- `oauth_authorization_server_metadata_url = self.openid_configuration_url.replace(_OPENID_WK, _OAUTH_WK)`  _instance-attribute_
- `openid_configuration_url = f'{issuer_url}{_OPENID_WK}'`  _instance-attribute_
- `project_id = project_id`  _instance-attribute_

**Inherited (13)**

- from `fastmcp.server.auth.auth.AuthProvider`: `challenge_scopes`, `get_middleware`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `set_mcp_path`
- from `fastmcp.server.auth.auth.RemoteAuthProvider`: `authorization_servers`, `get_challenge_scopes`, `resource_documentation`, `resource_name`, `scopes_supported`, `token_verifier`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Descope metadata provider for Dynamic Client Registration (DCR).

The provider accepts either a resource-specific Descope MCP Server URL such
as `/v1/apps/agentic/P.../M.../.well-known/openid-configuration` or a
project-level inbound app URL such as
`/v1/apps/P.../.well-known/openid-configuration`. The project-level URL is
the recommended configuration.

Tokens are accepted from any issuer the project mints: the project-level
issuer, the resource-scoped issuer of an MCP server, and the tenant-scoped
issuer used by cross-app access (XAA).

When neither `scopes_supported` nor `required_scopes` is provided, advertised
scopes are discovered lazily from the OpenID configuration. Use
`scopes_supported` and `required_scopes` together when the scopes clients
should request differ from the scopes enforced during token validation.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.providers.descope import DescopeProvider

    auth = DescopeProvider(
        config_url=(
            "https://api.descope.com/v1/apps/P.../"
            ".well-known/openid-configuration"
        ),
        base_url="https://your-fastmcp-server.com",
    )

    mcp = FastMCP("My App", auth=auth)
    ```

See [Descope's inbound app documentation](https://docs.descope.com/identity-federation/inbound-apps/creating-inbound-apps#method-2-dynamic-client-registration-dcr)
for DCR setup instructions.


## _DescopeJWTVerifier

`fastmcp.server.auth.providers.descope._DescopeJWTVerifier`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _DescopeJWTVerifier(JWTVerifier)
```

**Bases** `JWTVerifier`

**Declared members (2)**

- `agentic_issuer = f'{descope_base_url}/v1/apps/agentic/{project_id}'`  _instance-attribute_
- `project_issuer = f'{descope_base_url}/v1/apps/{project_id}'`  _instance-attribute_

**Inherited (19)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`
- from `fastmcp.server.auth.providers.jwt.JWTVerifier`: `algorithm`, `audience`, `issuer`, `jwks_uri`, `load_access_token`, `logger`, `public_key`, `ssrf_safe`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

JWT verifier that accepts every issuer form of a Descope project.

A project mints tokens under several issuers, and which one a token carries
depends on how it was minted rather than on the configuration URL this
server was given:

- project: `.../v1/apps/<project_id>`
- MCP server: `.../v1/apps/agentic/<project_id>/<server_id>`
- tenant (XAA): `.../v1/apps/<project_id>/<tenant_id>`

All of them are signed with the same project keys and carry the project ID
as their audience, so which one a token names does not narrow what it may
access. Any issuer scoped to the configured project is therefore accepted.


## _discover_scopes

`fastmcp.server.auth.providers.descope._discover_scopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _discover_scopes(openid_configuration_url: str) -> list[str] | None
```

## _parse_descope_config_url

`fastmcp.server.auth.providers.descope._parse_descope_config_url`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_descope_config_url(config_url: str) -> tuple[str, str, str, str]
```

