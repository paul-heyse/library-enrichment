# Where the two engines disagree

`inferred.tsv` holds types the source does not state. They come from **ty**, asked by
hover at each definition site. The pinned checker answers the module-level ones too, in
the batch report it produces anyway, so every row can carry a second opinion.

188 agree · 92 where ty is narrower · 103 differ · 62 the checker did not answer.

**92 of these are precision, not conflict.** ty narrows a constant to its literal type
where the checker gives the widened base. Both are right. Code
written against the narrow answer will not type-check under the other engine, which is why they are
counted separately rather than folded into either bucket.

`unanswered` is not disagreement. ty was asked about class attributes and function
returns as well as module-level names; the checker's `global_variables` covers only the
last of those, so most rows have nothing to compare against.

Comparison strips each dotted name to its leaf and drops rendering noise, because the
two engines spell types differently by design -- `Logger` against `logging.Logger` is
agreement, not conflict. A row below is a real difference of opinion about the type.

| Item | ty | checker |
|---|---|---|
| `fastmcp.cli.deploy.command.HostOption` | ````xml <special-form 'typing.Annotated[str | None, <metadata` | `typing.Annotated[str | None]` |
| `fastmcp.cli.deploy.command.JsonOption` | ````xml <special-form 'typing.Annotated[bool, <metadata>]'> `` | `typing.Annotated[bool]` |
| `fastmcp.cli.deploy.credentials.CredentialSource` | ````xml <special-form 'Literal["environment", "stored", "inte` | `type[typing.Literal['environment', 'interactive', 'stored']]` |
| `fastmcp.cli.deploy.horizon_client.DeviceTokenError` | ````xml <special-form 'Literal["authorization_pending", "slow` | `type[typing.Literal['access_denied', 'authorization_pending'` |
| `fastmcp.cli.deploy.output.CommandName` | ````xml <special-form 'Literal["login", "logout", "whoami"]'>` | `type[typing.Literal['login', 'logout', 'whoami']]` |
| `fastmcp.cli.deploy.output.ErrorCategory` | ````xml <special-form 'Literal["authentication_invalid", "aut` | `type[typing.Literal['authentication_invalid', 'authenticatio` |
| `fastmcp.cli.run.LogLevelType` | ````xml <special-form 'Literal["DEBUG", "INFO", "WARNING", "E` | `type[typing.Literal['CRITICAL', 'DEBUG', 'ERROR', 'INFO', 'W` |
| `fastmcp.cli.run.TransportType` | ````xml <special-form 'Literal["stdio", "http", "sse", "strea` | `type[typing.Literal['http', 'sse', 'stdio', 'streamable-http` |
| `fastmcp.client.client.ConnectMode` | ````xml <types.UnionType special-form 'str'> ``` --- How the ` | `type[str]` |
| `fastmcp.client.extension_hooks.InternalClientExtensionFactory` | ````xml <Callable special-form '(ElicitationFnT | None, /) ->` | `type[(mcp.client.session.ElicitationFnT | None) -> mcp.clien` |
| `fastmcp.client.transports.base.SessionKwargs` | ````xml <class 'ClientSessionKwargs'> ```` | `type[fastmcp.client.transports.base.ClientSessionKwargs]` |
| `fastmcp.contrib.bulk_tool_caller.example.mcp` | `FastMCP[Any]` | `fastmcp.server.server.FastMCP` |
| `fastmcp.contrib.component_manager.example.mcp` | `FastMCP[Any]` | `fastmcp.server.server.FastMCP` |
| `fastmcp.contrib.component_manager.example.mounted` | `FastMCP[Any]` | `fastmcp.server.server.FastMCP` |
| `fastmcp.contrib.mcp_mixin.example.mcp` | `FastMCP[Any]` | `fastmcp.server.server.FastMCP` |
| `fastmcp.decorators.FastMCPMeta` | ````xml <types.UnionType special-form 'ToolMeta | ResourceMet` | `type[fastmcp.prompts.function_prompt.PromptMeta | fastmcp.re` |
| `fastmcp.exceptions.FastMCPDeprecationWarning` | ````xml <class 'FastMCPDeprecationWarning'> ```` | `type[fastmcp._warnings.FastMCPDeprecationWarning]` |
| `fastmcp.exceptions.McpError` | ````xml <class 'mcp.shared.exceptions.MCPError'> | <class 'fa` | `type[fastmcp.exceptions.MCPError] | type[mcp.shared.exceptio` |
| `fastmcp.experimental.transforms.code_mode.DiscoveryToolFactory` | ````xml <Callable special-form '((Context, /) -> Awaitable[Se` | `type[((fastmcp.server.context.Context) -> typing.Awaitable[t` |
| `fastmcp.experimental.transforms.code_mode.GetToolCatalog` | ````xml <Callable special-form '(Context, /) -> Awaitable[Seq` | `type[(fastmcp.server.context.Context) -> typing.Awaitable[ty` |
| `fastmcp.experimental.transforms.code_mode.SearchFn` | ````xml <Callable special-form '( Sequence[Tool], str, / ) ->` | `type[(typing.Sequence[fastmcp.tools.base.Tool], str) -> typi` |
| `fastmcp.experimental.transforms.code_mode.ToolDetailLevel` | ````xml <special-form 'Literal["brief", "detailed", "full"]'>` | `type[typing.Literal['brief', 'detailed', 'full']]` |
| `fastmcp.mcp_config.CanonicalMCPServerTypes` | ````xml <types.UnionType special-form 'StdioMCPServer | Remot` | `type[fastmcp.mcp_config.RemoteMCPServer | fastmcp.mcp_config` |
| `fastmcp.mcp_config.MCPServerTypes` | ````xml <types.UnionType special-form 'StdioMCPServer | Remot` | `type[fastmcp.mcp_config.RemoteMCPServer | fastmcp.mcp_config` |
| `fastmcp.mcp_config.TransformingMCPServerTypes` | ````xml <types.UnionType special-form 'TransformingStdioMCPSe` | `type[fastmcp.mcp_config.TransformingRemoteMCPServer | fastmc` |
| `fastmcp.server.auth.providers.introspection.ClientAuthMethod` | ````xml <special-form 'Literal["client_secret_basic", "client` | `type[typing.Literal['client_secret_basic', 'client_secret_po` |
| `fastmcp.server.caching.CacheScope` | ````xml <special-form 'Literal["public", "private"]'> ``` ---` | `type[typing.Literal['private', 'public']]` |
| `fastmcp.server.completions.CompletionHandler` | ````xml <Callable special-form '( PromptReference | ResourceT` | `type[(fastmcp.server.completions.CompletionReference, mcp_ty` |
| `fastmcp.server.completions.CompletionReference` | ````xml <types.UnionType special-form 'PromptReference | Reso` | `type[mcp_types._types.PromptReference | mcp_types._types.Res` |
| `fastmcp.server.completions.CompletionValues` | ````xml <types.UnionType special-form 'Completion | list[str]` | `type[mcp_types._types.Completion | list[str] | tuple[str, ..` |
| `fastmcp.server.context.TransportType` | ````xml <special-form 'Literal["stdio", "sse", "streamable-ht` | `type[typing.Literal['sse', 'stdio', 'streamable-http']]` |
| `fastmcp.server.http.HostOriginProtection` | ````xml <types.UnionType special-form 'bool | Literal["auto"]` | `type[typing.Literal['auto'] | bool]` |
| `fastmcp.server.http.HostOriginProtectionMode` | ````xml <special-form 'Literal["auto", "strict"]'> ```` | `type[typing.Literal['auto', 'strict']]` |
| `fastmcp.server.lifespan.LifespanContextManagerFn` | ````xml <Callable special-form '(FastMCP[Any], /) -> Abstract` | `type[(fastmcp.server.server.FastMCP) -> contextlib.AbstractA` |
| `fastmcp.server.lifespan.LifespanFn` | ````xml <Callable special-form '(FastMCP[Any], /) -> AsyncIte` | `type[(fastmcp.server.server.FastMCP) -> typing.AsyncIterator` |
| `fastmcp.server.middleware.middleware.MiddlewarePhase` | ````xml <special-form 'Literal["all", "outer", "typed"]'> ```` | `type[typing.Literal['all', 'outer', 'typed']]` |
| `fastmcp.server.providers.aggregate.ProviderErrorStrategy` | ````xml <special-form 'Literal["warn", "raise"]'> ```` | `type[typing.Literal['raise', 'warn']]` |
| `fastmcp.server.providers.local_provider.decorators.tools.DuplicateBehavior` | ````xml <special-form 'Literal["error", "warn", "replace", "i` | `type[typing.Literal['error', 'ignore', 'replace', 'warn']]` |
| `fastmcp.server.providers.local_provider.local_provider.DuplicateBehavior` | ````xml <special-form 'Literal["error", "warn", "replace", "i` | `type[typing.Literal['error', 'ignore', 'replace', 'warn']]` |
| `fastmcp.server.providers.openapi.routing.ComponentFn` | ````xml <Callable special-form '( HTTPRoute, OpenAPITool | Op` | `type[(fastmcp.utilities.openapi.models.HTTPRoute, fastmcp.se` |

… and 63 more; see column 5 of `inferred.tsv`.

Neither answer is a promise upstream made. An inferred type is correct at this pin and
can change without a release note, which is why these rows are separated from the
declared ones rather than merged into them.
