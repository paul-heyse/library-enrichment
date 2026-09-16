# `mcp.server.auth.settings`

Distribution: `mcp`

## AuthSettings

Import as `mcp.server.lowlevel.server.AuthSettings`  ·  defined at `mcp.server.auth.settings.AuthSettings`

```python
class AuthSettings(BaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (8)**

- `client_registration_options: ClientRegistrationOptions | None = None`  _class-attribute, instance-attribute_
- `identity_assertion_enabled: bool = Field(default=False, description='Advertise and accept the SEP-990 Identity Assertion Authorization Grant (the RFC 7523 jwt-bearer grant carrying an ID-JAG) at the token endpoint, for enterprise IdP flows. The provider must implement `exchange_identity_assertion`.')`  _class-attribute, instance-attribute_
- `issuer_url: AnyHttpUrl = Field(..., description='OAuth authorization server URL that issues tokens for this resource server.')`  _class-attribute, instance-attribute_
- `required_scopes: list[str] | None = None`  _class-attribute, instance-attribute_
- `resource_server_url: AnyHttpUrl | None = Field(..., description='The URL of the MCP server to be used as the resource identifier and base route to look up OAuth Protected Resource Metadata.')`  _class-attribute, instance-attribute_
- `revocation_options: RevocationOptions | None = None`  _class-attribute, instance-attribute_
- `service_documentation_url: AnyHttpUrl | None = None`  _class-attribute, instance-attribute_
- `validate_token_resource: bool | None = Field(default=None, description="Only accept tokens the token verifier reports as issued for `resource_server_url` (`AccessToken.resource`, the RFC 8707 resource indicator). Enable it when your authorization server binds tokens to the `resource` the client requested; set it to False when your token verifier checks the token's audience itself. With `resource_server_url` set, leaving it unset warns and behaves as False; 3.0 makes True the default there.")`  _class-attribute, instance-attribute_

## ClientRegistrationOptions

Import as `mcp.server.auth.routes.ClientRegistrationOptions`  ·  defined at `mcp.server.auth.settings.ClientRegistrationOptions`

```python
class ClientRegistrationOptions(BaseModel)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (4)**

- `client_secret_expiry_seconds: int | None = None`  _class-attribute, instance-attribute_
- `default_scopes: list[str] | None = None`  _class-attribute, instance-attribute_
- `enabled: bool = False`  _class-attribute, instance-attribute_
- `valid_scopes: list[str] | None = None`  _class-attribute, instance-attribute_

## RevocationOptions

Import as `mcp.server.auth.routes.RevocationOptions`  ·  defined at `mcp.server.auth.settings.RevocationOptions`

```python
class RevocationOptions(BaseModel)
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (1)**

- `enabled: bool = False`  _class-attribute, instance-attribute_

