# `fastmcp.utilities.json_schema`

Distribution: `fastmcp`

## _LITERAL_KEYWORDS

`fastmcp.utilities.json_schema._LITERAL_KEYWORDS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LITERAL_KEYWORDS = frozenset({'default', 'const', 'examples', 'example', 'enum'})
```

## _METADATA_KEYS

`fastmcp.utilities.json_schema._METADATA_KEYS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_METADATA_KEYS = frozenset({'title', 'description', 'deprecated', 'readOnly', 'writeOnly'})
```

## _SCHEMA_KEYWORDS

`fastmcp.utilities.json_schema._SCHEMA_KEYWORDS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SCHEMA_KEYWORDS = frozenset({'type', 'properties', '$ref', 'items', 'allOf', 'oneOf', 'anyOf', 'required'})
```

## _SUBSCHEMA_LIST_KEYS

`fastmcp.utilities.json_schema._SUBSCHEMA_LIST_KEYS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_LIST_KEYS = frozenset({'allOf', 'anyOf', 'oneOf', 'prefixItems'})
```

## _SUBSCHEMA_MAP_KEYS

`fastmcp.utilities.json_schema._SUBSCHEMA_MAP_KEYS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_MAP_KEYS = frozenset({'properties', 'patternProperties', '$defs', 'definitions', 'dependentSchemas', 'dependencies'})
```

## _SUBSCHEMA_VALUE_KEYS

`fastmcp.utilities.json_schema._SUBSCHEMA_VALUE_KEYS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_VALUE_KEYS = frozenset({'items', 'additionalItems', 'additionalProperties', 'contains', 'contentSchema', 'propertyNames', 'unevaluatedItems', 'unevaluatedProperties', 'if', 'then', 'else', 'not'})
```

## _copy_schema

`fastmcp.utilities.json_schema._copy_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _copy_schema(schema: dict[str, Any]) -> dict[str, Any]
```

Return a deep copy of a JSON schema without recursing.

`copy.deepcopy` consumes stack frames in proportion to nesting depth, so a
deeply nested schema raises `RecursionError` before the traversals in this
module can apply their own depth guards — turning a schema that used to
compress into one that fails outright. Schemas are plain JSON, so an
explicit stack copies the containers at any depth and shares the immutable
scalars at the leaves.


## _defs_have_cycles

`fastmcp.utilities.json_schema._defs_have_cycles`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _defs_have_cycles(defs: dict[str, Any]) -> bool
```

Check whether any definitions in ``$defs`` form a reference cycle.

A cycle means a definition directly or transitively references itself
(e.g. Node → children → Node, or A → B → A).  ``jsonref.replace_refs``
silently produces Python-level object cycles for these, which Pydantic's
serializer rejects.


## _merge_ref_siblings

`fastmcp.utilities.json_schema._merge_ref_siblings`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _merge_ref_siblings(original: Any, dereferenced: Any, defs: dict[str, Any], visited: set[str] | None = None) -> Any
```

Merge sibling keywords from original $ref nodes into dereferenced schema.

When jsonref resolves $ref, it replaces the entire node with the referenced
definition, losing any sibling keywords like description, default, or examples.
This function walks both trees in parallel and merges those siblings back.

Args:
    original: The original schema with $ref and potential siblings
    dereferenced: The schema after jsonref processing
    defs: The $defs from the original schema, for looking up referenced definitions
    visited: Set of definition names already being processed (prevents cycles)

Returns:
    The dereferenced schema with sibling keywords restored


## _prune_param

`fastmcp.utilities.json_schema._prune_param`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _prune_param(schema: dict[str, Any], param: str) -> dict[str, Any]
```

Return a new schema with *param* removed from `properties`, `required`,
and (if no longer referenced) `$defs`.


## _require_property

`fastmcp.utilities.json_schema._require_property`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _require_property(schema: dict[str, Any], property_name: str) -> dict[str, Any]
```

Return a copy of *schema* with *property_name* in ``required``.


## _single_pass_optimize

`fastmcp.utilities.json_schema._single_pass_optimize`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _single_pass_optimize(schema: dict[str, Any], prune_titles: bool = False, prune_additional_properties: bool = False, prune_defs: bool = True) -> dict[str, Any]
```

Optimize JSON schemas in a single traversal for better performance.

This function combines three schema cleanup operations that would normally require
separate tree traversals:

1. **Remove unused definitions** (prune_defs): Finds and removes `$defs` entries
   that aren't referenced anywhere in the schema, reducing schema size.

2. **Remove titles** (prune_titles): Strips `title` fields throughout the schema
   to reduce verbosity while preserving functional information.

3. **Remove restrictive additionalProperties** (prune_additional_properties):
   Removes `"additionalProperties": false` constraints to make schemas more flexible.

**Performance Benefits:**
- Single tree traversal instead of multiple passes (2-3x faster)
- Immutable design prevents shared reference bugs
- Early termination prevents runaway recursion on deeply nested schemas

**Algorithm Overview:**
1. Traverse main schema, collecting $ref references and applying cleanups
2. Traverse $defs section to map inter-definition dependencies
3. Remove unused definitions based on reference analysis

Args:
    schema: JSON schema dict to optimize (not modified)
    prune_titles: Remove title fields for cleaner output
    prune_additional_properties: Remove "additionalProperties": false constraints
    prune_defs: Remove unused $defs entries to reduce size

Returns:
    A new optimized schema dict

Example:
    >>> schema = {
    ...     "type": "object",
    ...     "title": "MySchema",
    ...     "additionalProperties": False,
    ...     "$defs": {"UnusedDef": {"type": "string"}}
    ... }
    >>> result = _single_pass_optimize(schema, prune_titles=True, prune_defs=True)
    >>> # Result: {"type": "object", "additionalProperties": False}


## _strip_discriminator

`fastmcp.utilities.json_schema._strip_discriminator`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _strip_discriminator(obj: Any) -> Any
```

