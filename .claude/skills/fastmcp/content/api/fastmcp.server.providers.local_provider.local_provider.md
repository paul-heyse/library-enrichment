# `fastmcp.server.providers.local_provider.local_provider`

Distribution: `fastmcp`

## DuplicateBehavior

`fastmcp.server.providers.local_provider.local_provider.DuplicateBehavior`

```python
DuplicateBehavior = Literal['error', 'warn', 'replace', 'ignore']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["error", "warn", "replace", "ignore"]'> ````

## _C

`fastmcp.server.providers.local_provider.local_provider._C`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_C = TypeVar('_C', bound=FastMCPComponent)
```

## logger

`fastmcp.server.providers.local_provider.local_provider.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## LocalProvider

Import as `fastmcp.server.providers.LocalProvider`  ·  defined at `fastmcp.server.providers.local_provider.local_provider.LocalProvider`

```python
class LocalProvider(Provider, ToolDecoratorMixin, ResourceDecoratorMixin, PromptDecoratorMixin)
```

**Also exported as** `fastmcp.server.providers.LocalProvider`, `fastmcp.server.providers.local_provider.LocalProvider`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Provider`, `ToolDecoratorMixin`, `ResourceDecoratorMixin`, `PromptDecoratorMixin`

**Declared members (5)**

- `async def get_tasks(self) -> Sequence[FastMCPComponent]`  _async_
  Return components eligible for background task execution.
- `def remove_prompt(self, name: str, version: str | None = None) -> None`
  Remove prompt(s) from this provider's storage.
- `def remove_resource(self, uri: str, version: str | None = None) -> None`
  Remove resource(s) from this provider's storage.
- `def remove_template(self, uri_template: str, version: str | None = None) -> None`
  Remove resource template(s) from this provider's storage.
- `def remove_tool(self, name: str, version: str | None = None) -> None`
  Remove tool(s) from this provider's storage.

**Inherited (23)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tool`, `get_tool_by_hash`, `lifespan`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`
- from `fastmcp.server.providers.local_provider.decorators.prompts.PromptDecoratorMixin`: `add_prompt`, `prompt`
- from `fastmcp.server.providers.local_provider.decorators.resources.ResourceDecoratorMixin`: `add_resource`, `add_template`, `resource`
- from `fastmcp.server.providers.local_provider.decorators.tools.ToolDecoratorMixin`: `add_tool`, `tool`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider for locally-defined components.

Supports decorator-based registration (`@provider.tool`, `@provider.resource`,
`@provider.prompt`) and direct object registration methods.

When used standalone, LocalProvider uses default settings. When attached
to a FastMCP server via the server's decorators, server-level settings
like `_tool_serializer` and `_support_tasks_by_default` are injected.

Example:
    ```python
    from fastmcp.server.providers import LocalProvider

    # Standalone usage
    provider = LocalProvider()

    @provider.tool
    def greet(name: str) -> str:
        return f"Hello, {name}!"

    @provider.resource("data://config")
    def get_config() -> str:
        return '{"setting": "value"}'

    @provider.prompt
    def analyze(topic: str) -> list:
        return [{"role": "user", "content": f"Analyze: {topic}"}]

    # Attach to server(s)
    from fastmcp import FastMCP
    server = FastMCP("MyServer", providers=[provider])
    ```


