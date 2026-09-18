//! One declared process environment and resource contract; no inherited target environment.
use crate::config::{Execution, ExecutionResources};
use std::{collections::BTreeMap, io};

crate::native_vocabulary! {
pub enum Network {
    Offline = "offline",
    Registry = "registry",
}
}

crate::native_struct! {
pub struct Launch {
    image: String => crate::native_union::Rule::NonEmpty,
    containment: String => crate::native_union::Rule::NonEmpty,
    environment: BTreeMap<String, String> => crate::native_union::Rule::Map,
    network: Network => crate::native_union::Rule::Text,
    resources: ExecutionResources => crate::native_union::Rule::Text,
    output_bytes: usize => crate::native_union::Rule::Text,
    deadline_millis: u64 => crate::native_union::Rule::Text,
}
}

impl Launch {
    /// Selected by a Rust producer, independently reconstructed by native effect admission.
    pub fn for_execution(
        execution: &Execution,
        image: &str,
        containment: &str,
        acquisition: bool,
    ) -> io::Result<Self> {
        Ok(Self {
            image: image.into(),
            containment: containment.into(),
            environment: [
                (
                    "PATH",
                    "/opt/producers/bin:/usr/local/cargo/bin:/usr/local/bin:/usr/bin:/bin",
                ),
                ("LANG", "C.UTF-8"),
                ("HOME", "/capsule/.executor/home"),
                ("TMPDIR", "/capsule/.executor/tmp"),
                ("TMP", "/capsule/.executor/tmp"),
                ("TEMP", "/capsule/.executor/tmp"),
                ("XDG_CONFIG_HOME", "/opt/libenr-empty-config"),
                ("CARGO_HOME", "/capsule/cargo-home"),
                ("CARGO_TARGET_DIR", "/capsule/target"),
                ("RUSTUP_HOME", "/usr/local/rustup"),
                ("RUSTUP_TOOLCHAIN", "1.98.1"),
                ("PYTHONNOUSERSITE", "1"),
                ("PYTHONDONTWRITEBYTECODE", "1"),
                ("UV_NO_CONFIG", "1"),
                ("UV_PYTHON_DOWNLOADS", "never"),
                ("UV_CACHE_DIR", "/capsule/.executor/uv"),
            ]
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect(),
            network: if acquisition {
                Network::Registry
            } else {
                Network::Offline
            },
            resources: execution.resources()?,
            output_bytes: execution.output_bytes.clamp(1024, 1_048_576),
            deadline_millis: execution.deadline_seconds.clamp(1, 600) * 1000,
        })
    }

    pub fn validate(&self) -> io::Result<()> {
        let resources = &self.resources;
        if !self.image.strip_prefix("sha256:").is_some_and(|digest| {
            digest.len() == 64
                && digest
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        }) || self.containment.is_empty()
            || self.containment.len() > 4096
            || self.environment.is_empty()
            || self.environment.len() > 128
            || self.environment.iter().any(|(key, value)| {
                key.is_empty()
                    || key.len() > 256
                    || key.contains(['=', '\0'])
                    || value.len() > 65536
                    || value.contains('\0')
            })
            || resources.cpu_period_micros != 100_000
            || !(100_000..=409_600_000).contains(&resources.cpu_quota_micros)
            || !resources
                .cpu_quota_micros
                .is_multiple_of(resources.cpu_period_micros)
            || !(128 * 1024 * 1024..=1_048_576 * 1024 * 1024).contains(&resources.memory_bytes)
            || resources.swap_bytes != 0
            || !(16 * 1024 * 1024..=super::DATA_LIMIT).contains(&resources.scratch_bytes)
            || resources.scratch_bytes > resources.memory_bytes
            || !(16..=1_048_576).contains(&resources.pids)
            || !(1024..=1_048_576).contains(&self.output_bytes)
            || !(1..=600_000).contains(&self.deadline_millis)
        {
            return Err(io::Error::other("invalid exact process launch contract"));
        }
        Ok(())
    }
}
