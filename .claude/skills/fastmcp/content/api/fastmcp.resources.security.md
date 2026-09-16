# `fastmcp.resources.security`

Distribution: `fastmcp`

## DEFAULT_RESOURCE_SECURITY

Import as `fastmcp.server.server.DEFAULT_RESOURCE_SECURITY`  ·  defined at `fastmcp.resources.security.DEFAULT_RESOURCE_SECURITY`

```python
DEFAULT_RESOURCE_SECURITY = ResourceSecurity()
```

**Inferred type** (`ty`, not declared in the source): `ResourceSecurity`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Secure-by-default policy: traversal, absolute paths, and null bytes rejected.


## INHERIT_SECURITY

Import as `fastmcp.server.server.INHERIT_SECURITY`  ·  defined at `fastmcp.resources.security.INHERIT_SECURITY`

```python
INHERIT_SECURITY = InheritSecurity()
```

**Inferred type** (`ty`, not declared in the source): `InheritSecurity`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Sentinel instance signalling a template should inherit the server default.


## __all__

`fastmcp.resources.security.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ResourceSecurity']
```

## InheritSecurity

Import as `fastmcp.server.server.InheritSecurity`  ·  defined at `fastmcp.resources.security.InheritSecurity`

```python
class InheritSecurity
```

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Sentinel type: inherit the server-wide resource-security default.

Distinguishes "no per-component policy was set" (inherit whatever the
server configured) from an explicit ``None`` (screening disabled for
this component).


## ResourceSecurity

Import as `fastmcp.resources.ResourceSecurity`  ·  defined at `fastmcp.resources.security.ResourceSecurity`

```python
class ResourceSecurity
```

**Also exported as** `fastmcp.resources.ResourceSecurity`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `exempt_params: Set[str] = field(default_factory=frozenset)`  _class-attribute, instance-attribute_
  Parameter names to skip all checks for. Hyphenated URI-template spellings are accepted: `{git-ref}` is extracted as `git_ref`, and an exemption written either way matches it.
- `reject_absolute_paths: bool = True`  _class-attribute, instance-attribute_
  Reject values that look like absolute filesystem paths.
- `reject_null_bytes: bool = True`  _class-attribute, instance-attribute_
  Reject values containing NUL (`\x00`). Null bytes defeat string comparisons (`"..\x00" != ".."`) and can cause truncation in C extensions or subprocess calls.
- `reject_path_traversal: bool = True`  _class-attribute, instance-attribute_
  Reject values containing `..` as a path component.
- `def validate(self, params: Mapping[str, object]) -> str | None`
  Check all parameter values against the configured policy.

Security policy applied to extracted resource template parameters.

These checks run after a URI has matched a template and its
parameter values have been extracted and percent-decoded. They catch
path-traversal and absolute-path injection regardless of how the
value was encoded in the URI (literal, `%2F`, `%5C`, `%2E%2E`).

All checks default on. Screen a value like `HEAD~3..HEAD` (dots
inside a single segment) passes — only a standalone `..` segment is
treated as traversal.

Example:
    Opt a parameter out of screening (e.g. a git ref that may
    legitimately contain `..`):

    ```python
    from fastmcp.resources import ResourceSecurity

    @mcp.resource(
        "git://diff/{ref}",
        security=ResourceSecurity(exempt_params={"ref"}),
    )
    def git_diff(ref: str) -> str: ...
    ```


## _path_checks

`fastmcp.resources.security._path_checks`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _path_checks() -> tuple[Callable[[str], bool], Callable[[str], bool]]
```

Lazily load the SDK's path-safety helpers.

The screening logic lives in the `mcp` SDK, which is an optional
dependency of `fastmcp-slim`. Importing it at module top would make
`from fastmcp.resources import Resource` require the SDK, so the
import is deferred to the point of first use (and cached).


