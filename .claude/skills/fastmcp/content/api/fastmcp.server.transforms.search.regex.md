# `fastmcp.server.transforms.search.regex`

Distribution: `fastmcp`

## RegexSearchTransform

Import as `fastmcp.server.transforms.search.RegexSearchTransform`  ·  defined at `fastmcp.server.transforms.search.regex.RegexSearchTransform`

```python
class RegexSearchTransform(BaseSearchTransform)
```

**Also exported as** `fastmcp.server.transforms.search.RegexSearchTransform`

**Bases** `BaseSearchTransform`

**Inherited (16)**

- from `fastmcp.server.transforms.Transform`: `get_prompt`, `get_resource`, `get_resource_template`
- from `fastmcp.server.transforms.catalog.CatalogTransform`: `get_prompt_catalog`, `get_resource_catalog`, `get_resource_template_catalog`, `get_tool_catalog`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transform_prompts`, `transform_resource_templates`, `transform_resources`
- from `fastmcp.server.transforms.search.base.BaseSearchTransform`: `get_tool`, `transform_tools`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Search transform using regex pattern matching.

Tools are matched against their name, description, and parameter
information using ``re.search`` with ``re.IGNORECASE``.


