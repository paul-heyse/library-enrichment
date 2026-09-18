# Reader and verification commands

All scripts resolve inputs relative to their own skill copy. The reader requires only Python 3.10+
standard library; regeneration uses Python 3.14 (or zstd CLI), ast-grep and retained build inputs.
In a workspace requiring uv, use `uv run --no-project python` in place of `python3`.

```sh
python3 scripts/reference.py find --task "CDF residual row filter" --limit 2
python3 scripts/reference.py find --crate deltalake-core --input DataFrame
python3 scripts/reference.py find --effect "publish"
python3 scripts/reference.py show delta.commit --view contract
python3 scripts/reference.py show deltalake::DeltaTable --member write --view contract
python3 scripts/reference.py compare delta.replace delta.merge
```

`find` ranks authored task vocabulary, facets and matching operation paths; it is lexical discovery,
not semantic proof. `show` accepts a decision ID, canonical path or alias, with optional `--member`.
`brief`, `contract` and `evidence` views separate selection from exact implementation detail.
`--max-bytes` bounds the encoded JSON output. A fragmented response has `text_fragment`, a content
digest and `next_offset`; repeat with that offset and concatenate fragments until it is null.
Offsets refer to the serialized response, not record indices. Changing the request changes the stream.

```sh
python3 scripts/run_probes.py
python3 scripts/check_access.py
python3 build/test_contracts.py
python3 scripts/invalidation.py
python3 scripts/package.py reader --output /chosen/path/delta-reader.tar.gz
python3 scripts/package.py research --output /chosen/path/delta-research.tar.gz
```

Rust execution needs the pinned toolchain and dependencies already fetched when using `--offline`.
The reader/research bundles do not claim to vendor the entire Cargo graph. Runtime evidence is
rejected by the capability builder if its retained test/fixture/lock sources changed.

See [maintenance](../MAINTENANCE.md) for the complete replay and qualification workflow.
