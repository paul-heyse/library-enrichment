# `fastmcp.cli.install.cursor`

Distribution: `fastmcp`

## logger

`fastmcp.cli.install.cursor.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## cursor_command

Import as `fastmcp.cli.install.cursor_command`  ·  defined at `fastmcp.cli.install.cursor.cursor_command`

```python
async def cursor_command(server_spec: str, server_name: Annotated[str | None, cyclopts.Parameter(name=[--name, -n], help='Custom name for the server in Cursor')] = None, with_editable: Annotated[list[Path] | None, cyclopts.Parameter('--with-editable', help='Directory with pyproject.toml to install in editable mode (can be used multiple times)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, env_vars: Annotated[list[str] | None, cyclopts.Parameter(--env, help='Environment variables in KEY=VALUE format (can be used multiple times)')] = None, env_file: Annotated[Path | None, cyclopts.Parameter(--env - file, help='Load environment variables from .env file')] = None, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None, workspace: Annotated[Path | None, cyclopts.Parameter(--workspace, help='Install to workspace directory (will create .cursor/ inside it) instead of using deeplink')] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Install an MCP server in Cursor.

Args:
    server_spec: Python file to install, optionally with :object suffix


## generate_cursor_deeplink

`fastmcp.cli.install.cursor.generate_cursor_deeplink`

```python
def generate_cursor_deeplink(server_name: str, server_config: StdioMCPServer) -> str
```

Generate a Cursor deeplink for installing the MCP server.

Args:
    server_name: Name of the server
    server_config: Server configuration

Returns:
    Deeplink URL that can be clicked to install the server


## install_cursor

`fastmcp.cli.install.cursor.install_cursor`

```python
def install_cursor(file: Path, server_object: str | None, name: str, with_editable: list[Path] | None = None, with_packages: list[str] | None = None, env_vars: dict[str, str] | None = None, python_version: str | None = None, with_requirements: Path | None = None, project: Path | None = None, workspace: Path | None = None) -> bool
```

Install FastMCP server in Cursor.

Args:
    file: Path to the server file
    server_object: Optional server object name (for :object suffix)
    name: Name for the server in Cursor
    with_editable: Optional list of directories to install in editable mode
    with_packages: Optional list of additional packages to install
    env_vars: Optional dictionary of environment variables
    python_version: Optional Python version to use
    with_requirements: Optional requirements file to install from
    project: Optional project directory to run within
    workspace: Optional workspace directory for project-specific installation

Returns:
    True if installation was successful, False otherwise


## install_cursor_workspace

`fastmcp.cli.install.cursor.install_cursor_workspace`

```python
def install_cursor_workspace(file: Path, server_object: str | None, name: str, workspace_path: Path, with_editable: list[Path] | None = None, with_packages: list[str] | None = None, env_vars: dict[str, str] | None = None, python_version: str | None = None, with_requirements: Path | None = None, project: Path | None = None) -> bool
```

Install FastMCP server to workspace-specific Cursor configuration.

Args:
    file: Path to the server file
    server_object: Optional server object name (for :object suffix)
    name: Name for the server in Cursor
    workspace_path: Path to the workspace directory
    with_editable: Optional list of directories to install in editable mode
    with_packages: Optional list of additional packages to install
    env_vars: Optional dictionary of environment variables
    python_version: Optional Python version to use
    with_requirements: Optional requirements file to install from
    project: Optional project directory to run within

Returns:
    True if installation was successful, False otherwise


## open_deeplink

`fastmcp.cli.install.cursor.open_deeplink`

```python
def open_deeplink(deeplink: str) -> bool
```

Attempt to open a Cursor deeplink URL using the system's default handler.

Args:
    deeplink: The deeplink URL to open

Returns:
    True if the command succeeded, False otherwise


