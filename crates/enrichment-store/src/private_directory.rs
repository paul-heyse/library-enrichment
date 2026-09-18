//! Durable ownership of finite private inputs. Native control selects recovery work;
//! the driver only removes a declared directory after its physical readers have exited.
use crate::{
    retention::{CleanupObligation, Dependency, RetentionStore},
    runtime::QueryRuntime,
};
use datafusion::error::{DataFusionError, Result};
pub(crate) use enrichment_core::operation::retention::PrivateDirectoryKind as Kind;
use enrichment_core::{
    identity::PrivateDirectoryId,
    operation::retention::{DirectoryParent, PrivateDirectoryRef},
};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

pub struct PrivateDirectory {
    path: PathBuf,
    obligation: CleanupObligation,
    retention: RetentionStore,
    runtime: QueryRuntime,
    created: AtomicBool,
    release_slot: Option<crate::retention_tasks::ReleaseSlot>,
    reference: PrivateDirectoryRef,
    // Export is a separate CLI process. This coordinates whole-root operator removal;
    // exact table/version read authorization still comes from native retention plans.
    root_fence: Option<Arc<std::fs::File>>,
}
impl std::fmt::Debug for PrivateDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrivateDirectory")
            .field("path", &self.path)
            .finish_non_exhaustive()
    }
}
impl PrivateDirectory {
    pub(crate) async fn create(
        retention: &RetentionStore,
        runtime: &QueryRuntime,
        kind: Kind,
    ) -> Result<Arc<Self>> {
        Self::create_at(
            retention,
            runtime,
            PrivateDirectoryRef::Local {
                id: PrivateDirectoryId::new(),
                purpose: kind,
            },
        )
        .await
    }

    pub(crate) async fn export(
        retention: &RetentionStore,
        runtime: &QueryRuntime,
        parent: PathBuf,
    ) -> Result<Arc<Self>> {
        let external = runtime.blocking(move || observe_parent(&parent)).await??;
        Self::create_at(
            retention,
            runtime,
            PrivateDirectoryRef::Export {
                id: PrivateDirectoryId::new(),
                parent: external,
            },
        )
        .await
    }

