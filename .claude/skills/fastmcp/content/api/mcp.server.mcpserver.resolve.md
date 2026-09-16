# `mcp.server.mcpserver.resolve`

Distribution: `mcp`

## T

`mcp.server.mcpserver.resolve.T`

```python
T = TypeVar('T', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## _ELICITATION_RESULT_MEMBERS

`mcp.server.mcpserver.resolve._ELICITATION_RESULT_MEMBERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ELICITATION_RESULT_MEMBERS = (AcceptedElicitation, DeclinedElicitation, CancelledElicitation)
```

## _INPUT_REQUIRED_VERSION

`mcp.server.mcpserver.resolve._INPUT_REQUIRED_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_INPUT_REQUIRED_VERSION = '2026-07-28'
```

## _Marker

`mcp.server.mcpserver.resolve._Marker`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Marker = Elicit[Any] | Sample | ListRoots
```

The request markers a resolver may return.


## _STATE_VERSION

`mcp.server.mcpserver.resolve._STATE_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STATE_VERSION = 3
```

## __all__

`mcp.server.mcpserver.resolve.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['Resolve', 'Elicit', 'Sample', 'ListRoots', 'ElicitationResult', 'AcceptedElicitation', 'DeclinedElicitation', 'CancelledElicitation', 'find_resolved_parameters', 'build_resolver_plans', 'resolve_arguments', 'returns_input_required']
```

## logger

`mcp.server.mcpserver.resolve.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## Elicit

Import as `mcp.server.mcpserver.Elicit`  ·  defined at `mcp.server.mcpserver.resolve.Elicit`

```python
class Elicit(Generic[T])
```

**Also exported as** `mcp.server.mcpserver.Elicit`

**Bases** `Generic[T]`

**Declared members (1)**

- `message = message`  _instance-attribute_

A resolver's request to ask the client.

Returned from a resolver to signal that the value must be elicited. The
framework runs `ctx.elicit(message, schema)` and injects the outcome.


## ListRoots

Import as `mcp.server.mcpserver.ListRoots`  ·  defined at `mcp.server.mcpserver.resolve.ListRoots`

```python
class ListRoots
```

**Also exported as** `mcp.server.mcpserver.ListRoots`

A resolver's request for the client's roots via `roots/list`; the framework injects the `ListRootsResult`.


## Resolve

Import as `mcp.server.mcpserver.Resolve`  ·  defined at `mcp.server.mcpserver.resolve.Resolve`

```python
class Resolve
```

**Also exported as** `mcp.server.mcpserver.Resolve`

**Declared members (1)**

- `fn = fn`  _instance-attribute_

Marker for `Annotated[T, Resolve(fn)]`: fill the parameter by running `fn`.


## Sample

Import as `mcp.server.mcpserver.Sample`  ·  defined at `mcp.server.mcpserver.resolve.Sample`

```python
class Sample
```

**Also exported as** `mcp.server.mcpserver.Sample`

**Declared members (1)**

- `params = CreateMessageRequestParams(messages=messages, max_tokens=max_tokens, system_prompt=system_prompt, include_context=include_context, temperature=temperature, stop_sequences=stop_sequences, metadata=metadata, model_preferences=model_preferences, tools=tools, tool_choice=tool_choice)`  _instance-attribute_

A resolver's request to sample the client's LLM via `sampling/createMessage`.

The framework injects a `CreateMessageResult` (`CreateMessageResultWithTools` when `tools` or
`tool_choice` are given, which also requires the client's `sampling.tools`); requires the
`sampling` capability. On >= 2026-07-28 the request must render identically across retry
rounds, and the sampled result rides `request_state` on every later round. `include_context`
other than "none" is deprecated in the draft spec.


## _ParamPlan

`mcp.server.mcpserver.resolve._ParamPlan`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ParamPlan
```

**Declared members (3)**

- `kind: str = kind`  _instance-attribute_
- `resolve: Resolve | None = resolve`  _instance-attribute_
- `wants_union: bool = wants_union`  _instance-attribute_

How to fill one resolver parameter, decided once at registration.


## _Pending

`mcp.server.mcpserver.resolve._Pending`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Pending(Exception)
```

**Bases** `Exception`

Internal: a resolver needs client input not yet available this round.


## _Resolution

`mcp.server.mcpserver.resolve._Resolution`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Resolution
```

**Declared members (10)**

