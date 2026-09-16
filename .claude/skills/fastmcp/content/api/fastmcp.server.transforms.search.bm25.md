# `fastmcp.server.transforms.search.bm25`

Distribution: `fastmcp`

## BM25SearchTransform

Import as `fastmcp.server.transforms.search.BM25SearchTransform`  ·  defined at `fastmcp.server.transforms.search.bm25.BM25SearchTransform`

```python
class BM25SearchTransform(BaseSearchTransform)
```

**Also exported as** `fastmcp.server.transforms.search.BM25SearchTransform`

**Bases** `BaseSearchTransform`

**Inherited (16)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`
- from `fastmcp.server.transforms.catalog.CatalogTransform`: `get_prompt_catalog`, `get_resource_catalog`, `get_resource_template_catalog`, `get_tool_catalog`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transform_prompts`, `transform_resource_templates`, `transform_resources`
- from `fastmcp.server.transforms.search.base.BaseSearchTransform`: `get_tool`, `transform_tools`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Search transform using BM25 Okapi relevance ranking.

Maintains an in-memory index that is lazily rebuilt when the tool
catalog changes (detected via a hash of tool names).


## _BM25Index

`fastmcp.server.transforms.search.bm25._BM25Index`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _BM25Index
```

**Declared members (4)**

- `b = b`  _instance-attribute_
- `def build(self, documents: list[str]) -> None`
- `k1 = k1`  _instance-attribute_
- `def query(self, text: str, top_k: int) -> list[int]`
  Return indices of top_k documents sorted by BM25 score.

Self-contained BM25 Okapi index.


## _catalog_hash

`fastmcp.server.transforms.search.bm25._catalog_hash`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _catalog_hash(tools: Sequence[Tool]) -> str
```

SHA256 hash of sorted tool searchable text for staleness detection.


## _tokenize

`fastmcp.server.transforms.search.bm25._tokenize`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _tokenize(text: str) -> list[str]
```

Normalize and extract Unicode alphanumeric tokens.


