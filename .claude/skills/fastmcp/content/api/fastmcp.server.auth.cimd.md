# `fastmcp.server.auth.cimd`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.cimd.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## CIMDAssertionValidator

`fastmcp.server.auth.cimd.CIMDAssertionValidator`

```python
class CIMDAssertionValidator
```

**Declared members (3)**

- `MAX_ASSERTION_LIFETIME = 300`  _class-attribute, instance-attribute_
- `logger = get_logger(__name__)`  _instance-attribute_
- `async def validate_assertion(self, assertion: str, client_id: str, token_endpoint: str, cimd_doc: CIMDDocument) -> bool`  _async_
  Validate JWT assertion from client.

Validates JWT assertions for private_key_jwt CIMD clients.

Implements RFC 7523 (JSON Web Token (JWT) Profile for OAuth 2.0 Client
Authentication and Authorization Grants) for CIMD client authentication.

JTI replay protection uses TTL-based caching to ensure proper security:
- JTIs are cached with expiration matching the JWT's exp claim
- Expired JTIs are automatically cleaned up
- Maximum assertion lifetime is enforced (5 minutes)


## CIMDClientManager

Import as `fastmcp.server.auth.auth.CIMDClientManager`  ·  defined at `fastmcp.server.auth.cimd.CIMDClientManager`