- `answers: InputResponses = context.input_responses or {} if input_required else {}`  _instance-attribute_
- `asked = decoded.asked`  _instance-attribute_
- `cache: dict[Hashable, ElicitationResult[Any]] = {}`  _instance-attribute_
- `context = context`  _instance-attribute_
- `input_required = input_required`  _instance-attribute_
- `pending: InputRequests = {}`  _instance-attribute_
- `persist: dict[str, _StateEntry] = {}`  _instance-attribute_
- `plans = plans`  _instance-attribute_
- `state = decoded.outcomes`  _instance-attribute_
- `tool_args = tool_args`  _instance-attribute_

Per-`tools/call` resolution state, shared across the DAG walk.

`input_required` selects the transport: at >= 2026-07-28 requests are
batched into `pending` and surfaced as an `InputRequiredResult`; at older
revisions each marker is answered synchronously over the back-channel.


## _ResolverPlan

`mcp.server.mcpserver.resolve._ResolverPlan`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ResolverPlan
```

**Declared members (4)**

- `fn = fn`  _instance-attribute_
- `is_async = is_async`  _instance-attribute_
- `params = params`  _instance-attribute_
- `wire_key = wire_key`  _instance-attribute_

A resolver's parameters and whether it is async, analyzed once.


## _State

`mcp.server.mcpserver.resolve._State`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _State(BaseModel)
```

**Bases** `BaseModel`

**Declared members (3)**

- `asked: dict[str, str] = {}`  _class-attribute, instance-attribute_
  Question digest of each elicitation asked last round, keyed by wire key.
- `outcomes: dict[str, _StateEntry] = {}`  _class-attribute, instance-attribute_
- `v: int`  _instance-attribute_

The decoded `request_state`: resolver progress from earlier rounds.


## _StateEntry

`mcp.server.mcpserver.resolve._StateEntry`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _StateEntry(BaseModel)
```

**Bases** `BaseModel`

**Declared members (3)**

- `action: Literal['accept', 'decline', 'cancel']`  _instance-attribute_
- `data: Any = None`  _class-attribute, instance-attribute_
- `q: str | None = None`  _class-attribute, instance-attribute_
  Digest of the exact rendered question this outcome answered.

One resolver's recorded outcome inside `request_state`.


## _accepted

`mcp.server.mcpserver.resolve._accepted`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _accepted(data: Any) -> AcceptedElicitation[Any]
```

Wrap a resolved value as an accepted outcome without schema validation.

A resolver may return any type (the schema bound only constrains `Elicit[T]`),
and a value restored from `request_state` is already validated.


## _check_elicit_return

`mcp.server.mcpserver.resolve._check_elicit_return`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _check_elicit_return(return_annotation: Any, name: str) -> None
```

Validate the request-marker arms of a resolver's return annotation.

Raises:
    InvalidSignature: If the annotation has more than one marker arm.


## _contains_resolve

`mcp.server.mcpserver.resolve._contains_resolve`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _contains_resolve(annotation: Any) -> bool
```

True when a `Resolve` marker is nested inside `annotation` (e.g. a union member).


## _decode_state

`mcp.server.mcpserver.resolve._decode_state`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _decode_state(request_state: str | None) -> _State
```

Decode the per-call resolution progress from `request_state`.

Parsed with stdlib `json.loads` because `_encode_state` may emit escaped
lone surrogates, which pydantic's JSON parser rejects. The string arrives
boundary-authenticated, so malformed content or a version mismatch is
drift within the operator's own fleet (e.g. a rolling upgrade) and is
treated as "no progress yet".


## _encode_state

`mcp.server.mcpserver.resolve._encode_state`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _encode_state(outcomes: Mapping[str, _StateEntry], asked: Mapping[str, str]) -> str
```

Encode recorded outcomes and asked-question digests for the next round.

Outcome entries are already wire-shaped, so encoding is pure wrapping.


## _fulfil

`mcp.server.mcpserver.resolve._fulfil`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _fulfil(marker: _Marker, key: str, res: _Resolution) -> ElicitationResult[Any]
```

Turn a resolver's request marker into an outcome via the negotiated transport.


## _has_input_required_arm

`mcp.server.mcpserver.resolve._has_input_required_arm`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_input_required_arm(annotation: Any) -> bool
```

Walk an annotation's arms through `Annotated`, type aliases, and unions.


