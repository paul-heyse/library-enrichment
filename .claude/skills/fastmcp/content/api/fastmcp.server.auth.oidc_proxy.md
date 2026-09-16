# `fastmcp.server.auth.oidc_proxy`

Distribution: `fastmcp`

## DEFAULT_OIDC_DISCOVERY_TIMEOUT_SECONDS

Import as `fastmcp.server.auth.providers.aws.DEFAULT_OIDC_DISCOVERY_TIMEOUT_SECONDS`  ·  defined at `fastmcp.server.auth.oidc_proxy.DEFAULT_OIDC_DISCOVERY_TIMEOUT_SECONDS`

```python
DEFAULT_OIDC_DISCOVERY_TIMEOUT_SECONDS = 10
```

**Inferred type** (`ty`, not declared in the source): `Literal[10]`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`fastmcp.server.auth.oidc_proxy.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## OIDCConfiguration

Import as `fastmcp.server.auth.providers.auth0.OIDCConfiguration`  ·  defined at `fastmcp.server.auth.oidc_proxy.OIDCConfiguration`

```python
class OIDCConfiguration(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (45)**

- `acr_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `authorization_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `claim_types_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `claims_locales_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `claims_parameter_supported: bool | None = None`  _class-attribute, instance-attribute_
- `claims_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `code_challenge_methods_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `display_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `def get_oidc_configuration(cls, config_url: AnyHttpUrl, strict: bool | None, timeout_seconds: int | None) -> Self`  _classmethod_
  Get the OIDC configuration for the specified config URL.
- `grant_types_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `id_token_encryption_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `id_token_encryption_enc_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `id_token_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint_auth_methods_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `introspection_endpoint_auth_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `issuer: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `jwks_uri: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `op_policy_uri: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `op_tos_uri: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `registration_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `request_object_encryption_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `request_object_encryption_enc_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `request_object_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `request_parameter_supported: bool | None = None`  _class-attribute, instance-attribute_
- `request_uri_parameter_supported: bool | None = None`  _class-attribute, instance-attribute_
- `require_request_uri_registration: bool | None = None`  _class-attribute, instance-attribute_
- `response_modes_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `response_types_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `revocation_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `revocation_endpoint_auth_methods_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `revocation_endpoint_auth_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `scopes_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `service_documentation: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `signed_metadata: str | None = None`  _class-attribute, instance-attribute_
- `strict: bool = True`  _class-attribute, instance-attribute_
- `subject_types_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `token_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `token_endpoint_auth_methods_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `token_endpoint_auth_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `ui_locales_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `userinfo_encryption_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `userinfo_encryption_enc_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_
- `userinfo_endpoint: AnyHttpUrl | str | None = None`  _class-attribute, instance-attribute_
- `userinfo_signing_alg_values_supported: Sequence[str] | None = None`  _class-attribute, instance-attribute_

OIDC Configuration.

See:
    https://openid.net/specs/openid-connect-discovery-1_0.html#ProviderMetadata
    https://datatracker.ietf.org/doc/html/rfc8414#section-2


## OIDCProxy

Import as `fastmcp.server.auth.OIDCProxy`  ·  defined at `fastmcp.server.auth.oidc_proxy.OIDCProxy`

```python
class OIDCProxy(OAuthProxy)
```

**Also exported as** `fastmcp.server.auth.OIDCProxy`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthProxy`

**Declared members (4)**

- `def get_oidc_configuration(self, config_url: AnyHttpUrl, strict: bool | None, timeout_seconds: int | None) -> OIDCConfiguration`
  Gets the OIDC configuration for the specified configuration URL.
- `def get_token_verifier(self, algorithm: str | None = None, audience: str | None = None, required_scopes: list[str] | None = None, timeout_seconds: int | None = None) -> TokenVerifier`
  Creates the token verifier for the specified OIDC configuration and arguments.
- `oidc_config: OIDCConfiguration = self.get_oidc_configuration(config_url, strict, timeout_seconds)`  _instance-attribute_
- `required_scopes = required_scopes`  _instance-attribute_

**Inherited (27)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `resource_base_url`
- from `fastmcp.server.auth.auth.OAuthProvider`: `client_registration_options`, `get_well_known_routes`, `issuer_url`, `revocation_options`, `scopes_supported`, `service_documentation_url`, `verify_token`
- from `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`: `authorize`, `exchange_authorization_code`, `exchange_identity_assertion`, `exchange_refresh_token`, `get_client`, `get_routes`, `jwt_issuer`, `load_access_token`, `load_authorization_code`, `load_refresh_token`, `register_client`, `revoke_token`, `set_mcp_path`, `token_endpoint_url`, `update_default_scopes`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

OAuth provider that wraps OAuthProxy to provide configuration via an OIDC configuration URL.

This provider makes it easier to add OAuth protection for any upstream provider
that is OIDC compliant.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.auth.oidc_proxy import OIDCProxy

    # Simple OIDC based protection
    auth = OIDCProxy(
        config_url="https://oidc.config.url",
        client_id="your-oidc-client-id",
        client_secret="your-oidc-client-secret",
        base_url="https://your.server.url",
    )

    mcp = FastMCP("My Protected Server", auth=auth)
    ```


