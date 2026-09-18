//! Supported consumer environments. Producer-specific rustdoc identities remain separate.
use crate::{identity::Ecosystem, native_union::Rule};
crate::native_struct! { pub struct Target {
    ecosystem: Ecosystem => Rule::Text,
    toolchain: String => Rule::NonEmpty,
    target: String => Rule::NonEmpty,
    toolchain_aliases: Vec<String> => Rule::Sequence,
    target_aliases: Vec<String> => Rule::Sequence,
} }
impl Target {
    pub fn consumer(ecosystem: Ecosystem, image: &str) -> Self {
        let (toolchain, target, toolchain_aliases, target_aliases) = match ecosystem {
            Ecosystem::Python => (
                format!("python-{};{image}", super::producer::PYTHON_VERSION),
                "linux-x86_64",
                vec![
                    format!(
                        "python-{}",
                        super::producer::PYTHON_VERSION
                            .rsplit_once('.')
                            .expect("declared Python micro version")
                            .0
                    ),
                    format!("python-{}", super::producer::PYTHON_VERSION),
                ],
                vec!["linux".into(), "x86_64-manylinux_2_40".into()],
            ),
            Ecosystem::Rust => (
                format!("rust-{};{image}", super::producer::STABLE),
                super::producer::RUST_TARGET,
                vec![
                    super::producer::STABLE.into(),
                    format!("rust-{}", super::producer::STABLE),
                ],
                vec![],
            ),
        };
        Self {
            ecosystem,
            toolchain,
            target: target.into(),
            toolchain_aliases,
            target_aliases,
        }
    }
}
