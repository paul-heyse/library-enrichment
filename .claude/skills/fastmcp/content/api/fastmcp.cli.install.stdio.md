# `fastmcp.cli.install.stdio`

Distribution: `fastmcp`

## logger

`fastmcp.cli.install.stdio.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## install_stdio

`fastmcp.cli.install.stdio.install_stdio`

```python
def install_stdio(file: Path, server_object: str | None, with_editable: list[Path] | None = None, with_packages: list[str] | None = None, copy: bool = False, python_version: str | None = None, with_requirements: Path | None = None, project: Path | None = None) -> bool
```

Generate the stdio command for running a FastMCP server.

Args:
    file: Path to the server file
    server_object: Optional server object name (for :object suffix)
    with_editable: Optional list of directories to install in editable mode
    with_packages: Optional list of additional packages to install
    copy: If True, copy to clipboard instead of printing to stdout
    python_version: Optional Python version to use
    with_requirements: Optional requirements file to install from
    project: Optional project directory to run within

Returns:
    True if generation was successful, False otherwise


## stdio_command

Import as `fastmcp.cli.install.stdio_command`  ·  defined at `fastmcp.cli.install.stdio.stdio_command`

```python
async def stdio_command(server_spec: str, server_name: Annotated[str | None, cyclopts.Parameter(name=[--name, -n], help='Custom name for the server (used for dependency resolution)')] = None, with_editable: Annotated[list[Path] | None, cyclopts.Parameter('--with-editable', help='Directory with pyproject.toml to install in editable mode (can be used multiple times)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, copy: Annotated[bool, cyclopts.Parameter(--copy, help='Copy command to clipboard instead of printing to stdout')] = False, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Generate the stdio command for running a FastMCP server.

Outputs the shell command that an MCP host would use to start this server
over stdio transport. Useful for manual configuration or debugging.

Args:
    server_spec: Python file to run, optionally with :object suffix


