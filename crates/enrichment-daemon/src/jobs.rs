//! Durable concrete producer jobs. One journal is owned by the daemon; callers own interests.
use enrichment_core::{
    clock,
    execution::{JobData, VerifyRequest},
    wire::{Envelope, JobState},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

const FORMAT: &str = "concrete-jobs/4";
const MAX_RECORD_BYTES: u64 = 2 * 1024 * 1024;
const MAX_TERMINAL_CACHE: usize = 8;
const MAX_INTERESTS: usize = 256;

#[derive(Deserialize)]
struct JournalHeader {
    format: String,
    job_id: String,
    state: JobState,
}

/// Closed durable operation inputs; no generic workflow or arbitrary command payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "operation",
    content = "request",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum JobSpec {
    Verify(VerifyRequest),
    Inspect(enrichment_core::request::InspectRequest),
    Resolve(enrichment_core::request::ResolveRequest),
    Compare(enrichment_core::request::CompareRequest),
}
/// Immutable inputs selected by an acquisition before its publication is admitted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionStage {
    pub release_id: String,
    pub environment_id: String,
    pub context_id: String,
    pub attempt_id: String,
    pub input_artifact_ids: Vec<String>,
    pub result_artifact_id: String,
}
impl ResolutionStage {
    fn validate(&self) -> io::Result<()> {
        if self.release_id.is_empty()
            || self.environment_id.is_empty()
            || self.context_id.is_empty()
            || self.attempt_id.is_empty()
            || self.input_artifact_ids.is_empty()
            || self.input_artifact_ids.len() > 8192
            || !self.input_artifact_ids.contains(&self.result_artifact_id)
            || self
                .input_artifact_ids
                .iter()
                .any(|id| !enrichment_core::evidence::is_artifact_id(id))
            || self.input_artifact_ids.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(io::Error::other(
                "invalid exact resolution publication stage",
            ));
        }
        Ok(())
    }
}
impl From<VerifyRequest> for JobSpec {
    fn from(r: VerifyRequest) -> Self {
        Self::Verify(r)
    }
}
impl JobSpec {
    fn validate(&self) -> io::Result<()> {
        let pinned = |context: &str, snapshot: &Option<String>| {
            if context.is_empty() || snapshot.as_ref().is_none_or(|s| s.is_empty()) {
                Err(io::Error::other(
                    "durable inspection/verification requires a pinned context and snapshot",
                ))
            } else {
                Ok(())
            }
        };
        match self {
            Self::Verify(r) => pinned(&r.context_id, &r.snapshot_id),
            Self::Inspect(r) => {
                pinned(&r.context_id, &r.snapshot_id)?;
                let options = r.execution.as_ref().ok_or_else(|| {
                    io::Error::other("durable inspection requires explicit execution intent")
                })?;
                if options.intent == enrichment_core::request::InspectionIntent::Retained {
                    return Err(io::Error::other("retained reads are not producer jobs"));
                }
                options.validate(32768).map_err(io::Error::other)
            }
            Self::Resolve(r) => r.validate().map_err(io::Error::other),
            Self::Compare(r) => r.resolutions().map(|_| ()).map_err(io::Error::other),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobRecord {
    pub format: String,
    pub job_id: String,
    pub key: String,
    pub specification: JobSpec,
    pub state: JobState,
    pub stage: String,
    pub interests: BTreeSet<String>,
    pub detached_interests: BTreeSet<String>,
    pub submitted_at: String,
    pub updated_at: String,
    pub transitions: Vec<JobState>,
    pub result: Option<Envelope>,
    pub resolution: Option<ResolutionStage>,
}
impl JobRecord {
    pub fn data(&self, token: Option<String>) -> JobData {
        JobData {
            job_id: self.job_id.clone(),
            state: self.state,
            stage: self.stage.clone(),
            interest_token: token,
            active_interests: self.interests.len(),
            submitted_at: self.submitted_at.clone(),
            updated_at: self.updated_at.clone(),
            result: self.result.clone(),
        }
    }
    fn transition(&mut self, state: JobState, stage: &str) {
        self.state = state;
        self.stage = stage.into();
        self.updated_at = clock::now_rfc3339();
        self.transitions.push(state);
    }
}

pub fn terminal(state: JobState) -> bool {
    matches!(
        state,
        JobState::Succeeded | JobState::Partial | JobState::Failed | JobState::Cancelled
    )
}

#[derive(Debug)]
struct Entry {
    record: JobRecord,
    cancel: Arc<AtomicBool>,
    /// A terminal whose write failed must remain visible until restart reconciles its journal.
    durable: bool,
}

#[derive(Debug)]
pub struct Jobs {
    root: PathBuf,
    entries: Mutex<BTreeMap<String, Entry>>,
    pub permits: Arc<tokio::sync::Semaphore>,
    queue_limit: usize,
}
impl Jobs {
    /// Open the journal over the worker semaphore the cleanup supervisor also holds.
    ///
    /// The semaphore is passed in rather than built here so that one permit set covers both
    /// running work and unresolved container cleanup: a half-removed container must occupy a
    /// worker slot, or repeated removal failures would quietly exceed the concurrency bound.
    pub fn open(
        data: &Path,
        permits: Arc<tokio::sync::Semaphore>,
        queue_limit: usize,
    ) -> io::Result<Self> {
        Self::open_recover(data, permits, queue_limit, |_| Ok(None))
    }
    /// Reconcile committed publication before classifying interrupted work. No producer runs here.
    pub fn open_recover(
        data: &Path,
        permits: Arc<tokio::sync::Semaphore>,
        queue_limit: usize,
        mut recover: impl FnMut(&JobRecord) -> io::Result<Option<(JobState, Envelope)>>,
    ) -> io::Result<Self> {
        let root = data.join("jobs");
        std::fs::create_dir_all(&root)?;
        for child in std::fs::read_dir(&root)? {
            let child = child?;
            if !matches!(child.file_name().to_str(), Some("active" | "terminal"))
                || !child.file_type()?.is_dir()
            {
                return Err(io::Error::other(
                    "unsupported job storage layout; reset obsolete development evidence",
                ));
            }
        }
        std::fs::create_dir_all(root.join("active"))?;
        std::fs::create_dir_all(root.join("terminal"))?;
        std::fs::File::open(&root)?.sync_all()?;
        std::fs::File::open(data)?.sync_all()?;
        for entry in std::fs::read_dir(root.join("active"))? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                let temporary = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.strip_prefix('.'))
                    .and_then(|s| s.strip_suffix(".tmp"))
                    .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()));
                if !temporary {
                    return Err(io::Error::other("unknown active job record"));
                }
                drop(open_record(&path)?);
                std::fs::remove_file(&path)?;
                std::fs::File::open(root.join("active"))?.sync_all()?;
                continue;
            }
            let header: JournalHeader =
                serde_json::from_reader(std::io::BufReader::new(open_record(&path)?))?;
            validate_id(&header.job_id)?;
            if header.format != FORMAT {
                return Err(io::Error::other(
                    "unsupported job journal format; reset obsolete development evidence",
                ));
            }
            if path.file_stem().and_then(|s| s.to_str()) != Some(&header.job_id) {
                return Err(io::Error::other("job journal identity mismatch"));
            }
            if !terminal(header.state) {
                let mut record = read_record(&root, &header.job_id)?;
                if terminal(record.state) {
                    // Terminal rename/fsync won, but the process died before active unlink.
                    std::fs::remove_file(&path)?;
                    std::fs::File::open(root.join("active"))?.sync_all()?;
                    continue;
                }
                if let Some((state, result)) = recover(&record)? {
                    if !terminal(state) {
                        return Err(io::Error::other(
                            "recovery supplied a nonterminal publication",
                        ));
                    }
                    record.transition(state, "recovered committed result; producer was not rerun");
                    record.result = Some(bound_delivery(&root, result)?);
                    persist(&root, &record)?;
                    continue;
                }
                record.transition(JobState::Failed, "interrupted: daemon restarted; resubmit explicitly after inspecting execution state");
                record.result = Some(crate::envelope::error(
                    enrichment_core::wire::ErrorCode::VerificationFailed,
                    "The daemon restarted before this job reached a terminal outcome.",
                    "Inspect owned execution containers, then explicitly resubmit; runtime work is never automatically repeated.",
                    false,
                ));
                persist(&root, &record)?;
            }
        }
        Ok(Self {
            root,
            entries: Mutex::new(BTreeMap::new()),
            permits,
            queue_limit: queue_limit.clamp(1, 1024),
        })
    }

    /// Return a unique caller interest and whether this submission owns the new worker.
    pub fn submit(
        &self,
        key: String,
        request: impl Into<JobSpec>,
    ) -> io::Result<(JobRecord, String, bool)> {
        let specification = request.into();
        specification.validate()?;
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        let token = format!("interest_{}", uuid::Uuid::new_v4().simple());
        if let Some(entry) = entries.values_mut().find(|e| {
            e.record.key == key && matches!(e.record.state, JobState::Queued | JobState::Running)
        }) {
            if entry.record.interests.len() + entry.record.detached_interests.len() >= MAX_INTERESTS
            {
                return Err(io::Error::other("job subscriber bound exhausted"));
            }
            let mut next = entry.record.clone();
            next.interests.insert(token.clone());
            next.updated_at = clock::now_rfc3339();
            persist(&self.root, &next)?;
            entry.record = next.clone();
            return Ok((next, token, false));
        }
        if entries
            .values()
            .filter(|e| !terminal(e.record.state) || !e.durable)
            .count()
            >= self.queue_limit
        {
            return Err(io::Error::other(
                "execution queue is full; poll current jobs before submitting more",
            ));
        }
        let now = clock::now_rfc3339();
        let record = JobRecord {
            format: FORMAT.into(),
            job_id: format!("job_{}", uuid::Uuid::new_v4().simple()),
            key,
            specification,
            state: JobState::Queued,
            stage: "queued".into(),
            interests: BTreeSet::from([token.clone()]),
            detached_interests: BTreeSet::new(),
            submitted_at: now.clone(),
            updated_at: now,
            transitions: vec![JobState::Queued],
            result: None,
            resolution: None,
        };
        persist(&self.root, &record)?;
        entries.insert(
            record.job_id.clone(),
            Entry {
                record: record.clone(),
                cancel: Arc::new(AtomicBool::new(false)),
                durable: true,
            },
        );
        Ok((record, token, true))
    }
    pub fn counts(&self) -> (usize, usize) {
        let Ok(entries) = self.entries.lock() else {
            return (0, 0);
        };
        (
            entries
                .values()
                .filter(|e| e.record.state == JobState::Queued)
                .count(),
            entries
                .values()
                .filter(|e| {
                    matches!(
                        e.record.state,
                        JobState::Running | JobState::CancelRequested
                    )
                })
                .count(),
        )
    }
    pub fn request_shutdown(&self) -> io::Result<()> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        for entry in entries.values_mut().filter(|e| !terminal(e.record.state)) {
            let mut next = entry.record.clone();
            next.transition(
                JobState::CancelRequested,
                "daemon shutdown; waiting for owned execution cleanup",
            );
            persist(&self.root, &next)?;
            entry.record = next;
            entry.cancel.store(true, Ordering::Release);
        }
        Ok(())
    }
    pub fn get(&self, id: &str) -> io::Result<JobRecord> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        ensure_terminal_loaded(&self.root, &mut entries, id)?;
        let record = entries
            .get(id)
            .ok_or_else(|| io::Error::other("job disappeared"))?
            .record
            .clone();
        evict_terminals(&mut entries);
        Ok(record)
    }
    pub fn cancellation(&self, id: &str) -> io::Result<Arc<AtomicBool>> {
        self.entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?
            .get(id)
            .map(|e| e.cancel.clone())
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "unknown job"))
    }
    pub fn cancel(&self, id: &str, token: &str) -> io::Result<JobRecord> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        ensure_terminal_loaded(&self.root, &mut entries, id)?;
        evict_terminals(&mut entries);
        // Authenticate from a bounded disk record if eviction selected this terminal job.
        let record = if let Some(entry) = entries.get(id) {
            entry.record.clone()
        } else {
            read_record(&self.root, id)?
        };
        if !record.interests.contains(token) && !record.detached_interests.contains(token) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "a matching caller interest token is required",
            ));
        }
        let mut next = record;
        next.interests.remove(token);
        next.detached_interests.insert(token.into());
        if next.interests.is_empty() && !terminal(next.state) {
            next.transition(
                JobState::CancelRequested,
                "last caller detached; waiting for process cleanup",
            );
        }
        persist(&self.root, &next)?;
        if let Some(entry) = entries.get_mut(id) {
            entry.record = next.clone();
            if next.state == JobState::CancelRequested {
                entry.cancel.store(true, Ordering::Release);
            }
        }
        Ok(next)
    }
    pub fn start(&self, id: &str) -> io::Result<bool> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "unknown job"))?;
        if entry.record.state != JobState::Queued {
            return Ok(false);
        }
        let mut next = entry.record.clone();
        next.transition(JobState::Running, "preparing concrete operation inputs");
        persist(&self.root, &next)?;
        entry.record = next;
        Ok(true)
    }
    /// Linearize cancellation against publication admission while pinning exact selected inputs.
    pub fn pin_resolution(&self, id: &str, stage: ResolutionStage) -> io::Result<()> {
        stage.validate()?;
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| io::Error::other("unknown acquisition job"))?;
        if !matches!(entry.record.specification, JobSpec::Resolve(_))
            || entry.record.state != JobState::Running
            || entry.record.resolution.is_some()
        {
            return Err(io::Error::other(
                "resolution was cancelled, already pinned or is not running",
            ));
        }
        let mut next = entry.record.clone();
        next.resolution = Some(stage);
        next.stage = "publishing exact immutable resolution".into();
        next.updated_at = clock::now_rfc3339();
        persist(&self.root, &next)?;
        entry.record = next;
        Ok(())
    }
    pub fn finish(&self, id: &str, state: JobState, result: Envelope) -> io::Result<()> {
        if !terminal(state) {
            return Err(io::Error::other("finish requires a terminal state"));
        }
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "unknown job"))?;
        if terminal(entry.record.state) {
            return Err(io::Error::other("terminal evidence is immutable"));
        }
        let terminal = (|| {
            let result = bound_delivery(&self.root, result)?;
            let mut next = entry.record.clone();
            next.transition(state, "finished");
            next.result = Some(result);
            persist(&self.root, &next)?;
            Ok::<_, io::Error>(next)
        })();
        match terminal {
            Ok(next) => entry.record = next,
            Err(error) => {
                settle_failure(&self.root, entry, &error);
                return Err(error);
            }
        }
        evict_terminals(&mut entries);
        Ok(())
    }

    /// A task that exits before terminal delivery cannot remain indefinitely running.
    /// Existing visible terminal records win; failed persistence remains explicit in memory.
    pub fn fail_unfinished(&self, id: &str, error: &io::Error) -> io::Result<()> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| io::Error::other("job lock poisoned"))?;
        let entry = entries
            .get_mut(id)
            .ok_or_else(|| io::Error::other("unknown failed job"))?;
        if !terminal(entry.record.state) {
            settle_failure(&self.root, entry, error);
        }
        Ok(())
    }
}

