# The API you will actually meet

3,056 symbols is a flat list until something says which ones matter. This ranks by
how often the library itself references and calls each item, weighting callers above
bare references because a call is a stronger signal of a contract than a mention.

It is a measurement of *this library's own use of itself*, not of what callers do.
Something rare here can still be exactly what your code needs -- `create_proxy` is
called eight times upstream and is the answer to a common question.

| Item | Import as | Callers | References |
|---|---|---:|---:|
| `fastmcp.utilities.logging.get_logger` | `fastmcp.settings.get_logger` | 103 | 203 |
| `fastmcp.server.server.FastMCP` | `fastmcp.FastMCP` | 89 | 152 |
| `fastmcp.tools.base.Tool` | `fastmcp.tools.Tool` | 40 | 273 |
| `fastmcp.client.client.Client` | `fastmcp.Client` | 69 | 90 |
| `mcp.client.session.ClientSession` | `mcp.ClientSession` | 71 | 72 |
| `mcp.shared.exceptions.MCPError` | `mcp.MCPError` | 39 | 141 |
| `mcp_types._wire_base.WireModel` | *not importable* | 0 | 296 |
| `fastmcp.utilities.versions.VersionSpec` | `fastmcp.server.transforms.VersionSpec` | 35 | 129 |
| `fastmcp.server.middleware.middleware.CallNext` | `fastmcp.server.middleware.CallNext` | 44 | 78 |
| `fastmcp.server.context.Context` | `fastmcp.Context` | 28 | 87 |
| `fastmcp.resources.base.Resource` | `fastmcp.resources.Resource` | 16 | 134 |
| `fastmcp.server.providers.local_provider.local_provider.LocalProvider` | `fastmcp.server.providers.LocalProvider` | 41 | 34 |
| `fastmcp.server.providers.base.Provider` | `fastmcp.server.providers.Provider` | 37 | 42 |
| `fastmcp.resources.template.ResourceTemplate` | `fastmcp.resources.ResourceTemplate` | 16 | 110 |
| `mcp_types.jsonrpc.ErrorData` | `mcp_types.ErrorData` | 23 | 76 |
| `fastmcp.prompts.base.Prompt` | `fastmcp.prompts.Prompt` | 12 | 102 |
| `fastmcp.server.auth.oauth_proxy.proxy.OAuthProxy` | `fastmcp.server.auth.OAuthProxy` | 27 | 42 |
| `fastmcp.utilities.auth.parse_scopes` | `fastmcp.server.auth.providers.aws.parse_scopes` | 23 | 47 |
| `mcp.server.context.ServerRequestContext` | `mcp.server.ServerRequestContext` | 2 | 131 |
| `mcp_types._types.TextContent` | `mcp_types.TextContent` | 18 | 67 |
| `fastmcp.server.middleware.middleware.MiddlewareContext` | `fastmcp.server.middleware.MiddlewareContext` | 9 | 102 |
| `mcp.server.session.ServerSession` | `mcp.ServerSession` | 28 | 26 |
| `mcp.shared.message.SessionMessage` | `mcp.client.sse.SessionMessage` | 12 | 86 |
| `mcp_types._types.InputRequiredResult` | `mcp_types.InputRequiredResult` | 2 | 117 |
| `fastmcp.utilities.tasks.TaskConfig` | `fastmcp.decorators.TaskConfig` | 20 | 44 |
| `fastmcp.server.auth.auth.AccessToken` | `fastmcp.server.auth.AccessToken` | 15 | 58 |
| `fastmcp.utilities.authorization.AuthContext` | `fastmcp.server.auth.AuthContext` | 20 | 37 |
| `fastmcp.resources.base.ResourceResult` | `fastmcp.resources.ResourceResult` | 12 | 68 |
| `mcp.server.streamable_http.StreamableHTTPServerTransport` | `mcp.server.streamable_http_manager.StreamableHTTPServerTransport` | 27 | 7 |
| `fastmcp.utilities.authorization.AuthCheck` | `fastmcp.server.auth.AuthCheck` | 0 | 114 |
| `fastmcp.tools.base.ToolResult` | `fastmcp.tools.ToolResult` | 10 | 73 |
| `mcp.server.mcpserver.server.MCPServer` | `mcp.server.MCPServer` | 15 | 53 |
| `mcp.server.lowlevel.server.LifespanResultT` | `mcp.server.mcpserver.server.LifespanResultT` | 0 | 109 |
| `mcp.server.lowlevel.server.Server` | `mcp.server.Server` | 15 | 43 |
| `mcp.shared._context_streams.ContextSendStream` | `mcp.server.sse.ContextSendStream` | 21 | 16 |
| `fastmcp.server.auth.providers.jwt.JWTVerifier` | `fastmcp.server.auth.JWTVerifier` | 15 | 37 |
| `mcp.server.connection.Connection` | `mcp.server.runner.Connection` | 18 | 24 |
| `mcp.shared.auth.OAuthClientInformationFull` | `mcp.client.auth.utils.OAuthClientInformationFull` | 9 | 60 |
| `mcp.server.mcpserver.context.Context` | `mcp.server.mcpserver.Context` | 14 | 38 |
| `mcp.shared.jsonrpc_dispatcher.JSONRPCDispatcher` | `mcp.client.client.JSONRPCDispatcher` | 19 | 17 |
