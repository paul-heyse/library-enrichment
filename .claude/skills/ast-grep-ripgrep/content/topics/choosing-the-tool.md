# Choosing the tool

**Which of these tools answers my question at all?**

The two tools answer different questions and neither is a better version of the other. ripgrep answers **where does this text occur**; ast-grep answers **where does this syntax occur**. The distinction is not pedantic: a call written `foo(x)` and the word `foo` inside a comment are the same text and different syntax, so the tools disagree by design, and probe A013 measures the disagreement on a two-line fixture.

Pick the weakest instrument that can express the requirement. A literal beats a regex, a regex beats PCRE2, a node kind beats a code pattern, and a code pattern beats a relational YAML rule. Each step up costs speed, adds a way to be subtly wrong, and widens what you must verify. Escalate when the weaker form genuinely cannot express the constraint -- not because the stronger one feels more thorough.

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| A013 | confirmed | Does ast-grep ignore an occurrence that lives inside a comment? | `ast-grep run -l rust -p helper($X) src/main.rs` |
| P012 | confirmed | Does whole-pattern recursion match balanced delimiters, and what does the default engine do with the same pattern? | `rg -oP \((?:[^()]\|(?R))*\) docs/nested.txt` |
| P032 | confirmed | Does -F disable metacharacters? | `rg -F -c println!( src/main.rs` |

## Decision rules

- Text that is not code -- a comment, a docstring, a config key, a log string, a generated identifier -- is ripgrep's, always. ast-grep cannot match what is not a node.
- A question about structure -- is this a call, a declaration, an implementation, an import -- is ast-grep's. A regex that approximates structure will be wrong at the first line break.
- Needing to know *which* declaration a name refers to is neither tool's. Both establish occurrence; only a compiler or language server establishes identity.
- When both could work, ripgrep first: it is faster, and its output narrows the file set ast-grep then parses.

## Anti-patterns

- Reaching for `-P` because the pattern looks complicated. Most complicated-looking patterns are ordinary regexes.
- Rebuilding a parser out of regex. If the pattern is growing a case for every way the code might be formatted, the question was structural two revisions ago.
- Treating a match as proof that a symbol is used. It is proof that the text or the syntax occurs.

## Checklist

- Is the target text, or syntax?
- Is the weakest sufficient form being used?
- Does the conclusion need semantic identity that neither tool establishes?
