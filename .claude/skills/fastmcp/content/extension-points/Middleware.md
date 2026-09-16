# Middleware

Intercepting requests in flight. Twelve hooks, all async, all pass-through unless you override them.

Import as `fastmcp.server.middleware.Middleware`
Defined at `fastmcp.server.middleware.middleware.Middleware`.

```python
class Middleware
```

## Required

Nothing. Every member has a default, so a subclass that overrides none of them is valid -- and does nothing useful.

## Provided

Defaulted, and this is where the capability hides. The default is almost always the conservative answer, so an implementation that overrides none of these works correctly and supplies nothing.

```python
async def on_call_tool(self, context: MiddlewareContext[mt.CallToolRequestParams], call_next: CallNext[mt.CallToolRequestParams, ToolResult]) -> ToolResult
async def on_discover(self, context: MiddlewareContext[mt.DiscoverRequest], call_next: CallNext[mt.DiscoverRequest, mt.DiscoverResult | dict[str, Any]]) -> mt.DiscoverResult | dict[str, Any]
async def on_get_prompt(self, context: MiddlewareContext[mt.GetPromptRequestParams], call_next: CallNext[mt.GetPromptRequestParams, PromptResult]) -> PromptResult
async def on_initialize(self, context: MiddlewareContext[mt.InitializeRequest], call_next: CallNext[mt.InitializeRequest, mt.InitializeResult | None]) -> mt.InitializeResult | None
async def on_list_prompts(self, context: MiddlewareContext[mt.ListPromptsRequest], call_next: CallNext[mt.ListPromptsRequest, Sequence[Prompt]]) -> Sequence[Prompt]
async def on_list_resource_templates(self, context: MiddlewareContext[mt.ListResourceTemplatesRequest], call_next: CallNext[mt.ListResourceTemplatesRequest, Sequence[ResourceTemplate]]) -> Sequence[ResourceTemplate]
async def on_list_resources(self, context: MiddlewareContext[mt.ListResourcesRequest], call_next: CallNext[mt.ListResourcesRequest, Sequence[Resource]]) -> Sequence[Resource]
async def on_list_tools(self, context: MiddlewareContext[mt.ListToolsRequest], call_next: CallNext[mt.ListToolsRequest, Sequence[Tool]]) -> Sequence[Tool]
async def on_message(self, context: MiddlewareContext[Any], call_next: CallNext[Any, Any]) -> Any
async def on_notification(self, context: MiddlewareContext[mt.Notification[Any, Any]], call_next: CallNext[mt.Notification[Any, Any], Any]) -> Any
async def on_read_resource(self, context: MiddlewareContext[mt.ReadResourceRequestParams], call_next: CallNext[mt.ReadResourceRequestParams, ResourceResult]) -> ResourceResult
async def on_request(self, context: MiddlewareContext[mt.Request[Any, Any]], call_next: CallNext[mt.Request[Any, Any], Any]) -> Any
```

## Implementors (17)

Transitive. Read one before writing your own.

- `fastmcp.server.middleware.authorization.AuthMiddleware`
- `fastmcp.server.middleware.caching.ResponseCachingMiddleware`
- `fastmcp.server.middleware.dereference.DereferenceRefsMiddleware`
- `fastmcp.server.middleware.error_handling.ErrorHandlingMiddleware`
- `fastmcp.server.middleware.error_handling.RetryMiddleware`
- `fastmcp.server.middleware.logging.BaseLoggingMiddleware`
- `fastmcp.server.middleware.logging.LoggingMiddleware`
- `fastmcp.server.middleware.logging.StructuredLoggingMiddleware`
- `fastmcp.server.middleware.ping.PingMiddleware`
- `fastmcp.server.middleware.rate_limiting.RateLimitingMiddleware`
- `fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimitingMiddleware`
- `fastmcp.server.middleware.response_limiting.ResponseLimitingMiddleware`
- `fastmcp.server.middleware.timing.DetailedTimingMiddleware`
- `fastmcp.server.middleware.timing.TimingMiddleware`
- `fastmcp.server.middleware.tool_injection.ToolInjectionMiddleware`
- `fastmcp.server.providers.proxy.ProxyInitializeMiddleware`
- `fastmcp.server.providers.proxy.ProxyMetadataMiddleware`

## Demonstrated by 15 upstream file(s)

Matched as syntax -- `class X(Base)` -- not by searching for the name.

- [`corpus/tests/server/middleware/test_caching.py`](../corpus/tests/server/middleware/test_caching.py)
- [`corpus/tests/server/middleware/test_discovery_middleware.py`](../corpus/tests/server/middleware/test_discovery_middleware.py)
- [`corpus/tests/server/middleware/test_initialization_middleware.py`](../corpus/tests/server/middleware/test_initialization_middleware.py)
- [`corpus/tests/server/middleware/test_message_visibility.py`](../corpus/tests/server/middleware/test_message_visibility.py)
- [`corpus/tests/server/middleware/test_middleware.py`](../corpus/tests/server/middleware/test_middleware.py)
- [`corpus/tests/server/middleware/test_middleware_nested.py`](../corpus/tests/server/middleware/test_middleware_nested.py)
- [`corpus/tests/server/providers/proxy/test_proxy_headers.py`](../corpus/tests/server/providers/proxy/test_proxy_headers.py)
- [`corpus/tests/server/providers/proxy/test_proxy_server.py`](../corpus/tests/server/providers/proxy/test_proxy_server.py)
- [`corpus/tests/server/providers/proxy/test_server_metadata.py`](../corpus/tests/server/providers/proxy/test_server_metadata.py)
- [`corpus/tests/server/providers/test_fastmcp_provider.py`](../corpus/tests/server/providers/test_fastmcp_provider.py)
- [`corpus/tests/server/telemetry/test_server_tracing.py`](../corpus/tests/server/telemetry/test_server_tracing.py)
- [`corpus/tests/server/test_mrtr_guards.py`](../corpus/tests/server/test_mrtr_guards.py)

## Documentation

Prose: [`api/fastmcp.server.middleware.middleware.md`](../api/fastmcp.server.middleware.middleware.md) · records: [`model/fastmcp.server.middleware.middleware.json`](../model/fastmcp.server.middleware.middleware.json)
