# `fastmcp.utilities.authorization`

Distribution: `fastmcp`

## AuthCheck

Import as `fastmcp.server.auth.AuthCheck`  ·  defined at `fastmcp.utilities.authorization.AuthCheck`

```python
AuthCheck = Callable[[AuthContext], bool] | Callable[[AuthContext], Awaitable[bool]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form '((AuthContext, /) -> bool) | ((AuthContext, /) -> Awaitable[bool])'> ````

**Also exported as** `fastmcp.server.auth.AuthCheck`

_13 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## logger

`fastmcp.utilities.authorization.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AuthContext

Import as `fastmcp.server.auth.AuthContext`  ·  defined at `fastmcp.utilities.authorization.AuthContext`

```python
class AuthContext
```

**Also exported as** `fastmcp.server.auth.AuthContext`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `component: FastMCPComponent`  _instance-attribute_
- `token: AccessToken | None`  _instance-attribute_
- `tool: Tool | None`  _property_
  Backwards-compatible access to the component as a Tool.

Context passed to auth check callables.

Attributes:
    token: The current access token, or None if unauthenticated.
    component: The tool, resource, resource template, or prompt being accessed.
    tool: Backwards-compatible alias for component when it is a Tool.


## _RequireRoles

`fastmcp.utilities.authorization._RequireRoles`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _RequireRoles
```

**Declared members (1)**

- `required_roles: frozenset[str] = frozenset(roles)`  _instance-attribute_

Callable auth check requiring all of a fixed set of roles.

Deliberately not a :class:`_ScopeAwareCheck`. Roles are not scopes and
cannot be requested through OAuth, so a shortfall has no spec-correct
step-up representation.


## _RequireScopes

`fastmcp.utilities.authorization._RequireScopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _RequireScopes(_ScopeAwareCheck)
```

**Bases** `_ScopeAwareCheck`

**Declared members (2)**

- `def missing_scopes(self, ctx: AuthContext) -> set[str]`
- `required_scopes: frozenset[str] = frozenset(scopes)`  _instance-attribute_

Callable auth check requiring all of a fixed set of OAuth scopes.


## _RestrictTag

`fastmcp.utilities.authorization._RestrictTag`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _RestrictTag(_ScopeAwareCheck)
```

**Bases** `_ScopeAwareCheck`

**Declared members (3)**

- `def missing_scopes(self, ctx: AuthContext) -> set[str]`
- `required_scopes: frozenset[str] = frozenset(scopes)`  _instance-attribute_
- `tag = tag`  _instance-attribute_

Callable auth check requiring scopes only when a component has a tag.


## _ScopeAwareCheck

`fastmcp.utilities.authorization._ScopeAwareCheck`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ScopeAwareCheck
```

**Declared members (1)**

- `def missing_scopes(self, ctx: AuthContext) -> set[str]`
  Return the required scopes the token lacks (empty if satisfied).

Base for auth checks that can name the scopes a token is missing.

Ordinary auth checks are opaque booleans: on denial they reveal nothing
about *why*. Scope-based checks expose their unmet requirements through
`missing_scopes` so a shortfall can be surfaced as a spec-correct
``insufficient_scope`` step-up (SEP-2350 / RFC 6750 §3) naming exactly what
the caller must re-authorize for.


## _evaluate_check

`fastmcp.utilities.authorization._evaluate_check`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _evaluate_check(check: AuthCheck, ctx: AuthContext) -> bool
```

Evaluate a single auth check, masking unexpected failures as denial.

An ``AuthorizationError`` is the check's deliberate denial and propagates.
Any other exception is a bug in the check; it is logged and treated as a
denial so a broken check fails closed.


## require_roles

Import as `fastmcp.server.auth.require_roles`  ·  defined at `fastmcp.utilities.authorization.require_roles`

```python
def require_roles(roles: str = (), extract: Callable[[dict[str, Any]], Iterable[str]]) -> AuthCheck
```

**Also exported as** `fastmcp.server.auth.require_roles`

Require all of the given roles, read from the token's claims.

Roles and groups are not part of OIDC, so every identity provider puts them
somewhere different: `realm_access.roles` on Keycloak, `roles` on Microsoft
Entra, `cognito:groups` on AWS Cognito, `permissions` or a namespaced custom
claim on Auth0. `extract` receives the token's claims and returns the
caller's roles, which keeps that provider-specific knowledge at the call
site instead of guessing it here.

