---
name: typer-rich
description: Find what Typer, Rich and Typer's vendored Click can actually do, at pinned releases, from a prebuilt index of all three plus 855 files of upstream documentation, runnable tutorials, examples and tests, joined to 19 executed probes that capture what the output actually looks like. Use when building or changing a command-line interface, when printing, styling, tabulating, prompting or showing progress, when choosing between Typer's way and Rich's way of doing the same thing, when help or error output does not look the way you expected, when a width, a colour or a bracketed word is wrong, and to check whether code already written targets Click interop that Typer no longer supports. Do not use for general Python or terminal questions unrelated to these three libraries, and do not use it as a Click reference — the Click indexed here is Typer's private copy, not the package.
---

# Typer and Rich capability repository

Two things here read correctly, pass a type checker, and are wrong.

**`from rich import Console` raises `ImportError`.** Rich imports it under `if TYPE_CHECKING:`
and nowhere else. Measured: `ty check` and `pyrefly check` both report zero errors on that line.
Two static analysers certify an import Python refuses.

**A help string loses words.** `typer.Typer()` defaults to `rich_markup_mode="rich"`, so every
help string is parsed as markup, and a bracketed word that resolves to no style is deleted in
silence. `help="give a [path] here"` renders as `give a  here`. No exception, no warning.

Behind both sits the fact that invalidates most of what a model remembers about Typer:
**`typer/_click/` is Typer's own adapted copy of Click**, and `click` is not a Typer dependency —
`Requires-Dist` names `shellingham`, `rich`, `annotated-doc` and `colorama` on Windows, nothing
else. `import click` still works, because click is probably installed for some other reason, and
the object you get is unrelated to the one Typer runs. `typer.main.get_command(app)` returns a
`Command` whose MRO lands in `typer._click`, which no Click plugin will accept.

So the failure this repository exists to prevent is not "I could not find the answer". It is
**confidently writing an API that is not the one installed**, and its sibling, **confidently
predicting output that renders differently.**

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

| Subject | Pin | Indexed |
|---|---|---|
| `typer` | 0.27.2 (2026-08-28) | 248 items |
| `rich` | 15.0.0 (2026-04-12) | 553 items |
| `typer._click` | vendored Click, inside typer 0.27.2 | 201 items, as its own subject |

The resolved environment is 7 distributions, recorded in `content/PROVENANCE.json`, because the
declared pins are not a pin: three of the seven arrive through ranges, and `Syntax` output
depends on which Pygments resolves. `rich._unicode_data` (23 generated width tables) and
`rich._emoji_codes` are excluded from the API index as data, not API; their behaviour is covered
by probe instead.

1,002 symbols · 2,231 members · 6,396 parameters · 3,155 locations · 14,122 references ·
9,813 call edges · 51 assignability verdicts · **19 executed probes, 11 captured renders** ·
2 pinned Typer facts · 6 version rows (Rich, plus Typer 0.27.0) · 16 environment variables · 855 corpus files · 106 module pages ·
13 topics · 11 seams · 12 catalogs · 3 generated rules.

## Escalation ladder

Stop at the first rung that answers the question. The rungs are questions, not file kinds.

0. **Which construct do I actually want?** `content/topics/00-map.md`. Thirteen axes, each naming
   its entry points, the upstream guide, the decision rules and the anti-patterns. Start here:
   the expensive mistake with these libraries is reaching for the wrong construct, not
   misspelling the right one.

1. **Am I about to write `import click`, `typer-slim`, or `rich.__version__`?**
   `content/catalogs/breaking-changes.md` first. Typer's facts are stated in the present
   tense; Rich's rows carry upstream's own replacement text.
   ```
   rg -c 'typer' content/catalogs/breaking-changes.md
   ```

2. **Typer's way or Rich's way?** They present near-duplicate surfaces that are not
   interchangeable. `content/catalogs/same-job-different-answer.md`, or the measured half:
   ```
   rg -P '\t(confirmed|recorded)\t' content/index/behaviors.tsv | cut -f1,3,5
   ```

3. **Does it exist, and how do I import it?**
   ```
   rg -P '^rich\.table\.Table\t' content/index/symbols.tsv
   rg -P '^rich\.console\.Console\t' content/index/members.tsv | cut -f2,6
   ```
   Column 2 is the spelling to **import**; column 1 is where it is defined. Column 4 is the
   subject, and a `click` there is a warning rather than an address.

4. **It says `nameable: no` — so how do I ever meet it?** Column 7, `reachability`, and column 8,
   what reaches it. This is the whole vendored-Click answer: you never import a `Context`, you are
   handed one.
   ```
   rg -P '^typer\._click\.core\.Context\t' content/index/symbols.tsv | cut -f6,7,8
   rg -P '\tsubtype-of\t' content/index/symbols.tsv | cut -f1,8
   ```
   Then `content/catalogs/vendored-click.md` for the whole picture.

5. **What does this actually look like, and how wide?** `content/probes/00-index.md`, which shows
   every capture inline with the construction that produced it. This is the question no signature
   can answer.
   ```
   rg -P '\thelp-rendering\t' content/index/behaviors.tsv | cut -f1,5
   rg -F '<str>' content/probes/captures/T002.txt
   ```

6. **Why is my output plain, or 80 columns, or missing a word?**
   `content/catalogs/env-vars.md`, ordered by the layer each acts on. Two of them beat arguments
   you passed explicitly.
   ```
   rg -P '^\| `NO_COLOR`' content/catalogs/env-vars.md
   ```

7. **What do I write so my object renders, or my type parses?** `content/seams/00-map.md`, then
   the page. Eleven seams, each stating whether it is documented, merely observed, or private.
   `rich-utils-overrides` is the third kind, and the page exists so nobody ships a monkeypatch
   while calling it an API.
   ```
   rg -c 'private' content/seams/00-map.md
   ```

