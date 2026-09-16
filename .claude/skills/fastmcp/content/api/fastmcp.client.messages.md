# `fastmcp.client.messages`

Distribution: `fastmcp`

## Message

`fastmcp.client.messages.Message`

```python
Message: TypeAlias = mcp_types.ServerNotification | Exception
```

## MessageHandlerT

Import as `fastmcp.client.client.MessageHandlerT`  ·  defined at `fastmcp.client.messages.MessageHandlerT`

```python
MessageHandlerT: TypeAlias = MessageHandlerFnT
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## MessageHandler

Import as `fastmcp.client.client.MessageHandler`  ·  defined at `fastmcp.client.messages.MessageHandler`

```python
class MessageHandler
```

**Also exported as** `fastmcp.client.client.MessageHandler`

**Declared members (11)**

- `async def dispatch(self, message: Message) -> None`  _async_
- `async def on_cancelled(self, message: mcp_types.CancelledNotification) -> None`  _async_
- `async def on_exception(self, message: Exception) -> None`  _async_
- `async def on_logging_message(self, message: mcp_types.LoggingMessageNotification) -> None`  _async_
- `async def on_message(self, message: Message) -> None`  _async_
- `async def on_notification(self, message: mcp_types.ServerNotification) -> None`  _async_
- `async def on_progress(self, message: mcp_types.ProgressNotification) -> None`  _async_
- `async def on_prompt_list_changed(self, message: mcp_types.PromptListChangedNotification) -> None`  _async_
- `async def on_resource_list_changed(self, message: mcp_types.ResourceListChangedNotification) -> None`  _async_
- `async def on_resource_updated(self, message: mcp_types.ResourceUpdatedNotification) -> None`  _async_
- `async def on_tool_list_changed(self, message: mcp_types.ToolListChangedNotification) -> None`  _async_

This class is used to handle MCP messages sent to the client: notifications
and transport-level exceptions. Users can override any of the hooks.

Server-initiated *requests* (ping, sampling, roots) never reach this
handler: the stable MCP SDK v2's `message_handler` contract only delivers
`ServerNotification | Exception`, so a request has no wire path here.
Those are answered through the `Client`'s dedicated callbacks instead —
`sampling_handler=`, `roots=`, and `elicitation_handler=`.