    async fn create_at(
        retention: &RetentionStore,
        runtime: &QueryRuntime,
        reference: PrivateDirectoryRef,
    ) -> Result<Arc<Self>> {
        let release_slot = runtime.reserve_release()?;
        let root_fence = if matches!(reference, PrivateDirectoryRef::Export { .. }) {
            Some(crate::leases::shared(
                retention
                    .namespace()
                    .parent()
                    .ok_or_else(|| invalid("export primary root"))?,
            )?)
        } else {
            None
        };
        let path = directory_path(retention, &reference)?;
        let dependencies = vec![Dependency::PrivateDirectory {
            value: reference.clone(),
        }];
        let obligation = retention
            .create_obligation(format!("directory/{}", reference.id()), dependencies)
            .await?;
        // Install the drop owner before creating bytes, including failure paths.
        let owner = Arc::new(Self {
            path,
            obligation,
            retention: retention.clone(),
            runtime: runtime.clone(),
            created: AtomicBool::new(false),
            release_slot: Some(release_slot),
            reference,
            root_fence,
        });
        let create = owner.clone();
        runtime
            .blocking(move || {
                if directory_path(&create.retention, &create.reference)
                    .map_err(std::io::Error::other)?
                    != create.path
                {
                    return Err(std::io::Error::other("directory location changed"));
                }
                std::fs::create_dir_all(
                    create
                        .path
                        .parent()
                        .ok_or_else(|| std::io::Error::other("input parent"))?,
                )?;
                std::fs::create_dir(&create.path)?;
                create.created.store(true, Ordering::Release);
                std::fs::File::open(create.path.parent().unwrap())?.sync_all()
            })
            .await?
            .map_err(io_error)?;
        Ok(owner)
    }
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Verify the physical path while retaining its admitted semantic purpose.
    pub(crate) fn directory(&self, kind: Kind, path: &Path) -> std::io::Result<()> {
        if !matches!(&self.reference, PrivateDirectoryRef::Local { purpose, .. } if *purpose == kind)
        {
            return Err(std::io::Error::other(
                "private directory has the wrong role",
            ));
        }
        self.check_directory(path)
    }
    pub(crate) fn export_directory(&self, path: &Path) -> std::io::Result<()> {
        if !matches!(self.reference, PrivateDirectoryRef::Export { .. }) {
            return Err(std::io::Error::other("directory is not an export stage"));
        }
        self.check_directory(path)
    }
    fn check_directory(&self, path: &Path) -> std::io::Result<()> {
        if directory_path(&self.retention, &self.reference).map_err(std::io::Error::other)?
            != self.path
        {
            return Err(std::io::Error::other("directory location changed"));
        }
        let relative = path
            .strip_prefix(&self.path)
            .map_err(std::io::Error::other)?;
        let mut current = self.path.clone();
        if !std::fs::symlink_metadata(&current)?.is_dir() {
            return Err(std::io::Error::other("private directory was replaced"));
        }
        for component in relative.components() {
            if !matches!(component, std::path::Component::Normal(_)) {
                return Err(std::io::Error::other(
                    "private input has a noncanonical component",
                ));
            }
            current.push(component);
            if !std::fs::symlink_metadata(&current)?.is_dir() {
                return Err(std::io::Error::other(
                    "private input traverses a link or file",
                ));
            }
        }
        Ok(())
    }
    /// A child retains its own process witness through parent exit or a lost acknowledgement.
    pub async fn protect_child(&self, pid: u32) -> Result<()> {
        let process = self
            .runtime
            .blocking(move || crate::native_process::child(pid))
            .await??;
        self.retention
            .create_process_obligation(
                self.obligation.label.clone(),
                self.obligation.dependencies.clone(),
                process,
            )
            .await?;
        Ok(())
    }
}
impl Drop for PrivateDirectory {
    fn drop(&mut self) {
        let Some(slot) = self.release_slot.take() else {
            return;
        };
        let (path, obligation, retention, runtime) = (
            self.path.clone(),
            self.obligation.clone(),
            self.retention.clone(),
            self.runtime.clone(),
        );
        let created = self.created.load(Ordering::Acquire);
        let reference = self.reference.clone();
        let root_fence = self.root_fence.take();
        if let Err(error) = self.runtime.release_retention(slot, async move {
            let _root_fence = root_fence;
            retention
                .release_obligation(&obligation, obligation.dependencies.clone())
                .await?;
            retention.reconcile_processes().await?;
            let selected = retention
                .private_cleanup_candidates(Some(&reference))
                .await?;
            if !selected.is_empty() {
                if created {
                    remove(&runtime, path, reference).await?;
                }
                for row in selected {
                    retention.settle_removed(&row).await?;
                }
            }
            Ok(())
        }) {
            eprintln!("library-enrichment: durable input cleanup remains pending: {error}");
        }
    }
}

fn directory_path(retention: &RetentionStore, reference: &PrivateDirectoryRef) -> Result<PathBuf> {
    match reference {
        PrivateDirectoryRef::Local { id, purpose } => Ok(retention
            .namespace()
            .parent()
            .ok_or_else(|| invalid("input root"))?
            .join("private")
            .join(purpose.as_str())
            .join(id.component())),
        PrivateDirectoryRef::Export { id, parent } => external_path(parent, *id).map_err(io_error),
    }
}

fn observe_parent(parent: &Path) -> std::io::Result<DirectoryParent> {
    use std::os::unix::fs::MetadataExt;
    let parent = parent.canonicalize()?;
    let metadata = std::fs::symlink_metadata(&parent)?;
    let parent = parent
        .to_str()
        .filter(|p| p.len() <= 4096)
        .ok_or_else(|| std::io::Error::other("export parent is not bounded UTF-8"))?;
    if !metadata.is_dir() {
        return Err(std::io::Error::other("export parent is not a directory"));
    }
    Ok(DirectoryParent {
        parent: parent.into(),
        parent_device: metadata.dev(),
        parent_inode: metadata.ino(),
    })
}

