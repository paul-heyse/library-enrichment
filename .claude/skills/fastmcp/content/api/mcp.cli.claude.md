# `mcp.cli.claude`

Distribution: `mcp`

## logger

`mcp.cli.claude.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## get_claude_config_path

`mcp.cli.claude.get_claude_config_path`

```python
def get_claude_config_path() -> Path | None
```

Get the Claude config directory based on platform.


## get_uv_path

`mcp.cli.claude.get_uv_path`

```python
def get_uv_path() -> str
```

Get the full path to the uv executable.


## mcp_requirement

`mcp.cli.claude.mcp_requirement`

```python
def mcp_requirement(package: str = 'mcp') -> str
```

Requirement string pinning spawned environments to the running SDK version.

`uv run --with mcp` resolves the requirement in a fresh environment, where
an unpinned `mcp` means the latest stable release — not necessarily the
version the user installed (pre-releases in particular are never selected
without an explicit pin). Source builds carry dev/local version segments
that are not published to PyPI, so they fall back to the unpinned form,
as does a missing distribution (no metadata to pin from).


## update_claude_config

`mcp.cli.claude.update_claude_config`

```python
def update_claude_config(file_spec: str, server_name: str, with_editable: Path | None = None, with_packages: list[str] | None = None, env_vars: dict[str, str] | None = None) -> bool
```

Add or update an MCP server in Claude's configuration.

Args:
    file_spec: Path to the server file, optionally with :object suffix
    server_name: Name for the server in Claude's config
    with_editable: Optional directory to install in editable mode
    with_packages: Optional list of additional packages to install
    env_vars: Optional dictionary of environment variables. These are merged with
        any existing variables, with new values taking precedence.

Raises:
    RuntimeError: If Claude Desktop's config directory is not found, indicating
        Claude Desktop may not be installed or properly set up.