```python
from fastmcp.server.auth import require_roles

keycloak = require_roles("admin", extract=lambda c: c["realm_access"]["roles"])
cognito = require_roles("admins", extract=lambda c: c["cognito:groups"])
```

A token missing the claim entirely is denied rather than treated as an
error, so `extract` may index into the claims without guarding. An
extractor returning a bare string is treated as one role, since a provider
that stores a single role as a scalar is common.

Unlike `require_scopes`, this check cannot signal a shortfall: OAuth has no
way to request a role, so there is no `insufficient_scope` challenge to
emit. A role denial is therefore reported as a plain `AuthorizationError`,
and it suppresses any scope shortfall alongside it — a caller blocked by
their role must not be told to go obtain a scope that would not help.
Scope shortfalls are still reported normally whenever the role check
passes.

Args:
    *roles: Roles the caller must hold. All are required (AND logic).
    extract: Callable mapping the token's claims to the caller's roles.

Raises:
    ValueError: If no roles are given, which would allow any authenticated
        caller and is more likely a mistake than an intent.


## require_scopes

Import as `fastmcp.server.auth.require_scopes`  ·  defined at `fastmcp.utilities.authorization.require_scopes`

```python
def require_scopes(scopes: str = ()) -> AuthCheck
```

**Also exported as** `fastmcp.server.auth.require_scopes`

Require all of the given OAuth scopes.


## restrict_tag

Import as `fastmcp.server.auth.restrict_tag`  ·  defined at `fastmcp.utilities.authorization.restrict_tag`

```python
def restrict_tag(tag: str, scopes: list[str]) -> AuthCheck
```

**Also exported as** `fastmcp.server.auth.restrict_tag`

Require scopes when the accessed component has a specific tag.


## run_auth_checks

Import as `fastmcp.server.auth.run_auth_checks`  ·  defined at `fastmcp.utilities.authorization.run_auth_checks`

```python
async def run_auth_checks(checks: AuthCheck | list[AuthCheck], ctx: AuthContext) -> bool
```

**Also exported as** `fastmcp.server.auth.run_auth_checks`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run auth checks with AND logic, stopping at the first failure.


## run_auth_checks_with_shortfall

Import as `fastmcp.server.middleware.authorization.run_auth_checks_with_shortfall`  ·  defined at `fastmcp.utilities.authorization.run_auth_checks_with_shortfall`

```python
async def run_auth_checks_with_shortfall(checks: AuthCheck | list[AuthCheck], ctx: AuthContext) -> tuple[bool, list[str]]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run auth checks with AND logic, classifying the denial cause.

Returns ``(authorized, missing_scopes)``. ``missing_scopes`` names every
scope the caller must obtain to satisfy *all* scope requirements at once:
the union of the shortfalls across every scope-aware check, not just the
first one to fail. Reporting only the first would strand a caller in a
step-up loop — it obtains that scope, retries, and is denied again for the
next — so the union is what makes a single re-authorization converge.

The challenge is withheld entirely (an empty list, which the caller surfaces
as a plain ``AuthorizationError``) unless every non-scope check passes. A
custom policy denial — a tenant check, say — must never be reported as an
``insufficient_scope`` shortfall, and must never name the scopes of a
component the caller could not otherwise reach. To guarantee that, the
opaque checks are all evaluated before any scope is disclosed; a shortfall
is only reported once they have all passed.

An ``AuthorizationError`` raised by a check propagates unchanged.


## scope_requirements

Import as `fastmcp.server.middleware.authorization.scope_requirements`  ·  defined at `fastmcp.utilities.authorization.scope_requirements`

```python
def scope_requirements(checks: AuthCheck | list[AuthCheck], ctx: AuthContext) -> list[str] | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Scopes a check list requires but the token lacks, without running it.

Returns ``None`` when the list contains any opaque (non-scope) check. Such a
check might deny for a reason unrelated to scopes, and evaluating it here
would run authorization logic — with whatever side effects it carries —
outside its normal place in the chain. Since its verdict is unknown, its
siblings' scopes must not be disclosed either, so the whole list is withheld.

When every check is scope-aware, the result is their combined shortfall,
computed purely from the token and component (an empty list means the list is
already satisfied). This lets a shortfall be aggregated across authorization
layers without evaluating anything that would otherwise be skipped.


