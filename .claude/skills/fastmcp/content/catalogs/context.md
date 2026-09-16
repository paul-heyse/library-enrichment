# The Context object

What a tool body can do. `fastmcp.Context` exposes 34 members, of which
19 are `async`.

Obtain one by annotating a parameter `ctx: Context` on any tool, resource or prompt,
or by calling `fastmcp.server.dependencies.get_context()`.

| Member | Kind | Signature |
|---|---|---|
| `client_extension_settings` | function | `def client_extension_settings(self, identifier: str) -> dict[str, Any] | None` |
| `client_id` | property | `client_id: str | None` |
| `client_supports_extension` | function | `def client_supports_extension(self, extension_id: str) -> bool` |
| `close_sse_stream` | async | `async def close_sse_stream(self) -> None` |
| `debug` | async | `async def debug(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | N` |
| `delete_state` | async | `async def delete_state(self, key: str) -> None` |
| `disable_components` | async | `async def disable_components(self, names: set[str] | None = None, keys: set[str] | None = None, ` |
| `elicit` | async | `async def elicit(self, message: str, response_type: type[T] | list[str] | dict[str, dict[str, st` |
| `enable_components` | async | `async def enable_components(self, names: set[str] | None = None, keys: set[str] | None = None, v` |
| `error` | async | `async def error(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | N` |
| `fastmcp` | property | `fastmcp: FastMCP` |
| `get_prompt` | async | `async def get_prompt(self, name: str, arguments: dict[str, Any] | None = None) -> GetPromptResul` |
| `get_state` | async | `async def get_state(self, key: str) -> Any` |
| `info` | async | `async def info(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] | No` |
| `input_responses` | property | `input_responses: mcp_types.InputResponses | None` |
| `is_background_task` | property | `is_background_task: bool` |
| `lifespan_context` | property | `lifespan_context: dict[str, Any]` |
| `list_prompts` | async | `async def list_prompts(self) -> list[SDKPrompt]` |
| `list_resources` | async | `async def list_resources(self) -> list[SDKResource]` |
| `log` | async | `async def log(self, message: str, level: LoggingLevel | None = None, logger_name: str | None = N` |
| `origin_request_id` | property | `origin_request_id: str | None` |
| `read_resource` | async | `async def read_resource(self, uri: str | AnyUrl) -> ResourceResult` |
| `report_progress` | async | `async def report_progress(self, progress: float, total: float | None = None, message: str | None` |
| `request_context` | property | `request_context: FastMCPRequestContext | None` |
| `request_id` | property | `request_id: str` |
| `request_state` | property | `request_state: str | None` |
| `reset_visibility` | async | `async def reset_visibility(self) -> None` |
| `send_notification` | async | `async def send_notification(self, notification: mcp_types.ServerNotification) -> None` |
| `session` | property | `session: ServerSession` |
| `session_id` | property | `session_id: str` |
| `set_state` | async | `async def set_state(self, key: str, value: Any, serializable: bool = True) -> None` |
| `task_id` | property | `task_id: str | None` |
| `transport` | property | `transport: TransportType | None` |
| `warning` | async | `async def warning(self, message: str, logger_name: str | None = None, extra: Mapping[str, Any] |` |