## _is_context_annotation

`mcp.server.mcpserver.resolve._is_context_annotation`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_context_annotation(annotation: Any) -> bool
```

## _is_marker

`mcp.server.mcpserver.resolve._is_marker`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_marker(value: Any) -> TypeGuard[_Marker]
```

## _is_union

`mcp.server.mcpserver.resolve._is_union`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_union(annotation: Any) -> bool
```

## _outcome_from_state

`mcp.server.mcpserver.resolve._outcome_from_state`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _outcome_from_state(entry: _StateEntry, marker: _Marker) -> ElicitationResult[Any]
```

Rebuild an outcome from a decoded `request_state` entry.

Raises:
    ValidationError: If the entry does not fit the live marker.


## _render_request

`mcp.server.mcpserver.resolve._render_request`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _render_request(marker: _Marker) -> InputRequest
```

Render a marker as its wire request - the same shape on both transports.


## _request_digest

`mcp.server.mcpserver.resolve._request_digest`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _request_digest(request: InputRequest) -> str
```

Pin an outcome to the exact rendered question the client was shown.

A redeploy that rewords or reshapes a question re-asks it instead of reusing the recorded answer.


## _require_capability

`mcp.server.mcpserver.resolve._require_capability`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _require_capability(context: Context[Any, Any], marker: _Marker, key: str) -> None
```

Assert the client declared the capability `marker`'s request needs.

A bare `elicitation: {}` (the only shape before modes existed) counts as form support; url-only does not.

Raises:
    MCPError: With code `MISSING_REQUIRED_CLIENT_CAPABILITY` and a
        `requiredCapabilities` payload when the capability is not declared.


## _resolve

`mcp.server.mcpserver.resolve._resolve`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _resolve(fn: Callable[..., Any], res: _Resolution) -> ElicitationResult[Any]
```

Resolve one resolver, deduped within the call by its resolver identity.

Raises `_Pending` when the resolver (or one of its dependencies) needs client
input that has not arrived yet.


## _resolve_marker

`mcp.server.mcpserver.resolve._resolve_marker`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolve_marker(annotation: Any) -> tuple[Resolve | None, bool]
```

## _resolver_key

`mcp.server.mcpserver.resolve._resolver_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolver_key(fn: Callable[..., Any]) -> Hashable
```

Identity key for memoizing a resolver.

A bound method - pure-python (`inspect.ismethod`) or built-in (e.g. `obj.meth`
on a C-extension type) - is recreated on each attribute access, so `id(fn)`
differs every time. Key it by its underlying function (or name) plus its
`__self__` identity so `auth.login` referenced in two places memoizes to one
call. Everything else keys by `id`, so two distinct callables never collide
even if they compare equal.


## _resolver_name

`mcp.server.mcpserver.resolve._resolver_name`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _resolver_name(fn: Callable[..., Any]) -> str
```

Best-effort display name for error messages (callable objects lack `__name__`).


## _restore_outcome

`mcp.server.mcpserver.resolve._restore_outcome`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _restore_outcome(res: _Resolution, key: str, marker: _Marker, q: str) -> ElicitationResult[Any] | None
```

Restore `key`'s recorded outcome from a prior round, or `None` when absent.

An entry pinned to a question digest other than `q`, or that fails
validation against the live marker, is dropped as if no progress was
recorded, so the question is asked again.

Carries the original decoded entry forward unchanged in `res.persist`: if a
later resolver is still pending, the next round's `request_state` is built from
`res.persist`, so an earlier answer must stay there - byte-identical, never
re-derived - or it would be dropped and re-asked.


## _result_type

`mcp.server.mcpserver.resolve._result_type`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _result_type(marker: Sample | ListRoots) -> type[CreateMessageResult] | type[CreateMessageResultWithTools] | type[ListRootsResult]
```

The result model a `Sample`/`ListRoots` response must validate against.


## _state_key

`mcp.server.mcpserver.resolve._state_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _state_key(fn: Callable[..., Any]) -> str
```

Worker-stable base wire key for a resolver, derived only from registration data.