```python
class CIMDClientManager
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `allowed_redirect_uri_patterns = allowed_redirect_uri_patterns`  _instance-attribute_
- `default_scope = default_scope`  _instance-attribute_
- `enabled = enable_cimd`  _instance-attribute_
- `async def get_client(self, client_id_url: str)`  _async_
  Fetch CIMD document and create synthetic OAuth client.
- `def is_cimd_client_id(self, client_id: str) -> bool`
  Check if client_id is a CIMD URL.
- `logger = get_logger(__name__)`  _instance-attribute_
- `async def validate_private_key_jwt(self, assertion: str, client, token_endpoint: str) -> bool`  _async_
  Validate JWT assertion for private_key_jwt auth.

Manages all CIMD client operations for OAuth proxy.

This class encapsulates:
- CIMD client detection
- Document fetching and validation
- Synthetic OAuth client creation
- Private key JWT assertion validation

This allows the OAuth proxy to delegate all CIMD-specific logic to a
single, focused manager class.


## CIMDDocument

Import as `fastmcp.server.auth.oauth_proxy.models.CIMDDocument`  ·  defined at `fastmcp.server.auth.cimd.CIMDDocument`

```python
class CIMDDocument(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (18)**

- `client_id: AnyHttpUrl = Field(..., description='Must match the URL where this document is hosted')`  _class-attribute, instance-attribute_
- `client_name: str | None = Field(default=None, description='Human-readable name of the client')`  _class-attribute, instance-attribute_
- `client_uri: AnyHttpUrl | None = Field(default=None, description="URL of the client's home page")`  _class-attribute, instance-attribute_
- `contacts: list[str] | None = Field(default=None, description='Contact information for the client developer')`  _class-attribute, instance-attribute_
- `grant_types: list[str] = Field(default_factory=lambda: ['authorization_code'], description='OAuth grant types the client will use')`  _class-attribute, instance-attribute_
- `jwks: dict[str, Any] | None = Field(default=None, description="Client's JSON Web Key Set (for private_key_jwt)")`  _class-attribute, instance-attribute_
- `jwks_uri: AnyHttpUrl | None = Field(default=None, description="URL of the client's JSON Web Key Set (for private_key_jwt)")`  _class-attribute, instance-attribute_
- `logo_uri: AnyHttpUrl | None = Field(default=None, description="URL of the client's logo image")`  _class-attribute, instance-attribute_
- `policy_uri: AnyHttpUrl | None = Field(default=None, description="URL of the client's privacy policy")`  _class-attribute, instance-attribute_
- `redirect_uris: list[str] = Field(..., description='Array of allowed redirect URIs (may include wildcards like http://localhost:*/callback)')`  _class-attribute, instance-attribute_
- `response_types: list[str] = Field(default_factory=lambda: ['code'], description='OAuth response types the client will use')`  _class-attribute, instance-attribute_
- `scope: str | None = Field(default=None, description='Space-separated list of scopes the client may request')`  _class-attribute, instance-attribute_
- `software_id: str | None = Field(default=None, description='Unique identifier for the client software')`  _class-attribute, instance-attribute_
- `software_version: str | None = Field(default=None, description='Version of the client software')`  _class-attribute, instance-attribute_
- `token_endpoint_auth_method: Literal['none', 'private_key_jwt'] = Field(default='none', description='Authentication method for token endpoint (no shared secrets allowed)')`  _class-attribute, instance-attribute_
- `tos_uri: AnyHttpUrl | None = Field(default=None, description="URL of the client's terms of service")`  _class-attribute, instance-attribute_
- `def validate_auth_method(cls, v: str) -> str`  _classmethod_
  Ensure no shared-secret auth methods are used.
- `def validate_redirect_uris(cls, v: list[str]) -> list[str]`  _classmethod_
  Ensure redirect_uris is non-empty and each entry is a valid URI.

CIMD document per draft-parecki-oauth-client-id-metadata-document.

The client metadata document is a JSON document containing OAuth client
metadata. The client_id property MUST match the URL where this document
is hosted.

Key constraint: token_endpoint_auth_method MUST NOT use shared secrets
(client_secret_post, client_secret_basic, client_secret_jwt).

redirect_uris is required and must contain at least one entry.


## CIMDFetchError

Import as `fastmcp.cli.cimd.CIMDFetchError`  ·  defined at `fastmcp.server.auth.cimd.CIMDFetchError`

```python
class CIMDFetchError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Raised when CIMD document fetching fails.


## CIMDFetcher

Import as `fastmcp.cli.cimd.CIMDFetcher`  ·  defined at `fastmcp.server.auth.cimd.CIMDFetcher`

```python
class CIMDFetcher
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `DEFAULT_CACHE_TTL_SECONDS = 3600`  _class-attribute, instance-attribute_
- `MAX_CACHE_SIZE = 1000`  _class-attribute, instance-attribute_
- `MAX_RESPONSE_SIZE = 5120`  _class-attribute, instance-attribute_
- `async def fetch(self, client_id_url: str) -> CIMDDocument`  _async_
  Fetch and validate a CIMD document with SSRF protection.
- `def is_cimd_client_id(self, client_id: str) -> bool`
  Check if a client_id looks like a CIMD URL.
- `timeout = timeout`  _instance-attribute_
- `def validate_redirect_uri(self, doc: CIMDDocument, redirect_uri: str) -> bool`
  Validate that a redirect_uri is allowed by the CIMD document.

Fetch and validate CIMD documents with SSRF protection.

Delegates HTTP fetching to ssrf_safe_fetch_response, which provides DNS
pinning, IP validation, size limits, and timeout enforcement. Documents are
cached using HTTP caching semantics (Cache-Control/ETag/Last-Modified), with
a TTL fallback when response headers do not define caching behavior.


## CIMDValidationError

Import as `fastmcp.cli.cimd.CIMDValidationError`  ·  defined at `fastmcp.server.auth.cimd.CIMDValidationError`

```python
class CIMDValidationError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Raised when CIMD document validation fails.


## _CIMDCacheEntry

`fastmcp.server.auth.cimd._CIMDCacheEntry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CIMDCacheEntry
```

**Declared members (6)**

- `doc: CIMDDocument`  _instance-attribute_
- `etag: str | None`  _instance-attribute_
- `expires_at: float`  _instance-attribute_
- `freshness_lifetime: float`  _instance-attribute_
- `last_modified: str | None`  _instance-attribute_
- `must_revalidate: bool`  _instance-attribute_

Cached CIMD document and associated HTTP cache metadata.


## _CIMDCachePolicy

`fastmcp.server.auth.cimd._CIMDCachePolicy`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CIMDCachePolicy
```

**Declared members (6)**

- `etag: str | None`  _instance-attribute_
- `expires_at: float`  _instance-attribute_
- `freshness_lifetime: float`  _instance-attribute_
- `last_modified: str | None`  _instance-attribute_
- `must_revalidate: bool`  _instance-attribute_
- `no_store: bool`  _instance-attribute_

Normalized cache directives parsed from HTTP response headers.


## _jwk_to_pem

`fastmcp.server.auth.cimd._jwk_to_pem`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _jwk_to_pem(key_data: dict[str, Any]) -> str
```

