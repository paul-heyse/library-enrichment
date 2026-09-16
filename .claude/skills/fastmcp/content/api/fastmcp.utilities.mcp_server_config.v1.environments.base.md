# `fastmcp.utilities.mcp_server_config.v1.environments.base`

Distribution: `fastmcp`

## Environment

Import as `fastmcp.utilities.mcp_server_config.Environment`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.environments.base.Environment`

```python
class Environment(BaseModel, ABC)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.Environment`, `fastmcp.utilities.mcp_server_config.v1.environments.Environment`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`, `ABC`

**Declared members (3)**

- `def build_command(self, command: list[str]) -> list[str]`  _abstractmethod_
  Build the full command with environment setup.
- `async def prepare(self, output_dir: Path | None = None) -> None`  _async_
  Prepare the environment (optional, can be no-op).
- `type: str = Field(description='Environment type identifier')`  _class-attribute, instance-attribute_

Base class for environment configuration.


