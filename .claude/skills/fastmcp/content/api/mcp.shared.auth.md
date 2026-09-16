# `mcp.shared.auth`

Distribution: `mcp`

## DEFAULT_GRANT_TYPES

`mcp.shared.auth.DEFAULT_GRANT_TYPES`

```python
DEFAULT_GRANT_TYPES = ['authorization_code', 'refresh_token']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## JWT_BEARER_GRANT_TYPE

Import as `mcp.server.auth.routes.JWT_BEARER_GRANT_TYPE`  ·  defined at `mcp.shared.auth.JWT_BEARER_GRANT_TYPE`

```python
JWT_BEARER_GRANT_TYPE = 'urn:ietf:params:oauth:grant-type:jwt-bearer'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:grant-type:jwt-bearer"]`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TokenEndpointAuthMethod

Import as `mcp.client.auth.oauth2.TokenEndpointAuthMethod`  ·  defined at `mcp.shared.auth.TokenEndpointAuthMethod`

```python
TokenEndpointAuthMethod = Literal['none', 'client_secret_post', 'client_secret_basic', 'private_key_jwt']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["none", "client_secret_post", "client_secret_basic", "private_key_jwt"]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## AuthorizationCodeResult

Import as `mcp.client.auth.AuthorizationCodeResult`  ·  defined at `mcp.shared.auth.AuthorizationCodeResult`

```python
class AuthorizationCodeResult(BaseModel)
```

**Also exported as** `mcp.client.auth.AuthorizationCodeResult`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `code: str`  _instance-attribute_
- `iss: str | None = None`  _class-attribute, instance-attribute_
- `state: str | None = None`  _class-attribute, instance-attribute_

Authorization-code-grant redirect parameters returned by a callback handler.

`iss` carries the RFC 9207 authorization-response issuer when the authorization server
includes it in the redirect; the client validates it against the expected issuer.


## InvalidRedirectUriError

Import as `mcp.server.auth.handlers.authorize.InvalidRedirectUriError`  ·  defined at `mcp.shared.auth.InvalidRedirectUriError`

```python
class InvalidRedirectUriError(Exception)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (1)**

- `message = message`  _instance-attribute_

## InvalidScopeError

Import as `mcp.server.auth.handlers.authorize.InvalidScopeError`  ·  defined at `mcp.shared.auth.InvalidScopeError`

```python
class InvalidScopeError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (1)**

- `message = message`  _instance-attribute_

## OAuthClientInformationFull

Import as `mcp.client.auth.utils.OAuthClientInformationFull`  ·  defined at `mcp.shared.auth.OAuthClientInformationFull`

```python
class OAuthClientInformationFull(OAuthClientMetadataBase)
```

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthClientMetadataBase`

**Declared members (11)**

- `application_type: str | None = None`  _class-attribute, instance-attribute_
- `client_id: str`  _instance-attribute_
- `client_id_issued_at: int | None = None`  _class-attribute, instance-attribute_
- `client_secret: str | None = None`  _class-attribute, instance-attribute_
- `client_secret_expires_at: int | None = None`  _class-attribute, instance-attribute_
- `grant_types: list[str] = list(DEFAULT_GRANT_TYPES)`  _class-attribute, instance-attribute_
- `issuer: str | None = None`  _class-attribute, instance-attribute_
- `redirect_uris: list[AnyUrl] | None = None`  _class-attribute, instance-attribute_
- `token_endpoint_auth_method: str | None = None`  _class-attribute, instance-attribute_
- `def validate_redirect_uri(self, redirect_uri: AnyUrl | None) -> AnyUrl`
- `def validate_scope(self, requested_scope: str | None) -> list[str] | None`

**Inherited (12)**

- from `mcp.shared.auth.OAuthClientMetadataBase`: `client_name`, `client_uri`, `contacts`, `jwks`, `jwks_uri`, `logo_uri`, `policy_uri`, `response_types`, `scope`, `software_id`, `software_version`, `tos_uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

RFC 7591 OAuth 2.0 Dynamic Client Registration client information response
(client information plus metadata) - the authorization server's record of a
registered client. See https://datatracker.ietf.org/doc/html/rfc7591#section-3.2.1

A third-party authorization server "MAY reject or replace any of the client's
requested metadata values submitted during the registration and substitute them with
suitable values", so `application_type`, `token_endpoint_auth_method`, and `grant_types`
are typed to accept any string the server echoes, and `redirect_uris` may be absent or
empty. A member the server serializes as a placeholder - an explicit `null`, or `""` -
is read as an omitted key, so the field's default applies rather than the parse failing.
Whether a substituted value is usable is decided where the value is used, not at parse.
`redirect_uris` elements are still parsed as URLs, as the authorization server compares
them against a client's requested `redirect_uri`.


## OAuthClientMetadata

Import as `mcp.client.auth.utils.OAuthClientMetadata`  ·  defined at `mcp.shared.auth.OAuthClientMetadata`

```python
class OAuthClientMetadata(OAuthClientMetadataBase)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthClientMetadataBase`

**Declared members (4)**

- `application_type: Literal['web', 'native'] = 'native'`  _class-attribute, instance-attribute_
- `grant_types: list[Literal['authorization_code', 'refresh_token', 'urn:ietf:params:oauth:grant-type:jwt-bearer'] | str] = list(DEFAULT_GRANT_TYPES)`  _class-attribute, instance-attribute_
- `redirect_uris: list[AnyUrl] | None = Field(..., min_length=1)`  _class-attribute, instance-attribute_
- `token_endpoint_auth_method: TokenEndpointAuthMethod | None = None`  _class-attribute, instance-attribute_

**Inherited (12)**

- from `mcp.shared.auth.OAuthClientMetadataBase`: `client_name`, `client_uri`, `contacts`, `jwks`, `jwks_uri`, `logo_uri`, `policy_uri`, `response_types`, `scope`, `software_id`, `software_version`, `tos_uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

