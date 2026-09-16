# Middleware

`fastmcp.server.middleware.Middleware` declares 12 hooks. Override the ones you need; the rest
pass through.

| Hook |
|---|
| `on_call_tool` |
| `on_discover` |
| `on_get_prompt` |
| `on_initialize` |
| `on_list_prompts` |
| `on_list_resource_templates` |
| `on_list_resources` |
| `on_list_tools` |
| `on_message` |
| `on_notification` |
| `on_read_resource` |
| `on_request` |

## Built-in middleware (17 descendants)

| Class | Import as |
|---|---|
| `AuthMiddleware` | `fastmcp.server.middleware.AuthMiddleware` |
| `ResponseCachingMiddleware` | `fastmcp.server.middleware.caching.ResponseCachingMiddleware` |
| `DereferenceRefsMiddleware` | `fastmcp.server.middleware.dereference.DereferenceRefsMiddleware` |
| `ErrorHandlingMiddleware` | `fastmcp.server.middleware.error_handling.ErrorHandlingMiddleware` |
| `RetryMiddleware` | `fastmcp.server.middleware.error_handling.RetryMiddleware` |
| `BaseLoggingMiddleware` | `fastmcp.server.middleware.logging.BaseLoggingMiddleware` |
| `LoggingMiddleware` | `fastmcp.server.middleware.logging.LoggingMiddleware` |
| `StructuredLoggingMiddleware` | `fastmcp.server.middleware.logging.StructuredLoggingMiddleware` |
| `PingMiddleware` | `fastmcp.server.middleware.PingMiddleware` |
| `RateLimitingMiddleware` | `fastmcp.server.middleware.rate_limiting.RateLimitingMiddleware` |
| `SlidingWindowRateLimitingMiddleware` | `fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimitingMiddleware` |
| `ResponseLimitingMiddleware` | `fastmcp.server.middleware.response_limiting.ResponseLimitingMiddleware` |
| `DetailedTimingMiddleware` | `fastmcp.server.middleware.timing.DetailedTimingMiddleware` |
| `TimingMiddleware` | `fastmcp.server.middleware.timing.TimingMiddleware` |
| `ToolInjectionMiddleware` | `fastmcp.server.middleware.tool_injection.ToolInjectionMiddleware` |
| `ProxyInitializeMiddleware` | `fastmcp.server.providers.proxy.ProxyInitializeMiddleware` |
| `ProxyMetadataMiddleware` | `fastmcp.server.providers.proxy.ProxyMetadataMiddleware` |

Every hook is `async`. Writing one as `def` produces a coroutine where a value was
expected, and the failure surfaces far from the definition.

## Referenced how often

From `index/usage.tsv` — how much this library uses each of these itself. A low
count is not a warning: it measures internal use, not what callers need.

| Item | Callers | References |
|---|---:|---:|
| `fastmcp.server.middleware.timing.DetailedTimingMiddleware` | 7 | 0 |
| `fastmcp.server.providers.proxy.ProxyMetadataMiddleware` | 4 | 1 |
| `fastmcp.server.middleware.authorization.AuthMiddleware` | 3 | 3 |
| `fastmcp.server.middleware.logging.BaseLoggingMiddleware` | 3 | 2 |
| `fastmcp.server.middleware.response_limiting.ResponseLimitingMiddleware` | 2 | 0 |
| `fastmcp.server.middleware.middleware.Middleware` | 1 | 34 |
| `fastmcp.server.middleware.ping.PingMiddleware` | 1 | 2 |
| `fastmcp.server.middleware.dereference.DereferenceRefsMiddleware` | 1 | 2 |
| `fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimitingMiddleware` | 1 | 0 |
| `fastmcp.server.middleware.rate_limiting.RateLimitingMiddleware` | 1 | 0 |
| `fastmcp.server.middleware.error_handling.RetryMiddleware` | 1 | 0 |
| `fastmcp.server.middleware.error_handling.ErrorHandlingMiddleware` | 1 | 0 |