fn settle_failure(root: &Path, entry: &mut Entry, error: &io::Error) {
    if let Ok(committed) = read_record(root, &entry.record.job_id)
        && terminal(committed.state)
    {
        // Rename may have succeeded before a later fsync/unlink failed. Never overwrite it.
        entry.record = committed;
        entry.durable = false;
        return;
    }
    let detail: String = error.to_string().chars().take(1024).collect();
    let mut next = entry.record.clone();
    next.transition(JobState::Failed, "terminal delivery failed");
    let mut result = crate::envelope::error(
        enrichment_core::wire::ErrorCode::ArtifactUnavailable,
        format!("Job terminal delivery failed: {detail}"),
        "Inspect retained evidence and storage health before explicitly retrying.",
        false,
    );
    result.coverage.scope =
        "terminal delivery; previously published evidence remains independently retained".into();
    next.result = Some(result);
    entry.durable = persist(root, &next).is_ok();
    if !entry.durable {
        next.stage = "terminal delivery failed; journal could not be persisted; restart must reconcile retained state".into();
    }
    entry.record = next;
}

/// Journal size is independent of library size. Large delivery DTOs use the same immutable
/// overflow-artifact contract as tool replies. The facts remain in the native catalog; this
/// derived delivery artifact can be reconstructed after a crash without repeating a producer.
fn bound_delivery(root: &Path, result: Envelope) -> io::Result<Envelope> {
    let data = root
        .parent()
        .ok_or_else(|| io::Error::other("job data root missing"))?;
    crate::delivery::encode(
        &enrichment_store::BlobStore::open(data)?,
        result,
        crate::delivery::JOURNAL_BYTES,
        false,
    )
}

