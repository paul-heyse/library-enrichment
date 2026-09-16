# `fastmcp.utilities.mcp_server_config.v1.environments.uv`

Distribution: `fastmcp`

## logger

`fastmcp.utilities.mcp_server_config.v1.environments.uv.logger`

```python
logger = get_logger('cli.config')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## UVEnvironment

Import as `fastmcp.utilities.mcp_server_config.UVEnvironment`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.environments.uv.UVEnvironment`

```python
class UVEnvironment(Environment)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.UVEnvironment`, `fastmcp.utilities.mcp_server_config.v1.environments.UVEnvironment`

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Environment`

**Declared members (8)**

- `def build_command(self, command: list[str]) -> list[str]`
  Build complete uv run command with environment args and command to execute.
- `dependencies: list[str] | None = Field(default=None, description='Python packages to install with PEP 508 specifiers', examples=[['fastmcp>=2.0,<3', 'httpx', 'pandas>=2.0']])`  _class-attribute, instance-attribute_
- `editable: list[Path] | None = Field(default=None, description='Directories to install in editable mode', examples=[['.', '../my-package'], ['/path/to/package']])`  _class-attribute, instance-attribute_
- `async def prepare(self, output_dir: Path | None = None) -> None`  _async_
  Prepare the Python environment using uv.
- `project: Path | None = Field(default=None, description='Path to project directory containing pyproject.toml', examples=['.', '../my-project'])`  _class-attribute, instance-attribute_
- `python: str | None = Field(default=None, description='Python version constraint', examples=['3.10', '3.11', '3.12'])`  _class-attribute, instance-attribute_
- `requirements: Path | None = Field(default=None, description='Path to requirements.txt file', examples=['requirements.txt', '../requirements/prod.txt'])`  _class-attribute, instance-attribute_
- `type: Literal['uv'] = 'uv'`  _class-attribute, instance-attribute_

Configuration for Python environment setup.


