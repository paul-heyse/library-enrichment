# `fastmcp.server.providers.fastmcp_provider`

Distribution: `fastmcp`

## FastMCPProvider

Import as `fastmcp.server.providers.FastMCPProvider`  ·  defined at `fastmcp.server.providers.fastmcp_provider.FastMCPProvider`

```python
class FastMCPProvider(Provider)
```

**Also exported as** `fastmcp.server.providers.FastMCPProvider`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`

**Declared members (5)**

- `async def get_app_tool(self, app_name: str, tool_name: str) -> Tool | None`  _async_
  Delegate to nested server's get_app_tool, wrapping for middleware.
- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Return task-eligible components from the mounted server.
- `async def get_tool_by_hash(self, tool_hash: str, tool_name: str) -> Tool | None`  _async_
  Delegate to nested server's get_tool_by_hash, wrapping for middleware.
- `async def lifespan(self) -> AsyncIterator[None]`  _async_
  Start the mounted server's lifespan.
- `server = server`  _instance-attribute_

**Inherited (13)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider that wraps a FastMCP server.

This provider enables mounting one FastMCP server onto another, exposing
the mounted server's tools, resources, and prompts through the parent
server.

Components returned by this provider are wrapped in FastMCPProvider*
classes that delegate execution to the wrapped server's middleware chain.
This ensures middleware runs when components are executed.

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.server.providers import FastMCPProvider

    main = FastMCP("Main")
    sub = FastMCP("Sub")

    @sub.tool
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    # Mount directly - tools accessible by original names
    main.add_provider(FastMCPProvider(sub))

    # Or with namespace
    from fastmcp.server.transforms import Namespace
    provider = FastMCPProvider(sub)
    provider.add_transform(Namespace("sub"))
    main.add_provider(provider)
    ```

Note:
    Normally you would use `FastMCP.mount()` which handles proxy conversion
    and creates the provider with namespace automatically.


## FastMCPProviderPrompt

`fastmcp.server.providers.fastmcp_provider.FastMCPProviderPrompt`

```python
class FastMCPProviderPrompt(Prompt)
```

**Bases** `Prompt`

**Declared members (3)**

- `def get_span_attributes(self) -> dict[str, Any]`
- `async def render(self, arguments: dict[str, Any] | None = None) -> PromptResult`  _async_
  Delegate to the child server's render_prompt().
- `def wrap(cls, server: Any, prompt: Prompt) -> FastMCPProviderPrompt`  _classmethod_
  Wrap a Prompt to delegate rendering to the server's middleware.

**Inherited (19)**

- from `fastmcp.prompts.base.Prompt`: `KEY_PREFIX`, `arguments`, `auth`, `convert_result`, `from_function`, `to_mcp_prompt`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Prompt that delegates rendering to a wrapped server's render_prompt().

When `render()` is called, this prompt invokes the wrapped server's
`render_prompt()` method, ensuring the server's middleware chain is executed.


## FastMCPProviderResource

`fastmcp.server.providers.fastmcp_provider.FastMCPProviderResource`

```python
class FastMCPProviderResource(Resource)
```

**Bases** `Resource`

**Declared members (2)**

- `def get_span_attributes(self) -> dict[str, Any]`
- `def wrap(cls, server: Any, resource: Resource) -> FastMCPProviderResource`  _classmethod_
  Wrap a Resource to delegate reading to the server's middleware.

**Inherited (24)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `key`, `mime_type`, `name`, `read`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Resource that delegates reading to a wrapped server's read_resource().

When `read()` is called, this resource invokes the wrapped server's
`read_resource()` method, ensuring the server's middleware chain is executed.


## FastMCPProviderResourceTemplate

`fastmcp.server.providers.fastmcp_provider.FastMCPProviderResourceTemplate`

```python
class FastMCPProviderResourceTemplate(ResourceTemplate)
```

**Bases** `ResourceTemplate`

**Declared members (3)**

- `async def create_resource(self, uri: str, params: dict[str, Any]) -> Resource`  _async_
  Create a FastMCPProviderResource for the given URI.
- `def get_span_attributes(self) -> dict[str, Any]`
- `def wrap(cls, server: Any, template: ResourceTemplate) -> FastMCPProviderResourceTemplate`  _classmethod_
  Wrap a ResourceTemplate to create FastMCPProviderResources.

**Inherited (28)**

- from `fastmcp.resources.template.ResourceTemplate`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `from_mcp_template`, `key`, `matches`, `mime_type`, `parameters`, `read`, `resolve_security`, `security`, `set_default_mime_type`, `to_mcp_template`, `uri_template`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Resource template that creates FastMCPProviderResources.

When `create_resource()` is called, this template creates a
FastMCPProviderResource that will invoke the wrapped server's middleware
when read.


## FastMCPProviderTool

`fastmcp.server.providers.fastmcp_provider.FastMCPProviderTool`

```python
class FastMCPProviderTool(Tool)
```

**Bases** `Tool`

**Declared members (3)**

- `def get_span_attributes(self) -> dict[str, Any]`
- `async def run(self, arguments: dict[str, Any]) -> ToolResult`  _async_
  Delegate to the child server's call_tool().
- `def wrap(cls, server: Any, tool: Tool) -> FastMCPProviderTool`  _classmethod_
  Wrap a Tool to delegate execution to the server's middleware.

**Inherited (25)**

- from `fastmcp.tools.base.Tool`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `execution`, `from_function`, `from_tool`, `output_schema`, `parameters`, `return_type`, `timeout`, `to_mcp_tool`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Tool that delegates execution to a wrapped server's middleware.

When `run()` is called, this tool invokes the wrapped server's
`_call_tool_middleware()` method, ensuring the server's middleware
chain is executed.


