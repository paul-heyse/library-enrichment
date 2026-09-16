# `mcp.server.mcpserver.prompts.base`

Distribution: `mcp`

## PromptResult

`mcp.server.mcpserver.prompts.base.PromptResult`

```python
PromptResult = SyncPromptResult | Awaitable[SyncPromptResult]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'Sequence[str | TextContent | ImageContent | ... omitted 7 union elements] | TextContent | ImageContent | ... omitted 9 union elements'> ````

## SyncPromptResult

`mcp.server.mcpserver.prompts.base.SyncPromptResult`

```python
SyncPromptResult = _PromptResultItem | InputRequiredResult | Sequence[_PromptResultItem]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'Sequence[str | TextContent | ImageContent | ... omitted 7 union elements] | TextContent | ImageContent | ... omitted 8 union elements'> ````

## _PromptResultItem

`mcp.server.mcpserver.prompts.base._PromptResultItem`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PromptResultItem = str | ContentBlock | Image | Audio | Message | dict[str, Any]
```

## message_validator

`mcp.server.mcpserver.prompts.base.message_validator`

```python
message_validator: TypeAdapter[UserMessage | AssistantMessage] = TypeAdapter(Annotated[UserMessage | AssistantMessage, Field(union_mode='left_to_right')])
```

## AssistantMessage

Import as `mcp.server.mcpserver.AssistantMessage`  ·  defined at `mcp.server.mcpserver.prompts.base.AssistantMessage`

```python
class AssistantMessage(Message)
```

**Also exported as** `mcp.server.mcpserver.AssistantMessage`

**Bases** `Message`

**Declared members (1)**

- `role: Literal['user', 'assistant'] = 'assistant'`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `mcp.server.mcpserver.prompts.base.Message`: `content`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A message from the assistant.


## Message

Import as `mcp.server.mcpserver.Message`  ·  defined at `mcp.server.mcpserver.prompts.base.Message`

```python
class Message(BaseModel)
```

**Also exported as** `mcp.server.mcpserver.Message`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (2)**

- `content: ContentBlock`  _instance-attribute_
- `role: Literal['user', 'assistant']`  _instance-attribute_

Base class for all prompt messages.

`content` may be a plain string (wrapped in `TextContent`), an `Image` or `Audio`
helper (converted to `ImageContent` / `AudioContent`, reading the file for path-backed
helpers), or any ready-made content block.

Raises:
    OSError: If a path-backed `Image` or `Audio` cannot be read.


## Prompt

Import as `mcp.server.mcpserver.prompts.Prompt`  ·  defined at `mcp.server.mcpserver.prompts.base.Prompt`

```python
class Prompt(BaseModel)
```

**Also exported as** `mcp.server.mcpserver.prompts.Prompt`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (9)**

- `arguments: list[PromptArgument] | None = Field(None, description='Arguments that can be passed to the prompt')`  _class-attribute, instance-attribute_
- `context_kwarg: str | None = Field(None, description='Name of the kwarg that should receive context', exclude=True)`  _class-attribute, instance-attribute_
- `description: str | None = Field(None, description='Description of what the prompt does')`  _class-attribute, instance-attribute_
- `fn: Callable[..., PromptResult | Awaitable[PromptResult]] = Field(exclude=True)`  _class-attribute, instance-attribute_
- `def from_function(cls, fn: Callable[..., PromptResult | Awaitable[PromptResult]], name: str | None = None, title: str | None = None, description: str | None = None, icons: list[Icon] | None = None, context_kwarg: str | None = None) -> Prompt`  _classmethod_
  Create a Prompt from a function.
- `icons: list[Icon] | None = Field(default=None, description='Optional list of icons for this prompt')`  _class-attribute, instance-attribute_
- `name: str = Field(description='Name of the prompt')`  _class-attribute, instance-attribute_
- `async def render(self, arguments: dict[str, Any] | None, context: Context[LifespanContextT, RequestT]) -> list[Message] | InputRequiredResult`  _async_
  Render the prompt with arguments.
- `title: str | None = Field(None, description='Human-readable title of the prompt')`  _class-attribute, instance-attribute_

A prompt template that can be rendered with parameters.


## PromptArgument

`mcp.server.mcpserver.prompts.base.PromptArgument`

```python
class PromptArgument(BaseModel)
```

**Bases** `BaseModel`

**Declared members (3)**

- `description: str | None = Field(None, description='Description of what the argument does')`  _class-attribute, instance-attribute_
- `name: str = Field(description='Name of the argument')`  _class-attribute, instance-attribute_
- `required: bool = Field(default=False, description='Whether the argument is required')`  _class-attribute, instance-attribute_

An argument that can be passed to a prompt.


## UserMessage

Import as `mcp.server.mcpserver.UserMessage`  ·  defined at `mcp.server.mcpserver.prompts.base.UserMessage`

```python
class UserMessage(Message)
```

**Also exported as** `mcp.server.mcpserver.UserMessage`

**Bases** `Message`

**Declared members (1)**

- `role: Literal['user', 'assistant'] = 'user'`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `mcp.server.mcpserver.prompts.base.Message`: `content`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A message from the user.


