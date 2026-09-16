# `fastmcp.cli.deploy.configuration`

Distribution: `fastmcp`

## ConfigurationStore

Import as `fastmcp.cli.deploy.command.ConfigurationStore`  ·  defined at `fastmcp.cli.deploy.configuration.ConfigurationStore`

```python
class ConfigurationStore
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `def load(self) -> HorizonConfiguration`
- `path = state_directory / 'config.json'`  _instance-attribute_
- `def save(self, configuration: HorizonConfiguration) -> None`
- `def set_api_origin(self, api_origin: str, credentials: CredentialStore) -> HorizonConfiguration`
  Set the origin and clear credentials before an origin change.

Persist the Horizon API origin without organization state.


## HorizonConfiguration

Import as `fastmcp.cli.deploy.command.HorizonConfiguration`  ·  defined at `fastmcp.cli.deploy.configuration.HorizonConfiguration`

```python
class HorizonConfiguration(BaseModel)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (3)**

- `api_origin: str = Field(alias='apiOrigin')`  _class-attribute, instance-attribute_
- `schema_version: Literal[1] = Field(alias='schemaVersion')`  _class-attribute, instance-attribute_
- `def validate_api_origin(cls, value: str) -> str`  _classmethod_

