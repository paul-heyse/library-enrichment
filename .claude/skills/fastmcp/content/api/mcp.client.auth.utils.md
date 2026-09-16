# `mcp.client.auth.utils`

Distribution: `mcp`

## build_oauth_authorization_server_metadata_discovery_urls

Import as `mcp.client.auth.oauth2.build_oauth_authorization_server_metadata_discovery_urls`  ·  defined at `mcp.client.auth.utils.build_oauth_authorization_server_metadata_discovery_urls`

```python
def build_oauth_authorization_server_metadata_discovery_urls(auth_server_url: str | None, server_url: str) -> list[str]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Generate an ordered list of URLs for authorization server metadata discovery.

Args:
    auth_server_url: OAuth Authorization Server Metadata URL if found, otherwise None
    server_url: URL for the MCP server, used as a fallback if auth_server_url is None


## build_protected_resource_metadata_discovery_urls

Import as `mcp.client.auth.oauth2.build_protected_resource_metadata_discovery_urls`  ·  defined at `mcp.client.auth.utils.build_protected_resource_metadata_discovery_urls`

```python
def build_protected_resource_metadata_discovery_urls(www_auth_url: str | None, server_url: str) -> list[str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build ordered list of URLs to try for protected resource metadata discovery.

Per SEP-985, the client MUST:
1. Try resource_metadata from WWW-Authenticate header (if present)
2. Fall back to path-based well-known URI: /.well-known/oauth-protected-resource/{path}
3. Fall back to root-based well-known URI: /.well-known/oauth-protected-resource

Args:
    www_auth_url: Optional resource_metadata URL extracted from the WWW-Authenticate header
    server_url: Server URL

Returns:
    Ordered list of URLs to try for discovery


## create_client_info_from_metadata_url

Import as `mcp.client.auth.oauth2.create_client_info_from_metadata_url`  ·  defined at `mcp.client.auth.utils.create_client_info_from_metadata_url`

```python
def create_client_info_from_metadata_url(client_metadata_url: str, redirect_uris: list[AnyUrl] | None = None) -> OAuthClientInformationFull
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create client information using a URL-based client ID (CIMD).

When using URL-based client IDs, the URL itself becomes the client_id
and no client_secret is used (token_endpoint_auth_method="none").

Args:
    client_metadata_url: The URL to use as the client_id
    redirect_uris: The redirect URIs from the client metadata, recorded on the client
        information alongside the client_id

Returns:
    OAuthClientInformationFull with the URL as client_id


## create_client_registration_request

Import as `mcp.client.auth.oauth2.create_client_registration_request`  ·  defined at `mcp.client.auth.utils.create_client_registration_request`

```python
def create_client_registration_request(auth_server_metadata: OAuthMetadata | None, client_metadata: OAuthClientMetadata, auth_base_url: str) -> Request
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build a client registration request.


## create_oauth_metadata_request

Import as `mcp.client.auth.oauth2.create_oauth_metadata_request`  ·  defined at `mcp.client.auth.utils.create_oauth_metadata_request`

```python
def create_oauth_metadata_request(url: str) -> Request
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## credentials_match_issuer

Import as `mcp.client.auth.oauth2.credentials_match_issuer`  ·  defined at `mcp.client.auth.utils.credentials_match_issuer`

```python
def credentials_match_issuer(client_info: OAuthClientInformationFull, issuer: str, client_metadata_url: str | None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Whether stored client credentials may be reused against `issuer` (SEP-2352).

A URL-based client ID (CIMD) is portable across authorization servers — the same self-hosted
document is resolved by whichever server is in use — so it always matches; CIMD is identified
by the client ID being the configured `client_metadata_url`, not by URL shape (a registration
server may also issue URL-shaped IDs that are bound to it). Credentials with a recorded issuer
match only when it equals `issuer` (simple string comparison; a root issuer with and without
its trailing slash count as equal). Credentials with no recorded
issuer (pre-registered, or stored before issuer binding existed) carry no binding to enforce
and are left as-is.


## extract_field_from_www_auth

Import as `mcp.client.auth.oauth2.extract_field_from_www_auth`  ·  defined at `mcp.client.auth.utils.extract_field_from_www_auth`

```python
def extract_field_from_www_auth(response: Response, field_name: str) -> str | None
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract field from WWW-Authenticate header.

Returns:
    Field value if found in WWW-Authenticate header, None otherwise


## extract_resource_metadata_from_www_auth

Import as `mcp.client.auth.oauth2.extract_resource_metadata_from_www_auth`  ·  defined at `mcp.client.auth.utils.extract_resource_metadata_from_www_auth`

```python
def extract_resource_metadata_from_www_auth(response: Response) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract protected resource metadata URL from WWW-Authenticate header as per RFC 9728.

Returns:
    Resource metadata URL if found in WWW-Authenticate header, None otherwise


## extract_scope_from_www_auth

Import as `mcp.client.auth.oauth2.extract_scope_from_www_auth`  ·  defined at `mcp.client.auth.utils.extract_scope_from_www_auth`

```python
def extract_scope_from_www_auth(response: Response) -> str | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Extract scope parameter from WWW-Authenticate header as per RFC 6750.

Returns:
    Scope string if found in WWW-Authenticate header, None otherwise


## get_client_metadata_scopes

Import as `mcp.client.auth.oauth2.get_client_metadata_scopes`  ·  defined at `mcp.client.auth.utils.get_client_metadata_scopes`

```python
def get_client_metadata_scopes(www_authenticate_scope: str | None, protected_resource_metadata: ProtectedResourceMetadata | None, authorization_server_metadata: OAuthMetadata | None = None, client_grant_types: list[str] | None = None) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Select effective scopes and augment for refresh token support.


## handle_auth_metadata_response

Import as `mcp.client.auth.oauth2.handle_auth_metadata_response`  ·  defined at `mcp.client.auth.utils.handle_auth_metadata_response`

```python
async def handle_auth_metadata_response(response: Response) -> tuple[bool, OAuthMetadata | None]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## handle_protected_resource_response

Import as `mcp.client.auth.oauth2.handle_protected_resource_response`  ·  defined at `mcp.client.auth.utils.handle_protected_resource_response`

```python
async def handle_protected_resource_response(response: Response) -> ProtectedResourceMetadata | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handle protected resource metadata discovery response.

Per SEP-985, supports fallback when discovery fails at one URL.

Returns:
    ProtectedResourceMetadata if successfully discovered, None if we should try next URL


## handle_registration_response

Import as `mcp.client.auth.oauth2.handle_registration_response`  ·  defined at `mcp.client.auth.utils.handle_registration_response`

```python
async def handle_registration_response(response: Response) -> OAuthClientInformationFull
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Handle registration response.


## handle_token_response_scopes

Import as `mcp.client.auth.oauth2.handle_token_response_scopes`  ·  defined at `mcp.client.auth.utils.handle_token_response_scopes`

```python
async def handle_token_response_scopes(response: Response) -> OAuthToken
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse and validate a token response.

Parses token response JSON. Callers should check response.status_code before calling.

Args:
    response: HTTP response from token endpoint (status already checked by caller)

Returns:
    Validated OAuthToken model

Raises:
    OAuthTokenError: If response JSON is invalid


## is_valid_client_metadata_url

Import as `mcp.client.auth.oauth2.is_valid_client_metadata_url`  ·  defined at `mcp.client.auth.utils.is_valid_client_metadata_url`

```python
def is_valid_client_metadata_url(url: str | None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate that a URL is suitable for use as a client_id (CIMD).

The URL must be HTTPS with a non-root pathname.

Args:
    url: The URL to validate

Returns:
    True if the URL is a valid HTTPS URL with a non-root pathname


## issuers_match

Import as `mcp.client.auth.oauth2.issuers_match`  ·  defined at `mcp.client.auth.utils.issuers_match`

```python
def issuers_match(a: str, b: str) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Simple string comparison of two issuer identifiers (RFC 8414 section 3.3), except that a root
issuer with and without its trailing slash (`scheme://authority` and `scheme://authority/`) name
the same server.


## should_use_client_metadata_url

Import as `mcp.client.auth.oauth2.should_use_client_metadata_url`  ·  defined at `mcp.client.auth.utils.should_use_client_metadata_url`

```python
def should_use_client_metadata_url(oauth_metadata: OAuthMetadata | None, client_metadata_url: str | None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Determine if URL-based client ID (CIMD) should be used instead of DCR.

URL-based client IDs should be used when:
1. The server advertises client_id_metadata_document_supported=True
2. The client has a valid client_metadata_url configured

Args:
    oauth_metadata: OAuth authorization server metadata
    client_metadata_url: URL-based client ID (already validated)

Returns:
    True if CIMD should be used, False if DCR should be used


## union_scopes

Import as `mcp.client.auth.oauth2.union_scopes`  ·  defined at `mcp.client.auth.utils.union_scopes`

```python
def union_scopes(previous_scope: str | None, new_scope: str | None) -> str | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Merge two space-delimited scope strings, preserving order and dropping duplicates.

SEP-2350: on step-up re-authorization the client requests the union of previously requested
scopes and the newly challenged scopes, so escalating one operation does not drop the
permissions granted for another. Previously requested scopes come first; new scopes are
appended in order.


## validate_authorization_response_iss

Import as `mcp.client.auth.oauth2.validate_authorization_response_iss`  ·  defined at `mcp.client.auth.utils.validate_authorization_response_iss`

```python
def validate_authorization_response_iss(iss: str | None, oauth_metadata: OAuthMetadata | None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate the RFC 9207 `iss` authorization-response parameter.

Per RFC 9207 section 2.4, the client compares `iss` against the issuer of the
authorization server the request was sent to, using simple string comparison
(RFC 3986 section 6.2.1, i.e. without URL normalization), and rejects on mismatch.
A response that omits `iss` is rejected only when the server advertised support via
`authorization_response_iss_parameter_supported`.

Raises:
    OAuthFlowError: If `iss` is present and does not match, or is absent when the
        authorization server advertised support.


## validate_metadata_issuer

Import as `mcp.client.auth.oauth2.validate_metadata_issuer`  ·  defined at `mcp.client.auth.utils.validate_metadata_issuer`

```python
def validate_metadata_issuer(oauth_metadata: OAuthMetadata, expected_issuer: str) -> None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate that authorization server metadata `issuer` matches the discovery issuer.

Per RFC 8414 section 3.3 / SEP-2468, the `issuer` in the metadata must match the issuer
used to construct the well-known URL, compared as a simple string (RFC 3986 section 6.2.1).

Raises:
    OAuthFlowError: If the metadata issuer does not match `expected_issuer`.


