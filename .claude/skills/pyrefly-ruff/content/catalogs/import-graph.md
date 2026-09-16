# The import graph

`ruff analyze graph` maps each file to the files it imports. It is fast, needs no
configuration, and is narrower than its name in four ways that are all silent:

1. **File-level, not module-level.** Keys and values are paths relative to the working
   directory. There is no module-name representation, so joining it to anything that
   speaks dotted module names is your job.
2. **No `--output-format`.** It always writes JSON to stdout, and always prints
   `warning: \`ruff analyze graph\` is experimental and may change without warning`
   to stderr. Redirect the two separately.
3. **A path that does not exist yields `{}` and exit status 0.** Measured during
   acquisition, the status was `0`. Nothing tells
   *no imports* from *wrong path*, so check the key count, never the status.
4. **Third-party edges need `--python`.** Without a virtual environment it resolves only
   first-party files.

Measured output for the acquisition fixture:

```json
{
 "pkg/__init__.py": [],
 "pkg/app.py": [
  "pkg/core.py"
 ],
 "pkg/core.py": []
}
```

`--direction dependents` inverts it. `analyze.include-dependencies` in configuration adds
edges the scanner cannot see, which is the escape hatch for dynamic imports:

```toml
[tool.ruff.analyze.include-dependencies]
"foo/bar.py" = ["foo/baz/*.py"]
```

For call edges rather than import edges, see `cross-references.md` — this command has no
notion of a call.
