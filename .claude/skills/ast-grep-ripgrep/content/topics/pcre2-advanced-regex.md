# Regex beyond the default engine

**I need lookaround, backreferences, or something the default engine rejects.**

ripgrep ships two regex engines. The default is Rust's `regex`, which guarantees linear-time matching and therefore cannot offer lookaround, backreferences or recursion -- those features are what make a backtracking engine able to blow up. `-P` switches to PCRE2, which offers all of them and gives up the guarantee.

**PCRE2 10.48 is the baseline here**, asserted at build time, so everything below is simply available through `-P`. The `since` column in the construct table is history -- when upstream introduced a thing -- not a gate on whether you have it.

The trap is not knowing which features exist; it is assuming the default engine will **tell you** when one is missing. Sometimes it does, loudly and helpfully: lookaround produces an error naming `-P`. Sometimes it does not. Probe P012 records the case that matters -- the default engine parses `(?R)` as an inline group, silently degrades a balanced-delimiter pattern to `\([^()]*\)`, and returns `(b)` where PCRE2 returns `(a(b)c)`. A wrong answer, exit 0, no warning.

And do not read `rg --pcre2-version` to decide any of this. It prints a constant compiled into ripgrep at *its* build time, so it describes the build and not the runtime: on this machine it reads 10.45 while the linked library is 10.48, three releases of drift. If you genuinely need to know at runtime, ask PCRE2 itself with its own conditional, `(?(VERSION=10.48)yes|no)`, which is evaluated inside libpcre2.

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| lookahead | `(?=...) (?!...)` | no | requires -P | - | confirmed |
| lookbehind | `(?<=...) (?<!...)` | no | requires -P | - | confirmed |
| variable-lookbehind | `(?<=.{0,20}x)` | no | requires -P | 10.43 | confirmed |
| backreference | `\1` | no | requires -P | - | confirmed |
| named-backreference | `\k<n>` | no | requires -P | - | confirmed |
| match-reset | `\K` | no | requires -P | - | confirmed |
| possessive-quantifier | `a++ a*+` | no | requires -P | - | confirmed |
| atomic-group | `(?>...)` | no | requires -P | - | confirmed |
| branch-reset | `(?\|...)` | no | requires -P | - | confirmed |
| conditional | `(?(1)a\|b)` | no | requires -P | - | confirmed |
| recursion | `(?R)` | no | requires -P | - | confirmed |
| subroutine | `(?&name)` | no | requires -P | - | unknown |
| recursion-returned-captures | `(?&name(<cap>))` | no | requires -P | 10.47 | confirmed |
| control-verb-skip-fail | `(*SKIP)(*F)` | no | requires -P | - | confirmed |
| control-verb-commit | `(*COMMIT) (*PRUNE) (*THEN)` | no | requires -P | - | unknown |
| control-verb-accept | `(*ACCEPT) (*FAIL)` | no | requires -P | - | unknown |
| resource-limits | `(*LIMIT_MATCH=n) (*LIMIT_DEPTH=n) (*LIMIT_HEAP=n)` | no | requires -P | - | unknown |
| no-jit | `(*NO_JIT)` | no | requires -P | - | unknown |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P001 | confirmed | Does the default engine reject lookahead, and does -P accept it? | `rg -P -c foo(?= bar) docs/quotes.txt` |
| P002 | confirmed | Does positive lookahead match through -P? | `rg -oP he(?= said) docs/quotes.txt` |
| P003 | confirmed | Does positive lookbehind match through -P? | `rg -oP (?<=said )"hello" docs/quotes.txt` |
| P004 | confirmed | Is bounded variable-length lookbehind accepted? | `rg -coP (?<=.{0,20}said )\S+ docs/quotes.txt` |
| P005 | confirmed | Do numeric backreferences work through -P? | `rg -oP (\w+) \1 docs/notes.md` |
| P006 | confirmed | Do named backreferences work, so paired delimiters can be required? | `rg -oP (?<q>["'])\w+\k<q> docs/quotes.txt` |
| P008 | confirmed | Are possessive quantifiers accepted? | `rg -coP \w++  docs/notes.md` |
| P009 | confirmed | Are atomic groups accepted? | `rg -coP (?>\w+)  docs/notes.md` |
| P010 | confirmed | Is branch reset accepted, reusing capture numbers across alternatives? | `rg -oP (?\|(?<d>Sat)urday\|(?<d>Sun)day) docs/weekend.txt` |
| P011 | confirmed | Are conditional patterns accepted? | `rg -coP (")?\w+(?(1)"\|\b) docs/quotes.txt` |
| P012 | confirmed | Does whole-pattern recursion match balanced delimiters, and what does the default engine do with the same pattern? | `rg -oP \((?:[^()]\|(?R))*\) docs/nested.txt` |
| P013 | confirmed | Do subroutine calls return selected capture groups to the caller? | `rg -oP (?(DEFINE)(?<day>(?\|(?<s>Sat)urday\|(?<s>Sun)day)))(?&day(<s>)),\k<s> docs/weekend.txt` |
| P018 | confirmed | Does the (*SKIP)(*F) exclusion idiom work? | `rg -coP "[^"]*"(*SKIP)(*F)\|said docs/quotes.txt` |

## Decision rules

- Use `-P` when the constraint is genuinely relational -- this token only when preceded by that, this delimiter matched by the same delimiter. Not because the pattern is long.
- Feature-probe a version-gated construct against a two-line fixture with a control that must fail. Never gate on `--pcre2-version`.
- `(*SKIP)(*F)` excludes a region without a parser: match the region you want to discard, fail it, and let a later alternative match outside it.
- `\K` drops everything matched so far from the reported span -- the cheapest way to assert a prefix without printing it.
- Untrusted patterns belong on the default engine. PCRE2 backtracks, and a hostile pattern is a resource-consumption vector.

## Anti-patterns

- Assuming an unsupported construct will error. Lookaround errors; recursion degrades silently (P012).
- Reading `--pcre2-version` and concluding a feature is unavailable.
- Expecting `--regex-size-limit` or `--dfa-size-limit` to bound PCRE2. They bound the default engine only; see `unreachable.tsv`.
- Unbounded variable-length lookbehind. Every branch needs a known maximum, capped at 255 by default with no CLI setter.

## Checklist

- Could the default engine express this?
- If version-gated: is there a probe, with a control that fails?
- Is the pattern trusted?
- Would a structural query be more precise than a cleverer regex?
