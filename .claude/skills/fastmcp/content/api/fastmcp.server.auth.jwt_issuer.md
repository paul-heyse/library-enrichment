# `fastmcp.server.auth.jwt_issuer`

Distribution: `fastmcp`

## KDF_ITERATIONS

`fastmcp.server.auth.jwt_issuer.KDF_ITERATIONS`

```python
KDF_ITERATIONS = 1000000
```

**Inferred type** (`ty`, not declared in the source): `Literal[1000000]`

## KDF_ITERATIONS_TEST

`fastmcp.server.auth.jwt_issuer.KDF_ITERATIONS_TEST`

```python
KDF_ITERATIONS_TEST = 10
```

**Inferred type** (`ty`, not declared in the source): `Literal[10]`

## logger

`fastmcp.server.auth.jwt_issuer.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## JWTIssuer

Import as `fastmcp.server.auth.oauth_proxy.proxy.JWTIssuer`  ·  defined at `fastmcp.server.auth.jwt_issuer.JWTIssuer`

```python
class JWTIssuer
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `audience = audience`  _instance-attribute_
- `def issue_access_token(self, client_id: str, scopes: list[str], jti: str, expires_in: int = 3600, upstream_claims: dict[str, Any] | None = None, subject: str | None = None, extra_claims: dict[str, Any] | None = None) -> str`
  Issue a minimal FastMCP access token.
- `def issue_refresh_token(self, client_id: str, scopes: list[str], jti: str, expires_in: int, upstream_claims: dict[str, Any] | None = None) -> str`
  Issue a minimal FastMCP refresh token.
- `issuer = issuer`  _instance-attribute_
- `def verify_token(self, token: str, expected_token_use: str = 'access') -> dict[str, Any]`
  Verify and decode a FastMCP token.

Issues and validates FastMCP-signed JWT tokens using HS256.

This issuer creates JWT tokens for MCP clients with proper audience claims,
maintaining OAuth 2.0 token boundaries. Tokens are signed with HS256 using
a key derived from the upstream client secret.


## derive_jwt_key

Import as `fastmcp.server.auth.oauth_proxy.proxy.derive_jwt_key`  ·  defined at `fastmcp.server.auth.jwt_issuer.derive_jwt_key`

```python
def derive_jwt_key(high_entropy_material: str | None = None, low_entropy_material: str | None = None, salt: str) -> bytes
```

**Overloads** (the signature above is the runtime dispatcher):

- `def derive_jwt_key(high_entropy_material: str, salt: str) -> bytes`
- `def derive_jwt_key(low_entropy_material: str, salt: str) -> bytes`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Derive JWT signing key from a high-entropy or low-entropy key material and server salt.