fn external_path(location: &DirectoryParent, id: PrivateDirectoryId) -> std::io::Result<PathBuf> {
    use std::os::unix::fs::MetadataExt;
    let parent = Path::new(&location.parent);
    if !parent.is_absolute()
        || location.parent.len() > 4096
        || parent.components().any(|c| {
            !matches!(
                c,
                std::path::Component::RootDir | std::path::Component::Normal(_)
            )
        })
    {
        return Err(std::io::Error::other(
            "export staging path is not canonical",
        ));
    }
    for ancestor in parent.ancestors() {
        if !std::fs::symlink_metadata(ancestor)?.is_dir() {
            return Err(std::io::Error::other(
                "export parent traverses a link or file",
            ));
        }
    }
    let actual = std::fs::symlink_metadata(parent)?;
    if (actual.dev(), actual.ino()) != (location.parent_device, location.parent_inode) {
        return Err(std::io::Error::other("export parent identity changed"));
    }
    Ok(parent.join(format!(".evidence-bundle-{}", id.component())))
}

fn single_file_inventory(path: &Path, filename: &str) -> std::io::Result<()> {
    declared_files(path, &[filename.to_owned()])
}

fn declared_files(path: &Path, filenames: &[String]) -> std::io::Result<()> {
    use std::os::unix::fs::MetadataExt;
    let directory = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error),
    };
    if !directory.is_dir() {
        return Err(std::io::Error::other("input root replaced"));
    }
    let mut files = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if !filenames
            .iter()
            .any(|name| entry.file_name() == name.as_str())
            || !entry.file_type()?.is_file()
            || metadata.nlink() != 1
            || files.len() >= filenames.len()
        {
            return Err(std::io::Error::other(
                "input directory contains undeclared bytes",
            ));
        }
        files.push(entry.path());
    }
    Ok(())
}

async fn remove(
    runtime: &QueryRuntime,
    path: PathBuf,
    reference: PrivateDirectoryRef,
) -> Result<()> {
    let inventory = runtime
        .blocking(move || {
            if let PrivateDirectoryRef::Export { id, parent } = &reference
                && external_path(parent, *id)? != path
            {
                return Err(std::io::Error::other("export staging path changed"));
            }
            let maximum = match reference {
                PrivateDirectoryRef::Local {
                    purpose: Kind::Documents,
                    ..
                } => {
                    single_file_inventory(&path, "documents.arrow")?;
                    2
                }
                PrivateDirectoryRef::Local {
                    purpose: Kind::Worker,
                    ..
                } => {
                    single_file_inventory(&path, "worker.arrow")?;
                    2
                }
                PrivateDirectoryRef::Local {
                    purpose: Kind::Rustdoc,
                    ..
                } => {
                    let mut names = enrichment_core::producer::rustdoc::facts::Fact::ALL
                        .iter()
                        .map(|fact| format!("{}.arrow", fact.name()))
                        .collect::<Vec<_>>();
                    names.push(crate::native_rustdoc::INPUT_FILE.into());
                    declared_files(&path, &names)?;
                    names.len() + 1
                }
                // Archive extraction counts implicit directories; content and owner roots add two.
                PrivateDirectoryRef::Local {
                    purpose: Kind::Source,
                    ..
                } => enrichment_core::policy::ArchivePolicy::default().max_entries + 2,
                PrivateDirectoryRef::Export { .. } => crate::bundle::MAX_STAGING_ENTRIES,
            };
            crate::owned_tree::Inventory::read_bounded(&path, maximum)
        })
        .await??;
    let store = runtime
        .session()
        .runtime_env()
        .object_store(datafusion::execution::object_store::ObjectStoreUrl::local_filesystem())?;
    runtime
        .native_write(async move { inventory.remove(store).await })
        .await
}

