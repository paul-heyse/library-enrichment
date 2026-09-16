# `fastmcp.cli.deploy.horizon_client`

Distribution: `fastmcp`

## DEFAULT_HORIZON_API_ORIGIN

Import as `fastmcp.cli.deploy.configuration.DEFAULT_HORIZON_API_ORIGIN`  ·  defined at `fastmcp.cli.deploy.horizon_client.DEFAULT_HORIZON_API_ORIGIN`

```python
DEFAULT_HORIZON_API_ORIGIN = 'https://horizon.prefect.io'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://horizon.prefect.io"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DEVICE_AUTH_CLIENT_ID

`fastmcp.cli.deploy.horizon_client.DEVICE_AUTH_CLIENT_ID`

```python
DEVICE_AUTH_CLIENT_ID = 'fastmcp-cli'
```

**Inferred type** (`ty`, not declared in the source): `Literal["fastmcp-cli"]`

## DEVICE_AUTH_GRANT_TYPE

`fastmcp.cli.deploy.horizon_client.DEVICE_AUTH_GRANT_TYPE`

```python
DEVICE_AUTH_GRANT_TYPE = 'urn:ietf:params:oauth:grant-type:device_code'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:grant-type:device_code"]`

## DeviceTokenError

`fastmcp.cli.deploy.horizon_client.DeviceTokenError`

