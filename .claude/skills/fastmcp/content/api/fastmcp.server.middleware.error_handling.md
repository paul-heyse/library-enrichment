# `fastmcp.server.middleware.error_handling`

Distribution: `fastmcp`

## ErrorHandlingMiddleware

`fastmcp.server.middleware.error_handling.ErrorHandlingMiddleware`

```python
class ErrorHandlingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (7)**

- `error_callback = error_callback`  _instance-attribute_
- `error_counts = {}`  _instance-attribute_
- `def get_error_stats(self) -> dict[str, int]`
  Get error statistics for monitoring.
- `include_traceback = include_traceback`  _instance-attribute_
- `logger = logger or logging.getLogger('fastmcp.errors')`  _instance-attribute_
- `async def on_message(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Handle errors for all messages.
- `transform_errors = transform_errors`  _instance-attribute_

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_notification`, `on_read_resource`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that provides consistent error handling and logging.

Catches exceptions, logs them appropriately, and converts them to
proper MCP error responses. Also tracks error patterns for monitoring.

Example:
    ```python
    from fastmcp.server.middleware.error_handling import ErrorHandlingMiddleware
    import logging

    # Configure logging to see error details
    logging.basicConfig(level=logging.ERROR)

    mcp = FastMCP("MyServer")
    mcp.add_middleware(ErrorHandlingMiddleware())
    ```


## RetryMiddleware

`fastmcp.server.middleware.error_handling.RetryMiddleware`

```python
class RetryMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (7)**

- `backoff_multiplier = backoff_multiplier`  _instance-attribute_
- `base_delay = base_delay`  _instance-attribute_
- `logger = logger or logging.getLogger('fastmcp.retry')`  _instance-attribute_
- `max_delay = max_delay`  _instance-attribute_
- `max_retries = max_retries`  _instance-attribute_
- `async def on_request(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Implement retry logic for requests.
- `retry_exceptions = retry_exceptions`  _instance-attribute_

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that implements automatic retry logic for failed requests.

Retries requests that fail with transient errors, using exponential
backoff to avoid overwhelming the server or external dependencies.

Example:
    ```python
    from fastmcp.server.middleware.error_handling import RetryMiddleware

    # Retry up to 3 times with exponential backoff
    retry_middleware = RetryMiddleware(
        max_retries=3,
        retry_exceptions=(ConnectionError, TimeoutError)
    )

    mcp = FastMCP("MyServer")
    mcp.add_middleware(retry_middleware)
    ```


