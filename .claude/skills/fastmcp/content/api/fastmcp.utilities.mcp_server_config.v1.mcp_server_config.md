# `fastmcp.utilities.mcp_server_config.v1.mcp_server_config`

Distribution: `fastmcp`

## EnvironmentType

`fastmcp.utilities.mcp_server_config.v1.mcp_server_config.EnvironmentType`

```python
EnvironmentType: TypeAlias = UVEnvironment
```

## FASTMCP_JSON_SCHEMA

`fastmcp.utilities.mcp_server_config.v1.mcp_server_config.FASTMCP_JSON_SCHEMA`

```python
FASTMCP_JSON_SCHEMA = 'https://gofastmcp.com/public/schemas/fastmcp.json/v1.json'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://gofastmcp.com/public/schemas/fastmcp.json/v1.json"]`

## SourceType

`fastmcp.utilities.mcp_server_config.v1.mcp_server_config.SourceType`

```python
SourceType: TypeAlias = FileSystemSource
```

## logger

`fastmcp.utilities.mcp_server_config.v1.mcp_server_config.logger`

```python
logger = get_logger('cli.config')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## Deployment

Import as `fastmcp.utilities.mcp_server_config.Deployment`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.mcp_server_config.Deployment`

```python
class Deployment(BaseModel)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.Deployment`

**Bases** `BaseModel`

**Declared members (9)**

- `def apply_runtime_settings(self, config_path: Path | None = None) -> None`
  Apply runtime settings like environment variables and working directory.
- `args: list[str] | None = Field(default=None, description='Arguments to pass to the server (after --)', examples=[['--config', 'config.json', '--debug']])`  _class-attribute, instance-attribute_
- `cwd: str | None = Field(default=None, description='Working directory for the server process', examples=['.', './src', '/app'])`  _class-attribute, instance-attribute_
- `env: dict[str, str] | None = Field(default=None, description='Environment variables to set when running the server', examples=[{'API_KEY': 'secret', 'DEBUG': 'true'}])`  _class-attribute, instance-attribute_
- `host: str | None = Field(default=None, description='Host to bind to when using HTTP transport', examples=['127.0.0.1', '0.0.0.0', 'localhost'])`  _class-attribute, instance-attribute_
- `log_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None = Field(default=None, description='Log level for the server')`  _class-attribute, instance-attribute_
- `path: str | None = Field(default=None, description='URL path for the server endpoint', examples=['/mcp/', '/api/mcp/', '/sse/'])`  _class-attribute, instance-attribute_
- `port: int | None = Field(default=None, description='Port to bind to when using HTTP transport', examples=[8000, 3000, 5000])`  _class-attribute, instance-attribute_
- `transport: Literal['stdio', 'http', 'sse', 'streamable-http'] | None = Field(default=None, description='Transport protocol to use')`  _class-attribute, instance-attribute_

Configuration for server deployment and runtime settings.


## MCPServerConfig

Import as `fastmcp.utilities.mcp_server_config.MCPServerConfig`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.mcp_server_config.MCPServerConfig`

```python
class MCPServerConfig(BaseModel)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.MCPServerConfig`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (14)**

- `deployment: Deployment = Field(default_factory=lambda: Deployment(), description='Server deployment and runtime settings')`  _class-attribute, instance-attribute_
- `environment: EnvironmentType = Field(default_factory=lambda: UVEnvironment(), description='Python environment setup configuration')`  _class-attribute, instance-attribute_
- `def find_config(cls, start_path: Path | None = None) -> Path | None`  _classmethod_
  Find a fastmcp.json file in the specified directory.
- `def from_cli_args(cls, source: FileSystemSource, transport: Literal['stdio', 'http', 'sse', 'streamable-http'] | None = None, host: str | None = None, port: int | None = None, path: str | None = None, log_level: Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None = None, python: str | None = None, dependencies: list[str] | None = None, requirements: str | None = None, project: str | None = None, editable: str | None = None, env: dict[str, str] | None = None, cwd: str | None = None, args: list[str] | None = None) -> MCPServerConfig`  _classmethod_
  Create a config from CLI arguments.
- `def from_file(cls, file_path: Path) -> MCPServerConfig`  _classmethod_
  Load configuration from a JSON file.
- `async def prepare(self, skip_source: bool = False, output_dir: Path | None = None) -> None`  _async_
  Prepare environment and source for execution.
- `async def prepare_environment(self, output_dir: Path | None = None) -> None`  _async_
  Prepare the Python environment.
- `async def prepare_source(self) -> None`  _async_
  Prepare the source for loading.
- `async def run_server(self, kwargs: Any = {}) -> None`  _async_
  Load and run the server with this configuration.
- `schema_: str | None = Field(default='https://gofastmcp.com/public/schemas/fastmcp.json/v1.json', alias='$schema', description='JSON schema for IDE support and validation')`  _class-attribute, instance-attribute_
- `source: SourceType = Field(description='Source configuration for the server', examples=[{'path': 'server.py'}, {'path': 'server.py', 'entrypoint': 'app'}, {'type': 'filesystem', 'path': 'src/server.py', 'entrypoint': 'mcp'}])`  _class-attribute, instance-attribute_
- `def validate_deployment(cls, v: dict | Deployment) -> Deployment`  _classmethod_
  Validate and convert deployment to Deployment.
- `def validate_environment(cls, v: dict | Any) -> EnvironmentType`  _classmethod_
  Ensure environment has a type field for discrimination.
- `def validate_source(cls, v: dict | Source) -> SourceType`  _classmethod_
  Validate and convert source to proper format.

Configuration for a FastMCP server.

This configuration file allows you to specify all settings needed to run
a FastMCP server in a declarative format.


## generate_schema

Import as `fastmcp.utilities.mcp_server_config.generate_schema`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.mcp_server_config.generate_schema`

```python
def generate_schema(output_path: Path | str | None = None) -> dict[str, Any] | None
```

**Also exported as** `fastmcp.utilities.mcp_server_config.generate_schema`

Generate JSON schema for fastmcp.json files.

This is used to create the schema file that IDEs can use for
validation and auto-completion.

Args:
    output_path: Optional path to write the schema to. If provided,
                writes the schema and returns None. If not provided,
                returns the schema as a dictionary.

Returns:
    JSON schema as a dictionary if output_path is None, otherwise None


