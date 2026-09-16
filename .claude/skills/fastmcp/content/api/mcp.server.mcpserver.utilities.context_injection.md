# `mcp.server.mcpserver.utilities.context_injection`

Distribution: `mcp`

## find_context_parameter

Import as `mcp.server.mcpserver.server.find_context_parameter`  ·  defined at `mcp.server.mcpserver.utilities.context_injection.find_context_parameter`

```python
def find_context_parameter(fn: Callable[..., Any]) -> str | None
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Find the parameter that should receive the Context object.

Searches through the function's signature to find a parameter
with a Context type annotation.

Args:
    fn: The function to inspect

Returns:
    The name of the context parameter, or None if not found


## inject_context

Import as `mcp.server.mcpserver.prompts.base.inject_context`  ·  defined at `mcp.server.mcpserver.utilities.context_injection.inject_context`

```python
def inject_context(fn: Callable[..., Any], kwargs: dict[str, Any], context: Any | None, context_kwarg: str | None) -> dict[str, Any]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Inject context into function kwargs if needed.

Args:
    fn: The function that will be called
    kwargs: The current keyword arguments
    context: The context object to inject (if any)
    context_kwarg: The name of the parameter to inject into

Returns:
    Updated kwargs with context injected if applicable


