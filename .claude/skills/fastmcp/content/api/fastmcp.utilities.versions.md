# `fastmcp.utilities.versions`

Distribution: `fastmcp`

## C

`fastmcp.utilities.versions.C`

```python
C = TypeVar('C', bound=Any)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## VersionKey

`fastmcp.utilities.versions.VersionKey`

```python
class VersionKey
```

A comparable version key that handles None, PEP 440 versions, and strings.

Comparison order:
1. None (unversioned) sorts lowest
2. PEP 440 versions sort by semantic version order
3. Invalid versions (strings) sort lexicographically
4. When comparing PEP 440 vs string, PEP 440 comes first


## VersionSpec

Import as `fastmcp.server.transforms.VersionSpec`  ·  defined at `fastmcp.utilities.versions.VersionSpec`

```python
class VersionSpec
```

**Also exported as** `fastmcp.server.transforms.VersionSpec`

_22 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `eq: str | None = None`  _class-attribute, instance-attribute_
- `gte: str | None = None`  _class-attribute, instance-attribute_
- `def intersect(self, other: VersionSpec | None) -> VersionSpec`
  Return a spec that satisfies both this spec and other.
- `lt: str | None = None`  _class-attribute, instance-attribute_
- `def matches(self, version: str | None, match_none: bool = True) -> bool`
  Check if a version matches this spec.

Specification for filtering components by version.

Used by transforms and providers to filter components to a specific
version or version range. Unversioned components (version=None) always
match any spec.

Args:
    gte: If set, only versions >= this value match.
    lt: If set, only versions < this value match.
    eq: If set, only this exact version matches (gte/lt ignored).
        Matching is PEP 440-normalized and `v`-prefix insensitive, so
        `eq="v1.0"` matches a component versioned `"1.0"`, and `eq="1.0"`
        matches `"1"` (PEP 440 treats `1` and `1.0` as the same version).
        If a server registers two PEP 440-equivalent spellings of the
        same component (e.g. both `"1"` and `"1.0"`), they are the same
        version under this spec; selection among them is deterministic
        (see `version_sort_key`), not registration-order dependent.


## compare_versions

`fastmcp.utilities.versions.compare_versions`

```python
def compare_versions(a: str | None, b: str | None) -> int
```

Compare two version strings.

Args:
    a: First version string (or None).
    b: Second version string (or None).

Returns:
    -1 if a < b, 0 if a == b, 1 if a > b.

Example:
    ```python
    compare_versions("1.0", "2.0")  # Returns -1
    compare_versions("2.0", "1.0")  # Returns 1
    compare_versions(None, "1.0")   # Returns -1 (None < any version)
    ```


## dedupe_with_versions

Import as `fastmcp.server.transforms.catalog.dedupe_with_versions`  ·  defined at `fastmcp.utilities.versions.dedupe_with_versions`

```python
def dedupe_with_versions(components: Sequence[C], key_fn: Callable[[C], str]) -> list[C]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Deduplicate components by key, keeping highest version.

Groups components by key, selects the highest version from each group,
and injects available versions into meta if any component is versioned.

Args:
    components: Sequence of components to deduplicate.
    key_fn: Function to extract the grouping key from a component.

Returns:
    Deduplicated list with versions injected into meta.


## is_version_greater

`fastmcp.utilities.versions.is_version_greater`

```python
def is_version_greater(a: str | None, b: str | None) -> bool
```

Check if version a is greater than version b.

Args:
    a: First version string (or None).
    b: Second version string (or None).

Returns:
    True if a > b, False otherwise.


## max_version

`fastmcp.utilities.versions.max_version`

```python
def max_version(a: str | None, b: str | None) -> str | None
```

Return the greater of two versions.

Args:
    a: First version string (or None).
    b: Second version string (or None).

Returns:
    The greater version, or None if both are None.


## min_version

`fastmcp.utilities.versions.min_version`

```python
def min_version(a: str | None, b: str | None) -> str | None
```

Return the lesser of two versions.

Args:
    a: First version string (or None).
    b: Second version string (or None).

Returns:
    The lesser version, or None if both are None.


## parse_version_key

`fastmcp.utilities.versions.parse_version_key`

```python
def parse_version_key(version: str | None) -> VersionKey
```

Parse a version string into a sortable key.

Args:
    version: The version string, or None for unversioned.

Returns:
    A VersionKey suitable for sorting.


## version_sort_key

Import as `fastmcp.server.server.version_sort_key`  ·  defined at `fastmcp.utilities.versions.version_sort_key`

```python
def version_sort_key(component: FastMCPComponent) -> tuple[VersionKey, str]
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get a sort key for a component based on its version.

Use with sorted() or max() to order components by version.

The key is a `(VersionKey, raw)` tuple. The `VersionKey` orders by PEP 440
semantics (or lexicographically for non-PEP 440 strings); the raw version
string is a deterministic tie-breaker so that two components whose versions
are PEP 440-equivalent but spelled differently (e.g. `"1"` and `"1.0"`) are
ordered reproducibly instead of by registration order. The raw tie-breaker
only affects equivalent-version ties and never the primary version order,
so range/equality matching (which uses `VersionKey` directly) is unchanged.

Args:
    component: The component to get a sort key for.

Returns:
    A deterministic, sortable `(VersionKey, raw)` tuple.

Example:
    ```python
    tools = [tool_v1, tool_v2, tool_unversioned]
    highest = max(tools, key=version_sort_key)  # Returns tool_v2
    ```


