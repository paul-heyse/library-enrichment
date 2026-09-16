# `mcp.shared.path_security`

Distribution: `mcp`

## __all__

`mcp.shared.path_security.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['PathEscapeError', 'contains_path_traversal', 'is_absolute_path', 'safe_join']
```

## PathEscapeError

Import as `fastmcp.server.providers.skills.skill_provider.PathEscapeError`  ·  defined at `mcp.shared.path_security.PathEscapeError`

```python
class PathEscapeError(ValueError)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ValueError`

Raised by :func:`safe_join` when the resolved path escapes the base.


## contains_path_traversal

Import as `mcp.server.mcpserver.resources.templates.contains_path_traversal`  ·  defined at `mcp.shared.path_security.contains_path_traversal`

```python
def contains_path_traversal(value: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check whether a value, treated as a relative path, escapes its origin.

This is a **base-free** check: it does not know the sandbox root, so
it detects only whether ``..`` components would move above the
starting point. Use :func:`safe_join` when you know the root — it
additionally catches symlink escapes and absolute-path injection.

Note:
    This is a string-level check on the value as supplied. It does
    not model platform-specific filesystem normalisation (e.g. Win32
    stripping of trailing dots and spaces from the final path
    component). For filesystem access, use :func:`safe_join`, which
    resolves through the OS and verifies containment.

The check is component-based: ``..`` is dangerous only as a
standalone path segment, not as a substring. Both ``/`` and ``\``
are treated as separators.

Example::

    >>> contains_path_traversal("a/b/c")
    False
    >>> contains_path_traversal("../etc")
    True
    >>> contains_path_traversal("a/../../b")
    True
    >>> contains_path_traversal("a/../b")
    False
    >>> contains_path_traversal("1.0..2.0")
    False
    >>> contains_path_traversal("..")
    True

Args:
    value: A string that may be used as a filesystem path.

Returns:
    ``True`` if the path would escape its starting directory.


## is_absolute_path

Import as `mcp.server.mcpserver.resources.templates.is_absolute_path`  ·  defined at `mcp.shared.path_security.is_absolute_path`

```python
def is_absolute_path(value: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check whether a value is an absolute filesystem path.

Absolute paths are dangerous when joined onto a base: in Python,
``Path("/data") / "/etc/passwd"`` yields ``/etc/passwd`` — the
absolute right-hand side silently discards the base.

Detects POSIX absolute (``/foo``), Windows drive-absolute
(``C:\foo``) and drive-relative (``C:foo``), and Windows
UNC/root-relative (``\\server\share``, ``\foo``).

Example::

    >>> is_absolute_path("relative/path")
    False
    >>> is_absolute_path("/etc/passwd")
    True
    >>> is_absolute_path("C:\\Windows")
    True
    >>> is_absolute_path("")
    False

Args:
    value: A string that may be used as a filesystem path.

Returns:
    ``True`` if the path is absolute on any common platform.


## safe_join

Import as `fastmcp.server.providers.skills.skill_provider.safe_join`  ·  defined at `mcp.shared.path_security.safe_join`

```python
def safe_join(base: str | Path, parts: str = ()) -> Path
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Join path components onto a base, rejecting escapes.

Resolves the joined path and verifies it remains within ``base``.
This is the **gold-standard** check: it catches ``..`` traversal,
absolute-path injection, and symlink escapes that the base-free
checks cannot.

The symlink check is point-in-time: a directory swapped for a
symlink between this call and the caller's subsequent open would not
be re-checked. Handlers serving a tree that may be modified
concurrently should additionally open with ``O_NOFOLLOW`` or use
platform path-confinement primitives.

Example::

    >>> safe_join("/data/docs", "readme.txt")
    PosixPath('/data/docs/readme.txt')
    >>> safe_join("/data/docs", "../../../etc/passwd")
    Traceback (most recent call last):
    ...
    PathEscapeError: ...

Args:
    base: The sandbox root. May be relative; it will be resolved.
    parts: Path components to join. Each is checked for null bytes
        and absolute form before joining.

Returns:
    The resolved path, verified to be within ``base`` at resolution
    time.

Raises:
    PathEscapeError: If any part contains a null byte, any part is
        absolute, or the resolved path is not contained within the
        resolved base.