8. **What is this parameter's default, and is it keyword-only?**
   ```
   rg -P '^rich\.console\.Console\.__init__\t' content/index/parameters.tsv | cut -f4,5,6,7
   ```
   Rich's constructors are large keyword surfaces, and "what happens if I leave it out" is
   almost always the real question.

9. **Which class can I actually pass to `console.print()`?** `content/index/satisfies.tsv` —
   adjudicated by the checker, not matched by name. 42 classes satisfy `ConsoleRenderable`.
   ```
   rg -P '\trich\.console\.ConsoleRenderable\t' content/index/satisfies.tsv | cut -f1,3
   ```

10. **Where is this defined, and what does it inherit?**
    ```
    rg -P '^typer\.core\.TyperGroup\t' content/index/locations.tsv
    rg -P '^typer\._click\.core\.Command\t' content/index/descendants.tsv | cut -f2,3
    ```

11. **How does upstream actually do it?** The corpus is Typer's and Rich's own text at the pinned
    tags: 304 runnable tutorial examples, 363 tests, 139 documentation files, 37 Rich examples.
    ```
    rg -l 'add_typer' content/corpus/typer/docs_src | head
    rg -l '__rich_console__' content/corpus/rich | head
    ```

12. **Full prose for a resolved item.** Read `content/api/<module>.md`. The path is a rule, not a
    lookup: the module part of the canonical path, verbatim. `rich.table.Table` →
    `content/api/rich.table.md`; `typer._click.utils.echo` → `content/api/typer._click.utils.md`.

13. **What is my own code getting wrong?**
    ```
    ast-grep scan -c queries/sgconfig.yml --filter '^project-' queries/fixtures
    ```
    Swap `queries/fixtures` for the repository you are editing. Reach for this unprompted after
    writing Typer or Rich code. All three `project-*` rules are generated from the pinned
    breaking-change table rather than typed, so they cannot go stale at a re-pin.

## Rules that keep answers correct

**`typer._click` is not `click`.** It is Typer's adapted copy of Click, indexed as its own
subject because Typer's public API is defined in terms of it — `typer.echo` *is*
`typer._click.utils.echo`, and `typer.Context` subclasses `typer._click.core.Context`. Read a
`click` row as a fact about Typer's copy at this pin. Never cite one as a fact about the `click`
package, and never write the import.

**`nameable: no` is where the answer starts, not where it stops.** 201 items sit behind a private
module path and you will meet plenty of them. Column 7 says how: `importable` (Typer re-exports
it), `subclassed-via` (a class you can name inherits from it), `subtype-of` (you catch it through
an ancestor — six vendored Click exceptions are reached as `typer.TyperException`), `received` (it
arrives in a parameter), `declared` (it is what your annotation becomes), `internal`.

**Rich markup silently deletes text that is not a style.** `Console.print("status [ok]")` emits
`status ` (probe M001). `[path]`, `[key=value]` and `[a b]` all vanish; `[FILE]`, `[1-9]` and
`[--flag]` survive — the rule is "parses as a style name", not "contains a bracket", which is why
one working example proves nothing. An unbalanced `[/]` raises `MarkupError` instead, so neither
"always raises" nor "never raises" is safe. `typer.echo` preserves both.

**A captured render is a fact about a construction, not about a terminal.** Every capture pins
`width`, `color_system`, `force_terminal` and `legacy_windows` and runs under an environment that
inherits nothing. Change one and the bytes change. Quote the construction whenever you quote the
output.

**`confirmed` is stronger than `recorded`, and the difference is a control.** A probe whose
control did not come out differently demonstrates nothing and is never reported as a pass.

**Width is a property of the Console, not of the thing you print** — and even an explicitly
passed `width=` loses to `TERM=dumb` (probe E003). Cell width additionally depends on
`UNICODE_VERSION`, which is read from `os.environ` directly and bypasses `Console(_environ=...)`
(probe E005).

**Rich's extension surface is duck-typed, so the class tables under-report it.** An object with
`__rich_console__` is a renderable and has no base class; `descendants.tsv` will never list it.
`rich.abc.RichRenderable` has zero subclasses and its own docstring says not to extend it. Use
`content/seams/renderable-protocol.md`, not the hierarchy.

**The corpus is upstream's text, not this repository's claims.** Verbatim at `0.27.2` and
`v15.0.0`, verified by digest on every build. Quote it as upstream's word; quote `content/` as
this index's reading of the installed code.

**ast-grep establishes syntax, not semantics.** It resolves no imports and no types. A rule hit is
a question about your code, not a proven defect — every one is `severity: hint`.

**Absence from `symbols.tsv` has four causes**, and they are not the same: the name is a member
rather than a top-level item (check `members.tsv`); it re-exports out of the indexed set (check
`unresolved.tsv`); it lives in an excluded data module (`rich._unicode_data`,
`rich._emoji_codes`); or it genuinely does not exist at these pins — in which case
`catalogs/breaking-changes.md` may say so.

## Reporting

Cite the importable spelling and the file you read it in. When the claim is behavioural, cite the
probe id; when it is about output, cite the probe id **and the construction**, not the bytes
alone. When a capability exists but you are not recommending it, say so — the point of this
repository is that the caller learns the option existed. When the index is silent, report silence
rather than absence, and name which of the four causes applies.

## Additional references

Read `reference.md` for the table schemas, the probe protocol, the rule inventory, the known
limits and more query recipes. Read `queries/README.md` before changing a rule, and
`build/README.md` before rebuilding or re-pinning.
