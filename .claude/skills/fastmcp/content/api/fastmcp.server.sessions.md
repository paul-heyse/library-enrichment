# `fastmcp.server.sessions`

Distribution: `fastmcp`

## SESSION_ID_DESCRIPTION

`fastmcp.server.sessions.SESSION_ID_DESCRIPTION`

```python
SESSION_ID_DESCRIPTION: Final[str] = 'Session identifier. Use a tool to create a session, then pass the resulting id here to persist state across calls in the same session.'
```

## SessionId

`fastmcp.server.sessions.SessionId`

```python
SessionId = Annotated[str, _SessionIdMarker()]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'typing.Annotated[str, <metadata>]'> ````

## _MARKER_KEY

`fastmcp.server.sessions._MARKER_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MARKER_KEY: Final[str] = '_created'
```

## _STATE_KEY

`fastmcp.server.sessions._STATE_KEY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STATE_KEY: Final[str] = 'state'
```

## _USER_SESSION_ID

`fastmcp.server.sessions._USER_SESSION_ID`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_USER_SESSION_ID: Final[str] = '_user'
```

## logger

`fastmcp.server.sessions.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## InvalidSession

`fastmcp.server.sessions.InvalidSession`

```python
class InvalidSession(FastMCPError)
```

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A session id did not resolve to a session created under the current principal.

Raised by `get_session(session_id)` when the id was never created, or was
created under a different principal. The public message is deliberately
generic — the specific reason (which id, which principal) is logged at debug
level, not returned to the caller, so an attacker cannot distinguish "unknown
id" from "belongs to someone else".


## Session

Import as `fastmcp.server.dependencies.Session`  ·  defined at `fastmcp.server.sessions.Session`

```python
class Session
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `async def clear(self) -> None`  _async_
  Empty the session's user state but keep the session valid.
- `async def delete(self, key: str) -> None`  _async_
  Remove `key` from this session, if present (preserves the marker).
- `async def end(self) -> None`  _async_
  Invalidate the session — delete its one key and all of its state.
- `async def get(self, key: str, default: Any = None) -> Any`  _async_
  Return the value for `key`, or `default` when it is not set.
- `id: str | None`  _property_
  The session's identifier, or `None` for an injected per-user session.
- `async def set(self, key: str, value: Any) -> None`  _async_
  Store `value` under `key` in this session (read-modify-write).

Async accessors over one `(principal, session_id)` bucket of state.

A session's state is a single dict stored under one key. That dict holds user
state in a `state` sub-dict and a small creation marker alongside it, so a
created-but-empty session is still distinguishable from a missing one.
`get`/`set`/`delete` read-modify-write the sub-dict; `clear` empties the
sub-dict but keeps the session valid; `end` deletes the whole key. Writes
never impose a TTL — retention is entirely the server store's (configure it on
the store you pass to `FastMCP(session_state_store=...)`).

Concurrent writes to one session race on the read-modify-write; session state
is small and typically driven serially by one agent, so this is acceptable.


## SessionAuthError

`fastmcp.server.sessions.SessionAuthError`

```python
class SessionAuthError(FastMCPError)
```

**Bases** `FastMCPError`

**Inherited (1)**

- from `fastmcp.exceptions.FastMCPError`: `log_level`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

An injected `session: UserSession` was requested with no authenticated principal.

Per-user session injection keys off the request's authenticated principal, so
it is only meaningful under auth. A tool that needs cross-call state without
auth should take a `session_id: SessionId` argument instead.


## SessionProvider

`fastmcp.server.sessions.SessionProvider`

```python
class SessionProvider(Provider)
```

**Bases** `Provider`

**Inherited (17)**

- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `lifespan`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Provider contributing the session lifecycle tools.

Register it whenever a tool declares a `session_id: SessionId` argument:

```python
from fastmcp.server.sessions import SessionProvider

mcp.add_provider(SessionProvider())
```

It registers two tools:

- `create_session()` mints an unguessable `uuid4`, records the session, and
  returns the id.
- `end_session(session_id)` invalidates that session and deletes its state.

