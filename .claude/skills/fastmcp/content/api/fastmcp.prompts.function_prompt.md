# `fastmcp.prompts.function_prompt`

Distribution: `fastmcp`

## F

`fastmcp.prompts.function_prompt.F`

```python
F = TypeVar('F', bound=Callable[..., Any])
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## logger

`fastmcp.prompts.function_prompt.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DecoratedPrompt

`fastmcp.prompts.function_prompt.DecoratedPrompt`

```python
class DecoratedPrompt(Protocol)
```

**Bases** `Protocol`

Protocol for functions decorated with @prompt.


## FunctionPrompt

Import as `fastmcp.prompts.FunctionPrompt`  ·  defined at `fastmcp.prompts.function_prompt.FunctionPrompt`

```python
class FunctionPrompt(Prompt)
```

**Also exported as** `fastmcp.prompts.FunctionPrompt`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Prompt`

**Declared members (3)**

- `fn: SkipJsonSchema[Callable[..., Any]]`  _instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], metadata: PromptMeta | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> FunctionPrompt`  _classmethod_
  Create a Prompt from a function.
- `async def render(self, arguments: dict[str, Any] | None = None) -> PromptResult`  _async_
  Render the prompt with arguments.

**Inherited (19)**

- from `fastmcp.prompts.base.Prompt`: `KEY_PREFIX`, `arguments`, `auth`, `convert_result`, `get_span_attributes`, `to_mcp_prompt`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt that is a function.


## PromptMeta

Import as `fastmcp.decorators.PromptMeta`  ·  defined at `fastmcp.prompts.function_prompt.PromptMeta`

```python
class PromptMeta
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (10)**

- `auth: AuthCheck | list[AuthCheck] | None = None`  _class-attribute, instance-attribute_
- `description: str | None = None`  _class-attribute, instance-attribute_
- `enabled: bool = True`  _class-attribute, instance-attribute_
- `icons: list[Icon] | None = None`  _class-attribute, instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `name: str | None = None`  _class-attribute, instance-attribute_
- `tags: set[str] | None = None`  _class-attribute, instance-attribute_
- `title: str | None = None`  _class-attribute, instance-attribute_
- `type: Literal['prompt'] = field(default='prompt', init=False)`  _class-attribute, instance-attribute_
- `version: str | int | None = None`  _class-attribute, instance-attribute_

Metadata attached to functions by the @prompt decorator.


## prompt

Import as `fastmcp.prompts.prompt`  ·  defined at `fastmcp.prompts.function_prompt.prompt`

```python
def prompt(name_or_fn: str | Callable[..., Any] | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Any
```

**Overloads** (the signature above is the runtime dispatcher):

- `def prompt(fn: F) -> F`
- `def prompt(name_or_fn: str, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Callable[[F], F]`
- `def prompt(name_or_fn: None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Callable[[F], F]`

**Also exported as** `fastmcp.prompts.prompt`

Standalone decorator to mark a function as an MCP prompt.

Returns the original function with metadata attached. Register with a server
using mcp.add_prompt().


