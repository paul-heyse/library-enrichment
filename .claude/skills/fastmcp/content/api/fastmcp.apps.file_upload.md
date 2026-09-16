# `fastmcp.apps.file_upload`

Distribution: `fastmcp`

## _TEXT_EXTENSIONS

`fastmcp.apps.file_upload._TEXT_EXTENSIONS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TEXT_EXTENSIONS = frozenset(('.csv', '.json', '.txt', '.md', '.py', '.yaml', '.yml', '.toml'))
```

## FileUpload

`fastmcp.apps.file_upload.FileUpload`

```python
class FileUpload(FastMCPApp)
```

**Bases** `FastMCPApp`

**Declared members (3)**

- `def on_list(self, ctx: Context) -> list[dict[str, Any]]`
  List all stored files.
- `def on_read(self, name: str, ctx: Context) -> dict[str, Any]`
  Read a file's contents by name.
- `def on_store(self, files: list[dict[str, Any]], ctx: Context) -> list[dict[str, Any]]`
  Store uploaded files and return summaries.

**Inherited (22)**

- from `fastmcp.apps.app.FastMCPApp`: `add_tool`, `lifespan`, `name`, `run`, `tool`, `ui`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that adds file upload capabilities to a server.

Registers a drag-and-drop UI tool, a backend storage tool, and
model-visible tools for listing and reading uploaded files.

Files are scoped by MCP session and stored in memory by default.
Override ``on_store``, ``on_list``, and ``on_read`` for custom
persistence (filesystem, S3, database, etc.). Each method receives
the current ``Context``, giving access to session ID, auth tokens,
and request metadata for partitioning and authorization.

**Session scoping:** The default storage uses ``ctx.session_id`` to
isolate files by session. This works with stdio, SSE, and stateful
HTTP transports. In **stateless HTTP** mode, each request creates a
new session, so files won't persist across requests. For stateless
deployments, override the storage methods to partition by a stable
identifier from the auth context::

    class UserScopedUpload(FileUpload):
        def on_store(self, files, ctx):
            user_id = ctx.access_token["sub"]
            ...

Example::

    from fastmcp import FastMCP
    from fastmcp.apps.file_upload import FileUpload

    mcp = FastMCP("My Server")
    mcp.add_provider(FileUpload())


## _b64_decoded_size

`fastmcp.apps.file_upload._b64_decoded_size`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _b64_decoded_size(b64: str) -> int
```

Return the exact decoded byte-length of a base64 string without decoding it.


## _format_size

`fastmcp.apps.file_upload._format_size`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _format_size(size: int) -> str
```

## _make_summary

`fastmcp.apps.file_upload._make_summary`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_summary(entry: dict[str, Any]) -> dict[str, Any]
```