It owns no storage (session state lives in the server's configured
`session_state_store`) and imposes no TTL (retention is the store's). It
exists to mint and end owned session ids. Registration is not enforced: with
no provider, no id can be created, so every `get_session(...)` rejects —
a `session_id` tool without a provider simply cannot resolve a session.


## UserSession

`fastmcp.server.sessions.UserSession`

```python
class UserSession(Session)
```

**Bases** `Session`

**Inherited (6)**

- from `fastmcp.server.sessions.Session`: `clear`, `delete`, `end`, `get`, `id`, `set`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Annotation marker for the injected per-user session.

A `session: UserSession` parameter is **dependency-injected** like
`ctx: Context`: keyed by the request's authenticated principal, excluded from
the input schema, and requiring auth (it raises `SessionAuthError` with no
principal). It doubles as the injection *annotation* and the injected
type — the value a handler receives is a `UserSession`, which subclasses
`Session`, so `await session.get(...)`, `.set`, `.delete`, and `.clear` all
work exactly as on any other `Session`.

Unlike `session_id: SessionId`, the per-user bucket needs no `create_session`,
no `SessionProvider`, and no validation — it is always available under auth,
keyed directly by the caller's identity.

```python
from fastmcp.server.sessions import UserSession

@mcp.tool
async def remember(fact: str, session: UserSession) -> str:
    await session.set("fact", fact)
    return "noted"
```

Subclasses `Session` only so the framework's type-based injection detector can
key off it; it adds no behavior of its own.


## _CurrentSession

`fastmcp.server.sessions._CurrentSession`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _CurrentSession(Dependency['Session'])
```

**Bases** `Dependency['Session']`

Dependency that injects a per-user `Session` keyed by the request principal.

Mirrors `_CurrentContext`: a `session: UserSession` parameter is rewritten to
default to this dependency, so it is excluded from the input schema and
resolved at call time. Raises `SessionAuthError` when the request carries no
authenticated principal.


## _OptionalCurrentSession

`fastmcp.server.sessions._OptionalCurrentSession`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OptionalCurrentSession(Dependency['Session | None'])
```

**Bases** `Dependency['Session | None']`

Dependency for an *optional* per-user session (`session: UserSession | None`).

Mirrors `_OptionalCurrentContext`: when the request carries no authenticated
principal it injects `None` instead of raising, so a handler that declares the
parameter optional (default `None`) can run on unauthenticated requests and
branch on whether a session is available.


## _SessionIdMarker

`fastmcp.server.sessions._SessionIdMarker`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _SessionIdMarker
```

Metadata marker identifying a `SessionId`-annotated parameter.


## CurrentSession

`fastmcp.server.sessions.CurrentSession`

```python
def CurrentSession() -> Session
```

Inject the per-user `Session` for the current authenticated principal.

Rarely written explicitly — a `session: UserSession` parameter is rewritten
to this. Provided for parity with `CurrentContext()` when an explicit default
is preferred.


## OptionalCurrentSession

`fastmcp.server.sessions.OptionalCurrentSession`

```python
def OptionalCurrentSession() -> Session | None
```

Inject the per-user `Session`, or `None` when the request is unauthenticated.

Rarely written explicitly — a `session: UserSession | None = None` parameter
is rewritten to this. Provided for parity with `OptionalCurrentContext()`.


## _current_user_session

`fastmcp.server.sessions._current_user_session`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _current_user_session() -> UserSession | None
```

Build the per-user session for the current principal, or `None` if unauth.

Resolves the store through `get_server()` rather than `get_context()`: a
`task=True` tool whose only injected dependency is `UserSession` runs in a
Docket worker with no foreground context, and `get_server()` is task-aware (it
resolves via the task-server map in a worker).


## _principal_segment

`fastmcp.server.sessions._principal_segment`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _principal_segment(principal: str | None) -> str
```

A fixed-length, delimiter-safe key segment for a principal.

Hashing keeps an arbitrary principal string from injecting the `:` key
delimiter and bounds the key length. `None` (unauthenticated) collapses to a
single shared `anon` segment — without a principal there is no isolation wall.


## create_session

`fastmcp.server.sessions.create_session`

```python
async def create_session() -> str
```

Create a new session and return its identifier.

Mints an unguessable `uuid4`, records an initial session owned by the current
principal, and returns the id as a string. Store it and pass it back as a
`session_id` argument on later calls to persist state across a session — only
an id created this way resolves. State is keyed by the authenticated
principal, so the id organizes sessions within a user; on an unauthenticated
connection the id is the only thing standing between callers, which is why it
is unguessable.


## current_principal

`fastmcp.server.sessions.current_principal`

```python
def current_principal() -> str | None
```

The authenticated principal for the current request as a compact JSON string.

Returns the `(client_id, issuer, subject)` triple encoded as compact JSON, or
`None` on an unauthenticated request. Two users of one OAuth client are
distinct principals whenever the token verifier supplies a subject.


## end_session

`fastmcp.server.sessions.end_session`

```python
async def end_session(session_id: SessionId) -> str
```

End a session and delete all of its state.

Validates the id like any other resolution (an unknown or foreign id is
rejected), then deletes the session's key so the id no longer resolves.


## session_id_parameter_names

`fastmcp.server.sessions.session_id_parameter_names`

```python
def session_id_parameter_names(fn: Callable[..., object]) -> tuple[str, ...]
```

Names of a function's parameters annotated with `SessionId`.

Scans resolved type hints for `Annotated[str, _SessionIdMarker()]` metadata.
Returns an empty tuple when the hints cannot be resolved (the function then
simply carries no auto-populated session-id description).

`functools.partial` is unwrapped first, since `get_type_hints` rejects a
partial object — FastMCP supports registering a partial as a tool, and its
schema is still built from the underlying function, so its `SessionId`
parameters must be detected here too. Parameters the partial has already
bound — positionally or by keyword — are dropped, matching the tool's actual
argument surface (the partial's own signature already reflects this).


## session_storage_key

`fastmcp.server.sessions.session_storage_key`

```python
def session_storage_key(principal: str | None, session_id: str) -> str
```

The single storage key holding a session's state dict.

Keyed by `(principal, session_id)`: the principal is the isolation wall, the
id organizes sessions within it. A session's whole state lives under this one
key as a dict, so one key means one store TTL per session and `end` is a
single delete.


