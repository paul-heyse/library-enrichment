# `mcp.client.auth.extensions.client_credentials`

Distribution: `mcp`

## ClientCredentialsOAuthProvider

`mcp.client.auth.extensions.client_credentials.ClientCredentialsOAuthProvider`

```python
class ClientCredentialsOAuthProvider(OAuthClientProvider)
```

**Also exported as** `fastmcp.client.auth.client_credentials._SDKClientCredentialsOAuthProvider`

**Bases** `OAuthClientProvider`

**Inherited (3)**

- from `mcp.client.auth.oauth2.OAuthClientProvider`: `context`, `requires_response_body`
- from `mcp.shared._httpx_utils.RedirectAwareAuth`: `async_auth_flow`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth provider for client_credentials grant with client_id + client_secret.

This provider sets client_info directly, bypassing dynamic client registration.
Use this when you already have client credentials (client_id and client_secret).
Pass `issuer` to name the authorization server those credentials belong to: token
requests are then only built from authorization server metadata for that issuer, and
the flow stops if the MCP server leads anywhere else.

Example:
    ```python
    provider = ClientCredentialsOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        client_secret="my-client-secret",
        issuer="https://auth.example.com",
    )
    ```


## PrivateKeyJWTOAuthProvider

`mcp.client.auth.extensions.client_credentials.PrivateKeyJWTOAuthProvider`

```python
class PrivateKeyJWTOAuthProvider(OAuthClientProvider)
```

**Also exported as** `fastmcp.client.auth.client_credentials._SDKPrivateKeyJWTOAuthProvider`

**Bases** `OAuthClientProvider`

**Inherited (3)**

- from `mcp.client.auth.oauth2.OAuthClientProvider`: `context`, `requires_response_body`
- from `mcp.shared._httpx_utils.RedirectAwareAuth`: `async_auth_flow`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth provider for client_credentials grant with private_key_jwt authentication.

Uses RFC 7523 Section 2.2 for client authentication via JWT assertion.

The JWT assertion's audience MUST be the authorization server's issuer identifier
(per RFC 7523bis security updates). The `assertion_provider` callback receives
this audience value and must return a JWT with that audience. Pass `issuer` to name
the authorization server this client is registered with: an assertion is then only
minted once metadata for that issuer has been discovered, and token requests are only
built from that metadata.

**Option 1: Pre-built JWT via Workload Identity Federation**

In production scenarios, the JWT assertion is typically obtained from a workload
identity provider (e.g., GCP, AWS IAM, Azure AD):

    ```python
    async def get_workload_identity_token(audience: str) -> str:
        # Fetch JWT from your identity provider
        # The JWT's audience must match the provided audience parameter
        return await fetch_token_from_identity_provider(audience=audience)

    provider = PrivateKeyJWTOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        assertion_provider=get_workload_identity_token,
        issuer="https://auth.example.com",
    )
    ```

**Option 2: Static pre-built JWT**

If you have a static JWT that doesn't need the audience parameter:

    ```python
    provider = PrivateKeyJWTOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        assertion_provider=static_assertion_provider(my_prebuilt_jwt),
        issuer="https://auth.example.com",
    )
    ```

**Option 3: SDK-signed JWT (for testing/simple setups)**

For testing or simple deployments, use `SignedJWTParameters.create_assertion_provider()`:

    ```python
    jwt_params = SignedJWTParameters(
        issuer="my-client-id",
        subject="my-client-id",
        signing_key=private_key_pem,
    )
    provider = PrivateKeyJWTOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        assertion_provider=jwt_params.create_assertion_provider(),
        issuer="https://auth.example.com",
    )
    ```


## SignedJWTParameters

Import as `fastmcp.client.auth.SignedJWTParameters`  ·  defined at `mcp.client.auth.extensions.client_credentials.SignedJWTParameters`

```python
class SignedJWTParameters(BaseModel)
```

**Also exported as** `fastmcp.client.auth.SignedJWTParameters`, `fastmcp.client.auth.client_credentials.SignedJWTParameters`

**Bases** `BaseModel`

**Declared members (7)**

- `additional_claims: dict[str, Any] | None = Field(default=None, description='Additional claims.')`  _class-attribute, instance-attribute_
- `def create_assertion_provider(self) -> Callable[[str], Awaitable[str]]`
  Create an assertion provider callback for use with PrivateKeyJWTOAuthProvider.
- `issuer: str = Field(description='Issuer for JWT assertions (typically client_id).')`  _class-attribute, instance-attribute_
- `lifetime_seconds: int = Field(default=300, description='Lifetime of generated JWT in seconds.')`  _class-attribute, instance-attribute_
- `signing_algorithm: str = Field(default='RS256', description='Algorithm for signing JWT assertions.')`  _class-attribute, instance-attribute_
- `signing_key: str = Field(description='Private key for JWT signing (PEM format).')`  _class-attribute, instance-attribute_
- `subject: str = Field(description='Subject identifier for JWT assertions (typically client_id).')`  _class-attribute, instance-attribute_

Parameters for creating SDK-signed JWT assertions.

Use `create_assertion_provider()` to create an assertion provider callback
for use with `PrivateKeyJWTOAuthProvider`.

Example:
    ```python
    jwt_params = SignedJWTParameters(
        issuer="my-client-id",
        subject="my-client-id",
        signing_key=private_key_pem,
    )
    provider = PrivateKeyJWTOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        assertion_provider=jwt_params.create_assertion_provider(),
        issuer="https://auth.example.com",
    )
    ```


## _checked_issuer

`mcp.client.auth.extensions.client_credentials._checked_issuer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _checked_issuer(issuer: str | None) -> str | None
```

## _preferred_authorization_server

`mcp.client.auth.extensions.client_credentials._preferred_authorization_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _preferred_authorization_server(advertised: list[str], issuer: str | None) -> str
```

The advertised server matching the configured issuer if there is one, else the first.


## _require_metadata_for_configured_issuer

`mcp.client.auth.extensions.client_credentials._require_metadata_for_configured_issuer`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _require_metadata_for_configured_issuer(context: OAuthContext, issuer: str | None) -> None
```

With an issuer configured, a token request is only built from metadata discovered for that issuer.

Anything else held is dropped along with the tokens, so the next request starts discovery afresh
rather than refreshing against it.


## static_assertion_provider

Import as `fastmcp.client.auth.static_assertion_provider`  ·  defined at `mcp.client.auth.extensions.client_credentials.static_assertion_provider`

```python
def static_assertion_provider(token: str) -> Callable[[str], Awaitable[str]]
```

**Also exported as** `fastmcp.client.auth.client_credentials.static_assertion_provider`, `fastmcp.client.auth.static_assertion_provider`

Create an assertion provider that returns a static JWT token.

Use this when you have a pre-built JWT (e.g., from workload identity federation)
that doesn't need the audience parameter.

Example:
    ```python
    provider = PrivateKeyJWTOAuthProvider(
        server_url="https://api.example.com",
        storage=my_token_storage,
        client_id="my-client-id",
        assertion_provider=static_assertion_provider(my_prebuilt_jwt),
        issuer="https://auth.example.com",
    )
    ```

Args:
    token: The pre-built JWT assertion string.

Returns:
    An async callback suitable for use as an assertion_provider.