```python
DeviceTokenError = Literal['authorization_pending', 'slow_down', 'access_denied', 'expired_token']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["authorization_pending", "slow_down", "access_denied", "expired_token"]'> ````

## ResponseModelT

`fastmcp.cli.deploy.horizon_client.ResponseModelT`

```python
ResponseModelT = TypeVar('ResponseModelT', bound=_ResponseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## DeviceAccessToken

`fastmcp.cli.deploy.horizon_client.DeviceAccessToken`

```python
class DeviceAccessToken(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (3)**

- `access_token: SecretStr`  _instance-attribute_
- `def require_nonempty_access_token(cls, value: SecretStr) -> SecretStr`  _classmethod_
- `token_type: Literal['Bearer']`  _instance-attribute_

## DeviceAuthorization

Import as `fastmcp.cli.deploy.output.DeviceAuthorization`  ·  defined at `fastmcp.cli.deploy.horizon_client.DeviceAuthorization`

```python
class DeviceAuthorization(_ResponseModel)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_ResponseModel`

**Declared members (6)**

- `device_code: Annotated[str, Field(min_length=1)]`  _instance-attribute_
- `expires_in: Annotated[int, Field(gt=0)]`  _instance-attribute_
- `interval: Annotated[int, Field(gt=0)]`  _instance-attribute_
- `user_code: Annotated[str, Field(min_length=1)]`  _instance-attribute_
- `verification_uri: Annotated[str, Field(pattern='^https?://')]`  _instance-attribute_
- `verification_uri_complete: Annotated[str, Field(pattern='^https?://')]`  _instance-attribute_

## DeviceMetadata

Import as `fastmcp.cli.deploy.command.DeviceMetadata`  ·  defined at `fastmcp.cli.deploy.horizon_client.DeviceMetadata`

```python
class DeviceMetadata
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `architecture: str | None = None`  _class-attribute, instance-attribute_
- `client_version: str | None = None`  _class-attribute, instance-attribute_
- `device_name: str | None = None`  _class-attribute, instance-attribute_
- `platform: str | None = None`  _class-attribute, instance-attribute_

## DeviceTokenPoll

`fastmcp.cli.deploy.horizon_client.DeviceTokenPoll`

```python
class DeviceTokenPoll
```

**Declared members (2)**

- `access_token: SecretStr | None = None`  _class-attribute, instance-attribute_
- `error: DeviceTokenError | None = None`  _class-attribute, instance-attribute_

## HorizonClient

Import as `fastmcp.cli.deploy.command.HorizonClient`  ·  defined at `fastmcp.cli.deploy.horizon_client.HorizonClient`

```python
class HorizonClient
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (7)**

- `async def aclose(self) -> None`  _async_
- `api_origin = normalize_api_origin(api_origin)`  _instance-attribute_
- `async def create_device_authorization(self, metadata: DeviceMetadata | None = None) -> DeviceAuthorization`  _async_
- `async def exchange_device_authorization(self, device_code: str) -> DeviceTokenPoll`  _async_
- `async def get_current_user(self) -> HorizonUser`  _async_
- `async def list_organizations(self) -> tuple[HorizonOrganization, ...]`  _async_
- `async def revoke_current_api_key(self) -> None`  _async_

Call the Horizon routes used by FastMCP CLI authentication.


## HorizonError

`fastmcp.cli.deploy.horizon_client.HorizonError`

```python
class HorizonError(RuntimeError)
```

**Bases** `RuntimeError`

A safe Horizon client error.


## HorizonOrganization

`fastmcp.cli.deploy.horizon_client.HorizonOrganization`

```python
class HorizonOrganization(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (3)**

- `id: str`  _instance-attribute_
- `name: str`  _instance-attribute_
- `slug: str`  _instance-attribute_

## HorizonResponseError

Import as `fastmcp.cli.deploy.command.HorizonResponseError`  ·  defined at `fastmcp.cli.deploy.horizon_client.HorizonResponseError`

```python
class HorizonResponseError(HorizonError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `HorizonError`

**Declared members (1)**

- `status_code = status_code`  _instance-attribute_

Horizon returned an unexpected response.


## HorizonUnauthorizedError

Import as `fastmcp.cli.deploy.command.HorizonUnauthorizedError`  ·  defined at `fastmcp.cli.deploy.horizon_client.HorizonUnauthorizedError`

```python
class HorizonUnauthorizedError(HorizonError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `HorizonError`

The Horizon credential was rejected.


## HorizonUnavailableError

Import as `fastmcp.cli.deploy.command.HorizonUnavailableError`  ·  defined at `fastmcp.cli.deploy.horizon_client.HorizonUnavailableError`

```python
class HorizonUnavailableError(HorizonError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `HorizonError`

The Horizon API could not be reached.


## HorizonUser

Import as `fastmcp.cli.deploy.output.HorizonUser`  ·  defined at `fastmcp.cli.deploy.horizon_client.HorizonUser`

```python
class HorizonUser(_ResponseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_ResponseModel`

**Declared members (3)**

- `email: str`  _instance-attribute_
- `id: str`  _instance-attribute_
- `name: str | None`  _instance-attribute_

## _CurrentUserResponse

`fastmcp.cli.deploy.horizon_client._CurrentUserResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentUserResponse(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (1)**

- `user: HorizonUser`  _instance-attribute_

## _DeviceTokenErrorResponse

`fastmcp.cli.deploy.horizon_client._DeviceTokenErrorResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _DeviceTokenErrorResponse(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (1)**

- `error: DeviceTokenError`  _instance-attribute_

## _OrganizationsResponse

`fastmcp.cli.deploy.horizon_client._OrganizationsResponse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OrganizationsResponse(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (2)**

- `items: tuple[HorizonOrganization, ...]`  _instance-attribute_
- `meta: _PaginationMeta`  _instance-attribute_

## _PaginationMeta

`fastmcp.cli.deploy.horizon_client._PaginationMeta`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _PaginationMeta(_ResponseModel)
```

**Bases** `_ResponseModel`

**Declared members (2)**

- `limit: int`  _instance-attribute_
- `nextCursor: str | None`  _instance-attribute_

## _ResponseModel

`fastmcp.cli.deploy.horizon_client._ResponseModel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ResponseModel(BaseModel)
```

**Bases** `BaseModel`

## normalize_api_origin

Import as `fastmcp.cli.deploy.credentials.normalize_api_origin`  ·  defined at `fastmcp.cli.deploy.horizon_client.normalize_api_origin`

```python
def normalize_api_origin(value: str) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate and normalize a Horizon API origin.


