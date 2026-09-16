# `mcp.server.mcpserver.tools.base`

Distribution: `mcp`

## Tool

Import as `mcp.server.mcpserver.tools.Tool`  ·  defined at `mcp.server.mcpserver.tools.base.Tool`

```python
class Tool(BaseModel)
```

**Also exported as** `mcp.server.mcpserver.tools.Tool`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (16)**

- `annotations: ToolAnnotations | None = Field(None, description='Optional annotations for the tool')`  _class-attribute, instance-attribute_
- `context_kwarg: str | None = Field(None, description='Name of the kwarg that should receive context')`  _class-attribute, instance-attribute_
- `description: str = Field(description='Description of what the tool does')`  _class-attribute, instance-attribute_
- `fn: Callable[..., Any] = Field(exclude=True)`  _class-attribute, instance-attribute_
- `fn_metadata: FuncMetadata = Field(description='Metadata about the function including a pydantic model for tool arguments')`  _class-attribute, instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], name: str | None = None, title: str | None = None, description: str | None = None, context_kwarg: str | None = None, annotations: ToolAnnotations | None = None, icons: list[Icon] | None = None, meta: dict[str, Any] | None = None, structured_output: bool | None = None) -> Tool`  _classmethod_
  Create a Tool from a function.
- `icons: list[Icon] | None = Field(default=None, description='Optional list of icons for this tool')`  _class-attribute, instance-attribute_
- `is_async: bool = Field(description='Whether the tool is async')`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = Field(default=None, description='Optional metadata for this tool')`  _class-attribute, instance-attribute_
- `name: str = Field(description='Name of the tool')`  _class-attribute, instance-attribute_
- `output_schema: dict[str, Any] | None`  _cached, property_
- `parameters: dict[str, Any] = Field(description='JSON schema for tool parameters')`  _class-attribute, instance-attribute_
- `resolved_params: dict[str, Any] = Field(default_factory=lambda: {}, exclude=True, description='Parameters filled by resolvers, mapped to (Resolve, wants_union)')`  _class-attribute, instance-attribute_
- `resolver_plans: dict[Hashable, Any] = Field(default_factory=lambda: {}, exclude=True, description='Static per-resolver parameter plans')`  _class-attribute, instance-attribute_
- `async def run(self, arguments: dict[str, Any], context: Context[LifespanContextT, RequestT], convert_result: bool = False) -> Any`  _async_
  Run the tool with arguments.
- `title: str | None = Field(None, description='Human-readable title of the tool')`  _class-attribute, instance-attribute_

Internal tool registration info.


