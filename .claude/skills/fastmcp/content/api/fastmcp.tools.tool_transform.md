# `fastmcp.tools.tool_transform`

Distribution: `fastmcp`

## _FRAMEWORK_META_NAMESPACES

`fastmcp.tools.tool_transform._FRAMEWORK_META_NAMESPACES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FRAMEWORK_META_NAMESPACES = ('fastmcp', 'ui')
```

## _current_tool

`fastmcp.tools.tool_transform._current_tool`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_current_tool: ContextVar[TransformedTool | None] = ContextVar('_current_tool', default=None)
```

## logger

`fastmcp.tools.tool_transform.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ArgTransform

Import as `fastmcp.tools.base.ArgTransform`  ·  defined at `fastmcp.tools.tool_transform.ArgTransform`

```python
class ArgTransform
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (8)**

- `default: Any | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `default_factory: Callable[[], Any] | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `description: str | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `examples: Any | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `hide: bool = False`  _class-attribute, instance-attribute_
- `name: str | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `required: Literal[True] | NotSetT = NotSet`  _class-attribute, instance-attribute_
- `type: Any | NotSetT = NotSet`  _class-attribute, instance-attribute_

Configuration for transforming a parent tool's argument.

This class allows fine-grained control over how individual arguments are transformed
when creating a new tool from an existing one. You can rename arguments, change their
descriptions, add default values, or hide them from clients while passing constants.

Attributes:
    name: New name for the argument. Use None to keep original name, or ... for no change.
    description: New description for the argument. Use None to remove description, or ... for no change.
    default: New default value for the argument. Use ... for no change.
    default_factory: Callable that returns a default value. Cannot be used with default.
    type: New type for the argument. Use ... for no change.
    hide: If True, hide this argument from clients but pass a constant value to parent.
    required: If True, make argument required (remove default). Use ... for no change.
    examples: Examples for the argument. Use ... for no change.

Examples:
    Rename argument 'old_name' to 'new_name'
    ```python
    ArgTransform(name="new_name")
    ```

    Change description only
    ```python
    ArgTransform(description="Updated description")
    ```

    Add a default value (makes argument optional)
    ```python
    ArgTransform(default=42)
    ```

    Add a default factory (makes argument optional)
    ```python
    ArgTransform(default_factory=lambda: time.time())
    ```

    Change the type
    ```python
    ArgTransform(type=str)
    ```

    Hide the argument entirely from clients
    ```python
    ArgTransform(hide=True)
    ```

    Hide argument but pass a constant value to parent
    ```python
    ArgTransform(hide=True, default="constant_value")
    ```

    Hide argument but pass a factory-generated value to parent
    ```python
    ArgTransform(hide=True, default_factory=lambda: uuid.uuid4().hex)
    ```

    Make an optional parameter required (removes any default)
    ```python
    ArgTransform(required=True)
    ```

    Combine multiple transformations
    ```python
    ArgTransform(name="new_name", description="New desc", default=None, type=int)
    ```


## ArgTransformConfig

`fastmcp.tools.tool_transform.ArgTransformConfig`

```python
class ArgTransformConfig(FastMCPBaseModel)
```

**Bases** `FastMCPBaseModel`

**Declared members (7)**

- `default: str | int | float | bool | None = Field(default=None, description='The new default value for the argument.')`  _class-attribute, instance-attribute_
- `description: str | None = Field(default=None, description='The new description for the argument.')`  _class-attribute, instance-attribute_
- `examples: Any | None = Field(default=None, description='Examples of the argument.')`  _class-attribute, instance-attribute_
- `hide: bool = Field(default=False, description='Whether to hide the argument from the tool.')`  _class-attribute, instance-attribute_
- `name: str | None = Field(default=None, description='The new name for the argument.')`  _class-attribute, instance-attribute_
- `required: Literal[True] | None = Field(default=None, description='Whether the argument is required.')`  _class-attribute, instance-attribute_
- `def to_arg_transform(self) -> ArgTransform`
  Convert the argument transform to a FastMCP argument transform.

A model for requesting a single argument transform.


## ToolTransformConfig

Import as `fastmcp.server.server.ToolTransformConfig`  ·  defined at `fastmcp.tools.tool_transform.ToolTransformConfig`

```python
class ToolTransformConfig(FastMCPBaseModel)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (9)**

- `def apply(self, tool: Tool) -> TransformedTool`
  Create a TransformedTool from a provided tool and this transformation configuration.
- `arguments: dict[str, ArgTransformConfig] = Field(default_factory=dict, description='A dictionary of argument transforms to apply to the tool.')`  _class-attribute, instance-attribute_
- `description: str | None = Field(default=None, description='The new description of the tool.')`  _class-attribute, instance-attribute_
- `enabled: bool = Field(default=True, description='Whether the tool is enabled. If False, the tool will be hidden from clients.')`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = Field(default=None, description='The new meta information for the tool.')`  _class-attribute, instance-attribute_
- `name: str | None = Field(default=None, description='The new name for the tool.')`  _class-attribute, instance-attribute_
- `tags: Annotated[set[str], BeforeValidator(_convert_set_default_none)] = Field(default_factory=set, description='The new tags for the tool.')`  _class-attribute, instance-attribute_
- `title: str | None = Field(default=None, description='The new title of the tool.')`  _class-attribute, instance-attribute_
- `version: str | None = Field(default=None, description='The new version for the tool.')`  _class-attribute, instance-attribute_