Recursively remove OpenAPI ``discriminator`` keys from a schema.

Pydantic emits ``discriminator.mapping`` with values like
``#/$defs/ClassName``.  After ``$defs`` are inlined and removed by
``dereference_refs``, those mapping entries dangle.  The keyword is an
OpenAPI extension — the ``anyOf`` variants already carry ``const`` on
the discriminant field, so the mapping is redundant.

Only strips ``discriminator`` when it appears alongside ``anyOf`` or
``oneOf``, which is where the OpenAPI keyword lives.  A property
*named* ``discriminator`` (inside ``properties``) is left alone.


## _strip_remote_refs

`fastmcp.utilities.json_schema._strip_remote_refs`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _strip_remote_refs(obj: Any) -> Any
```

Return a deep copy of *obj* with non-local ``$ref`` values removed.

Local refs (starting with ``#``) are kept intact.  Remote refs
(``http://``, ``https://``, ``file://``, or any other URI scheme) are
stripped so that ``jsonref.replace_refs`` never attempts to fetch an
external resource.  This prevents SSRF / LFI when proxying schemas
from untrusted servers.


## compress_schema

Import as `fastmcp.resources.template.compress_schema`  ·  defined at `fastmcp.utilities.json_schema.compress_schema`

```python
def compress_schema(schema: dict[str, Any], prune_params: list[str] | None = None, prune_additional_properties: bool = False, prune_titles: bool = False, dereference: bool = False) -> dict[str, Any]
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Compress and optimize a JSON schema for MCP compatibility.

Args:
    schema: The schema to compress
    prune_params: List of parameter names to remove from properties
    prune_additional_properties: Whether to remove additionalProperties: false.
        Defaults to False to maintain MCP client compatibility, as some clients
        (e.g., Claude) require additionalProperties: false for strict validation.
    prune_titles: Whether to remove title fields from the schema
    dereference: Whether to dereference $ref by inlining definitions.
        Defaults to False; dereferencing is typically handled by
        middleware at serve-time instead.


## dereference_refs

Import as `fastmcp.server.middleware.dereference.dereference_refs`  ·  defined at `fastmcp.utilities.json_schema.dereference_refs`

```python
def dereference_refs(schema: dict[str, Any]) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Resolve all $ref references in a JSON schema by inlining definitions.

This function resolves $ref references that point to $defs, replacing them
with the actual definition content while preserving sibling keywords (like
description, default, examples) that Pydantic places alongside $ref.

This is necessary because some MCP clients (e.g., VS Code Copilot) don't
properly handle $ref in tool input schemas.

For self-referencing/circular schemas where full dereferencing is not possible,
this function falls back to resolving only the root-level $ref while preserving
$defs for nested references.

Only local ``$ref`` values (those starting with ``#``) are resolved.
Remote URIs (``http://``, ``file://``, etc.) are stripped before
resolution to prevent SSRF / local-file-inclusion attacks when proxying
schemas from untrusted servers.

Args:
    schema: JSON schema dict that may contain $ref references

Returns:
    A new schema dict with $ref resolved where possible and $defs removed
    when no longer needed

Example:
    >>> schema = {
    ...     "$defs": {"Category": {"enum": ["a", "b"], "type": "string"}},
    ...     "properties": {"cat": {"$ref": "#/$defs/Category", "default": "a"}}
    ... }
    >>> resolved = dereference_refs(schema)
    >>> # Result: {"properties": {"cat": {"enum": ["a", "b"], "type": "string", "default": "a"}}}


## replace_refs

`fastmcp.utilities.json_schema.replace_refs`

```python
def replace_refs(args: Any = (), kwargs: Any = {}) -> Any
```

Call jsonref lazily while preserving the module's patchable boundary.


## require_discriminator_property

Import as `fastmcp.utilities.openapi.json_schema_converter.require_discriminator_property`  ·  defined at `fastmcp.utilities.json_schema.require_discriminator_property`

```python
def require_discriminator_property(schema: dict[str, Any]) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Keep an OpenAPI discriminator's tag mandatory after the keyword is dropped.

Returns a copy of *schema* with ``discriminator.propertyName`` added to each
``anyOf``/``oneOf`` variant's ``required`` list. A Pydantic discriminated
union whose tag has a default omits that tag from ``required``; without this,
an untagged payload passes the generated schema but fails later in the source
model with ``union_tag_not_found``. No-op if there is no string
``propertyName``.


## resolve_root_ref

`fastmcp.utilities.json_schema.resolve_root_ref`

```python
def resolve_root_ref(schema: dict[str, Any]) -> dict[str, Any]
```

Resolve $ref at root level to meet MCP spec requirements.

MCP specification requires outputSchema to have "type": "object" at the root level.
When Pydantic generates schemas for self-referential models, it uses $ref at the
root level pointing to $defs. This function resolves such references by inlining
the referenced definition while preserving $defs for nested references.

Args:
    schema: JSON schema dict that may have $ref at root level

Returns:
    A new schema dict with root-level $ref resolved, or the original schema
    if no resolution is needed

Example:
    >>> schema = {
    ...     "$defs": {"Node": {"type": "object", "properties": {...}}},
    ...     "$ref": "#/$defs/Node"
    ... }
    >>> resolved = resolve_root_ref(schema)
    >>> # Result: {"type": "object", "properties": {...}, "$defs": {...}}


