# `fastmcp.server.auth.oauth_proxy.proxy`

Distribution: `fastmcp`

## _ID_JAG_GRANT_MARKER

`fastmcp.server.auth.oauth_proxy.proxy._ID_JAG_GRANT_MARKER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ID_JAG_GRANT_MARKER = 'id_jag'
```

## _REFRESH_LOCK_CACHE_SIZE

`fastmcp.server.auth.oauth_proxy.proxy._REFRESH_LOCK_CACHE_SIZE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_REFRESH_LOCK_CACHE_SIZE = 10000
```

## _pending_application_type

`fastmcp.server.auth.oauth_proxy.proxy._pending_application_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_pending_application_type: ContextVar[Literal['web', 'native'] | None] = ContextVar('_pending_application_type', default=None)
```

## logger

`fastmcp.server.auth.oauth_proxy.proxy.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OAuthProxy

Import as `fastmcp.server.auth.OAuthProxy`  ·  defined at `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`

```python
class OAuthProxy(OAuthProvider, ConsentMixin)
```

**Also exported as** `fastmcp.server.auth.OAuthProxy`, `fastmcp.server.auth.oauth_proxy.OAuthProxy`

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthProvider`, `ConsentMixin`

**Declared members (15)**

- `async def authorize(self, client: OAuthClientInformationFull, params: AuthorizationParams) -> str`  _async_
  Start OAuth transaction and route through consent interstitial.
- `async def exchange_authorization_code(self, client: OAuthClientInformationFull, authorization_code: AuthorizationCode) -> OAuthToken`  _async_
  Exchange authorization code for FastMCP-issued tokens.
- `async def exchange_identity_assertion(self, client: OAuthClientInformationFull, params: IdentityAssertionParams) -> OAuthToken`  _async_
  Exchange a SEP-990 ID-JAG for a short-lived FastMCP access token.
- `async def exchange_refresh_token(self, client: OAuthClientInformationFull, refresh_token: RefreshToken, scopes: list[str]) -> OAuthToken`  _async_
  Exchange FastMCP refresh token for new FastMCP access token.
- `async def get_client(self, client_id: str) -> OAuthClientInformationFull | None`  _async_
  Get client information by ID. This is generally the random ID provided to the DCR client during registration, not the upstream client ID.
- `def get_routes(self, mcp_path: str | None = None) -> list[Route]`
  Get OAuth routes with custom handlers for better error UX.
- `jwt_issuer: JWTIssuer`  _property_
  Get the JWT issuer, ensuring it has been initialized.
- `async def load_access_token(self, token: str) -> AccessToken | None`  _async_
  Validate FastMCP JWT by swapping for upstream token.
- `async def load_authorization_code(self, client: OAuthClientInformationFull, authorization_code: str) -> AuthorizationCode | None`  _async_
  Load authorization code for validation.
- `async def load_refresh_token(self, client: OAuthClientInformationFull, refresh_token: str) -> RefreshToken | None`  _async_
  Load refresh token metadata from distributed storage.
- `async def register_client(self, client_info: OAuthClientInformationFull) -> None`  _async_
  Register a client locally
- `async def revoke_token(self, token: AccessToken | RefreshToken) -> None`  _async_
  Revoke token locally and with upstream server if supported.
- `def set_mcp_path(self, mcp_path: str | None) -> None`
  Set the MCP endpoint path and create JWTIssuer with correct audience.
- `token_endpoint_url: str`  _property_
  The token endpoint URL, as advertised in the authorization server metadata.
- `def update_default_scopes(self, scopes: list[str]) -> None`
  Update the default scopes advertised to clients and used for DCR/CIMD fallback.

**Inherited (13)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `required_scopes`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth provider that presents a DCR-compliant interface while proxying to non-DCR IDPs.

Purpose
-------
MCP clients expect OAuth providers to support Dynamic Client Registration (DCR),
where clients can register themselves dynamically and receive unique credentials.
Most enterprise IDPs (Google, GitHub, Azure AD, etc.) don't support DCR and require
pre-registered OAuth applications with fixed credentials.

This proxy bridges that gap by:
- Presenting a full DCR-compliant OAuth interface to MCP clients
- Translating DCR registration requests to use pre-configured upstream credentials
- Proxying all OAuth flows to the upstream IDP with appropriate translations
- Managing the state and security requirements of both protocols

