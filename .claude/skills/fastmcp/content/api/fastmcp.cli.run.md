# `fastmcp.cli.run`

Distribution: `fastmcp`

## LogLevelType

`fastmcp.cli.run.LogLevelType`

```python
LogLevelType = Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]'> ````

## TransportType

`fastmcp.cli.run.TransportType`

```python
TransportType = Literal['stdio', 'http', 'sse', 'streamable-http']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["stdio", "http", "sse", "streamable-http"]'> ````

## WATCHED_EXTENSIONS

`fastmcp.cli.run.WATCHED_EXTENSIONS`

```python
WATCHED_EXTENSIONS: set[str] = {'.py', '.js', '.ts', '.jsx', '.tsx', '.html', '.md', '.mdx', '.txt', '.xml', '.css', '.scss', '.sass', '.less', '.json', '.yaml', '.yml', '.toml', '.vue', '.svelte', '.graphql', '.gql', '.svg', '.png', '.jpg', '.jpeg', '.gif', '.ico', '.webp', '.mp3', '.mp4', '.wav', '.webm', '.woff', '.woff2', '.ttf', '.eot'}
```

## logger

`fastmcp.cli.run.logger`

```python
logger = get_logger('cli.run')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _terminate_process

`fastmcp.cli.run._terminate_process`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _terminate_process(process: asyncio.subprocess.Process) -> None
```

Terminate a subprocess and all its children.

Sends SIGTERM to the process group first for graceful shutdown,
then falls back to SIGKILL if the process doesn't exit in time.


## _watch_filter

`fastmcp.cli.run._watch_filter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _watch_filter(_change: Change, path: str) -> bool
```

Filter for files that should trigger reload.


## create_client_server

`fastmcp.cli.run.create_client_server`

```python
def create_client_server(url: str) -> Any
```

Create a FastMCP server from a client URL.

Args:
    url: The URL to connect to

Returns:
    A FastMCP server instance


## create_mcp_config_server

`fastmcp.cli.run.create_mcp_config_server`

```python
def create_mcp_config_server(mcp_config_path: Path) -> FastMCP[None]
```

Create a FastMCP server from a MCPConfig.


## is_url

`fastmcp.cli.run.is_url`

```python
def is_url(path: str) -> bool
```

Check if a string is a URL.


## load_mcp_server_config

`fastmcp.cli.run.load_mcp_server_config`

```python
def load_mcp_server_config(config_path: Path) -> MCPServerConfig
```

Load a FastMCP configuration from a fastmcp.json file.

Args:
    config_path: Path to fastmcp.json file

Returns:
    MCPServerConfig object


## run_command

`fastmcp.cli.run.run_command`

```python
async def run_command(server_spec: str, transport: TransportType | None = None, host: str | None = None, port: int | None = None, path: str | None = None, log_level: LogLevelType | None = None, server_args: list[str] | None = None, show_banner: bool = True, use_direct_import: bool = False, skip_source: bool = False, stateless: bool = False) -> None
```

Run a MCP server or connect to a remote one.

Args:
    server_spec: Python file, object specification (file:obj), config file, or URL
    transport: Transport protocol to use
    host: Host to bind to when using http transport
    port: Port to bind to when using http transport
    path: Path to bind to when using http transport
    log_level: Log level
    server_args: Additional arguments to pass to the server
    show_banner: Whether to show the server banner
    use_direct_import: Whether to use direct import instead of subprocess
    skip_source: Whether to skip source preparation step
    stateless: Whether to run in stateless mode (no session)


## run_module_command

`fastmcp.cli.run.run_module_command`

```python
def run_module_command(module_name: str, env_command_builder: Callable[[list[str]], list[str]] | None = None, extra_args: list[str] | None = None) -> None
```

Run a Python module directly using ``python -m <module>``.

When ``-m`` is used, the module manages its own server startup.
No server-object discovery or transport overrides are applied.

Args:
    module_name: Dotted module name (e.g. ``my_package``).
    env_command_builder: An optional callable that wraps a command list
        with environment setup (e.g. ``UVEnvironment.build_command``).
    extra_args: Extra arguments forwarded after the module name.


## run_v1_server_async

`fastmcp.cli.run.run_v1_server_async`

```python
async def run_v1_server_async(server: SDKServer, host: str | None = None, port: int | None = None, transport: TransportType | None = None) -> None
```

Run a FastMCP 1.x server using async methods.

Args:
    server: FastMCP 1.x server instance
    host: Host to bind to
    port: Port to bind to
    transport: Transport protocol to use


## run_with_reload

`fastmcp.cli.run.run_with_reload`

```python
async def run_with_reload(cmd: list[str], reload_dirs: list[Path] | None = None, is_stdio: bool = False) -> None
```

Run a command with file watching and auto-reload.

Args:
    cmd: Command to run as subprocess (should include --no-reload)
    reload_dirs: Directories to watch for changes (default: cwd)
    is_stdio: Whether this is stdio transport