/// The caller has reconciled process identity and holds the service writer lock.
pub(crate) async fn recover(retention: &RetentionStore, runtime: &QueryRuntime) -> Result<()> {
    for obligation in retention.private_cleanup_candidates(None).await? {
        let reference = directory_reference(&obligation)?.clone();
        let path = directory_path(retention, &reference)?;
        remove(runtime, path, reference).await?;
        retention.settle_removed(&obligation).await?;
    }
    Ok(())
}

fn directory_reference(obligation: &CleanupObligation) -> Result<&PrivateDirectoryRef> {
    match obligation.dependencies.as_slice() {
        [Dependency::PrivateDirectory { value }] => Ok(value),
        _ => Err(invalid(
            "directory obligation requires one exact directory dependency",
        )),
    }
}

// The directory's ID groups owners; full scope equality prevents another purpose/location
// using the same bytes from escaping that group. No text label participates in selection.
fn directory_rows(source: &str) -> String {
    format!("SELECT o.*, dependency.private_directory.value AS directory,
        native_coalesce(dependency.private_directory.value.local.id,dependency.private_directory.value.export.id) AS directory_id
        FROM (SELECT *,unnest(dependencies) AS dependency FROM {source}) o
        WHERE dependency.kind='private_directory'")
}

pub(crate) async fn cleanup_plan(
    session: &datafusion::prelude::SessionContext,
    obligations: datafusion::dataframe::DataFrame,
) -> Result<datafusion::dataframe::DataFrame> {
    use datafusion::prelude::col;
    use enrichment_core::native_union::NativeStruct;
    crate::native_catalog::work(session, "private_obligations", obligations.into_view())?;
    let selected = session.sql(&format!("WITH directories AS ({}) SELECT o.* FROM directories o
        LEFT ANTI JOIN directories live ON live.directory_id=o.directory_id
            AND (NOT live.physical_released OR (live.directory IS DISTINCT FROM o.directory) OR array_length(live.dependencies)<>1)
        WHERE o.physical_released AND NOT o.settled AND array_length(o.dependencies)=1", directory_rows("private_obligations"))).await?;
    selected.select(
        CleanupObligation::fields()
            .iter()
            .map(|field| col(field.name()))
            .collect::<Vec<_>>(),
    )
}

pub(crate) async fn admission_rules(
    invariants: &mut crate::invariants::Invariants,
    session: &datafusion::prelude::SessionContext,
) -> Result<()> {
    crate::native_catalog::work(
        session,
        "private_obligations",
        session
            .table("state.records.cleanup_obligations")
            .await?
            .into_view(),
    )?;
    let directories = session.sql(&directory_rows("private_obligations")).await?;
    crate::native_catalog::work(session, "directory_owners", directories.into_view())?;
    invariants.push(
        session
            .sql("SELECT obligation_id FROM directory_owners WHERE array_length(dependencies)<>1")
            .await?,
        "directory_dependency_cardinality",
        "retention",
    )?;
    invariants.push(session.sql("SELECT a.obligation_id FROM directory_owners a JOIN directory_owners b ON a.directory_id=b.directory_id WHERE a.directory IS DISTINCT FROM b.directory").await?, "directory_immutable_scope", "retention")?;
    let history = session
        .sql(&directory_rows("state.history.cleanup_obligations"))
        .await?;
    crate::native_catalog::work(session, "directory_history", history.into_view())?;
    invariants.push(session.sql("SELECT c.obligation_id FROM state.records.cleanup_obligations c JOIN directory_history h ON c.obligation_id=h.obligation_id WHERE c.dependencies<>h.dependencies").await?, "directory_obligation_immutable_scope", "retention")?;
    invariants.push(session.sql("SELECT a.obligation_id FROM directory_owners a JOIN directory_history h ON a.directory_id=h.directory_id WHERE ((a.directory IS DISTINCT FROM h.directory) OR (h.obligation_id=a.obligation_id AND h.settled AND NOT a.settled))").await?, "directory_no_resurrection", "retention")?;
    invariants.push(session.sql("WITH ranked AS (SELECT *,row_number() OVER (PARTITION BY obligation_id ORDER BY sequence DESC) AS rank FROM directory_history), retired AS (SELECT directory_id FROM ranked WHERE rank=1 GROUP BY directory_id HAVING bool_and(settled)) SELECT a.obligation_id FROM directory_owners a JOIN retired r ON a.directory_id=r.directory_id LEFT ANTI JOIN directory_history h ON a.obligation_id=h.obligation_id").await?, "directory_retired_identity", "retention")?;
    Ok(())
}
fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
fn io_error(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn plan19_export_cleanup_requires_matching_durable_location_and_exit() -> Result<()> {
        use enrichment_core::native_union::NativeStruct;
        let scratch = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(scratch.path(), Default::default())?;
        let directory = PrivateDirectoryRef::Export {
            id: PrivateDirectoryId::new(),
            parent: DirectoryParent {
                parent: "/owned/export".into(),
                parent_device: 7,
                parent_inode: 11,
            },
        };
        let ready = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::new(),
            label: "arbitrary diagnostic label".into(),
            process: crate::native_process::current()?,
            dependencies: vec![Dependency::PrivateDirectory { value: directory }],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let mut live = ready.clone();
        live.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        live.physical_released = false;
        live.label = "a different label cannot hide a live owner".into();
        let mut wrong = ready.clone();
        wrong.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        if let Dependency::PrivateDirectory {
            value: PrivateDirectoryRef::Export { parent, .. },
        } = &mut wrong.dependencies[0]
        {
            parent.parent_inode = 12;
        }
        let mut misplaced = ready.clone();
        misplaced
            .dependencies
            .push(misplaced.dependencies[0].clone());
        for (rows, expected) in [
            (vec![ready.clone()], 1),
            (vec![ready.clone(), live], 0),
            (vec![ready.clone(), wrong], 0),
            (vec![misplaced], 0),
        ] {
            let session = runtime.session();
            let selected = cleanup_plan(
                &session,
                crate::native_catalog::batch(
                    &session,
                    "private_directory",
                    CleanupObligation::batch(&rows)?,
                )?,
            )
            .await?;
            let selected = runtime.records::<CleanupObligation>(selected, 8).await?;
            assert_eq!(selected.len(), expected);
        }
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn typed_directory_admission_preserves_scope_and_partial_settlement() -> Result<()> {
        use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
        use enrichment_core::native_union::NativeStruct;
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let id = PrivateDirectoryId::new();
        let reference = PrivateDirectoryRef::Local {
            id,
            purpose: Kind::Source,
        };
        let first = CleanupObligation {
            obligation_id: enrichment_core::identity::CleanupObligationId::new(),
            label: "first".into(),
            process: crate::native_process::current()?,
            dependencies: vec![Dependency::PrivateDirectory {
                value: reference.clone(),
            }],
            physical_released: true,
            settled: false,
            sequence: 1,
        };
        let check =
            async |current: &[CleanupObligation], history: &[CleanupObligation]| -> Result<()> {
                let tables = |rows: &[CleanupObligation]| -> Result<Tables> {
                    Ok(Tables::from([(
                        "cleanup_obligations".into(),
                        crate::native_catalog::batch(
                            &runtime.session(),
                            "directory",
                            CleanupObligation::batch(rows)?,
                        )?
                        .into_view(),
                    )]))
                };
                let catalog = BoundCatalog::default()
                    .with_schema(BindingKind::FoldedRecords, tables(current)?)
                    .with_schema(BindingKind::ValidatedHistory, tables(history)?);
                let session = runtime.bound_session(std::collections::BTreeMap::from([(
                    "state".into(),
                    Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
                )]))?;
                let mut invariants = crate::invariants::Invariants::default();
                admission_rules(&mut invariants, &session).await?;
                runtime.admit(invariants).await
            };
        check(std::slice::from_ref(&first), &[]).await?;
        let mut changed = first.clone();
        changed.dependencies = vec![Dependency::PrivateDirectory {
            value: PrivateDirectoryRef::Local {
                id,
                purpose: Kind::Worker,
            },
        }];
        assert!(
            check(std::slice::from_ref(&changed), std::slice::from_ref(&first))
                .await
                .is_err()
        );
        changed.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        assert!(check(&[first.clone(), changed], &[]).await.is_err());
        // An existing directory owner cannot be retargeted or disguised as another resource.
        for dependency in [
            Dependency::PrivateDirectory {
                value: PrivateDirectoryRef::Local {
                    id: PrivateDirectoryId::new(),
                    purpose: Kind::Source,
                },
            },
            Dependency::TableScope {
                table_uri: "other".into(),
            },
        ] {
            let mut rebound = first.clone();
            rebound.dependencies = vec![dependency];
            assert!(
                check(&[rebound], std::slice::from_ref(&first))
                    .await
                    .is_err()
            );
        }
        let mut duplicate = first.clone();
        duplicate
            .dependencies
            .push(duplicate.dependencies[0].clone());
        assert!(check(&[duplicate], &[]).await.is_err());
        let mut second = first.clone();
        second.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        second.label = "different label".into();
        let mut third = second.clone();
        third.obligation_id = enrichment_core::identity::CleanupObligationId::new();
        let mut retired = first.clone();
        retired.sequence = 2;
        retired.settled = true;
        // Settling one of three owners must not block the remaining acknowledgements.
        check(
            &[retired.clone(), second.clone(), third.clone()],
            &[first.clone(), second.clone(), third.clone()],
        )
        .await?;
        let mut second_done = second.clone();
        second_done.sequence = 3;
        second_done.settled = true;
        check(
            &[retired.clone(), second_done, third.clone()],
            &[first.clone(), retired.clone(), second.clone(), third],
        )
        .await?;
        assert!(
            check(std::slice::from_ref(&first), std::slice::from_ref(&retired))
                .await
                .is_err()
        );
        // A fully removed directory cannot acquire another physical owner.
        assert!(
            check(&[retired.clone(), second], &[first, retired])
                .await
                .is_err()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[test]
    fn plan19_export_directory_refuses_parent_replacement_and_escape() -> std::io::Result<()> {
        let root = tempfile::tempdir()?;
        let parent = root.path().join("destination");
        std::fs::create_dir(&parent)?;
        let location = observe_parent(&parent)?;
        let id = PrivateDirectoryId::new();
        assert_eq!(
            external_path(&location, id)?,
            parent.join(format!(".evidence-bundle-{}", id.component()))
        );
        assert!(PrivateDirectoryId::from_component("../unrelated").is_err());
        std::fs::rename(&parent, root.path().join("previous"))?;
        std::fs::create_dir(&parent)?;
        assert!(external_path(&location, id).is_err());
        std::fs::remove_dir(&parent)?;
        std::os::unix::fs::symlink(root.path().join("previous"), &parent)?;
        assert!(external_path(&location, id).is_err());
        Ok(())
    }
    #[test]
    fn plan19_input_removal_refuses_unknown_files_and_links() -> std::io::Result<()> {
        let root = tempfile::tempdir()?;
        let input = root.path().join("owned");
        std::fs::create_dir(&input)?;
        std::fs::write(input.join("documents.arrow"), b"bounded input")?;
        std::fs::write(input.join("unclaimed"), b"preserve")?;
        assert!(single_file_inventory(&input, "documents.arrow").is_err());
        assert!(input.join("documents.arrow").exists());
        std::fs::remove_file(input.join("unclaimed"))?;
        let other = root.path().join("other");
        std::fs::hard_link(input.join("documents.arrow"), &other)?;
        assert!(single_file_inventory(&input, "documents.arrow").is_err());
        std::fs::remove_file(other)?;
        single_file_inventory(&input, "documents.arrow")?;
        Ok(())
    }
}
