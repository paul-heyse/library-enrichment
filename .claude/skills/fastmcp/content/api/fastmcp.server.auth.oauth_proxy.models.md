# `fastmcp.server.auth.oauth_proxy.models`

Distribution: `fastmcp`

## DEFAULT_ACCESS_TOKEN_EXPIRY_NO_REFRESH_SECONDS

Import as `fastmcp.server.auth.oauth_proxy.proxy.DEFAULT_ACCESS_TOKEN_EXPIRY_NO_REFRESH_SECONDS`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.DEFAULT_ACCESS_TOKEN_EXPIRY_NO_REFRESH_SECONDS`

```python
DEFAULT_ACCESS_TOKEN_EXPIRY_NO_REFRESH_SECONDS: Final[int] = 60 * 60 * 24 * 365
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS

Import as `fastmcp.server.auth.oauth_proxy.proxy.DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS`

```python
DEFAULT_ACCESS_TOKEN_EXPIRY_SECONDS: Final[int] = 60 * 60
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DEFAULT_AUTH_CODE_EXPIRY_SECONDS

Import as `fastmcp.server.auth.oauth_proxy.proxy.DEFAULT_AUTH_CODE_EXPIRY_SECONDS`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.DEFAULT_AUTH_CODE_EXPIRY_SECONDS`

```python
DEFAULT_AUTH_CODE_EXPIRY_SECONDS: Final[int] = 5 * 60
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS

Import as `fastmcp.server.auth.oauth_proxy.proxy.DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS`

```python
DEFAULT_REFRESH_TOKEN_EXPIRY_SECONDS: Final[int] = 60 * 60 * 24 * 365
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## HTTP_TIMEOUT_SECONDS

Import as `fastmcp.server.auth.oauth_proxy.proxy.HTTP_TIMEOUT_SECONDS`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.HTTP_TIMEOUT_SECONDS`

```python
HTTP_TIMEOUT_SECONDS: Final[int] = 30
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ClientCode

