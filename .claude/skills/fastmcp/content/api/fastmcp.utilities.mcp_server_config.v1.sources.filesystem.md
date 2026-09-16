# `fastmcp.utilities.mcp_server_config.v1.sources.filesystem`

Distribution: `fastmcp`

## logger

`fastmcp.utilities.mcp_server_config.v1.sources.filesystem.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## FileSystemSource

Import as `fastmcp.utilities.mcp_server_config.FileSystemSource`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.sources.filesystem.FileSystemSource`

```python
class FileSystemSource(Source)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.FileSystemSource`

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Source`

**Declared members (5)**

- `entrypoint: str | None = Field(default=None, description='Name of server instance or factory function (a no-arg function that returns a FastMCP server)')`  _class-attribute, instance-attribute_
- `async def load_server(self) -> Any`  _async_
  Load server from filesystem.
- `def parse_path_with_object(cls, v: str) -> str`  _classmethod_
  Parse path:object syntax and extract the object name.
- `path: str = Field(description='Path to Python file containing the server')`  _class-attribute, instance-attribute_
- `type: Literal['filesystem'] = 'filesystem'`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `fastmcp.utilities.mcp_server_config.v1.sources.base.Source`: `prepare`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Source for local Python files.


