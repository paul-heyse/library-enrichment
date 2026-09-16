# `fastmcp.server.providers.filesystem`

Distribution: `fastmcp`

## logger

`fastmcp.server.providers.filesystem.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FileSystemProvider

Import as `fastmcp.server.providers.FileSystemProvider`  ·  defined at `fastmcp.server.providers.filesystem.FileSystemProvider`

```python
class FileSystemProvider(LocalProvider)
```

**Also exported as** `fastmcp.server.providers.FileSystemProvider`

**Bases** `LocalProvider`

**Inherited (28)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `get_tool_by_hash`, `lifespan`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`
- from `fastmcp.server.providers.local_provider.decorators.prompts.PromptDecoratorMixin`: `add_prompt`, `prompt`
- from `fastmcp.server.providers.local_provider.decorators.resources.ResourceDecoratorMixin`: `add_resource`, `add_template`, `resource`
- from `fastmcp.server.providers.local_provider.decorators.tools.ToolDecoratorMixin`: `add_tool`, `tool`
- from `fastmcp.server.providers.local_provider.local_provider.LocalProvider`: `get_tasks`, `remove_prompt`, `remove_resource`, `remove_template`, `remove_tool`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that discovers components from the filesystem.

Scans a directory for Python files and registers any Tool, Resource,
ResourceTemplate, or Prompt objects found. Components are created using
the standalone decorators:
- @tool from fastmcp.tools
- @resource from fastmcp.resources
- @prompt from fastmcp.prompts

Args:
    root: Root directory to scan. Defaults to current directory.
    reload: If True, re-scan files on every request (dev mode).
        Defaults to False (scan once at init, cache results).

Example:
    ```python
    # In mcp/tools.py
    from fastmcp.tools import tool

    @tool
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    # In main.py
    from pathlib import Path

    from fastmcp import FastMCP
    from fastmcp.server.providers import FileSystemProvider

    # Path relative to this file
    mcp = FastMCP("MyServer", providers=[FileSystemProvider(Path(__file__).parent / "mcp")])

    # Dev mode - re-scan on every request
    mcp = FastMCP("MyServer", providers=[FileSystemProvider(Path(__file__).parent / "mcp", reload=True)])
    ```


