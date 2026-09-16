# `mcp.server.mcpserver.prompts.manager`

Distribution: `mcp`

## logger

`mcp.server.mcpserver.prompts.manager.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## PromptManager

Import as `mcp.server.mcpserver.prompts.PromptManager`  ·  defined at `mcp.server.mcpserver.prompts.manager.PromptManager`

```python
class PromptManager
```

**Also exported as** `mcp.server.mcpserver.prompts.PromptManager`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `def add_prompt(self, prompt: Prompt) -> Prompt`
  Add a prompt to the manager.
- `def get_prompt(self, name: str) -> Prompt | None`
  Get prompt by name.
- `def list_prompts(self) -> list[Prompt]`
  List all registered prompts.
- `def remove_prompt(self, name: str) -> None`
  Remove a prompt by name.
- `async def render_prompt(self, name: str, arguments: dict[str, Any] | None, context: Context[LifespanContextT, RequestT]) -> list[Message] | InputRequiredResult`  _async_
  Render a prompt by name with arguments.
- `warn_on_duplicate_prompts = warn_on_duplicate_prompts`  _instance-attribute_

Manages MCPServer prompts.