Architecture Overview
--------------------
The proxy maintains a single OAuth app registration with the upstream provider
while allowing unlimited MCP clients to register and authenticate dynamically.
It implements the complete OAuth 2.1 + DCR specification for clients while
translating to whatever OAuth variant the upstream provider requires.

Key Translation Challenges Solved
---------------------------------
1. Dynamic Client Registration:
   - MCP clients expect to register dynamically and get unique credentials
   - Upstream IDPs require pre-registered apps with fixed credentials
   - Solution: Accept DCR requests, return shared upstream credentials

2. Dynamic Redirect URIs:
   - MCP clients use random localhost ports that change between sessions
   - Upstream IDPs require fixed, pre-registered redirect URIs
   - Solution: Use proxy's fixed callback URL with upstream, forward to client's dynamic URI

3. Authorization Code Mapping:
   - Upstream returns codes for the proxy's redirect URI
   - Clients expect codes for their own redirect URIs
   - Solution: Exchange upstream code server-side, issue new code to client

4. State Parameter Collision:
   - Both client and proxy need to maintain state through the flow
   - Only one state parameter available in OAuth
   - Solution: Use transaction ID as state with upstream, preserve client's state

5. Token Management:
   - Clients may expect different token formats/claims than upstream provides
   - Need to track tokens for revocation and refresh
   - Solution: Store token relationships, forward upstream tokens transparently

OAuth Flow Implementation
------------------------
1. Client Registration (DCR):
   - Accept any client registration request
   - Store ProxyDCRClient that accepts dynamic redirect URIs

2. Authorization:
   - Store transaction mapping client details to proxy flow
   - Redirect to upstream with proxy's fixed redirect URI
   - Use transaction ID as state parameter with upstream

3. Upstream Callback:
   - Exchange upstream authorization code for tokens (server-side)
   - Generate new authorization code bound to client's PKCE challenge
   - Redirect to client's original dynamic redirect URI

4. Token Exchange:
   - Validate client's code and PKCE verifier
   - Return previously obtained upstream tokens
   - Clean up one-time use authorization code

5. Token Refresh:
   - Forward refresh requests to upstream
   - Handle token rotation if upstream issues new refresh token
   - Update local token mappings

State Management
---------------
The proxy maintains minimal but crucial state via pluggable storage (client_storage):
- _oauth_transactions: Active authorization flows with client context
- _client_codes: Authorization codes with PKCE challenges and upstream tokens
- _jti_mapping_store: Maps FastMCP token JTIs to upstream token IDs
- _refresh_token_store: Refresh token metadata (keyed by token hash)

All state is stored in the configured client_storage backend (Redis, disk, etc.)
enabling horizontal scaling across multiple instances.

Security Considerations
----------------------
- Refresh tokens stored by hash only (defense in depth if storage compromised)
- PKCE enforced end-to-end (client to proxy, proxy to upstream)
- Authorization codes are single-use with short expiry
- Transaction IDs are cryptographically random
- All state is cleaned up after use to prevent replay
- Token validation delegates to upstream provider

Provider Compatibility
---------------------
Works with any OAuth 2.0 provider that supports:
- Authorization code flow
- Fixed redirect URI (configured in provider's app settings)
- Standard token endpoint

Handles provider-specific requirements:
- Google: Ensures minimum scope requirements
- GitHub: Compatible with OAuth Apps and GitHub Apps
- Azure AD: Handles tenant-specific endpoints
- Generic: Works with any spec-compliant provider


## _ApplicationTypeRegistrationHandler

`fastmcp.server.auth.oauth_proxy.proxy._ApplicationTypeRegistrationHandler`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ApplicationTypeRegistrationHandler
```

**Declared members (1)**

- `async def handle(self, request: Request) -> Response`  _async_

Recover the DCR `application_type` the SDK handler drops (SEP-837).

The SDK's `RegistrationHandler` validates the request body into an
`OAuthClientMetadata` (which carries `application_type`) but omits the field
when constructing the `OAuthClientInformationFull` it passes to
`register_client`. This thin wrapper re-parses `application_type` from the
same request body and publishes it on a ContextVar so `register_client` can
enforce the web/native redirect rules, then delegates to the SDK handler
unchanged. Reading `request.body()` here is safe: Starlette caches the body,
so the SDK handler's own read returns the same bytes.


## _assertion_granted_scopes

`fastmcp.server.auth.oauth_proxy.proxy._assertion_granted_scopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _assertion_granted_scopes(claims: dict[str, Any]) -> list[str]
```

Scopes granted by an ID-JAG, from its `scope` or `scp` claim.


