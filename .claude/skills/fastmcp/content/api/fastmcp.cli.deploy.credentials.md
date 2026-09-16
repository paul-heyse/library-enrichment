# `fastmcp.cli.deploy.credentials`

Distribution: `fastmcp`

## CredentialSource

`fastmcp.cli.deploy.credentials.CredentialSource`

```python
CredentialSource = Literal['environment', 'stored', 'interactive']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["environment", "stored", "interactive"]'> ````

## AuthState

`fastmcp.cli.deploy.credentials.AuthState`

```python
class AuthState(BaseModel)
```

**Bases** `BaseModel`

**Declared members (3)**

- `api_key: SecretStr = Field(alias='apiKey')`  _class-attribute, instance-attribute_
- `def require_nonempty_api_key(cls, value: SecretStr) -> SecretStr`  _classmethod_
- `schema_version: Literal[1] = Field(alias='schemaVersion')`  _class-attribute, instance-attribute_

## AuthenticationRequiredError

Import as `fastmcp.cli.deploy.command.AuthenticationRequiredError`  ·  defined at `fastmcp.cli.deploy.credentials.AuthenticationRequiredError`

```python
class AuthenticationRequiredError(RuntimeError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RuntimeError`

No Horizon credential is available without interactive authorization.


## CredentialStore

Import as `fastmcp.cli.deploy.command.CredentialStore`  ·  defined at `fastmcp.cli.deploy.credentials.CredentialStore`

```python
class CredentialStore
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def clear(self) -> None`
- `def clear_if_matches(self, api_key: SecretStr | str, expected_api_origin: str) -> None`
  Clear a key only while its Horizon origin and value are active.
- `def load(self) -> SecretStr | None`
- `path = state_directory / 'auth.json'`  _instance-attribute_
- `def save(self, api_key: SecretStr | str) -> None`
- `def save_for_origin(self, api_key: SecretStr | str, expected_api_origin: str) -> None`
  Save a key only while its issuing Horizon origin is active.

Persist the active personal Horizon API key.


## ResolvedCredential

Import as `fastmcp.cli.deploy.command.ResolvedCredential`  ·  defined at `fastmcp.cli.deploy.credentials.ResolvedCredential`

```python
class ResolvedCredential
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `api_key: SecretStr`  _instance-attribute_
- `source: CredentialSource`  _instance-attribute_

## resolve_credential

`fastmcp.cli.deploy.credentials.resolve_credential`

```python
async def resolve_credential(store: CredentialStore, environ: Mapping[str, str] | None = None, authorize: Callable[[], Awaitable[SecretStr]] | None = None, expected_api_origin: str | None = None) -> ResolvedCredential
```

Resolve environment, stored, then interactive credentials.


## revoke_and_clear_credential

`fastmcp.cli.deploy.credentials.revoke_and_clear_credential`

```python
async def revoke_and_clear_credential(client: HorizonClient, store: CredentialStore) -> None
```

Attempt remote revocation and always remove the stored credential.


