# Languages, grammars and custom parsers

**Is my language supported, and what if it is not?**

ast-grep's built-in language set is not enumerable from `--help`, which links to a web page instead. So `languages.tsv` is built by **probing the binary** with candidate names, and records what this binary accepted and what it rejected.

Node kinds come from the shipped schemas -- and the two schema sources disagree. `languages.json` and the per-language `<lang>_rule.json` each list kinds the other omits, so disputed kinds are adjudicated by running them. Three published kinds turned out to be rejected by this binary: `python/except_group_clause`, `rust/constrained_type_parameter` and `rust/optional_type_parameter`. A rule using one of those fails with exit 8.

Beyond the built-ins, `customLanguages` registers a Tree-sitter grammar from a native library, and `languageGlobs` routes an unusual extension to a known language. Custom grammars load native code, which makes an untrusted `sgconfig.yml` a trust decision.

## Decision rules

- Check `languages.tsv` before assuming support. It records this binary's answer, not a web page's.
- Check `kinds.tsv` for the adjudication column before using a kind from documentation.
- `languageGlobs` for an odd extension; `customLanguages` only when the grammar is genuinely absent.
- A kind list is evidence about the grammar this ast-grep bundles, not about tree-sitter in general.

## Anti-patterns

- Trusting the published language table over the installed binary.
- Using a kind from a blog post without checking it is accepted.
- Loading a custom parser from an untrusted repository's config.

## Checklist

- Does `languages.tsv` say this binary accepts the language?
- Is the kind marked accepted?
- Does anything here load native code from a config you did not write?