fn validate_id(id: &str) -> io::Result<()> {
    if !id
        .strip_prefix("job_")
        .is_some_and(|s| s.len() == 32 && s.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "invalid service job identity",
        ));
    }
    Ok(())
}

fn open_record(path: &Path) -> io::Result<std::io::Take<std::fs::File>> {
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.len() > MAX_RECORD_BYTES {
        return Err(io::Error::other("invalid or oversized job journal"));
    }
    Ok(std::fs::File::open(path)?.take(MAX_RECORD_BYTES))
}

fn read_record(root: &Path, id: &str) -> io::Result<JobRecord> {
    validate_id(id)?;
    let terminal_path = root.join("terminal").join(format!("{id}.json"));
    let path = match std::fs::symlink_metadata(&terminal_path) {
        Ok(_) => terminal_path,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            root.join("active").join(format!("{id}.json"))
        }
        Err(e) => return Err(e),
    };
    let record: JobRecord = serde_json::from_reader(std::io::BufReader::new(open_record(&path)?))?;
    record.specification.validate()?;
    if let Some(stage) = &record.resolution {
        stage.validate()?;
        if !matches!(record.specification, JobSpec::Resolve(_)) {
            return Err(io::Error::other(
                "non-resolution job contains an exact resolution stage",
            ));
        }
    }
    if record.format != FORMAT
        || record.job_id != id
        || record.interests.len() + record.detached_interests.len() > MAX_INTERESTS
    {
        return Err(io::Error::other(
            "job journal identity or subscriber bound mismatch",
        ));
    }
    Ok(record)
}

