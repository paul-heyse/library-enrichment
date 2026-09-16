# `fastmcp.cli.install.mcp_json`

Distribution: `fastmcp`

## logger

`fastmcp.cli.install.mcp_json.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## install_mcp_json

`fastmcp.cli.install.mcp_json.install_mcp_json`

```python
def install_mcp_json(file: Path, server_object: str | None, name: str, with_editable: list[Path] | None = None, with_packages: list[str] | None = None, env_vars: dict[str, str] | None = None, copy: bool = False, python_version: str | None = None, with_requirements: Path | None = None, project: Path | None = None) -> bool
```

Generate MCP configuration JSON for manual installation.

Args:
    file: Path to the server file
    server_object: Optional server object name (for :object suffix)
    name: Name for the server in MCP config
    with_editable: Optional list of directories to install in editable mode
    with_packages: Optional list of additional packages to install
    env_vars: Optional dictionary of environment variables
    copy: If True, copy to clipboard instead of printing to stdout
    python_version: Optional Python version to use
    with_requirements: Optional requirements file to install from
    project: Optional project directory to run within

Returns:
    True if generation was successful, False otherwise


## mcp_json_command

Import as `fastmcp.cli.install.mcp_json_command`  ·  defined at `fastmcp.cli.install.mcp_json.mcp_json_command`

```python
async def mcp_json_command(server_spec: str, server_name: Annotated[str | None, cyclopts.Parameter(name=[--name, -n], help='Custom name for the server in MCP config')] = None, with_editable: Annotated[list[Path] | None, cyclopts.Parameter('--with-editable', help='Directory with pyproject.toml to install in editable mode (can be used multiple times)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, env_vars: Annotated[list[str] | None, cyclopts.Parameter(--env, help='Environment variables in KEY=VALUE format (can be used multiple times)')] = None, env_file: Annotated[Path | None, cyclopts.Parameter(--env - file, help='Load environment variables from .env file')] = None, copy: Annotated[bool, cyclopts.Parameter(--copy, help='Copy configuration to clipboard instead of printing to stdout')] = False, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Generate MCP configuration JSON for manual installation.

Args:
    server_spec: Python file to install, optionally with :object suffix


