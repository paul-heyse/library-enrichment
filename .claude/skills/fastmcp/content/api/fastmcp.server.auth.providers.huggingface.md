# `fastmcp.server.auth.providers.huggingface`

Distribution: `fastmcp`

## DEFAULT_HUGGINGFACE_SCOPES

`fastmcp.server.auth.providers.huggingface.DEFAULT_HUGGINGFACE_SCOPES`

```python
DEFAULT_HUGGINGFACE_SCOPES = ['openid', 'profile']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## HUGGINGFACE_AUTHORIZATION_ENDPOINT

`fastmcp.server.auth.providers.huggingface.HUGGINGFACE_AUTHORIZATION_ENDPOINT`

```python
HUGGINGFACE_AUTHORIZATION_ENDPOINT = 'https://huggingface.co/oauth/authorize'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://huggingface.co/oauth/authorize"]`

## HUGGINGFACE_TOKEN_ENDPOINT

`fastmcp.server.auth.providers.huggingface.HUGGINGFACE_TOKEN_ENDPOINT`

```python
HUGGINGFACE_TOKEN_ENDPOINT = 'https://huggingface.co/oauth/token'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://huggingface.co/oauth/token"]`

## HUGGINGFACE_USERINFO_ENDPOINT

`fastmcp.server.auth.providers.huggingface.HUGGINGFACE_USERINFO_ENDPOINT`

```python
HUGGINGFACE_USERINFO_ENDPOINT = 'https://huggingface.co/oauth/userinfo'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://huggingface.co/oauth/userinfo"]`

## HUGGINGFACE_WHOAMI_ENDPOINT

`fastmcp.server.auth.providers.huggingface.HUGGINGFACE_WHOAMI_ENDPOINT`

```python
HUGGINGFACE_WHOAMI_ENDPOINT = 'https://huggingface.co/api/whoami-v2'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://huggingface.co/api/whoami-v2"]`

## logger

`fastmcp.server.auth.providers.huggingface.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## HuggingFaceProvider

`fastmcp.server.auth.providers.huggingface.HuggingFaceProvider`

```python
class HuggingFaceProvider(OAuthProxy)
```

**Bases** `OAuthProxy`

**Declared members (1)**

- `required_scopes = required_scopes_final`  _instance-attribute_

**Inherited (27)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Complete Hugging Face OAuth provider for FastMCP.


## HuggingFaceTokenVerifier

`fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier`

```python
class HuggingFaceTokenVerifier(TokenVerifier)
```

**Bases** `TokenVerifier`

**Declared members (2)**

- `timeout_seconds = timeout_seconds`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a Hugging Face OAuth token using the userinfo endpoint.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier for Hugging Face OAuth access tokens.

Hugging Face OAuth access tokens are opaque, so validation is performed by
calling Hugging Face's userinfo endpoint.


## _extract_scopes

`fastmcp.server.auth.providers.huggingface._extract_scopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_scopes(data: Mapping[str, Any]) -> list[str]
```

