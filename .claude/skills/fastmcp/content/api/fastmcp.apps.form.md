# `fastmcp.apps.form`

Distribution: `fastmcp`

## _FORM_SUPPORTS_DEFAULTS

`fastmcp.apps.form._FORM_SUPPORTS_DEFAULTS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FORM_SUPPORTS_DEFAULTS = Version(prefab_ui.__version__) >= Version('0.19.1')
```

## FormInput

`fastmcp.apps.form.FormInput`

```python
class FormInput(FastMCPApp)
```

**Bases** `FastMCPApp`

**Inherited (22)**

- from `fastmcp.apps.app.FastMCPApp`: `add_tool`, `lifespan`, `name`, `run`, `tool`, `ui`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that collects structured input via a Pydantic model.

Define a model for the data you need, and ``FormInput`` generates
a form from it using ``Form.from_model()``. Field types, labels,
descriptions, and validation are all derived from the model.

Optionally provide an ``on_submit`` callback to process the
validated data. The callback receives a model instance and returns
a string that goes back to the LLM. Without a callback, the
validated JSON is sent directly.

Example::

    from pydantic import BaseModel
    from fastmcp import FastMCP
    from fastmcp.apps.form import FormInput

    class Contact(BaseModel):
        name: str
        email: str

    mcp = FastMCP("My Server")
    mcp.add_provider(FormInput(model=Contact))

With a callback::

    def save_contact(contact: Contact) -> str:
        db.insert(contact.model_dump())
        return f"Saved {contact.name}"

    mcp.add_provider(FormInput(model=Contact, on_submit=save_contact))


## _backfill_boolean_defaults

`fastmcp.apps.form._backfill_boolean_defaults`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _backfill_boolean_defaults(model: type[pydantic.BaseModel], data: dict[str, Any]) -> dict[str, Any]
```

Fill in missing boolean fields with their model defaults.

HTML checkboxes omit the field entirely when unchecked, so the
submitted data dict won't contain a key for ``False`` booleans.
This backfills those missing keys so Pydantic validation succeeds.


