# `fastmcp.server.middleware.timing`

Distribution: `fastmcp`

## DetailedTimingMiddleware

`fastmcp.server.middleware.timing.DetailedTimingMiddleware`

```python
class DetailedTimingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (9)**

- `log_level = log_level`  _instance-attribute_
- `logger = logger or logging.getLogger('fastmcp.timing.detailed')`  _instance-attribute_
- `async def on_call_tool(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time tool execution.
- `async def on_get_prompt(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time prompt retrieval.
- `async def on_list_prompts(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time prompt listing.
- `async def on_list_resource_templates(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time resource template listing.
- `async def on_list_resources(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time resource listing.
- `async def on_list_tools(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time tool listing.
- `async def on_read_resource(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time resource reading.

**Inherited (5)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_discover`, `on_initialize`, `on_message`, `on_notification`, `on_request`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Enhanced timing middleware with per-operation breakdowns.

Provides detailed timing information for different types of MCP operations,
allowing you to identify performance bottlenecks in specific operations.

Example:
    ```python
    from fastmcp.server.middleware.timing import DetailedTimingMiddleware
    import logging

    # Configure logging to see the output
    logging.basicConfig(level=logging.INFO)

    mcp = FastMCP("MyServer")
    mcp.add_middleware(DetailedTimingMiddleware())
    ```


## TimingMiddleware

`fastmcp.server.middleware.timing.TimingMiddleware`

```python
class TimingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (3)**

- `log_level = log_level`  _instance-attribute_
- `logger = logger or logging.getLogger('fastmcp.timing')`  _instance-attribute_
- `async def on_request(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Time request execution and log the results.

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that logs the execution time of requests.

Only measures and logs timing for request messages (not notifications).
Provides insights into performance characteristics of your MCP server.

Example:
    ```python
    from fastmcp.server.middleware.timing import TimingMiddleware

    mcp = FastMCP("MyServer")
    mcp.add_middleware(TimingMiddleware())

    # Now all requests will be timed and logged
    ```


