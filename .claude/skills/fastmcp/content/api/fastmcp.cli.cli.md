# `fastmcp.cli.cli`

Distribution: `fastmcp`

## app

Import as `fastmcp.cli.app`  ·  defined at `fastmcp.cli.cli.app`

```python
app = cyclopts.App(name='fastmcp', help='FastMCP - The fast, Pythonic way to build MCP servers and clients.', version=fastmcp.__version__, default_parameter=Parameter(negative=()))
```

**Inferred type** (`ty`, not declared in the source): `App`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## console

`fastmcp.cli.cli.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## dev_app

`fastmcp.cli.cli.dev_app`

```python
dev_app = cyclopts.App(name='dev', help='Development tools for MCP servers')
```

**Inferred type** (`ty`, not declared in the source): `App`

## logger

`fastmcp.cli.cli.logger`

```python
logger = get_logger('cli')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## project_app

`fastmcp.cli.cli.project_app`

```python
project_app = cyclopts.App(name='project', help='Manage FastMCP projects')
```

**Inferred type** (`ty`, not declared in the source): `App`

## _get_npx_command

`fastmcp.cli.cli._get_npx_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_npx_command()
```

Get the correct npx command for the current platform.


## _parse_env_var

`fastmcp.cli.cli._parse_env_var`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_env_var(env_var: str) -> tuple[str, str]
```

Parse environment variable string in format KEY=VALUE.


## apps

`fastmcp.cli.cli.apps`

```python
async def apps(server_spec: str, mcp_port: Annotated[int, cyclopts.Parameter(--mcp - port, help="Port for the user's MCP server")] = 8000, dev_port: Annotated[int, cyclopts.Parameter(--dev - port, help='Port for the FastMCP dev UI')] = 8080, reload: Annotated[bool, cyclopts.Parameter(--reload, negative=--no - reload, help='Auto-reload the MCP server on file changes')] = True, host: Annotated[str, cyclopts.Parameter(--host, help='Host to bind to')] = '127.0.0.1', log_panel: Annotated[bool, cyclopts.Parameter(--log - panel, negative=--no - log - panel, help='Log panel feature in FastMCP dev UI')] = True) -> None
```

Preview a FastMCPApp UI in the browser.

Starts the MCP server from SERVER_SPEC on --mcp-port, launches a local
dev UI on --dev-port with a tool picker and AppBridge host, then opens
the browser automatically.

Requires fastmcp[apps] to be installed (prefab-ui).


## inspect

`fastmcp.cli.cli.inspect`

```python
async def inspect(server_spec: str | None = None, format: Annotated[InspectFormat | None, cyclopts.Parameter(name=[--format, -f], help='Output format: fastmcp (FastMCP-specific) or mcp (MCP protocol). Required when using -o.')] = None, output: Annotated[Path | None, cyclopts.Parameter(name=[--output, -o], help='Output file path for the JSON report. If not specified, outputs to stdout when format is provided.')] = None, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, skip_env: Annotated[bool, cyclopts.Parameter(--skip - env, help='Skip environment configuration (for internal use when already in a uv environment)')] = False) -> None
```

Inspect an MCP server and display information or generate a JSON report.

This command analyzes an MCP server. Without flags, it displays a text summary.
Use --format to output complete JSON data.

Examples:
    # Show text summary
    fastmcp inspect server.py

    # Output FastMCP format JSON to stdout
    fastmcp inspect server.py --format fastmcp

    # Save MCP protocol format to file (format required with -o)
    fastmcp inspect server.py --format mcp -o manifest.json

    # Inspect from fastmcp.json configuration
    fastmcp inspect fastmcp.json
    fastmcp inspect  # auto-detect fastmcp.json

Args:
    server_spec: Python file to inspect, optionally with :object suffix, or fastmcp.json


## inspector

`fastmcp.cli.cli.inspector`

```python
async def inspector(server_spec: str | None = None, with_editable: Annotated[list[Path] | None, cyclopts.Parameter('--with-editable', help='Directory containing pyproject.toml to install in editable mode (can be used multiple times)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, inspector_version: Annotated[str | None, cyclopts.Parameter(--inspector - version, help='Version of the MCP Inspector to use')] = None, ui_port: Annotated[int | None, cyclopts.Parameter(--ui - port, help='Port for the MCP Inspector UI')] = None, server_port: Annotated[int | None, cyclopts.Parameter(--server - port, help='Port for the MCP Inspector Proxy server')] = None, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None, reload: Annotated[bool, cyclopts.Parameter(--reload, help='Enable auto-reload on file changes (enabled by default)', negative=--no - reload)] = True, reload_dir: Annotated[list[Path] | None, cyclopts.Parameter(--reload - dir, help='Directories to watch for changes (default: current directory)')] = None, module: Annotated[bool, cyclopts.Parameter(name=[--module, -m], help='Run a Python module (python -m <module>) instead of importing a server object')] = False) -> None
```

Run an MCP server with the MCP Inspector for development.

Args:
    server_spec: Python file to run, optionally with :object suffix, or None to auto-detect fastmcp.json


## prepare

`fastmcp.cli.cli.prepare`

```python
async def prepare(config_path: Annotated[str | None, cyclopts.Parameter(help='Path to fastmcp.json configuration file')] = None, output_dir: Annotated[str | None, cyclopts.Parameter(help='Directory to create the persistent environment in')] = None, skip_source: Annotated[bool, cyclopts.Parameter(help='Skip source preparation (e.g., git clone)')] = False) -> None
```

Prepare a FastMCP project by creating a persistent uv environment.

This command creates a persistent uv project with all dependencies installed:
- Creates a pyproject.toml with dependencies from the config
- Installs all Python packages into a .venv
- Prepares the source (git clone, download, etc.) unless --skip-source

After running this command, you can use:
fastmcp run <config> --project <output-dir>

This is useful for:
- CI/CD pipelines with separate build and run stages
- Docker images where you prepare during build
- Production deployments where you want fast startup times

Example:
    fastmcp project prepare myserver.json --output-dir ./prepared-env
    fastmcp run myserver.json --project ./prepared-env


## run

`fastmcp.cli.cli.run`

```python
async def run(server_spec: str | None = None, server_args: str = (), transport: Annotated[run_module.TransportType | None, cyclopts.Parameter(name=[--transport, -t], help='Transport protocol to use')] = None, host: Annotated[str | None, cyclopts.Parameter(--host, help='Host to bind to when using http transport (default: 127.0.0.1)')] = None, port: Annotated[int | None, cyclopts.Parameter(name=[--port, -p], help='Port to bind to when using http transport (default: 8000)')] = None, path: Annotated[str | None, cyclopts.Parameter(--path, help='The route path for the server (default: /mcp for http transport, /sse for sse transport)')] = None, log_level: Annotated[Literal['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL'] | None, cyclopts.Parameter(name=[--log - level, -l], help='Log level')] = None, no_banner: Annotated[bool, cyclopts.Parameter(--no - banner, help="Don't show the server banner")] = False, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, skip_source: Annotated[bool, cyclopts.Parameter(--skip - source, help='Skip source preparation step (use when source is already prepared)')] = False, skip_env: Annotated[bool, cyclopts.Parameter(--skip - env, help='Skip environment configuration (for internal use when already in a uv environment)')] = False, reload: Annotated[bool, cyclopts.Parameter(--reload, negative=--no - reload, help='Enable auto-reload on file changes (development mode)')] = False, reload_dir: Annotated[list[Path] | None, cyclopts.Parameter(--reload - dir, help='Directories to watch for changes (default: current directory)')] = None, stateless: Annotated[bool, cyclopts.Parameter(--stateless, help='Run in stateless mode (no session, used internally for reload)')] = False, module: Annotated[bool, cyclopts.Parameter(name=[--module, -m], help='Run a Python module (python -m <module>) instead of importing a server object')] = False) -> None
```

Run an MCP server or connect to a remote one.

The server can be specified in several ways:
1. Module approach: "server.py" - runs the module directly, looking for an object named 'mcp', 'server', or 'app'
2. Import approach: "server.py:app" - imports and runs the specified server object
3. URL approach: "http://server-url" - connects to a remote server and creates a proxy
4. MCPConfig file: "mcp.json" - runs as a proxy server for the MCP Servers in the MCPConfig file
5. FastMCP config: "fastmcp.json" - runs server using FastMCP configuration
6. No argument: looks for fastmcp.json in current directory
7. Module mode: "-m my_module" - runs the module directly via python -m

Server arguments can be passed after -- :
fastmcp run server.py -- --config config.json --debug

Args:
    server_spec: Python file, object specification (file:obj), config file, URL, or None to auto-detect


## version

`fastmcp.cli.cli.version`

```python
def version(copy: Annotated[bool, cyclopts.Parameter(--copy, help='Copy version information to clipboard')] = False)
```

Display version information and platform details.


## with_argv

`fastmcp.cli.cli.with_argv`

```python
def with_argv(args: list[str] | None)
```

Temporarily replace sys.argv if args provided.

This context manager is used at the CLI boundary to inject
server arguments when needed, without mutating sys.argv deep
in the source loading logic.

Args are provided without the script name, so we preserve sys.argv[0]
and replace the rest.


