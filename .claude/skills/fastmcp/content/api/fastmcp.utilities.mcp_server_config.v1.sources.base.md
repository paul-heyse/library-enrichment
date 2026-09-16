# `fastmcp.utilities.mcp_server_config.v1.sources.base`

Distribution: `fastmcp`

## Source

Import as `fastmcp.utilities.mcp_server_config.Source`  ·  defined at `fastmcp.utilities.mcp_server_config.v1.sources.base.Source`

```python
class Source(BaseModel, ABC)
```

**Also exported as** `fastmcp.utilities.mcp_server_config.Source`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`, `ABC`

**Declared members (3)**

- `async def load_server(self) -> Any`  _abstractmethod, async_
  Load and return the FastMCP server instance.
- `async def prepare(self) -> None`  _async_
  Prepare the source (download, clone, install, etc).
- `type: str = Field(description='Source type identifier')`  _class-attribute, instance-attribute_

Abstract base class for all source types.