Provides a way to transform a tool.


## TransformedTool

Import as `fastmcp.tools.base.TransformedTool`  ·  defined at `fastmcp.tools.tool_transform.TransformedTool`

```python
class TransformedTool(Tool)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Tool`

**Declared members (6)**

- `fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `forwarding_fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `def from_tool(cls, tool: Tool | Callable[..., Any], name: str | None = None, version: str | NotSetT | None = NotSet, title: str | NotSetT | None = NotSet, description: str | NotSetT | None = NotSet, tags: set[str] | None = None, transform_fn: Callable[..., Any] | None = None, transform_args: dict[str, ArgTransform] | None = None, annotations: ToolAnnotations | NotSetT | None = NotSet, output_schema: dict[str, Any] | NotSetT | None = NotSet, meta: dict[str, Any] | NotSetT | None = NotSet) -> TransformedTool`  _classmethod_
  Create a transformed tool from a parent tool.
- `parent_tool: SkipJsonSchema[Tool]`  _instance-attribute_
- `async def run(self, arguments: dict[str, Any]) -> ToolResult`  _async_
  Run the tool with context set for forward() functions.
- `transform_args: dict[str, ArgTransform]`  _instance-attribute_

**Inherited (25)**

- from `fastmcp.tools.base.Tool`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `execution`, `from_function`, `get_span_attributes`, `output_schema`, `parameters`, `return_type`, `timeout`, `to_mcp_tool`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A tool that is transformed from another tool.

This class represents a tool that has been created by transforming another tool.
It supports argument renaming, schema modification, custom function injection,
structured output control, and provides context for the forward() and forward_raw() functions.

The transformation can be purely schema-based (argument renaming, dropping, etc.)
or can include a custom function that uses forward() to call the parent tool
with transformed arguments. Output schemas and structured outputs are automatically
inherited from the parent tool but can be overridden or disabled.

Attributes:
    parent_tool: The original tool that this tool was transformed from.
    fn: The function to execute when this tool is called (either the forwarding
        function for pure transformations or a custom user function).
    forwarding_fn: Internal function that handles argument transformation and
        validation when forward() is called from custom functions.


## _apply_meta_override

`fastmcp.tools.tool_transform._apply_meta_override`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _apply_meta_override(source_meta: dict[str, Any] | None, override: dict[str, Any] | None | NotSetT) -> dict[str, Any] | None
```

Apply a transform's ``meta=`` override, preserving framework namespaces.

An override replaces the caller-facing meta wholesale, which is what users
expect. Framework-owned namespaces are carried across regardless, since a
transform that renames a tool must not silently unwire it — values the
override supplies for those namespaces still win key by key.


## _set_visibility_metadata

`fastmcp.tools.tool_transform._set_visibility_metadata`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _set_visibility_metadata(tool: Tool, enabled: bool) -> None
```

Set visibility state in tool metadata.

This uses the same metadata format as the Visibility transform,
so tools marked here will be filtered by the standard visibility system.

Args:
    tool: Tool to mark.
    enabled: Whether the tool should be visible to clients.


## apply_transformations_to_tools

`fastmcp.tools.tool_transform.apply_transformations_to_tools`

```python
def apply_transformations_to_tools(tools: dict[str, Tool], transformations: dict[str, ToolTransformConfig]) -> dict[str, Tool]
```

Apply a list of transformations to a list of tools. Tools that do not have any transformations
are left unchanged.

Note: tools dict is keyed by prefixed key (e.g., "tool:my_tool"),
but transformations are keyed by tool name (e.g., "my_tool").


## forward

Import as `fastmcp.tools.forward`  ·  defined at `fastmcp.tools.tool_transform.forward`

```python
async def forward(kwargs: Any = {}) -> ToolResult
```

**Also exported as** `fastmcp.tools.forward`

Forward to parent tool with argument transformation applied.

This function can only be called from within a transformed tool's custom
function. It applies argument transformation (renaming, validation) before
calling the parent tool.

For example, if the parent tool has args `x` and `y`, but the transformed
tool has args `a` and `b`, and an `transform_args` was provided that maps `x` to
`a` and `y` to `b`, then `forward(a=1, b=2)` will call the parent tool with
`x=1` and `y=2`.

Args:
    **kwargs: Arguments to forward to the parent tool (using transformed names).

Returns:
    The ToolResult from the parent tool execution.

Raises:
    RuntimeError: If called outside a transformed tool context.
    TypeError: If provided arguments don't match the transformed schema.


## forward_raw

Import as `fastmcp.tools.forward_raw`  ·  defined at `fastmcp.tools.tool_transform.forward_raw`

```python
async def forward_raw(kwargs: Any = {}) -> ToolResult
```

**Also exported as** `fastmcp.tools.forward_raw`

Forward directly to parent tool without transformation.

This function bypasses all argument transformation and validation, calling the parent
tool directly with the provided arguments. Use this when you need to call the parent
with its original parameter names and structure.

For example, if the parent tool has args `x` and `y`, then `forward_raw(x=1,
y=2)` will call the parent tool with `x=1` and `y=2`.

Args:
    **kwargs: Arguments to pass directly to the parent tool (using original names).

Returns:
    The ToolResult from the parent tool execution.

Raises:
    RuntimeError: If called outside a transformed tool context.


