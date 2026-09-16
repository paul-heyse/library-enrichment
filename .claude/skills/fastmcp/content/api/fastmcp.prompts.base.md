# `fastmcp.prompts.base`

Distribution: `fastmcp`

## __all__

`fastmcp.prompts.base.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Message', 'Prompt', 'PromptArgument', 'PromptResult']
```

## logger

`fastmcp.prompts.base.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## InputRequiredPromptResult

Import as `fastmcp.server.providers.proxy.InputRequiredPromptResult`  ·  defined at `fastmcp.prompts.base.InputRequiredPromptResult`

```python
class InputRequiredPromptResult(PromptResult)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `PromptResult`

**Declared members (1)**

- `input_required: mcp_types.InputRequiredResult = Field(description='The client-input request this leg resolved to (SEP-2322)')`  _class-attribute, instance-attribute_

**Inherited (4)**

- from `fastmcp.prompts.base.PromptResult`: `description`, `messages`, `meta`, `to_mcp_prompt_result`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

The full result of a single multi-round-trip prompt leg (SEP-2322).

`InputRequiredResult` is a result type, not a `tools/call` feature: any
request may resolve to one. When a prompt returns an `InputRequiredResult`
from its body to ask the client for input, that ask is the legitimate
result of this `prompts/get` — so FastMCP wraps it in this `PromptResult`
subclass, mirroring `InputRequiredToolResult`, and it flows through the
middleware chain as an ordinary return value.

Invariant: the wrapped `InputRequiredResult` is never rendered as prompt
messages. `messages` is always empty; the wire handler (`_on_get_prompt`)
reads `.input_required` and returns it to the runner.


## Message

Import as `fastmcp.prompts.Message`  ·  defined at `fastmcp.prompts.base.Message`

```python
class Message(pydantic.BaseModel)
```

**Also exported as** `fastmcp.prompts.Message`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `pydantic.BaseModel`

**Declared members (3)**

- `content: TextContent | ImageContent | AudioContent | EmbeddedResource`  _instance-attribute_
- `role: Literal['user', 'assistant']`  _instance-attribute_
- `def to_mcp_prompt_message(self) -> PromptMessage`
  Convert to MCP PromptMessage.

Wrapper for prompt message with auto-serialization.

Accepts any content - strings pass through, other types
(dict, list, BaseModel) are JSON-serialized to text.

Example:
    ```python
    from fastmcp.prompts import Message

    # String content (user role by default)
    Message("Hello, world!")

    # Explicit role
    Message("I can help with that.", role="assistant")

    # Auto-serialized to JSON
    Message({"key": "value"})
    Message(["item1", "item2"])
    ```


## Prompt

Import as `fastmcp.prompts.Prompt`  ·  defined at `fastmcp.prompts.base.Prompt`

```python
class Prompt(FastMCPComponent)
```

**Also exported as** `fastmcp.prompts.Prompt`

_20 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPComponent`

**Declared members (8)**

- `KEY_PREFIX: str = 'prompt'`  _class-attribute_
- `arguments: list[PromptArgument] | None = Field(default=None, description='Arguments that can be passed to the prompt')`  _class-attribute, instance-attribute_
- `auth: SkipJsonSchema[AuthCheck | list[AuthCheck] | None] = Field(default=None, description='Authorization checks for this prompt', exclude=True)`  _class-attribute, instance-attribute_
- `def convert_result(self, raw_value: Any) -> PromptResult`
  Convert a raw return value to PromptResult.
- `def from_function(cls, fn: Callable[..., Any], name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> FunctionPrompt`  _classmethod_
  Create a Prompt from a function.
- `def get_span_attributes(self) -> dict[str, Any]`
- `async def render(self, arguments: dict[str, Any] | None = None) -> str | list[Message | str] | PromptResult`  _async_
  Render the prompt with arguments.
- `def to_mcp_prompt(self, overrides: Any = {}) -> SDKPrompt`
  Convert the prompt to an MCP prompt.

**Inherited (13)**

- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `key`, `make_key`, `meta`, `name`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A prompt template that can be rendered with parameters.


## PromptArgument

Import as `fastmcp.prompts.PromptArgument`  ·  defined at `fastmcp.prompts.base.PromptArgument`

```python
class PromptArgument(FastMCPBaseModel)
```

**Also exported as** `fastmcp.prompts.PromptArgument`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `FastMCPBaseModel`

**Declared members (3)**

- `description: str | None = Field(default=None, description='Description of what the argument does')`  _class-attribute, instance-attribute_
- `name: str = Field(description='Name of the argument')`  _class-attribute, instance-attribute_
- `required: bool = Field(default=False, description='Whether the argument is required')`  _class-attribute, instance-attribute_

An argument that can be passed to a prompt.


## PromptResult

Import as `fastmcp.prompts.PromptResult`  ·  defined at `fastmcp.prompts.base.PromptResult`

```python
class PromptResult(pydantic.BaseModel)
```

**Also exported as** `fastmcp.prompts.PromptResult`

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `pydantic.BaseModel`

**Declared members (4)**

- `description: str | None = None`  _class-attribute, instance-attribute_
- `messages: list[Message]`  _instance-attribute_
- `meta: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `def to_mcp_prompt_result(self) -> GetPromptResult`
  Convert to MCP GetPromptResult.

Canonical result type for prompt rendering.

Provides explicit control over prompt responses: multiple messages,
roles, and metadata at both the message and result level.

Accepts:
    - str: Wrapped as single Message (user role)
    - list[Message]: Used directly for multiple messages or custom roles

Example:
    ```python
    from fastmcp import FastMCP
    from fastmcp.prompts import PromptResult, Message

    mcp = FastMCP()

    # Simple string content
    @mcp.prompt
    def greet() -> PromptResult:
        return PromptResult("Hello!")

    # Multiple messages with roles
    @mcp.prompt
    def conversation() -> PromptResult:
        return PromptResult([
            Message("What's the weather?"),
            Message("It's sunny today.", role="assistant"),
        ])
    ```