RFC 7591 OAuth 2.0 Dynamic Client Registration request metadata: what an MCP
client sends when it registers. Field values are narrowed to what this SDK will put
on the wire; parsing the authorization server's response is `OAuthClientInformationFull`'s
job. See https://datatracker.ietf.org/doc/html/rfc7591#section-2


## OAuthClientMetadataBase

`mcp.shared.auth.OAuthClientMetadataBase`

```python
class OAuthClientMetadataBase(BaseModel)
```

**Bases** `BaseModel`

**Declared members (12)**

- `client_name: str | None = None`  _class-attribute, instance-attribute_
- `client_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `contacts: list[str] | None = None`  _class-attribute, instance-attribute_
- `jwks: Any | None = None`  _class-attribute, instance-attribute_
- `jwks_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `logo_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `policy_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `response_types: list[str] = ['code']`  _class-attribute, instance-attribute_
- `scope: str | None = None`  _class-attribute, instance-attribute_
- `software_id: str | None = None`  _class-attribute, instance-attribute_
- `software_version: str | None = None`  _class-attribute, instance-attribute_
- `tos_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_

RFC 7591 OAuth 2.0 Dynamic Client Registration metadata shared verbatim by the
registration request (`OAuthClientMetadata`) and the authorization server's record of a
registered client (`OAuthClientInformationFull`). Fields whose acceptable values differ
between the two - what this SDK sends versus what a third-party server may echo - are
declared on each model rather than here.
See https://datatracker.ietf.org/doc/html/rfc7591#section-2


## OAuthMetadata

Import as `mcp.client.auth.utils.OAuthMetadata`  ·  defined at `mcp.shared.auth.OAuthMetadata`

```python
class OAuthMetadata(BaseModel)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (24)**

- `authorization_endpoint: AnyHttpUrl`  _instance-attribute_
- `authorization_grant_profiles_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `authorization_response_iss_parameter_supported: bool | None = None`  _class-attribute, instance-attribute_
- `client_id_metadata_document_supported: bool | None = None`  _class-attribute, instance-attribute_
- `code_challenge_methods_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `grant_types_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint_auth_methods_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint_auth_signing_alg_values_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `issuer: AnyHttpUrl`  _instance-attribute_
- `op_policy_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `op_tos_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `registration_endpoint: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `response_modes_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `response_types_supported: list[str] = ['code']`  _class-attribute, instance-attribute_
- `revocation_endpoint: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `revocation_endpoint_auth_methods_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `revocation_endpoint_auth_signing_alg_values_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `scopes_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `service_documentation: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `token_endpoint: AnyHttpUrl`  _instance-attribute_
- `token_endpoint_auth_methods_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `token_endpoint_auth_signing_alg_values_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `ui_locales_supported: list[str] | None = None`  _class-attribute, instance-attribute_

RFC 8414 OAuth 2.0 Authorization Server Metadata.
See https://datatracker.ietf.org/doc/html/rfc8414#section-2


## OAuthToken

Import as `mcp.client.auth.utils.OAuthToken`  ·  defined at `mcp.shared.auth.OAuthToken`

```python
class OAuthToken(BaseModel)
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (6)**

- `access_token: str`  _instance-attribute_
- `expires_in: int | None = None`  _class-attribute, instance-attribute_
- `def normalize_token_type(cls, v: str | None) -> str | None`  _classmethod_
- `refresh_token: str | None = None`  _class-attribute, instance-attribute_
- `scope: str | None = None`  _class-attribute, instance-attribute_
- `token_type: Literal['Bearer'] = 'Bearer'`  _class-attribute, instance-attribute_

See https://datatracker.ietf.org/doc/html/rfc6749#section-5.1


## ProtectedResourceMetadata

Import as `mcp.client.auth.utils.ProtectedResourceMetadata`  ·  defined at `mcp.shared.auth.ProtectedResourceMetadata`

```python
class ProtectedResourceMetadata(BaseModel)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (14)**

- `authorization_details_types_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `authorization_servers: list[AnyHttpUrl] = Field(..., min_length=1)`  _class-attribute, instance-attribute_
- `bearer_methods_supported: list[str] | None = Field(default=['header'])`  _class-attribute, instance-attribute_
- `dpop_bound_access_tokens_required: bool | None = None`  _class-attribute, instance-attribute_
- `dpop_signing_alg_values_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `jwks_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `resource: AnyHttpUrl`  _instance-attribute_
- `resource_documentation: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `resource_name: str | None = None`  _class-attribute, instance-attribute_
- `resource_policy_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `resource_signing_alg_values_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `resource_tos_uri: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `scopes_supported: list[str] | None = None`  _class-attribute, instance-attribute_
- `tls_client_certificate_bound_access_tokens: bool | None = None`  _class-attribute, instance-attribute_

RFC 9728 OAuth 2.0 Protected Resource Metadata.
See https://datatracker.ietf.org/doc/html/rfc9728#section-2


## _empty_str_to_none

`mcp.shared.auth._empty_str_to_none`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _empty_str_to_none(v: object) -> object
```

