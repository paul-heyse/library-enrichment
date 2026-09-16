# `fastmcp.server.providers.local_provider.decorators.prompts`

Distribution: `fastmcp`

## F

`fastmcp.server.providers.local_provider.decorators.prompts.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## PromptDecoratorMixin

Import as `fastmcp.server.providers.local_provider.decorators.PromptDecoratorMixin`  ·  defined at `fastmcp.server.providers.local_provider.decorators.prompts.PromptDecoratorMixin`

```python
class PromptDecoratorMixin
```

**Also exported as** `fastmcp.server.providers.local_provider.decorators.PromptDecoratorMixin`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `def add_prompt(self: LocalProvider, prompt: Prompt | Callable[..., Any]) -> Prompt`
  Add a prompt to this provider's storage.
- `def prompt(self: LocalProvider, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, enabled: bool = True, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt | partial[Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt]`
  Decorator to register a prompt.

Mixin class providing prompt decorator functionality for LocalProvider.

This mixin contains all methods related to:
- Prompt registration via add_prompt()
- Prompt decorator (@provider.prompt)