fn ensure_terminal_loaded(
    root: &Path,
    entries: &mut BTreeMap<String, Entry>,
    id: &str,
) -> io::Result<()> {
    if !entries.contains_key(id) {
        let record = read_record(root, id)?;
        if !terminal(record.state) {
            return Err(io::Error::other(
                "nonterminal job lost its worker ownership",
            ));
        }
        entries.insert(
            id.into(),
            Entry {
                record,
                cancel: Arc::new(AtomicBool::new(false)),
                durable: true,
            },
        );
    }
    Ok(())
}

fn evict_terminals(entries: &mut BTreeMap<String, Entry>) {
    let terminal_ids: Vec<_> = entries
        .iter()
        .filter(|(_, e)| terminal(e.record.state) && e.durable)
        .map(|(id, _)| id.clone())
        .collect();
    for id in terminal_ids
        .iter()
        .take(terminal_ids.len().saturating_sub(MAX_TERMINAL_CACHE))
    {
        entries.remove(id);
    }
}

fn persist(root: &Path, record: &JobRecord) -> io::Result<()> {
    record.specification.validate()?;
    if let Some(stage) = &record.resolution {
        stage.validate()?;
        if !matches!(record.specification, JobSpec::Resolve(_)) {
            return Err(io::Error::other(
                "non-resolution job contains an exact resolution stage",
            ));
        }
    }
    if record.format != FORMAT {
        return Err(io::Error::other("invalid journal format"));
    }
    validate_id(&record.job_id)?;
    let bytes = serde_json::to_vec(record)?;
    if bytes.len() as u64 > MAX_RECORD_BYTES
        || record.interests.len() + record.detached_interests.len() > MAX_INTERESTS
    {
        return Err(io::Error::other("job journal exceeds storage bound"));
    }
    let directory = root.join(if terminal(record.state) {
        "terminal"
    } else {
        "active"
    });
    let target = directory.join(format!("{}.json", record.job_id));
    let temp = root
        .join("active")
        .join(format!(".{}.tmp", uuid::Uuid::new_v4().simple()));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    std::fs::rename(&temp, &target)?;
    std::fs::File::open(&directory)?.sync_all()?;
    std::fs::File::open(root.join("active"))?.sync_all()?;
    if terminal(record.state) {
        enrichment_store::publication_probe::hit(
            root.parent()
                .ok_or_else(|| io::Error::other("job data root missing"))?,
            enrichment_store::publication_probe::Point::JournalTerminalDurable,
        )?;
        match std::fs::remove_file(root.join("active").join(format!("{}.json", record.job_id))) {
            Ok(()) => std::fs::File::open(root.join("active"))?.sync_all()?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failed_terminal_delivery_leaves_a_failed_job_and_restart_never_reexecutes() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 1).unwrap();
        let (job, _, _) = jobs.submit("delivery".into(), request()).unwrap();
        jobs.start(&job.job_id).unwrap();
        std::fs::write(dir.path().join("blobs"), b"unavailable blob root").unwrap();
        let result = crate::envelope::error(
            enrichment_core::wire::ErrorCode::VerificationFailed,
            "result",
            "inspect",
            false,
        );
        assert!(jobs.finish(&job.job_id, JobState::Failed, result).is_err());
        assert_eq!(jobs.get(&job.job_id).unwrap().state, JobState::Failed);
        drop(jobs);
        let reopened = Jobs::open(dir.path(), permits(1), 1).unwrap();
        assert!(
            reopened
                .get(&job.job_id)
                .unwrap()
                .result
                .unwrap()
                .summary
                .contains("terminal delivery failed")
        );
    }

    #[test]
    fn unavailable_terminal_storage_keeps_failure_visible_and_bounds_new_admission() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 1).unwrap();
        let (job, _, _) = jobs.submit("delivery".into(), request()).unwrap();
        jobs.start(&job.job_id).unwrap();
        let terminal = jobs.root.join("terminal");
        std::fs::remove_dir(&terminal).unwrap();
        std::fs::write(&terminal, b"unavailable terminal directory").unwrap();
        let result = crate::envelope::error(
            enrichment_core::wire::ErrorCode::VerificationFailed,
            "result",
            "inspect",
            false,
        );
        assert!(jobs.finish(&job.job_id, JobState::Failed, result).is_err());
        let record = jobs.get(&job.job_id).unwrap();
        assert_eq!(record.state, JobState::Failed);
        assert!(record.stage.contains("could not be persisted"));
        assert!(jobs.submit("another".into(), request()).is_err());
        drop(jobs);
        std::fs::remove_file(&terminal).unwrap();
        std::fs::create_dir(&terminal).unwrap();
        let reopened = Jobs::open(dir.path(), permits(1), 1).unwrap();
        let record = reopened.get(&job.job_id).unwrap();
        assert_eq!(record.state, JobState::Failed);
        assert!(record.stage.contains("interrupted"));
    }
    fn permits(n: usize) -> Arc<tokio::sync::Semaphore> {
        Arc::new(tokio::sync::Semaphore::new(n))
    }
    fn request() -> VerifyRequest {
        VerifyRequest {
            context_id: "ctx_test".into(),
            snapshot_id: Some("snap_test".into()),
            snippet: "assert True".into(),
            mode: enrichment_core::execution::ProbeMode::Runtime,
            profile: enrichment_core::policy::ExecutionProfile::Runtime,
            test_intent: None,
            max_bytes: None,
        }
    }
    #[test]
    fn oversized_success_is_durable_without_exceeding_the_journal_bound() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let (job, _, _) = jobs.submit("large result".into(), request()).unwrap();
        jobs.start(&job.job_id).unwrap();
        let payload = "é".repeat(800_000);
        let result = crate::envelope::ok(
            "large native result",
            crate::ops::common::to_object(&serde_json::json!({"payload":payload})),
            enrichment_core::wire::Coverage {
                scope: "large delivery".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: Vec::new(),
            },
        );
        jobs.finish(&job.job_id, JobState::Succeeded, result)
            .unwrap();
        let record = jobs.get(&job.job_id).unwrap();
        let envelope = record.result.unwrap();
        let id = envelope.data["result_artifact_id"].as_str().unwrap();
        let blobs = enrichment_store::BlobStore::open(dir.path()).unwrap();
        let artifact = blobs.find(id).unwrap().unwrap();
        let saved: serde_json::Value =
            serde_json::from_slice(&blobs.read(&artifact.sha256).unwrap()).unwrap();
        assert_eq!(saved["status"], "ok");
        assert_eq!(saved["data"]["payload"], payload);
        assert!(saved.get("request_id").is_none());
        assert!(
            std::fs::metadata(
                dir.path()
                    .join("jobs/terminal")
                    .join(format!("{}.json", job.job_id))
            )
            .unwrap()
            .len()
                < MAX_RECORD_BYTES
        );
        drop(jobs);
        let reopened = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let record = reopened.get(&job.job_id).unwrap();
        assert_eq!(record.state, JobState::Succeeded);
        assert_eq!(record.result.unwrap().data, envelope.data);
    }

    #[test]
    fn durable_jobs_preserve_two_interests_and_terminal_evidence_across_restart() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let (a, token_a, fresh) = jobs.submit("same".into(), request()).unwrap();
        assert!(fresh);
        let (b, token_b, fresh) = jobs.submit("same".into(), request()).unwrap();
        assert!(!fresh);
        assert_eq!(a.job_id, b.job_id);
        assert_ne!(token_a, token_b);
        jobs.start(&a.job_id).unwrap();
        assert!(jobs.cancel(&a.job_id, "wrong").is_err());
        assert_eq!(
            jobs.cancel(&a.job_id, &token_a).unwrap().state,
            JobState::Running
        );
        assert!(
            !jobs
                .cancellation(&a.job_id)
                .unwrap()
                .load(Ordering::Acquire)
        );
        assert_eq!(
            jobs.cancel(&a.job_id, &token_b).unwrap().state,
            JobState::CancelRequested
        );
        assert!(
            jobs.cancellation(&a.job_id)
                .unwrap()
                .load(Ordering::Acquire)
        );
        let result = crate::envelope::error(
            enrichment_core::wire::ErrorCode::VerificationFailed,
            "cancelled after cleanup",
            "resubmit explicitly",
            false,
        );
        jobs.finish(&a.job_id, JobState::Cancelled, result.clone())
            .unwrap();
        assert!(jobs.finish(&a.job_id, JobState::Succeeded, result).is_err());
        drop(jobs);
        let reopened = Jobs::open(dir.path(), permits(1), 2).unwrap();
        assert_eq!(reopened.get(&a.job_id).unwrap().state, JobState::Cancelled);
        assert_eq!(
            reopened.get(&a.job_id).unwrap().transitions,
            vec![
                JobState::Queued,
                JobState::Running,
                JobState::CancelRequested,
                JobState::Cancelled
            ]
        );
    }
    #[test]
    fn committed_publication_wins_over_interruption_and_late_cancellation() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let (job, token, _) = jobs.submit("query".into(), request()).unwrap();
        jobs.start(&job.job_id).unwrap();
        jobs.cancel(&job.job_id, &token).unwrap();
        drop(jobs);
        let result = crate::envelope::ok(
            "committed result",
            Default::default(),
            enrichment_core::wire::Coverage {
                scope: "fixture".into(),
                indexed: Default::default(),
                missing: Default::default(),
                limitations: vec![],
            },
        );
        let jobs = Jobs::open_recover(dir.path(), permits(1), 2, |record| {
            assert_eq!(record.job_id, job.job_id);
            assert_eq!(record.state, JobState::CancelRequested);
            Ok(Some((JobState::Succeeded, result.clone())))
        })
        .unwrap();
        assert_eq!(jobs.get(&job.job_id).unwrap().result, Some(result));
        assert_eq!(
            jobs.cancel(&job.job_id, &token).unwrap().state,
            JobState::Succeeded
        );
        drop(jobs);
        let jobs = Jobs::open_recover(dir.path(), permits(1), 2, |_| {
            panic!("terminal replay requires no publication lookup or execution")
        })
        .unwrap();
        assert_eq!(jobs.get(&job.job_id).unwrap().state, JobState::Succeeded);
    }
    #[test]
    fn terminal_cache_eviction_preserves_disk_results_and_bounds_unauthorized_cancels() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let mut records = Vec::new();
        for i in 0..MAX_TERMINAL_CACHE * 3 {
            let (record, token, _) = jobs.submit(format!("request-{i}"), request()).unwrap();
            let result = crate::envelope::error(
                enrichment_core::wire::ErrorCode::VerificationFailed,
                format!("completed-{i}"),
                "inspect retained result",
                false,
            );
            jobs.finish(&record.job_id, JobState::Failed, result)
                .unwrap();
            records.push((record.job_id, token));
            assert!(jobs.entries.lock().unwrap().len() <= MAX_TERMINAL_CACHE);
        }
        drop(jobs);
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        assert!(jobs.entries.lock().unwrap().is_empty());
        for (i, (id, token)) in records.iter().enumerate() {
            assert!(jobs.cancel(id, "unrelated-interest").is_err());
            assert!(jobs.entries.lock().unwrap().len() <= MAX_TERMINAL_CACHE);
            let record = jobs.cancel(id, token).unwrap();
            assert_eq!(record.state, JobState::Failed);
            assert_eq!(record.result.unwrap().summary, format!("completed-{i}"));
            assert!(jobs.entries.lock().unwrap().len() <= MAX_TERMINAL_CACHE);
            assert_eq!(jobs.get(id).unwrap().state, JobState::Failed);
        }
        assert!(jobs.get("../../outside").is_err());
    }
    #[test]
    fn subscriber_and_journal_storage_bounds_reject_without_overwriting() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 2).unwrap();
        let (record, _, _) = jobs.submit("same".into(), request()).unwrap();
        for _ in 1..MAX_INTERESTS {
            jobs.submit("same".into(), request()).unwrap();
        }
        assert!(jobs.submit("same".into(), request()).is_err());
        assert_eq!(
            jobs.get(&record.job_id).unwrap().interests.len(),
            MAX_INTERESTS
        );
        let mut oversized = record.clone();
        oversized.stage = "x".repeat(MAX_RECORD_BYTES as usize);
        assert!(persist(&jobs.root, &oversized).is_err());
        assert_eq!(
            read_record(&jobs.root, &record.job_id).unwrap().stage,
            "queued"
        );
        let path = jobs
            .root
            .join("active")
            .join(format!("{}.json", record.job_id));
        std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .unwrap()
            .set_len(MAX_RECORD_BYTES + 1)
            .unwrap();
        drop(jobs);
        assert!(Jobs::open(dir.path(), permits(1), 2).is_err());
    }
    #[test]
    fn durable_jobs_restart_never_reexecutes_interrupted_runtime_and_queue_is_bounded() {
        let dir = tempfile::tempdir().unwrap();
        let jobs = Jobs::open(dir.path(), permits(1), 1).unwrap();
        let (a, _, _) = jobs.submit("one".into(), request()).unwrap();
        assert!(jobs.submit("two".into(), request()).is_err());
        jobs.start(&a.job_id).unwrap();
        drop(jobs);
        let reopened = Jobs::open(dir.path(), permits(1), 1).unwrap();
        let a = reopened.get(&a.job_id).unwrap();
        assert_eq!(a.state, JobState::Failed);
        assert!(a.stage.contains("interrupted"));
        assert!(a.result.is_some());
        assert!(reopened.submit("one".into(), request()).unwrap().2);
    }
}
