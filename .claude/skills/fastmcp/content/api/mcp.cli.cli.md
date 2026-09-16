# `mcp.cli.cli`

Distribution: `mcp`

## app

Import as `mcp.cli.app`  ·  defined at `mcp.cli.cli.app`

```python
app = typer.Typer(name='mcp', help='MCP development tools', add_completion=False, no_args_is_help=True)
```

**Inferred type** (`ty`, not declared in the source): `Typer`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`mcp.cli.cli.logger`

```python
logger = get_logger('cli')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _build_uv_command

`mcp.cli.cli._build_uv_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_uv_command(file_spec: str, with_editable: Path | None = None, with_packages: list[str] | None = None) -> list[str]
```

Build the uv run command that runs an MCP server through mcp run.


## _get_npx_command

`mcp.cli.cli._get_npx_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_npx_command()
```

Get the correct npx command for the current platform.


## _import_server

`mcp.cli.cli._import_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _import_server(file: Path, server_object: str | None = None)
```

Import an MCP server from a file.

Args:
    file: Path to the file
    server_object: Optional object name in format "module:object" or just "object"

Returns:
    The server object


## _parse_env_var

`mcp.cli.cli._parse_env_var`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_env_var(env_var: str) -> tuple[str, str]
```

Parse environment variable string in format KEY=VALUE.


## _parse_file_path

`mcp.cli.cli._parse_file_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_file_path(file_spec: str) -> tuple[Path, str | None]
```

Parse a file path that may include a server object specification.

Args:
    file_spec: Path to file, optionally with :object suffix

Returns:
    Tuple of (file_path, server_object)


## dev

`mcp.cli.cli.dev`

```python
def dev(file_spec: str = typer.Argument(..., help='Python file to run, optionally with :object suffix'), with_editable: Annotated[Path | None, typer.Option('--with-editable', -e, help='Directory containing pyproject.toml to install in editable mode', exists=True, file_okay=False, resolve_path=True)] = None, with_packages: Annotated[list[str], typer.Option('--with', help='Additional packages to install')] = []) -> None
```

Run an MCP server with the MCP Inspector.


## install

`mcp.cli.cli.install`

```python
def install(file_spec: str = typer.Argument(..., help='Python file to run, optionally with :object suffix'), server_name: Annotated[str | None, typer.Option(--name, -n, help="Custom name for the server (defaults to server's name attribute or file name)")] = None, with_editable: Annotated[Path | None, typer.Option('--with-editable', -e, help='Directory containing pyproject.toml to install in editable mode', exists=True, file_okay=False, resolve_path=True)] = None, with_packages: Annotated[list[str], typer.Option('--with', help='Additional packages to install')] = [], env_vars: Annotated[list[str], typer.Option(--env - var, -v, help='Environment variables in KEY=VALUE format')] = [], env_file: Annotated[Path | None, typer.Option(--env - file, -f, help='Load environment variables from a .env file', exists=True, file_okay=True, dir_okay=False, resolve_path=True)] = None) -> None
```

Install an MCP server in the Claude desktop app.

Environment variables are preserved once added and only updated if new values
are explicitly provided.


## run

`mcp.cli.cli.run`

```python
def run(file_spec: str = typer.Argument(..., help='Python file to run, optionally with :object suffix'), transport: Annotated[str | None, typer.Option(--transport, -t, help='Transport protocol to use (stdio, sse, or streamable-http)')] = None) -> None
```

Run an MCP server.

The server can be specified in two ways:
1. Module approach: server.py - runs the module directly, expecting a server.run() call.
2. Import approach: server.py:app - imports and runs the specified server object.

Note: This command runs the server directly. You are responsible for ensuring
all dependencies are available.
For dependency management, use `mcp install` or `mcp dev` instead.


## version

`mcp.cli.cli.version`

```python
def version() -> None
```

Show the MCP version.


