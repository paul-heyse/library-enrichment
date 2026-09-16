# `fastmcp.client.mixins.prompts`

Distribution: `fastmcp`

## AUTO_PAGINATION_MAX_PAGES

`fastmcp.client.mixins.prompts.AUTO_PAGINATION_MAX_PAGES`

```python
AUTO_PAGINATION_MAX_PAGES = 250
```

**Inferred type** (`ty`, not declared in the source): `Literal[250]`

## logger

`fastmcp.client.mixins.prompts.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ClientPromptsMixin

Import as `fastmcp.client.mixins.ClientPromptsMixin`  ·  defined at `fastmcp.client.mixins.prompts.ClientPromptsMixin`

```python
class ClientPromptsMixin
```

**Also exported as** `fastmcp.client.mixins.ClientPromptsMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `async def get_prompt(self: Client, name: str, arguments: dict[str, Any] | None = None, version: str | None = None, meta: dict[str, Any] | None = None) -> mcp_types.GetPromptResult`  _async_
  Retrieve a rendered prompt message list from the server.
- `async def get_prompt_mcp(self: Client, name: str, arguments: dict[str, Any] | None = None, meta: dict[str, Any] | None = None) -> mcp_types.GetPromptResult`  _async_
  Send a prompts/get request and return the complete MCP protocol result.
- `async def list_prompts(self: Client, max_pages: int = AUTO_PAGINATION_MAX_PAGES) -> list[mcp_types.Prompt]`  _async_
  Retrieve all prompts available on the server.
- `async def list_prompts_mcp(self: Client, cursor: str | None = None, cache_mode: CacheMode = 'use') -> mcp_types.ListPromptsResult`  _async_
  Send a prompts/list request and return the complete MCP protocol result.

Mixin providing prompt-related methods for Client.


