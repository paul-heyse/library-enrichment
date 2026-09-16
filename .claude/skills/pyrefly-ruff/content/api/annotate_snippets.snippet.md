# `annotate_snippets::snippet`

Crate `ruff_annotate_snippets` · 12 public items · structured records in [`model/annotate_snippets.snippet.json`](../model/annotate_snippets.snippet.json)

## AnnotationKind

`enum` · `annotate_snippets::snippet::AnnotationKind`

Also reachable as `ruff_annotate_snippets::AnnotationKind`

```rust
enum AnnotationKind
```

**Variants**: `Primary`, `Context`, `Visible`

**Derives**: Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn span<'a>(self, span: Range<usize>) -> Annotation<'a>
```

The type of [`Annotation`] being applied to a [`Snippet`]

---

## Element

`enum` · `annotate_snippets::snippet::Element`

Also reachable as `ruff_annotate_snippets::Element`

```rust
enum Element<'a>
```

**Variants**: `Message`, `Cause`, `Suggestion`, `Origin`, `Padding`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**via `core::convert::From`**

```rust
fn from(value: Padding) -> Self
fn from(value: Origin<'a>) -> Self
fn from(value: Snippet<'a, Patch<'a>>) -> Self
fn from(value: Snippet<'a, Annotation<'a>>) -> Self
fn from(value: Message<'a>) -> Self
```

A section of content within a [`Group`]

---

## Annotation

`struct` · `annotate_snippets::snippet::Annotation`

Also reachable as `ruff_annotate_snippets::Annotation`

```rust
struct Annotation<'a>
```

**Derives**: Clone, Debug

**Methods** (3)

```rust
fn hide_snippet(self, yes: bool) -> Self
fn highlight_source(self, highlight_source: bool) -> Self
fn label(self, label: impl Into<OptionCow<'a>>) -> Self
```

Highlight and describe a span of text within a [`Snippet`]

See [`AnnotationKind`] to create an annotation.

# Example

```rust
# #[allow(clippy::needless_doctest_main)]
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};

fn main() {
    let source = r#"                annotations: vec![SourceAnnotation {
                label: "expected struct `annotate_snippets::snippet::Slice`, found reference"
                    ,
                range: <22, 25>,"#;
    let report =
        &[Level::ERROR
            .primary_title("expected type, found `22`")
            .element(
                Snippet::source(source)
                    .line_start(26)
                    .path("examples/footer.rs")
                    .annotation(AnnotationKind::Primary.span(193..195).label(
                        "expected struct `annotate_snippets::snippet::Slice`, found reference",
                    ))
                    .annotation(
                        AnnotationKind::Context
                            .span(34..50)
                            .label("while parsing this struct"),
                    ),
            )];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
```

<svg width="860px" height="182px" xmlns="http://www.w3.org/2000/svg">
  <style>
    .fg { fill: #AAAAAA }
    .bg { fill: #000000 }
    .fg-bright-blue { fill: #5555FF }
    .fg-bright-red { fill: #FF5555 }
    .container {
      padding: 0 10px;
      line-height: 18px;
    }
    .bold { font-weight: bold; }
    tspan {
      font: 14px SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
      white-space: pre;
      line-height: 18px;
    }
  </style>

  <rect width="100%" height="100%" y="0" rx="4.5" class="bg" />

  <text xml:space="preserve" class="container fg">
    <tspan x="10px" y="28px"><tspan class="fg-bright-red bold">error</tspan><tspan class="bold">: expected type, found `22`</tspan>
</tspan>
    <tspan x="10px" y="46px"><tspan>  </tspan><tspan class="fg-bright-blue bold"> ╭▸ </tspan><tspan>examples/footer.rs:29:25</tspan>
</tspan>
    <tspan x="10px" y="64px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="82px"><tspan class="fg-bright-blue bold">26</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>                 annotations: vec![SourceAnnotation {</tspan>
</tspan>
    <tspan x="10px" y="100px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>                                   </tspan><tspan class="fg-bright-blue bold">────────────────</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">while parsing this struct</tspan>
</tspan>
    <tspan x="10px" y="118px"><tspan>   </tspan><tspan class="fg-bright-blue bold">┆</tspan>
</tspan>
    <tspan x="10px" y="136px"><tspan class="fg-bright-blue bold">29</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>                 range: &lt;22, 25&gt;,</tspan>
</tspan>
    <tspan x="10px" y="154px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰╴</tspan><tspan>                        </tspan><tspan class="fg-bright-red bold">━━</tspan><tspan> </tspan><tspan class="fg-bright-red bold">expected struct `annotate_snippets::snippet::Slice`, found reference</tspan>
</tspan>
    <tspan x="10px" y="172px">
</tspan>
  </text>

</svg>

---

## Group

`struct` · `annotate_snippets::snippet::Group`

Also reachable as `ruff_annotate_snippets::Group`

```rust
struct Group<'a>
```

**Derives**: Clone, Debug

**Methods** (6)

```rust
fn element(self, section: impl Into<Element<'a>>) -> Self
fn elements(self, sections: impl IntoIterator<Item = impl Into<Element<'a>>>) -> Self
fn is_empty(&self) -> bool
fn lineno_offset(self, offset: usize) -> Self
fn with_level(level: Level<'a>) -> Self
fn with_title(title: Title<'a>) -> Self
```

A [`Title`] with supporting [context][Element] within a [`Report`]

[Decor][crate::renderer::DecorStyle] is used to visually connect [`Element`]s of a `Group`.

Generally, you will create separate group's for:
- New [`Snippet`]s, especially if they need their own [`AnnotationKind::Primary`]
- Each logically distinct set of [suggestions][Patch`]

# Example

```rust
# #[allow(clippy::needless_doctest_main)]
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use anstyle::AnsiColor;
use anstyle::Effects;
use anstyle::Style;

fn main() {
    let source = r#"// Make sure "highlighted" code is colored purple

//@ compile-flags: --error-format=human --color=always
//@ edition:2018

use core::pin::Pin;
use core::future::Future;
use core::any::Any;

fn query(_: fn(Box<(dyn Any + Send + '_)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>>) {}

fn wrapped_fn<'a>(_: Box<(dyn Any + Send)>) -> Pin<Box<(
    dyn Future<Output = Result<Box<(dyn Any + 'static)>, String>> + Send + 'static
)>> {
    Box::pin(async { Err("nope".into()) })
}

fn main() {
    query(wrapped_fn);
}"#;

    const MAGENTA: Style = AnsiColor::Magenta.on_default().effects(Effects::BOLD);
    let message = format!(
        "expected fn pointer `{MAGENTA}for<'a>{MAGENTA:#} fn(Box<{MAGENTA}(dyn Any + Send + 'a){MAGENTA:#}>) -> Pin<_>`
      found fn item `fn(Box<{MAGENTA}(dyn Any + Send + 'static){MAGENTA:#}>) -> Pin<_> {MAGENTA}{{wrapped_fn}}{MAGENTA:#}`",
    );

    let report = &[
        Level::ERROR
            .primary_title("mismatched types")
            .id("E0308")
            .element(
                Snippet::source(source)
                    .path("$DIR/highlighting.rs")
                    .annotation(
                        AnnotationKind::Primary
                            .span(553..563)
                            .label("one type is more general than the other"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(547..552)
                            .label("arguments to this function are incorrect"),
                    ),
            )
            .element(Level::NOTE.message(&message)),
        Level::NOTE
            .secondary_title("function defined here")
            .element(
                Snippet::source(source)
                    .path("$DIR/highlighting.rs")
                    .annotation(AnnotationKind::Context.span(200..333).label(""))
                    .annotation(AnnotationKind::Primary.span(194..199)),
            ),
    ];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
```
<svg width="768px" height="362px" xmlns="http://www.w3.org/2000/svg">
  <style>
    .fg { fill: #AAAAAA }
    .bg { fill: #000000 }
    .fg-bright-blue { fill: #5555FF }
    .fg-bright-green { fill: #55FF55 }
    .fg-bright-red { fill: #FF5555 }
    .fg-magenta { fill: #AA00AA }
    .container {
      padding: 0 10px;
      line-height: 18px;
    }
    .bold { font-weight: bold; }
    tspan {
      font: 14px SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
      white-space: pre;
      line-height: 18px;
    }
  </style>

  <rect width="100%" height="100%" y="0" rx="4.5" class="bg" />

  <text xml:space="preserve" class="container fg">
    <tspan x="10px" y="28px"><tspan class="fg-bright-red bold">error[E0308]</tspan><tspan class="bold">: mismatched types</tspan>
</tspan>
    <tspan x="10px" y="46px"><tspan>  </tspan><tspan class="fg-bright-blue bold"> ╭▸ </tspan><tspan>$DIR/highlighting.rs:21:11</tspan>
</tspan>
    <tspan x="10px" y="64px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="82px"><tspan class="fg-bright-blue bold">21</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     query(wrapped_fn);</tspan>
</tspan>
    <tspan x="10px" y="100px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     </tspan><tspan class="fg-bright-blue bold">┬────</tspan><tspan> </tspan><tspan class="fg-bright-red bold">━━━━━━━━━━</tspan><tspan> </tspan><tspan class="fg-bright-red bold">one type is more general than the other</tspan>
</tspan>
    <tspan x="10px" y="118px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="136px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     </tspan><tspan class="fg-bright-blue bold">arguments to this function are incorrect</tspan>
</tspan>
    <tspan x="10px" y="154px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="172px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰ </tspan><tspan class="bold">note</tspan><tspan>: expected fn pointer `</tspan><tspan class="fg-magenta bold">for&lt;'a&gt;</tspan><tspan> fn(Box&lt;</tspan><tspan class="fg-magenta bold">(dyn Any + Send + 'a)</tspan><tspan>&gt;) -&gt; Pin&lt;_&gt;`</tspan>
</tspan>
    <tspan x="10px" y="190px"><tspan>      found fn item `fn(Box&lt;</tspan><tspan class="fg-magenta bold">(dyn Any + Send + 'static)</tspan><tspan>&gt;) -&gt; Pin&lt;_&gt; </tspan><tspan class="fg-magenta bold">{wrapped_fn}</tspan><tspan>`</tspan>
</tspan>
    <tspan x="10px" y="208px"><tspan class="fg-bright-green bold">note</tspan><tspan>: function defined here</tspan>
</tspan>
    <tspan x="10px" y="226px"><tspan>  </tspan><tspan class="fg-bright-blue bold"> ╭▸ </tspan><tspan>$DIR/highlighting.rs:10:4</tspan>
</tspan>
    <tspan x="10px" y="244px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="262px"><tspan class="fg-bright-blue bold">10</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>   fn query(_: fn(Box&lt;(dyn Any + Send + '_)&gt;) -&gt; Pin&lt;Box&lt;(</tspan>
</tspan>
    <tspan x="10px" y="280px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">┌────</tspan><tspan class="fg-bright-green bold">━━━━━</tspan><tspan class="fg-bright-blue bold">─┘</tspan>
</tspan>
    <tspan x="10px" y="298px"><tspan class="fg-bright-blue bold">11</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     dyn Future&lt;Output = Result&lt;Box&lt;(dyn Any + 'static)&gt;, String&gt;&gt; + Send + 'static</tspan>
</tspan>
    <tspan x="10px" y="316px"><tspan class="fg-bright-blue bold">12</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan> )&gt;&gt;) {}</tspan>
</tspan>
    <tspan x="10px" y="334px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰╴└───┘</tspan>
</tspan>
    <tspan x="10px" y="352px">
</tspan>
  </text>

</svg>

---

## Message

`struct` · `annotate_snippets::snippet::Message`

Also reachable as `ruff_annotate_snippets::Message`

```rust
struct Message<'a>
```

**Derives**: Clone, Debug

A text [`Element`] in a [`Group`]

See [`Level::message`] to create this.

---

## OptionCow

`struct` · `annotate_snippets::snippet::OptionCow`

Also reachable as `ruff_annotate_snippets::OptionCow`

```rust
struct OptionCow<'a>
```

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: &'a String) -> Self
fn from(value: String) -> Self
fn from(value: &'a str) -> Self
fn from(value: Cow<'a, str>) -> Self
fn from(value: &'a Cow<'a, str>) -> Self
fn from(value: Option<T>) -> Self
```

---

## Origin

`struct` · `annotate_snippets::snippet::Origin`

Also reachable as `ruff_annotate_snippets::Origin`

```rust
struct Origin<'a>
```

**Implements**: `core::convert::From`

**Derives**: Clone, Debug

**Methods** (4)

```rust
fn cell_index(self, index: Option<usize>) -> Self
fn char_column(self, char_column: usize) -> Self
fn line(self, line: usize) -> Self
fn path(path: impl Into<Cow<'a, str>>) -> Self
```

**via `core::convert::From`**

```rust
fn from(origin: Cow<'a, str>) -> Self
```

A source location [`Element`] in a [`Group`]

If you have source available, see instead [`Snippet`]

# Example

```rust
# use annotate_snippets::{Group, Snippet, AnnotationKind, Level, Origin};
let report = &[
    Level::ERROR.primary_title("mismatched types").id("E0308")
        .element(
            Origin::path("$DIR/mismatched-types.rs")
        )
];
```

---

## Padding

`struct` · `annotate_snippets::snippet::Padding`

Also reachable as `ruff_annotate_snippets::Padding`

```rust
struct Padding
```

**Derives**: Clone, Debug

A whitespace [`Element`] in a [`Group`]

---

## Patch

`struct` · `annotate_snippets::snippet::Patch`

Also reachable as `ruff_annotate_snippets::Patch`

```rust
struct Patch<'a>
```

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn new(span: Range<usize>, replacement: impl Into<Cow<'a, str>>) -> Self
```

Suggested edit to the [`Snippet`]

See [`Snippet::patch`]

# Example

```rust
# #[allow(clippy::needless_doctest_main)]
use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet, renderer::DecorStyle};

fn main() {
    let source = r#"
#![allow(dead_code)]
struct U <T> {
    wtf: Option<Box<U<T>>>,
    x: T,
}
fn main() {
    U {
        wtf: Some(Box(U {
            wtf: None,
            x: (),
        })),
        x: ()
    };
    let _ = std::collections::HashMap(); 
    let _ = std::collections::HashMap {};
    let _ = Box {};
}
"#;

    let report = &[
        Level::ERROR
            .primary_title(
                "cannot construct `Box<_, _>` with struct literal syntax due to private fields",
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .annotation(AnnotationKind::Primary.span(295..298)),
            )
            .element(Level::NOTE.message("private fields `0` and `1` that were not provided")),
        Level::HELP
            .secondary_title(
                "you might have meant to use an associated function to build this type",
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new(_)")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_uninit()")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_zeroed()")),
            )
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(298..301, "::new_in(_, _)")),
            )
            .element(Level::NOTE.no_name().message("and 12 other candidates")),
        Level::HELP
            .secondary_title("consider using the `Default` trait")
            .element(
                Snippet::source(source)
                    .path("$DIR/multi-suggestion.rs")
                    .patch(Patch::new(295..295, "<"))
                    .patch(Patch::new(
                        298..301,
                        " as std::default::Default>::default()",
                    )),
            ),
    ];

    let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
    anstream::println!("{}", renderer.render(report));
}
```

<svg width="740px" height="524px" xmlns="http://www.w3.org/2000/svg">
  <style>
    .fg { fill: #AAAAAA }
    .bg { fill: #000000 }
    .fg-bright-blue { fill: #5555FF }
    .fg-bright-cyan { fill: #55FFFF }
    .fg-bright-green { fill: #55FF55 }
    .fg-bright-red { fill: #FF5555 }
    .container {
      padding: 0 10px;
      line-height: 18px;
    }
    .bold { font-weight: bold; }
    tspan {
      font: 14px SFMono-Regular, Consolas, Liberation Mono, Menlo, monospace;
      white-space: pre;
      line-height: 18px;
    }
  </style>

  <rect width="100%" height="100%" y="0" rx="4.5" class="bg" />

  <text xml:space="preserve" class="container fg">
    <tspan x="10px" y="28px"><tspan class="fg-bright-red bold">error</tspan><tspan class="bold">: cannot construct `Box&lt;_, _&gt;` with struct literal syntax due to private fields</tspan>
</tspan>
    <tspan x="10px" y="46px"><tspan>  </tspan><tspan class="fg-bright-blue bold"> ╭▸ </tspan><tspan>$DIR/multi-suggestion.rs:17:13</tspan>
</tspan>
    <tspan x="10px" y="64px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="82px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>     let _ = Box {};</tspan>
</tspan>
    <tspan x="10px" y="100px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan><tspan>             </tspan><tspan class="fg-bright-red bold">━━━</tspan>
</tspan>
    <tspan x="10px" y="118px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="136px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰ </tspan><tspan class="bold">note</tspan><tspan>: private fields `0` and `1` that were not provided</tspan>
</tspan>
    <tspan x="10px" y="154px"><tspan class="fg-bright-cyan bold">help</tspan><tspan>: you might have meant to use an associated function to build this type</tspan>
</tspan>
    <tspan x="10px" y="172px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╭╴</tspan>
</tspan>
    <tspan x="10px" y="190px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-red">- </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-red"> {}</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="208px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-green">+ </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-green">::new(_)</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="226px"><tspan>   </tspan><tspan class="fg-bright-blue bold">├╴</tspan>
</tspan>
    <tspan x="10px" y="244px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-red">- </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-red"> {}</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="262px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-green">+ </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-green">::new_uninit()</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="280px"><tspan>   </tspan><tspan class="fg-bright-blue bold">├╴</tspan>
</tspan>
    <tspan x="10px" y="298px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-red">- </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-red"> {}</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="316px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-green">+ </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-green">::new_zeroed()</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="334px"><tspan>   </tspan><tspan class="fg-bright-blue bold">├╴</tspan>
</tspan>
    <tspan x="10px" y="352px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-red">- </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-red"> {}</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="370px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-green">+ </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-green">::new_in(_, _)</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="388px"><tspan>   </tspan><tspan class="fg-bright-blue bold">│</tspan>
</tspan>
    <tspan x="10px" y="406px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰ </tspan><tspan>and 12 other candidates</tspan>
</tspan>
    <tspan x="10px" y="424px"><tspan class="fg-bright-cyan bold">help</tspan><tspan>: consider using the `Default` trait</tspan>
</tspan>
    <tspan x="10px" y="442px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╭╴</tspan>
</tspan>
    <tspan x="10px" y="460px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-red">- </tspan><tspan>    let _ = Box</tspan><tspan class="fg-bright-red"> {}</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="478px"><tspan class="fg-bright-blue bold">17</tspan><tspan> </tspan><tspan class="fg-bright-green">+ </tspan><tspan>    let _ = </tspan><tspan class="fg-bright-green">&lt;</tspan><tspan>Box</tspan><tspan class="fg-bright-green"> as std::default::Default&gt;::default()</tspan><tspan>;</tspan>
</tspan>
    <tspan x="10px" y="496px"><tspan>   </tspan><tspan class="fg-bright-blue bold">╰╴</tspan>
</tspan>
    <tspan x="10px" y="514px">
</tspan>
  </text>

</svg>

---

## Snippet

`struct` · `annotate_snippets::snippet::Snippet`

Also reachable as `ruff_annotate_snippets::Snippet`

```rust
struct Snippet<'a, T>
```

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn annotation(self, annotation: Annotation<'a>) -> Snippet<'a, Annotation<'a>>
fn annotations(self, annotation: impl IntoIterator<Item = Annotation<'a>>) -> Self
fn cell_index(self, index: Option<usize>) -> Self
fn fold(self, fold: bool) -> Self
fn line_start(self, line_start: usize) -> Self
fn patch(self, patch: Patch<'a>) -> Snippet<'a, Patch<'a>>
fn patches(self, patches: impl IntoIterator<Item = Patch<'a>>) -> Self
fn path(self, path: impl Into<OptionCow<'a>>) -> Self
fn source(source: impl Into<Cow<'a, str>>) -> Self
```

A source view [`Element`] in a [`Group`]

If you do not have [source][Snippet::source] available, see instead [`Origin`]

`Snippet`s come in the following styles (`T`):
- With [`Annotation`]s, see [`Snippet::annotation`]
- With [`Patch`]s, see [`Snippet::patch`]

---

## Title

`struct` · `annotate_snippets::snippet::Title`

Also reachable as `ruff_annotate_snippets::Title`

```rust
struct Title<'a>
```

**Derives**: Clone, Debug

**Methods** (5)

```rust
fn element(self, section: impl Into<Element<'a>>) -> Group<'a>
fn elements(self, sections: impl IntoIterator<Item = impl Into<Element<'a>>>) -> Group<'a>
fn id(self, id: impl Into<Cow<'a, str>>) -> Self
fn id_url(self, url: impl Into<Cow<'a, str>>) -> Self
fn is_fixable(self, yes: bool) -> Self
```

A title that introduces a [`Group`], describing the main point

To create a `Title`, see [`Level::primary_title`] or [`Level::secondary_title`].

# Example

```rust
# use annotate_snippets::*;
let report = &[
    Group::with_title(
        Level::ERROR.primary_title("mismatched types").id("E0308")
    ),
    Group::with_title(
        Level::HELP.secondary_title("function defined here")
    ),
];
```

---

## Report

`type_alias` · `annotate_snippets::snippet::Report`

Also reachable as `ruff_annotate_snippets::Report`

```rust
type Report<'a> = &'a [Group<'a>]
```

A [diagnostic message][Title] and any associated [context][Element] to help users
understand it

The first [`Group`] is the ["primary" group][Level::primary_title], ie it contains the diagnostic
message.

All subsequent [`Group`]s are for distinct pieces of [context][Level::secondary_title].
The primary group will be visually distinguished to help tell them apart.

---
