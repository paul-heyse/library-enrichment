# `tracing_subscriber::filter::env::builder`

Crate `tracing-subscriber` · 1 public items · structured records in [`model/tracing_subscriber.filter.env.builder.json`](../model/tracing_subscriber.filter.env.builder.json)

## Builder

`struct` · `tracing_subscriber::filter::env::builder::Builder`

Also reachable as `tracing_subscriber::filter::Builder`

```rust
struct Builder
```

**Derives**: Clone, Debug, Default

**Methods** (8)

```rust
fn from_env(&self) -> Result<EnvFilter, FromEnvError>
fn from_env_lossy(&self) -> EnvFilter
fn parse<S: AsRef<str>>(&self, dirs: S) -> Result<EnvFilter, filter::directive::ParseError>
fn parse_lossy<S: AsRef<str>>(&self, dirs: S) -> EnvFilter
fn try_from_env(&self) -> Result<EnvFilter, FromEnvError>
fn with_default_directive(self, default_directive: Directive) -> Self
fn with_env_var(self, var: impl ToString) -> Self
fn with_regex(self, regex: bool) -> Self
```

A [builder] for constructing new [`EnvFilter`]s.

[builder]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

---
