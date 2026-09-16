# `fastmcp.cli.install.goose`

Distribution: `fastmcp`

## logger

`fastmcp.cli.install.goose.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _build_uvx_command

`fastmcp.cli.install.goose._build_uvx_command`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_uvx_command(server_spec: str, python_version: str | None = None, with_packages: list[str] | None = None) -> list[str]
```

Build a uvx command for running a FastMCP server.

Goose requires uvx (not uv run) as the command. The uvx format is:
    uvx [--with pkg] [--python X] fastmcp run <spec>

uvx automatically infers that the `fastmcp` command comes from the
`fastmcp` package, so --from is not needed.


## _slugify

`fastmcp.cli.install.goose._slugify`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _slugify(name: str) -> str
```

Convert a display name to a URL-safe identifier.

Lowercases, replaces non-alphanumeric runs with hyphens,
and strips leading/trailing hyphens.


## generate_goose_deeplink

`fastmcp.cli.install.goose.generate_goose_deeplink`

```python
def generate_goose_deeplink(name: str, command: str, args: list[str], description: str = 'MCP server installed via FastMCP') -> str
```

Generate a Goose deeplink for installing an MCP extension.

Args:
    name: Human-readable display name for the extension.
    command: The executable command (e.g. "uv").
    args: Arguments to the command.
    description: Short description shown in Goose.

Returns:
    A goose://extension?... deeplink URL.


## goose_command

Import as `fastmcp.cli.install.goose_command`  ·  defined at `fastmcp.cli.install.goose.goose_command`

```python
async def goose_command(server_spec: str, server_name: Annotated[str | None, cyclopts.Parameter(name=[--name, -n], help='Custom name for the extension in Goose')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, env_vars: Annotated[list[str] | None, cyclopts.Parameter(--env, help='Environment variables in KEY=VALUE format (can be used multiple times)')] = None, env_file: Annotated[Path | None, cyclopts.Parameter(--env - file, help='Load environment variables from .env file')] = None, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Install an MCP server in Goose.

Uses uvx to run the server. Environment variables are not included
in the deeplink; use `fastmcp install mcp-json` to generate a full
config for manual installation.

Args:
    server_spec: Python file to install, optionally with :object suffix


## install_goose

`fastmcp.cli.install.goose.install_goose`

```python
def install_goose(file: Path, server_object: str | None, name: str, with_packages: list[str] | None = None, python_version: str | None = None) -> bool
```

Install FastMCP server in Goose via deeplink.

Args:
    file: Path to the server file.
    server_object: Optional server object name (for :object suffix).
    name: Name for the extension in Goose.
    with_packages: Optional list of additional packages to install.
    python_version: Optional Python version to use.

Returns:
    True if installation was successful, False otherwise.


