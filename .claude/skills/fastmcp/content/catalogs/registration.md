# Registration surface

How components get into a server, and what each decorator leaves behind.

## What the decorators return

`@mcp.tool` returns **the function unchanged**, stamped with a `__fastmcp__` marker.
It does not return a `FunctionTool`. The `FunctionTool` is constructed later, inside
`add_tool`. This matters because the implementation signature says otherwise:

```
implementation:  -> Callable[[AnyFunction], FunctionTool] | FunctionTool | partial[...]
both overloads:  -> F        (the decorated function, unchanged)
runtime:         <class 'function'>, with __fastmcp__ set
```

Three sources, three answers. The overloads and the runtime agree; the implementation
annotation is the odd one out, and it is the one a naive index would publish.

Consequence for your code: the decorated name stays callable as an ordinary function,
so unit-testing it directly works, and type checkers see the original signature.

## Decorators on `FastMCP`

| Decorator | Signature |
|---|---|
| `@mcp.completion` | `def completion(self, handler: CompletionHandler | None = None) -> CompletionHandler | Callable[[CompletionHandler], CompletionHandler]` |
| `@mcp.custom_route` | `def custom_route(self: FastMCP, path: str, methods: list[str], name: str | None = None, include_in_schema: bool = True) -> Callable[[Callable[[Request], Awaitable[Response]]], Callable[[Request], Awaitable[Response]]]` |
| `@mcp.prompt` | `def prompt(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, meta: dict[str, Any] | None = None, auth: AuthCheck | list[AuthCheck] | None = None) -> Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt | partial[Callable[[AnyFunction], FunctionPrompt] | FunctionPrompt]` |
| `@mcp.resource` | `def resource(self, uri: str, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, mime_type: str | None = None, tags: set[str] | None = None, annotations: Annotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, app: AppConfig | dict[str, Any] | bool | None = None, auth: AuthCheck | list[AuthCheck] | None = None, security: ResourceSecurity | None | InheritSecurity = INHERIT_SECURITY) -> Callable[[F], F]` |
| `@mcp.tool` | `def tool(self, name_or_fn: str | AnyFunction | None = None, name: str | None = None, version: str | int | None = None, title: str | None = None, description: str | None = None, icons: list[mcp_types.Icon] | None = None, tags: set[str] | None = None, output_schema: dict[str, Any] | NotSetT | None = NotSet, annotations: ToolAnnotations | dict[str, Any] | None = None, meta: dict[str, Any] | None = None, app: AppConfig | dict[str, Any] | bool | None = None, task: bool | TaskConfig | None = None, timeout: float | None = None, auth: AuthCheck | list[AuthCheck] | None = None, run_in_thread: bool = True) -> Callable[[AnyFunction], FunctionTool] | FunctionTool | partial[Callable[[AnyFunction], FunctionTool] | FunctionTool]` |

Each has an imperative twin (`add_tool`, `add_resource`, `add_template`, `add_prompt`,
`add_middleware`, `add_provider`, `add_transform`, `add_extension`) for registering
something you did not write with a decorator.

## One URI rule worth knowing

`@mcp.resource("res://x")` and `@mcp.resource("res://{id}")` produce **different**
runtime objects -- a resource and a resource template -- discriminated by whether the
URI carries a parameter. Nothing in the signature says so.

## Full keyword detail

Read `content/api/fastmcp.server.server.md` for every keyword, type and default, and
`content/index/overloads.tsv` for the overload set of any dispatching callable.
