# Protocol conformance

Structural conformance is not decidable by reading member names, which is why this
index refused to guess at it. It *is* decidable by asking the type checker, so each row
below is a probe it adjudicated:

```python
def _p(c: SomeClass) -> SomeProtocol:
    return c
```

No diagnostic means assignable. A `bad-return` names the obligation that failed.

45 of 74 probed pairs are assignable, across
13 protocols.

**What this does not say.** Only classes whose public members are a superset of the
protocol's were probed -- a class missing a member cannot satisfy it, so asking would
waste a probe, but it also means absence from this table is not a verdict. And the
conformance suite records that this checker deviates on **variance**
(`protocols_variance.py`, and the ParamSpec and TypeVarTuple variance tests), so a
verdict that turns on variance is worth confirming rather than trusting.

## SandboxProvider

Import as `fastmcp.experimental.transforms.code_mode.SandboxProvider`.

Satisfied by 1:

- `fastmcp.experimental.transforms.code_mode.MontySandboxProvider`

Near misses, with the obligation that failed:

- `fastmcp.apps.app.FastMCPApp` — Returned type `FastMCPApp` is not assignable to declared return type `SandboxProvider`   `FastMCPApp.run` has type `(transport: Literal['http', 'sse', 'stdio', 'streamable-http'] | None = None, **kwar
- `fastmcp.server.mixins.transport.TransportMixin` — Returned type `TransportMixin` is not assignable to declared return type `SandboxProvider`   `TransportMixin.run` has type `(transport: Literal['http', 'sse', 'stdio', 'streamable-http'] | None = None
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderTool` — Returned type `FastMCPProviderTool` is not assignable to declared return type `SandboxProvider`   `FastMCPProviderTool.run` has type `(arguments: dict[str, Any]) -> Coroutine[Unknown, Unknown, ToolRes
- `fastmcp.server.providers.openapi.components.OpenAPITool` — Returned type `OpenAPITool` is not assignable to declared return type `SandboxProvider`   `OpenAPITool.run` has type `(arguments: dict[str, Any]) -> Coroutine[Unknown, Unknown, ToolResult]`, which is
- `fastmcp.server.providers.proxy.ProxyTool` — Returned type `ProxyTool` is not assignable to declared return type `SandboxProvider`   `ProxyTool.run` has type `(arguments: dict[str, Any], context: Context | None = None) -> Coroutine[Unknown, Unkn
- `fastmcp.tools.base.Tool` — Returned type `Tool` is not assignable to declared return type `SandboxProvider`   `Tool.run` has type `(arguments: dict[str, Any]) -> Coroutine[Unknown, Unknown, ToolResult]`, which is not assignable

## ProgressLike

Import as `fastmcp.dependencies.ProgressLike`.

Satisfied by 2:

- `fastmcp.server.dependencies.InMemoryProgress`
- `fastmcp.server.dependencies.Progress`

## TokenStorage

Import as `mcp.client.auth.TokenStorage`.

Satisfied by 1:

- `fastmcp.client.auth.oauth.TokenStorageAdapter`

## ResponseCacheStore

Import as `mcp.client.ResponseCacheStore`.

Satisfied by 2:

- `fastmcp.client.caching.KeyValueResponseCacheStore`
- `mcp.client.caching.InMemoryResponseCacheStore`

Near misses, with the obligation that failed:

- `fastmcp.server.sessions.Session` — Returned type `Session` is not assignable to declared return type `ResponseCacheStore`   `Session.get` has type `(key: str, default: Any = None) -> Coroutine[Unknown, Unknown, Any]`, which is not assi

## OAuthAuthorizationServerProvider

Import as `mcp.server.auth.routes.OAuthAuthorizationServerProvider`.

Satisfied by 1:

- `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy`

## TokenVerifier

Import as `mcp.server.lowlevel.server.TokenVerifier`.

Satisfied by 18:

- `fastmcp.server.auth.auth.MultiAuth`
- `fastmcp.server.auth.auth.OAuthProvider`
- `fastmcp.server.auth.auth.RemoteAuthProvider`
- `fastmcp.server.auth.auth.TokenVerifier`
- `fastmcp.server.auth.providers.aws.AWSCognitoTokenVerifier`
- `fastmcp.server.auth.providers.clerk.ClerkTokenVerifier`
- `fastmcp.server.auth.providers.debug.DebugTokenVerifier`
- `fastmcp.server.auth.providers.discord.DiscordTokenVerifier`
- `fastmcp.server.auth.providers.github.GitHubTokenVerifier`
- `fastmcp.server.auth.providers.google.GoogleTokenVerifier`
- `fastmcp.server.auth.providers.huggingface.HuggingFaceTokenVerifier`
- `fastmcp.server.auth.providers.in_memory.InMemoryOAuthProvider`
- … and 6 more

Near misses, with the obligation that failed:

- `fastmcp.server.auth.jwt_issuer.JWTIssuer` — Returned type `JWTIssuer` is not assignable to declared return type `TokenVerifier`   `JWTIssuer.verify_token` has type `(token: str, expected_token_use: str = 'access') -> dict[str, Any]`, which is n

## RequestStateCodec

Import as `mcp.server.mcpserver.RequestStateCodec`.

Satisfied by 1:

- `mcp.server.request_state.AESGCMRequestStateCodec`

## SubscriptionBus

Import as `mcp.server.mcpserver.server.SubscriptionBus`.

Satisfied by 1:

- `mcp.server.subscriptions.InMemorySubscriptionBus`

## ReadStream

Import as `mcp.server.runner.ReadStream`.

Satisfied by 1:

- `mcp.shared._context_streams.ContextReceiveStream`

## WriteStream

Import as `mcp.server.runner.WriteStream`.

Satisfied by 1:

- `mcp.shared._context_streams.ContextSendStream`

## DispatchContext

Import as `mcp.server.runner.DispatchContext`.

Satisfied by 4:

- `mcp.server._streamable_http_modern._SingleExchangeDispatchContext`
- `mcp.server.runner._NoServerRequestsDispatchContext`
- `mcp.shared.direct_dispatcher._DirectDispatchContext`
- `mcp.shared.jsonrpc_dispatcher._JSONRPCDispatchContext`

## Dispatcher

Import as `mcp.client.client.Dispatcher`.

Satisfied by 2:

- `mcp.shared.direct_dispatcher.DirectDispatcher`
- `mcp.shared.jsonrpc_dispatcher.JSONRPCDispatcher`

Near misses, with the obligation that failed:

- `fastmcp.apps.app.FastMCPApp` — Returned type `FastMCPApp` is not assignable to declared return type `Dispatcher[Unknown]`   `FastMCPApp.run` has type `(transport: Literal['http', 'sse', 'stdio', 'streamable-http'] | None = None, **
- `fastmcp.experimental.transforms.code_mode.MontySandboxProvider` — Returned type `MontySandboxProvider` is not assignable to declared return type `Dispatcher[Unknown]`   `MontySandboxProvider.run` has type `(code: str, *, inputs: dict[str, Any] | None = None, externa
- `fastmcp.server.mixins.transport.TransportMixin` — Returned type `TransportMixin` is not assignable to declared return type `Dispatcher[Unknown]`   `TransportMixin.run` has type `(transport: Literal['http', 'sse', 'stdio', 'streamable-http'] | None =
- `fastmcp.server.providers.fastmcp_provider.FastMCPProviderTool` — Returned type `FastMCPProviderTool` is not assignable to declared return type `Dispatcher[Unknown]`   `FastMCPProviderTool.run` has type `(arguments: dict[str, Any]) -> Coroutine[Unknown, Unknown, Too
- `fastmcp.server.providers.openapi.components.OpenAPITool` — Returned type `OpenAPITool` is not assignable to declared return type `Dispatcher[Unknown]`   `OpenAPITool.run` has type `(arguments: dict[str, Any]) -> Coroutine[Unknown, Unknown, ToolResult]`, which
- `fastmcp.server.providers.proxy.ProxyTool` — Returned type `ProxyTool` is not assignable to declared return type `Dispatcher[Unknown]`   `ProxyTool.run` has type `(arguments: dict[str, Any], context: Context | None = None) -> Coroutine[Unknown,

## Outbound

Import as `mcp.shared.peer.Outbound`.

Satisfied by 10:

- `mcp.server._streamable_http_modern._SingleExchangeDispatchContext`
- `mcp.server.connection.Connection`
- `mcp.server.connection._NoChannelOutbound`
- `mcp.server.runner._NoServerRequestsDispatchContext`
- `mcp.shared.context.BaseContext`
- `mcp.shared.direct_dispatcher.DirectDispatcher`
- `mcp.shared.direct_dispatcher._DirectDispatchContext`
- `mcp.shared.jsonrpc_dispatcher.JSONRPCDispatcher`
- `mcp.shared.jsonrpc_dispatcher._JSONRPCDispatchContext`
- `mcp.shared.peer.ClientPeer`
