# `mcp.server.mcpserver.resources.base`

Distribution: `mcp`

## Resource

Import as `mcp.server.mcpserver.resources.Resource`  ·  defined at `mcp.server.mcpserver.resources.base.Resource`

```python
class Resource(BaseModel, abc.ABC)
```

**Also exported as** `mcp.server.mcpserver.resources.Resource`

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`, `abc.ABC`

**Declared members (10)**

- `annotations: Annotations | None = Field(default=None, description='Optional annotations for the resource')`  _class-attribute, instance-attribute_
- `description: str | None = Field(description='Description of the resource', default=None)`  _class-attribute, instance-attribute_
- `icons: list[Icon] | None = Field(default=None, description='Optional list of icons for this resource')`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = Field(default=None, description='Optional metadata for this resource')`  _class-attribute, instance-attribute_
- `mime_type: str = Field(default='text/plain', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `name: str | None = Field(description='Name of the resource', default=None)`  _class-attribute, instance-attribute_
- `async def read(self) -> str | bytes`  _abstractmethod, async_
  Read the resource content.
- `def set_default_name(cls, name: str | None, info: ValidationInfo) -> str`  _classmethod_
  Set default name from URI if not provided.
- `title: str | None = Field(description='Human-readable title of the resource', default=None)`  _class-attribute, instance-attribute_
- `uri: str = Field(default=..., description='URI of the resource')`  _class-attribute, instance-attribute_

Base class for all resources.


