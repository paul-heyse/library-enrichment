# `fastmcp.cli.install.claude_code`

Distribution: `fastmcp`

## logger

`fastmcp.cli.install.claude_code.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## check_claude_code_available

`fastmcp.cli.install.claude_code.check_claude_code_available`

```python
def check_claude_code_available() -> bool
```

Check if Claude Code CLI is available.


## claude_code_command

Import as `fastmcp.cli.install.claude_code_command`  ·  defined at `fastmcp.cli.install.claude_code.claude_code_command`

```python
async def claude_code_command(server_spec: str, server_name: Annotated[str | None, cyclopts.Parameter(name=[--name, -n], help='Custom name for the server in Claude Code')] = None, with_editable: Annotated[list[Path] | None, cyclopts.Parameter('--with-editable', help='Directory with pyproject.toml to install in editable mode (can be used multiple times)')] = None, with_packages: Annotated[list[str] | None, cyclopts.Parameter('--with', help='Additional packages to install (can be used multiple times)')] = None, env_vars: Annotated[list[str] | None, cyclopts.Parameter(--env, help='Environment variables in KEY=VALUE format (can be used multiple times)')] = None, env_file: Annotated[Path | None, cyclopts.Parameter(--env - file, help='Load environment variables from .env file')] = None, python: Annotated[str | None, cyclopts.Parameter(--python, help='Python version to use (e.g., 3.10, 3.11)')] = None, with_requirements: Annotated[Path | None, cyclopts.Parameter('--with-requirements', help='Requirements file to install dependencies from')] = None, project: Annotated[Path | None, cyclopts.Parameter(--project, help='Run the command within the given project directory')] = None) -> None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Install an MCP server in Claude Code.

Args:
    server_spec: Python file to install, optionally with :object suffix


## find_claude_command

`fastmcp.cli.install.claude_code.find_claude_command`

```python
def find_claude_command() -> str | None
```

Find the Claude Code CLI command.

Checks common installation locations since 'claude' is often a shell alias
that doesn't work with subprocess calls.


## install_claude_code

`fastmcp.cli.install.claude_code.install_claude_code`

```python
def install_claude_code(file: Path, server_object: str | None, name: str, with_editable: list[Path] | None = None, with_packages: list[str] | None = None, env_vars: dict[str, str] | None = None, python_version: str | None = None, with_requirements: Path | None = None, project: Path | None = None) -> bool
```

Install FastMCP server in Claude Code.

Args:
    file: Path to the server file
    server_object: Optional server object name (for :object suffix)
    name: Name for the server in Claude Code
    with_editable: Optional list of directories to install in editable mode
    with_packages: Optional list of additional packages to install
    env_vars: Optional dictionary of environment variables
    python_version: Optional Python version to use
    with_requirements: Optional requirements file to install from
    project: Optional project directory to run within

Returns:
    True if installation was successful, False otherwise


