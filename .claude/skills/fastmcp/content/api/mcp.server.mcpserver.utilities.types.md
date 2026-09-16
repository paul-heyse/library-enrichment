# `mcp.server.mcpserver.utilities.types`

Distribution: `mcp`

## Audio

Import as `mcp.server.mcpserver.Audio`  ·  defined at `mcp.server.mcpserver.utilities.types.Audio`

```python
class Audio
```

**Also exported as** `mcp.server.mcpserver.Audio`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `data = data`  _instance-attribute_
- `path = Path(path) if path else None`  _instance-attribute_
- `def to_audio_content(self) -> AudioContent`
  Convert to MCP AudioContent.

Helper class for returning audio from tools.


## Image

Import as `mcp.server.mcpserver.Image`  ·  defined at `mcp.server.mcpserver.utilities.types.Image`

```python
class Image
```

**Also exported as** `mcp.server.mcpserver.Image`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `data = data`  _instance-attribute_
- `path = Path(path) if path else None`  _instance-attribute_
- `def to_image_content(self) -> ImageContent`
  Convert to MCP ImageContent.

Helper class for returning images from tools.


