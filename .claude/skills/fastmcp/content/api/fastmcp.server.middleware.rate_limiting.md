# `fastmcp.server.middleware.rate_limiting`

Distribution: `fastmcp`

## RateLimitError

`fastmcp.server.middleware.rate_limiting.RateLimitError`

```python
class RateLimitError(MCPError)
```

**Bases** `MCPError`

Error raised when rate limit is exceeded.


## RateLimitingMiddleware

`fastmcp.server.middleware.rate_limiting.RateLimitingMiddleware`

```python
class RateLimitingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (7)**

- `burst_capacity = burst_capacity or int(max_requests_per_second * 2)`  _instance-attribute_
- `get_client_id = get_client_id`  _instance-attribute_
- `global_limit = global_limit`  _instance-attribute_
- `global_limiter = TokenBucketRateLimiter(self.burst_capacity, self.max_requests_per_second)`  _instance-attribute_
- `limiters: dict[str, TokenBucketRateLimiter] = defaultdict(lambda: TokenBucketRateLimiter(self.burst_capacity, self.max_requests_per_second))`  _instance-attribute_
- `max_requests_per_second = max_requests_per_second`  _instance-attribute_
- `async def on_request(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Apply rate limiting to requests.

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that implements rate limiting to prevent server abuse.

Uses a token bucket algorithm by default, allowing for burst traffic
while maintaining a sustainable long-term rate.

Example:
    ```python
    from fastmcp.server.middleware.rate_limiting import RateLimitingMiddleware

    # Allow 10 requests per second with bursts up to 20
    rate_limiter = RateLimitingMiddleware(
        max_requests_per_second=10,
        burst_capacity=20
    )

    mcp = FastMCP("MyServer")
    mcp.add_middleware(rate_limiter)
    ```


## SlidingWindowRateLimiter

`fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimiter`

```python
class SlidingWindowRateLimiter
```

**Declared members (4)**

- `async def is_allowed(self) -> bool`  _async_
  Check if a request is allowed.
- `max_requests = max_requests`  _instance-attribute_
- `requests = deque()`  _instance-attribute_
- `window_seconds = window_seconds`  _instance-attribute_

Sliding window rate limiter implementation.


## SlidingWindowRateLimitingMiddleware

`fastmcp.server.middleware.rate_limiting.SlidingWindowRateLimitingMiddleware`

```python
class SlidingWindowRateLimitingMiddleware(Middleware)
```

**Bases** `Middleware`

**Declared members (5)**

- `get_client_id = get_client_id`  _instance-attribute_
- `limiters: dict[str, SlidingWindowRateLimiter] = defaultdict(lambda: SlidingWindowRateLimiter(self.max_requests, self.window_seconds))`  _instance-attribute_
- `max_requests = max_requests`  _instance-attribute_
- `async def on_request(self, context: MiddlewareContext, call_next: CallNext) -> Any`  _async_
  Apply sliding window rate limiting to requests.
- `window_seconds = window_minutes * 60`  _instance-attribute_

**Inherited (11)**

- from `fastmcp.server.middleware.middleware.Middleware`: `on_call_tool`, `on_discover`, `on_get_prompt`, `on_initialize`, `on_list_prompts`, `on_list_resource_templates`, `on_list_resources`, `on_list_tools`, `on_message`, `on_notification`, `on_read_resource`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Middleware that implements sliding window rate limiting.

Uses a sliding window approach which provides more precise rate limiting
but uses more memory to track individual request timestamps.

Example:
    ```python
    from fastmcp.server.middleware.rate_limiting import SlidingWindowRateLimitingMiddleware

    # Allow 100 requests per minute
    rate_limiter = SlidingWindowRateLimitingMiddleware(
        max_requests=100,
        window_minutes=1
    )

    mcp = FastMCP("MyServer")
    mcp.add_middleware(rate_limiter)
    ```


## TokenBucketRateLimiter

`fastmcp.server.middleware.rate_limiting.TokenBucketRateLimiter`

```python
class TokenBucketRateLimiter
```

**Declared members (5)**

- `capacity = capacity`  _instance-attribute_
- `async def consume(self, tokens: int = 1) -> bool`  _async_
  Try to consume tokens from the bucket.
- `last_refill = time.time()`  _instance-attribute_
- `refill_rate = refill_rate`  _instance-attribute_
- `tokens = capacity`  _instance-attribute_

Token bucket implementation for rate limiting.


