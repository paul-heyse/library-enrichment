//! Bounded observation of declared inputs inside a link-free extracted revision.
//! This records candidate inputs across targets/features; it does not resolve a Cargo build.
use super::RevisionInputs;
use crate::{archive::Extracted, identity::Ecosystem, producer::source};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path, PathBuf},
};

const MANIFEST_LIMIT: u64 = 1024 * 1024;
const TOTAL_LIMIT: usize = 16 * 1024 * 1024;
const PACKAGE_LIMIT: usize = 512;

struct Collector<'a> {
    extracted: &'a Extracted,
    required: BTreeSet<PathBuf>,
    roots: BTreeSet<PathBuf>,
    manifests: BTreeMap<PathBuf, toml::Value>,
    bytes: usize,
}

pub(super) fn collect(
    archive_sha256: String,
    extracted: &Extracted,
    subdir: &str,
    ecosystem: Ecosystem,
    manifest: &str,
) -> Result<RevisionInputs, String> {
    let wrapper = extracted
        .top_level
        .as_deref()
        .ok_or("archive wrapper missing")?;
    let package = confined(Path::new(wrapper), subdir)?;
    if manifest.len() > MANIFEST_LIMIT as usize {
        return Err("revision manifest exceeds input collection byte limit".into());
    }
    let mut collector = Collector {
        extracted,
        required: BTreeSet::new(),
        roots: BTreeSet::from([package.clone()]),
        manifests: BTreeMap::new(),
        bytes: manifest.len(),
    };
    let filename = match ecosystem {
        Ecosystem::Rust => "Cargo.toml",
        Ecosystem::Python => "pyproject.toml",
    };
    collector.required.insert(package.join(filename));
    let parsed = toml::from_str(manifest).map_err(|e| e.to_string())?;
    if ecosystem == Ecosystem::Rust {
        collector.manifests.insert(package.join(filename), parsed);
        let mut pending = vec![package.clone()];
        let mut visited = BTreeSet::new();
        while let Some(root) = pending.pop() {
            if !visited.insert(root.clone()) {
                continue;
            }
            if visited.len() > PACKAGE_LIMIT {
                return Err(
                    "revision declared package count exceeds input collection limit".into(),
                );
            }
            collector.roots.insert(root.clone());
            collector.ancestors(&root);
            let Some(value) = collector.manifest(&root.join("Cargo.toml"))? else {
                continue;
            };
            let workspace = collector.workspace(&root, &value)?;
            collector.targets(&root, &value)?;
            collector.package_files(&root, &value, workspace.as_ref())?;
            collector.dependencies(&root, &value, workspace.as_ref(), &mut pending)?;
        }
    } else {
        collector.python_files(&package, &parsed)?;
    }
    let missing_inputs = collector
        .required
        .iter()
        .filter(|path| !extracted.root.join(path).is_file())
        .map(|p| p.to_string_lossy().into_owned())
        .collect();
    Ok(RevisionInputs {
        archive_sha256,
        policy: crate::archive::REVISION_EXTRACTION_POLICY.into(),
        omissions: extracted.omissions.clone(),
        selected_package: package.to_string_lossy().into_owned(),
        declared_inputs: collector
            .required
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
        source_roots: collector
            .roots
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect(),
        missing_inputs,
    })
}

fn confined(base: &Path, text: &str) -> Result<PathBuf, String> {
    let mut value = base.to_owned();
    for component in Path::new(text).components() {
        match component {
            Component::Normal(part) => value.push(part),
            Component::CurDir => {}
            Component::ParentDir if value.components().count() > 1 => {
                value.pop();
            }
            _ => return Err("declared revision input escapes archive wrapper".into()),
        }
    }
    Ok(value)
}