Import as `fastmcp.server.auth.oauth_proxy.proxy.ClientCode`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.ClientCode`

```python
class ClientCode(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (9)**

- `client_id: str`  _instance-attribute_
- `code: str`  _instance-attribute_
- `code_challenge: str | None`  _instance-attribute_
- `code_challenge_method: str`  _instance-attribute_
- `created_at: float`  _instance-attribute_
- `expires_at: float`  _instance-attribute_
- `idp_tokens: dict[str, Any]`  _instance-attribute_
- `redirect_uri: str`  _instance-attribute_
- `scopes: list[str]`  _instance-attribute_

Client authorization code with PKCE and upstream tokens.

Stored server-side after upstream IdP callback. Contains the upstream
tokens bound to the client's PKCE challenge for secure token exchange.


## ConsentCSRFToken

Import as `fastmcp.server.auth.oauth_proxy.proxy.ConsentCSRFToken`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.ConsentCSRFToken`

```python
class ConsentCSRFToken(BaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (2)**

- `expires_at: float`  _instance-attribute_
- `txn_id: str`  _instance-attribute_

One CSRF token issued for one render of the consent page.

Stored under a key derived from the token itself rather than on the
transaction. Every render of a consent page issues its own token, and two
renders can be in flight at once (a reload, a browser preload, an extension
re-fetching the URL). Appending to a list on the transaction loses one of
them whenever that happens: `AsyncKeyValue` has no compare-and-swap, so two
handlers read the same transaction, each append their own token, and the
second write drops the first. Giving each token its own key makes the
writes independent, which holds across processes sharing one backend.


## JTIMapping

Import as `fastmcp.server.auth.oauth_proxy.proxy.JTIMapping`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.JTIMapping`

```python
class JTIMapping(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `created_at: float`  _instance-attribute_
- `jti: str`  _instance-attribute_
- `upstream_token_id: str`  _instance-attribute_

Maps FastMCP token JTI to upstream token ID.

This allows stateless JWT validation while still being able to look up
the corresponding upstream token when tools need to access upstream APIs.


## OAuthTransaction

Import as `fastmcp.server.auth.oauth_proxy.proxy.OAuthTransaction`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.OAuthTransaction`

```python
class OAuthTransaction(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (13)**

- `client_id: str`  _instance-attribute_
- `client_redirect_uri: str`  _instance-attribute_
- `client_state: str`  _instance-attribute_
- `code_challenge: str | None`  _instance-attribute_
- `code_challenge_method: str`  _instance-attribute_
- `consent_token: str | None = None`  _class-attribute, instance-attribute_
- `created_at: float`  _instance-attribute_
- `csrf_expires_at: float | None = None`  _class-attribute, instance-attribute_
- `csrf_token: str | None = None`  _class-attribute, instance-attribute_
- `proxy_code_verifier: str | None = None`  _class-attribute, instance-attribute_
- `resource: str | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str]`  _instance-attribute_
- `txn_id: str`  _instance-attribute_

OAuth transaction state for consent flow.

Stored server-side to track active authorization flows with client context.
Includes CSRF tokens for consent protection per MCP security best practices.


## ProxyDCRClient

Import as `fastmcp.server.auth.oauth_proxy.proxy.ProxyDCRClient`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.ProxyDCRClient`

```python
class ProxyDCRClient(OAuthClientInformationFull)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthClientInformationFull`

**Declared members (6)**

- `allow_unregistered_redirect_uris: bool = Field(default=False, exclude=True)`  _class-attribute, instance-attribute_
- `allowed_redirect_uri_patterns: list[str] | None = Field(default=None)`  _class-attribute, instance-attribute_
- `cimd_document: CIMDDocument | None = Field(default=None)`  _class-attribute, instance-attribute_
- `cimd_fetched_at: float | None = Field(default=None)`  _class-attribute, instance-attribute_
- `client_name: str | None = Field(default=None)`  _class-attribute, instance-attribute_
- `def validate_redirect_uri(self, redirect_uri: AnyUrl | None) -> AnyUrl`
  Validate redirect URI against proxy patterns and optionally CIMD redirect_uris.

Client for DCR proxy with configurable redirect URI validation.

This special client class is critical for the OAuth proxy to work correctly
with Dynamic Client Registration (DCR). Here's why it exists:

Problem:
--------
When MCP clients use OAuth, they dynamically register with random localhost
ports (e.g., http://localhost:55454/callback). The OAuth proxy needs to:
1. Accept these dynamic redirect URIs from clients based on configured patterns
2. Use its own fixed redirect URI with the upstream provider (Google, GitHub, etc.)
3. Forward the authorization code back to the client's dynamic URI

Solution:
---------
This class validates redirect URIs against configurable patterns,
while the proxy internally uses its own fixed redirect URI with the upstream
provider. This allows the flow to work even when clients reconnect with
different ports or when tokens are cached.

Without proper validation, clients could get "Redirect URI not registered" errors
when trying to authenticate with cached tokens, or security vulnerabilities could
arise from accepting arbitrary redirect URIs.


## RefreshTokenMetadata

Import as `fastmcp.server.auth.oauth_proxy.proxy.RefreshTokenMetadata`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.RefreshTokenMetadata`

```python
class RefreshTokenMetadata(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (4)**

- `client_id: str`  _instance-attribute_
- `created_at: float`  _instance-attribute_
- `expires_at: int | None = None`  _class-attribute, instance-attribute_
- `scopes: list[str]`  _instance-attribute_

Metadata for a refresh token, stored keyed by token hash.

We store only metadata (not the token itself) for security - if storage
is compromised, attackers get hashes they can't reverse into usable tokens.


## UpstreamTokenSet

Import as `fastmcp.server.auth.oidc_proxy.UpstreamTokenSet`  ·  defined at `fastmcp.server.auth.oauth_proxy.models.UpstreamTokenSet`

```python
class UpstreamTokenSet(BaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (10)**

- `access_token: str`  _instance-attribute_
- `client_id: str`  _instance-attribute_
- `created_at: float`  _instance-attribute_
- `expires_at: float`  _instance-attribute_
- `raw_token_data: dict[str, Any] = Field(default_factory=dict)`  _class-attribute, instance-attribute_
- `refresh_token: str | None`  _instance-attribute_
- `refresh_token_expires_at: float | None`  _instance-attribute_
- `scope: str`  _instance-attribute_
- `token_type: str`  _instance-attribute_
- `upstream_token_id: str`  _instance-attribute_

Stored upstream OAuth tokens from identity provider.

These tokens are obtained from the upstream provider (Google, GitHub, etc.)
and stored in plaintext within this model. Encryption is handled transparently
at the storage layer via FernetEncryptionWrapper. Tokens are never exposed to MCP clients.


## _hash_token

`fastmcp.server.auth.oauth_proxy.models._hash_token`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _hash_token(token: str) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Hash a token for secure storage lookup.

Uses SHA-256 to create a one-way hash. The original token cannot be
recovered from the hash, providing defense in depth if storage is compromised.


## _matches_registered_loopback_redirect_uri

`fastmcp.server.auth.oauth_proxy.models._matches_registered_loopback_redirect_uri`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _matches_registered_loopback_redirect_uri(redirect_uri: AnyUrl, registered_uri: AnyUrl) -> bool
```

## _matches_registered_redirect_uri

`fastmcp.server.auth.oauth_proxy.models._matches_registered_redirect_uri`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _matches_registered_redirect_uri(redirect_uri: AnyUrl, registered_uris: list[AnyUrl] | None) -> bool
```

## _redirect_uri_path

`fastmcp.server.auth.oauth_proxy.models._redirect_uri_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _redirect_uri_path(uri_path: str) -> str
```