`input_requests`/`request_state` must round-trip through the client and resume on
any worker (stateless HTTP), so the key carries no `id(...)`: it is the resolver's
`module:qualname` (a callable object uses its type's). Distinct resolvers that
share this base - two instances of one method, two closures from one factory - are
disambiguated deterministically by `build_resolver_plans` (`base`, `base#1`, ...).


## _type_hints

`mcp.server.mcpserver.resolve._type_hints`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _type_hints(fn: Callable[..., Any]) -> dict[str, Any]
```

Resolve type hints for a function or a callable object.

`typing.get_type_hints` raises on a callable *instance*; fall back to its
`__call__`. Returns an empty mapping when hints cannot be resolved, matching
`find_context_parameter`'s tolerance so callables without annotations (or with
unresolvable ones) simply have no resolved parameters.


## _unwrap

`mcp.server.mcpserver.resolve._unwrap`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _unwrap(outcome: ElicitationResult[Any], name: str) -> Any
```

## _uses_input_required

`mcp.server.mcpserver.resolve._uses_input_required`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _uses_input_required(protocol_version: str | None) -> bool
```

True when this request must elicit via `InputRequiredResult` (>= 2026-07-28).

Older revisions still carry a standalone `elicitation/create` server-to-client
request, so the framework keeps the synchronous `ctx.elicit()` path for them.


## _wants_union

`mcp.server.mcpserver.resolve._wants_union`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _wants_union(type_arg: Any) -> bool
```

True when `type_arg` is an `ElicitationResult` member (or a union of them).

Handles the subscripted `ElicitationResult[T]` alias (a `TypeAliasType` whose
union is on the origin's `__value__`), the bare `ElicitationResult` alias (the
`__value__` is on `type_arg` itself), an explicit `AcceptedElicitation[T] | ...`
union, and a single member.


## build_resolver_plans

Import as `mcp.server.mcpserver.tools.base.build_resolver_plans`  ·  defined at `mcp.server.mcpserver.resolve.build_resolver_plans`

```python
def build_resolver_plans(resolved_params: Mapping[str, tuple[Resolve, bool]], tool_arg_names: set[str]) -> dict[Hashable, _ResolverPlan]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Statically analyze the resolver DAG rooted at a tool's resolved parameters.

Raises:
    InvalidSignature: If a resolver has a cyclic dependency, or a resolver
        parameter cannot be classified (not a `Context`, a nested `Resolve`,
        or a tool argument by name).


## find_resolved_parameters

Import as `mcp.server.mcpserver.tools.base.find_resolved_parameters`  ·  defined at `mcp.server.mcpserver.resolve.find_resolved_parameters`

```python
def find_resolved_parameters(fn: Callable[..., Any]) -> dict[str, tuple[Resolve, bool]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Find parameters of `fn` annotated `Annotated[_, Resolve(...)]`.

Returns a mapping of parameter name to `(Resolve, wants_union)`, where
`wants_union` is True when the annotated type is an `ElicitationResult` member
(the consumer wants the full outcome rather than the unwrapped model).


## resolve_arguments

Import as `mcp.server.mcpserver.tools.base.resolve_arguments`  ·  defined at `mcp.server.mcpserver.resolve.resolve_arguments`

```python
async def resolve_arguments(resolved_params: Mapping[str, tuple[Resolve, bool]], plans: Mapping[Hashable, _ResolverPlan], tool_args: Mapping[str, Any], context: Context[Any, Any]) -> dict[str, Any] | InputRequiredResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve every `Resolve`-marked tool parameter into a concrete value.

Returns the mapping of tool parameter name to injected value when every
resolver is satisfied. When a resolver still needs client input (and the
negotiated protocol is >= 2026-07-28), returns an `InputRequiredResult`
carrying the batched questions instead; the tool body is not run.

Each question is asked once - its answer is carried in `request_state` across
rounds and satisfies the question when the resolver asks it again. Resolver
bodies themselves may re-run on each round; a recorded answer is consulted
only when the body asks, never in place of running it.

Raises:
    ToolError: If an elicited value is declined or cancelled and the consumer
        asked for the unwrapped model (rather than the result union).


## returns_input_required

Import as `mcp.server.mcpserver.tools.base.returns_input_required`  ·  defined at `mcp.server.mcpserver.resolve.returns_input_required`

```python
def returns_input_required(fn: Callable[..., Any]) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

True when `fn`'s return annotation carries an `InputRequiredResult` arm.

Used at tool registration to reject combining `Resolve(...)` parameters with a
hand-rolled `InputRequiredResult` flow: a call has a single
`input_responses`/`request_state` channel, so the two flows would overwrite
each other's state and the call could never converge.


