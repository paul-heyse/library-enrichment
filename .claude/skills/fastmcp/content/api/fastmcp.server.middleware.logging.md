# `fastmcp.server.middleware.logging`

Distribution: `fastmcp`

## BaseLoggingMiddleware

`fastmcp.server.middleware.logging.BaseLoggingMiddleware`

```python
class BaseLoggingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (10)**

- `estimate_payload_tokens: bool`  _instance-attribute_
- `include_payload_length: bool`  _instance-attribute_
- `include_payloads: bool`  _instance-attribute_
- `log_level: int`  _instance-attribute_
- `logger: Logger`  _instance-attribute_
- `max_payload_length: int | None`  _instance-attribute_
- `methods: list[str] | None`  _instance-attribute_
- `async def on_message(self, context: MiddlewareContext[Any], call_next: CallNext[Any, Any]) -> Any`  _async_
  Log messages for configured methods.
- `payload_serializer: Callable[[Any], str] | None`  _instance-attribute_
- `structured_logging: bool`  _instance-attribute_

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Base class for logging middleware.


## LoggingMiddleware

`fastmcp.server.middleware.logging.LoggingMiddleware`

```python
class LoggingMiddleware(BaseLoggingMiddleware)
```

**Bases** `BaseLoggingMiddleware`

**Declared members (9)**

- `estimate_payload_tokens: bool = estimate_payload_tokens`  _instance-attribute_
- `include_payload_length: bool = include_payload_length`  _instance-attribute_
- `include_payloads: bool = include_payloads`  _instance-attribute_
- `log_level = log_level`  _instance-attribute_
- `logger: Logger = logger or logging.getLogger('fastmcp.middleware.logging')`  _instance-attribute_
- `max_payload_length: int = max_payload_length`  _instance-attribute_
- `methods: list[str] | None = methods`  _instance-attribute_
- `payload_serializer: Callable[[Any], str] | None = payload_serializer`  _instance-attribute_
- `structured_logging: bool = False`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.server.middleware.logging.BaseLoggingMiddleware`: `on_message`
- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that provides comprehensive request and response logging.

Logs all MCP messages with configurable detail levels. Useful for debugging,
monitoring, and understanding server usage patterns.

Example:
    ```python
    from fastmcp.server.middleware.logging import LoggingMiddleware
    import logging

    # Configure logging
    logging.basicConfig(level=logging.INFO)

    mcp = FastMCP("MyServer")
    mcp.add_middleware(LoggingMiddleware())
    ```


## StructuredLoggingMiddleware

`fastmcp.server.middleware.logging.StructuredLoggingMiddleware`

```python
class StructuredLoggingMiddleware(BaseLoggingMiddleware)
```

**Bases** `BaseLoggingMiddleware`

**Declared members (9)**

- `estimate_payload_tokens: bool = estimate_payload_tokens`  _instance-attribute_
- `include_payload_length: bool = include_payload_length`  _instance-attribute_
- `include_payloads: bool = include_payloads`  _instance-attribute_
- `log_level: int = log_level`  _instance-attribute_
- `logger: Logger = logger or logging.getLogger('fastmcp.middleware.structured_logging')`  _instance-attribute_
- `max_payload_length: int | None = None`  _instance-attribute_
- `methods: list[str] | None = methods`  _instance-attribute_
- `payload_serializer: Callable[[Any], str] | None = payload_serializer`  _instance-attribute_
- `structured_logging: bool = True`  _instance-attribute_

**Inherited (12)**

- from `fastmcp.server.middleware.logging.BaseLoggingMiddleware`: `on_message`
- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that provides structured JSON logging for better log analysis.

Outputs structured logs that are easier to parse and analyze with log
aggregation tools like ELK stack, Splunk, or cloud logging services.

Example:
    ```python
    from fastmcp.server.middleware.logging import StructuredLoggingMiddleware
    import logging

    mcp = FastMCP("MyServer")
    mcp.add_middleware(StructuredLoggingMiddleware())
    ```


## _get_duration_ms

`fastmcp.server.middleware.logging._get_duration_ms`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_duration_ms(start_time: float) -> float
```

## default_serializer

`fastmcp.server.middleware.logging.default_serializer`

```python
def default_serializer(data: Any) -> str
```

The default serializer for Payloads in the logging middleware.


