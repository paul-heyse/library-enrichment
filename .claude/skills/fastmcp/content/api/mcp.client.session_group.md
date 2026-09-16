# `mcp.client.session_group`

Distribution: `mcp`

## ServerParameters

`mcp.client.session_group.ServerParameters`

```python
ServerParameters: TypeAlias = StdioServerParameters | SseServerParameters | StreamableHttpParameters
```

## ClientSessionGroup

Import as `mcp.ClientSessionGroup`  ·  defined at `mcp.client.session_group.ClientSessionGroup`

```python
class ClientSessionGroup
```

**Also exported as** `mcp.ClientSessionGroup`

**Declared members (8)**

- `async def call_tool(self, name: str, arguments: dict[str, Any] | None = None, read_timeout_seconds: float | None = None, progress_callback: ProgressFnT | None = None, input_responses: types.InputResponses | None = None, request_state: str | None = None, meta: types.RequestParamsMeta | None = None, allow_input_required: bool = False) -> types.CallToolResult | types.InputRequiredResult`  _async_
  Executes a tool given its name and arguments.
- `async def connect_to_server(self, server_params: ServerParameters, session_params: ClientSessionParameters | None = None) -> mcp.ClientSession`  _async_
  Connects to a single MCP server.
- `async def connect_with_session(self, server_info: types.Implementation, session: mcp.ClientSession) -> mcp.ClientSession`  _async_
  Connects to a single MCP server.
- `async def disconnect_from_server(self, session: mcp.ClientSession) -> None`  _async_
  Disconnects from a single MCP server.
- `prompts: dict[str, types.Prompt]`  _property_
  Returns the prompts as a dictionary of names to prompts.
- `resources: dict[str, types.Resource]`  _property_
  Returns the resources as a dictionary of names to resources.
- `sessions: list[mcp.ClientSession]`  _property_
  Returns the list of sessions being managed.
- `tools: dict[str, types.Tool]`  _property_
  Returns the tools as a dictionary of names to tools.

Client for managing connections to multiple MCP servers.

This class is responsible for encapsulating management of server connections.
It aggregates tools, resources, and prompts from all connected servers.

For auxiliary handlers, such as resource subscription, this is delegated to
the client and can be accessed via the session.

Example:
    ```python
    name_fn = lambda name, server_info: f"{(server_info.name)}_{name}"
    async with ClientSessionGroup(component_name_hook=name_fn) as group:
        for server_param in server_params:
            await group.connect_to_server(server_param)
        ...
    ```


## ClientSessionParameters

`mcp.client.session_group.ClientSessionParameters`

```python
class ClientSessionParameters
```

**Declared members (7)**

- `client_info: types.Implementation | None = None`  _class-attribute, instance-attribute_
- `elicitation_callback: ElicitationFnT | None = None`  _class-attribute, instance-attribute_
- `list_roots_callback: ListRootsFnT | None = None`  _class-attribute, instance-attribute_
- `logging_callback: LoggingFnT | None = None`  _class-attribute, instance-attribute_
- `message_handler: MessageHandlerFnT | None = None`  _class-attribute, instance-attribute_
- `read_timeout_seconds: float | None = None`  _class-attribute, instance-attribute_
- `sampling_callback: SamplingFnT | None = None`  _class-attribute, instance-attribute_

Parameters for establishing a client session to an MCP server.


## SseServerParameters

`mcp.client.session_group.SseServerParameters`

```python
class SseServerParameters(BaseModel)
```

**Bases** `BaseModel`

**Declared members (4)**

- `headers: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `sse_read_timeout: float = 300.0`  _class-attribute, instance-attribute_
- `timeout: float = 5.0`  _class-attribute, instance-attribute_
- `url: str`  _instance-attribute_

Parameters for initializing an sse_client.


## StreamableHttpParameters

`mcp.client.session_group.StreamableHttpParameters`

```python
class StreamableHttpParameters(BaseModel)
```

**Bases** `BaseModel`

**Declared members (5)**

- `headers: dict[str, Any] | None = None`  _class-attribute, instance-attribute_
- `sse_read_timeout: float = 300.0`  _class-attribute, instance-attribute_
- `terminate_on_close: bool = True`  _class-attribute, instance-attribute_
- `timeout: float = 30.0`  _class-attribute, instance-attribute_
- `url: str`  _instance-attribute_

Parameters for initializing a streamable_http_client.


