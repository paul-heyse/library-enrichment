# Text that is not plain ASCII UTF-8

**The text has accents, another script, or another encoding.**

Both engines are Unicode-aware by default, which is usually what you want and occasionally expensive. `\w`, `\b` and `\d` cover far more than ASCII, and under `-P` ripgrep enables both UTF and UCP so PCRE2 behaves the same way. `--no-unicode` narrows them back to ASCII, which is a real speedup when searching identifiers in code that has none.

The baseline carries **Unicode 17.0.0** -- PCRE2 10.48's headline change, and the only large user-visible difference from 10.47. Four scripts arrived with it: Sidetic, Tai_Yo, Tolong_Siki and Beria_Erfe. That also means the extent of `\p{L}`, `\p{Nd}`, `\p{Han}`, `\w` and `\d` is wider than any reference written against Unicode 16 will say.

For set algebra over character classes, the form that works through `rg -P` is the Perl-style `(?[A & B])`. Since 10.48 it also composes with lookarounds, which was an internal compile error before (P035). The UTS#18 spelling `[A&&[B]]` is a genuine PCRE2 feature that ripgrep cannot reach, because it needs a compile option `rg` never sets -- probe P016 records what actually happens, and `unreachable.tsv` carries the conclusion. This is the clearest case in the repository of a feature existing in the library and not in the tool.

Encoding is separate from Unicode. `-E` transcodes on the way in; `-E none` disables it and searches raw bytes.

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| unicode-property | `\p{L}` | yes | yes | - | confirmed |
| extended-class-perl | `(?[A & B])` | no | requires -P | 10.45 | confirmed |
| extended-class-with-lookaround | `(?[\d-[1]]).(?<!x)` | no | requires -P | 10.48 | confirmed |
| unicode-17-script | `\p{Sidetic}` | no | requires -P | 10.48 | confirmed |
| extended-class-uts18 | `[A&&[B]]` | no | no | 10.45 | recorded |
| script-run | `(*sr:...)` | no | requires -P | - | confirmed |
| class-nested-rust | `[\w&&[^0-9]]` | yes | default-only | - | unknown |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P015 | confirmed | Is the Perl-style extended character class (?[...]) accepted? | `rg -oP (?[\p{L} & \p{ASCII}])+ docs/unicode.txt` |
| P016 | recorded | Is the UTS#18 [A&&[B]] form usable through stock rg? | `rg -oP [\p{L}&&[\p{Greek}]]+ docs/unicode.txt` |
| P017 | confirmed | Are script runs accepted? | `rg -coP (*sr:\w+) docs/unicode.txt` |
| P035 | confirmed | Can an extended class appear in the same pattern as a lookaround? | `rg -oP (?[\d-[1]]).(?<!x) docs/eclass.txt` |
| P036 | confirmed | Do the Unicode 17.0 scripts resolve, and does the engine still reject junk? | `rg -oP \p{Sidetic} docs/unicode17.txt` |

## Decision rules

- `--no-unicode` when searching ASCII identifiers in bulk.
- `(?[A & B])` for set algebra under `-P`, never `[A&&[B]]`.
- Extended classes compose with lookarounds at this baseline; on 10.47 the same pattern was a compile error, so code written against an older reference may have worked around a limit that is gone.
- `(*sr:...)` to require one script throughout -- useful against mixed-script spoofing, and note its answers moved with Unicode 17.
- `-E` for a known encoding; `-E none` when byte offsets must line up with the file on disk.

## Anti-patterns

- Using the UTS#18 class form and assuming it intersected anything.
- Assuming `\w` is ASCII.
- Comparing byte offsets across a transcoding boundary.

## Checklist

- Is Unicode matching needed here, or just costing time?
- Is the class-algebra form the Perl one?
- Do byte offsets need to match the on-disk bytes?