impl Collector<'_> {
    fn present(&self, path: &Path) -> bool {
        self.extracted.root.join(path).is_file()
            || self
                .extracted
                .omissions
                .iter()
                .any(|o| Path::new(&o.path) == path)
    }

    fn ancestors(&mut self, root: &Path) {
        for ancestor in root.ancestors().filter(|p| !p.as_os_str().is_empty()) {
            for name in [
                "Cargo.toml",
                "Cargo.lock",
                ".cargo/config",
                ".cargo/config.toml",
            ] {
                let path = ancestor.join(name);
                if self.present(&path) {
                    self.required.insert(path);
                }
            }
        }
    }

    fn manifest(&mut self, path: &Path) -> Result<Option<toml::Value>, String> {
        self.required.insert(path.to_owned());
        if let Some(value) = self.manifests.get(path) {
            return Ok(Some(value.clone()));
        }
        if !self.extracted.root.join(path).is_file() {
            return Ok(None);
        }
        if self.manifests.len() >= PACKAGE_LIMIT {
            return Err("revision manifest count exceeds input collection limit".into());
        }
        let bytes = source::read_file_limited(&self.extracted.root.join(path), MANIFEST_LIMIT)
            .map_err(|e| e.to_string())?;
        self.bytes += bytes.len();
        if self.bytes > TOTAL_LIMIT {
            return Err("revision manifests exceed total input collection byte limit".into());
        }
        let text = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        let parsed: toml::Value = toml::from_str(text).map_err(|e| e.to_string())?;
        self.manifests.insert(path.to_owned(), parsed.clone());
        Ok(Some(parsed))
    }

    fn workspace(
        &mut self,
        root: &Path,
        value: &toml::Value,
    ) -> Result<Option<(PathBuf, toml::Value)>, String> {
        if value.get("workspace").is_some() {
            return Ok(Some((root.to_owned(), value.clone())));
        }
        if let Some(path) = value
            .get("package")
            .and_then(|p| p.get("workspace"))
            .and_then(toml::Value::as_str)
        {
            let root = confined(root, path)?;
            return Ok(self.manifest(&root.join("Cargo.toml"))?.map(|v| (root, v)));
        }
        for ancestor in root
            .ancestors()
            .skip(1)
            .filter(|p| !p.as_os_str().is_empty())
        {
            let path = ancestor.join("Cargo.toml");
            if self.present(&path)
                && let Some(value) = self.manifest(&path)?
                && value.get("workspace").is_some()
            {
                return Ok(Some((ancestor.to_owned(), value)));
            }
        }
        Ok(None)
    }

    fn targets(&mut self, root: &Path, value: &toml::Value) -> Result<(), String> {
        if let Some(path) = value
            .get("lib")
            .and_then(|v| v.get("path"))
            .and_then(toml::Value::as_str)
        {
            self.required.insert(confined(root, path)?);
        } else if self.present(&root.join("src/lib.rs")) {
            self.required.insert(root.join("src/lib.rs"));
        }
        for kind in ["bin", "example", "test", "bench"] {
            if let Some(targets) = value.get(kind).and_then(toml::Value::as_array) {
                for target in targets {
                    if let Some(path) = target.get("path").and_then(toml::Value::as_str) {
                        self.required.insert(confined(root, path)?);
                    }
                }
            }
        }
        let build = value.get("package").and_then(|p| p.get("build"));
        if let Some(path) = build.and_then(toml::Value::as_str) {
            self.required.insert(confined(root, path)?);
        } else if build.and_then(toml::Value::as_bool) != Some(false)
            && self.present(&root.join("build.rs"))
        {
            self.required.insert(root.join("build.rs"));
        }
        Ok(())
    }

    fn package_files(
        &mut self,
        root: &Path,
        value: &toml::Value,
        workspace: Option<&(PathBuf, toml::Value)>,
    ) -> Result<(), String> {
        for key in ["readme", "license-file"] {
            let field = value.get("package").and_then(|p| p.get(key));
            let (base, field) = if field
                .and_then(|f| f.get("workspace"))
                .and_then(toml::Value::as_bool)
                == Some(true)
            {
                let Some((base, manifest)) = workspace else {
                    continue;
                };
                (
                    base.as_path(),
                    manifest
                        .get("workspace")
                        .and_then(|w| w.get("package"))
                        .and_then(|p| p.get(key)),
                )
            } else {
                (root, field)
            };
            if let Some(path) = field.and_then(toml::Value::as_str) {
                self.required.insert(confined(base, path)?);
            }
        }
        Ok(())
    }

    fn dependencies(
        &mut self,
        root: &Path,
        value: &toml::Value,
        workspace: Option<&(PathBuf, toml::Value)>,
        pending: &mut Vec<PathBuf>,
    ) -> Result<(), String> {
        let tables = std::iter::once(value).chain(
            value
                .get("target")
                .and_then(toml::Value::as_table)
                .into_iter()
                .flat_map(|t| t.values()),
        );
        for table in tables {
            for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
                if let Some(dependencies) = table.get(kind).and_then(toml::Value::as_table) {
                    for (name, dependency) in dependencies {
                        let (base, dependency) =
                            if dependency.get("workspace").and_then(toml::Value::as_bool)
                                == Some(true)
                            {
                                let Some((base, manifest)) = workspace else {
                                    continue;
                                };
                                let Some(inherited) = manifest
                                    .get("workspace")
                                    .and_then(|w| w.get("dependencies"))
                                    .and_then(|d| d.get(name))
                                else {
                                    continue;
                                };
                                (base.as_path(), inherited)
                            } else {
                                (root, dependency)
                            };
                        if let Some(path) = dependency.get("path").and_then(toml::Value::as_str) {
                            let package = confined(base, path)?;
                            self.required.insert(package.join("Cargo.toml"));
                            if self.roots.insert(package.clone()) {
                                pending.push(package);
                            }
                            if self.roots.len() > PACKAGE_LIMIT {
                                return Err("revision declared package count exceeds input collection limit".into());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn python_files(&mut self, root: &Path, value: &toml::Value) -> Result<(), String> {
        for key in ["readme", "license"] {
            if let Some(field) = value.get("project").and_then(|p| p.get(key)) {
                let path = field
                    .get("file")
                    .and_then(toml::Value::as_str)
                    .or_else(|| (key == "readme").then(|| field.as_str()).flatten());
                if let Some(path) = path {
                    self.required.insert(confined(root, path)?);
                }
            }
        }
        Ok(())
    }
}
