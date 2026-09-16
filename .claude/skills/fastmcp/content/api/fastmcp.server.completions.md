# `fastmcp.server.completions`

Distribution: `fastmcp`

## CompletionHandler

Import as `fastmcp.server.server.CompletionHandler`  ·  defined at `fastmcp.server.completions.CompletionHandler`

```python
CompletionHandler = Callable[[CompletionReference, mcp_types.CompletionArgument, mcp_types.CompletionContext | None], Awaitable[CompletionValues] | CompletionValues]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '( PromptReference | ResourceTemplateReference, CompletionArgument, CompletionContext | None, / ) -> Awaitable[Completion | list[str] | tuple[str, ...] | None] | Completion | list[str] | tuple[str, ...] | None'> ``` --- A server's completion handler. Called with the reference, the argument being completed, and the optional context of already-supplied argument values. May be sync or async.`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A server's completion handler.

Called with the reference, the argument being completed, and the optional
context of already-supplied argument values. May be sync or async.


## CompletionReference

`fastmcp.server.completions.CompletionReference`

```python
CompletionReference = mcp_types.PromptReference | mcp_types.ResourceTemplateReference
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'PromptReference | ResourceTemplateReference'> ``` --- The reference a completion request targets: a prompt or a resource template.`

The reference a completion request targets: a prompt or a resource template.


## CompletionValues

Import as `fastmcp.server.mixins.mcp_operations.CompletionValues`  ·  defined at `fastmcp.server.completions.CompletionValues`

```python
CompletionValues = mcp_types.Completion | list[str] | tuple[str, ...] | None
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'Completion | list[str] | tuple[str, ...] | None'> ``` --- What a completion handler may return. - ``Completion`` — used verbatim (carries the optional ``total`` / ``has_more`` &nbsp;&nbsp;pagination hints). - ``list[str]`` / ``tuple[str, ...]`` — wrapped into a ``Completion``. A bare &nbsp;&nbsp;``str`` is deliberately excluded: it satisfies ``Sequence[str]`` but is almost &nbsp;&nbsp;always a mistake, and ``normalize_completion`` rejects it at runtime — naming &nbsp;&nbsp;concrete collections keeps the annotation and the runtime guard in agreement. - ``None`` — treated as "no candidates" (an empty completion).`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

What a completion handler may return.

- ``Completion`` — used verbatim (carries the optional ``total`` / ``has_more``
  pagination hints).
- ``list[str]`` / ``tuple[str, ...]`` — wrapped into a ``Completion``. A bare
  ``str`` is deliberately excluded: it satisfies ``Sequence[str]`` but is almost
  always a mistake, and ``normalize_completion`` rejects it at runtime — naming
  concrete collections keeps the annotation and the runtime guard in agreement.
- ``None`` — treated as "no candidates" (an empty completion).


## MAX_COMPLETION_VALUES

`fastmcp.server.completions.MAX_COMPLETION_VALUES`

```python
MAX_COMPLETION_VALUES = 100
```

**Inferred type** (`ty`, not declared in the source): `Literal[100]`

## normalize_completion

Import as `fastmcp.server.mixins.mcp_operations.normalize_completion`  ·  defined at `fastmcp.server.completions.normalize_completion`

```python
def normalize_completion(result: CompletionValues) -> mcp_types.Completion
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Coerce a handler's return value into a wire ``Completion``.

A returned ``str`` is rejected: it is almost always a mistake (the value
would iterate into one-character candidates), so it raises rather than
silently producing surprising output.

The MCP contract caps a completion at 100 values, so a longer result is
truncated to the first 100 with ``has_more`` set — a handler that returns
thousands of matches emits a conforming response rather than an oversized
one that strict clients reject.


