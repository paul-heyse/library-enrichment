//! Resolve a table or a view the first time a query names it, and not before.
//!
//! # Why
//!
//! This is a one-shot binary: it builds a session, registers everything, answers one question and
//! exits. Registering everything is most of what it does. Measured at twenty tables and fifty
//! views, a query that reads four tables spent **250 ms** preparing -- 63 ms opening Delta tables,
//! 67 ms building the run-filtered base views, 118 ms planning the projections -- before ~80 ms of
//! actually answering. Session construction, the usual suspect, is 1.2 ms.
//!
//! None of that work is wasted on a *slow* path; it is wasted on a path nobody took. The catalog's
//! own DataFusion notes list "registering every table eagerly when a lazy schema provider would
//! resolve on demand" as an anti-pattern, and `SchemaProvider` is built for this:
//!
//! - `table_names()` is **sync** and enumerates without resolving, so `information_schema.tables`
//!   and `codesearch-query projections` still list the whole retrieval surface.
//! - `table()` is **async** and is called only for references the parser found, so a table nobody
//!   named is never opened and a view nobody read is never planned.
//! - `table_type()` has a default that resolves, and the trait's own documentation says to override
//!   it when `table()` is expensive. This does.
//!
//! # What this does not give up
//!
//! §7 argues for eager registration on the grounds that *"a projection that will not plan is a
//! broken retrieval surface, and discovering that at query time -- per caller, per invocation -- is
//! how a catalog ends up with a projection nobody has run in months"*. That argument is right, and
//! it is preserved: `information_schema.views` and `.columns` call `table()` for **every** name, so
//! a query against either plans everything. `model/tests/projections.rs` already queries it, and
//! `just check-projections` makes it a named gate step rather than a side effect of a test.
//!
//! The guarantee therefore moves from *every invocation* to *every gate run*, which is where a
//! developer-time guarantee belongs. Paying it per caller bought nothing a caller could use.

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, Weak};

use datafusion::catalog::{SchemaProvider, TableProvider};
use datafusion::common::Result as DFResult;
use datafusion::error::DataFusionError;
use datafusion::execution::SessionState;
use datafusion::logical_expr::TableType;
use parking_lot::RwLock;

/// How one named table is produced, the first time somebody asks for it.
#[async_trait::async_trait]
pub trait Resolver: Debug + Send + Sync {
    /// Every name this resolver can produce, without producing any of them.
    fn names(&self) -> Vec<String>;

    /// Produce one. `None` means "not mine", which is not an error.
    async fn resolve(&self, name: &str) -> DFResult<Option<Arc<dyn TableProvider>>>;
}

/// A schema whose members are resolved on first reference and then remembered.
#[derive(Debug)]
pub struct LazySchema {
    resolver: Arc<dyn Resolver>,
    /// Resolved members, plus anything registered directly. A `Mutex` and never held across an
    /// `await`: resolving a view plans SQL, which calls back into this same schema for the tables
    /// the view reads, and a lock held across that would deadlock on the first nested reference.
    resolved: Mutex<BTreeMap<String, Arc<dyn TableProvider>>>,
}

impl LazySchema {
    pub fn new(resolver: Arc<dyn Resolver>) -> Self {
        Self {
            resolver,
            resolved: Mutex::new(BTreeMap::new()),
        }
    }

    fn cached(&self, name: &str) -> Option<Arc<dyn TableProvider>> {
        self.resolved
            .lock()
            .ok()
            .and_then(|m| m.get(name).map(Arc::clone))
    }
}

#[async_trait::async_trait]
impl SchemaProvider for LazySchema {
    fn table_names(&self) -> Vec<String> {
        let mut names = self.resolver.names();
        if let Ok(m) = self.resolved.lock() {
            for k in m.keys() {
                if !names.contains(k) {
                    names.push(k.clone());
                }
            }
        }
        names.sort();
        names.dedup();
        names
    }

    fn table_exist(&self, name: &str) -> bool {
        self.table_names().iter().any(|n| n == name)
    }

    async fn table(&self, name: &str) -> DFResult<Option<Arc<dyn TableProvider>>> {
        if let Some(hit) = self.cached(name) {
            return Ok(Some(hit));
        }
        let Some(table) = self.resolver.resolve(name).await? else {
            return Ok(None);
        };
        if let Ok(mut m) = self.resolved.lock() {
            m.insert(name.to_string(), Arc::clone(&table));
        }
        Ok(Some(table))
    }

    /// Answer without resolving.
    ///
    /// Every member of this schema is a base table or a view, and `information_schema.tables` wants
    /// only which. Falling back to the default -- which calls `table()` -- would make listing the
    /// catalog open every Delta table and plan every view, which is the whole cost this module
    /// exists to avoid.
    async fn table_type(&self, name: &str) -> DFResult<Option<TableType>> {
        if let Some(hit) = self.cached(name) {
            return Ok(Some(hit.table_type()));
        }
        Ok(self.table_exist(name).then_some(TableType::View))
    }

    fn register_table(
        &self,
        name: String,
        table: Arc<dyn TableProvider>,
    ) -> DFResult<Option<Arc<dyn TableProvider>>> {
        // Supported so a caller can inject a provider directly -- which is how the fixture tests
        // put `MemTable`s under the same names the query binary resolves from Delta, and therefore
        // how they exercise the real views rather than a parallel registration path.
        let mut m = self
            .resolved
            .lock()
            .map_err(|e| DataFusionError::Execution(format!("lazy schema lock poisoned: {e}")))?;
        Ok(m.insert(name, table))
    }

    fn deregister_table(&self, name: &str) -> DFResult<Option<Arc<dyn TableProvider>>> {
        let mut m = self
            .resolved
            .lock()
            .map_err(|e| DataFusionError::Execution(format!("lazy schema lock poisoned: {e}")))?;
        Ok(m.remove(name))
    }
}

/// Plans a named SQL definition into a `ViewTable`, on demand.
///
/// Holds a **weak** reference to the session state. A view lives in the session's catalog, so a
/// strong one would be a cycle that never drops -- and in a one-shot process a leak that outlives
/// the answer is a leak nobody would ever see fail.
#[derive(Debug)]
pub struct SqlViews {
    definitions: Vec<(String, String)>,
    state: Weak<RwLock<SessionState>>,
}

impl SqlViews {
    pub fn new(definitions: Vec<(String, String)>, state: Weak<RwLock<SessionState>>) -> Self {
        Self { definitions, state }
    }
}

#[async_trait::async_trait]
impl Resolver for SqlViews {
    fn names(&self) -> Vec<String> {
        self.definitions.iter().map(|(n, _)| n.clone()).collect()
    }

    async fn resolve(&self, name: &str) -> DFResult<Option<Arc<dyn TableProvider>>> {
        let Some((_, sql)) = self.definitions.iter().find(|(n, _)| n == name) else {
            return Ok(None);
        };
        let state = self.state.upgrade().ok_or_else(|| {
            DataFusionError::Execution(format!(
                "the session that owns view `{name}` is gone, so it cannot be planned"
            ))
        })?;
        // Cloned, and the guard dropped, BEFORE planning. Planning resolves the view's own table
        // references through this same catalog, which takes the lock again -- and `parking_lot`'s
        // `RwLock` is not reentrant, so holding it across the plan would deadlock on the first
        // nested reference rather than fail.
        let state = { state.read().clone() };
        let plan = state.create_logical_plan(sql).await.map_err(|e| {
            DataFusionError::Plan(format!("view `{name}` does not plan: {e}\n  sql: {sql}"))
        })?;
        Ok(Some(Arc::new(datafusion::datasource::ViewTable::new(
            plan,
            Some(sql.clone()),
        ))))
    }
}

/// Try each resolver in turn, first match wins.
///
/// The default schema holds two different kinds of member -- the raw `<name>__all` providers and
/// the run-filtered `<name>` views over them -- produced by two different mechanisms. Chaining
/// keeps each mechanism ignorant of the other rather than giving one a special case for the other's
/// names.
#[derive(Debug)]
pub struct Chain(pub Vec<Arc<dyn Resolver>>);

#[async_trait::async_trait]
impl Resolver for Chain {
    fn names(&self) -> Vec<String> {
        self.0.iter().flat_map(|r| r.names()).collect()
    }

    async fn resolve(&self, name: &str) -> DFResult<Option<Arc<dyn TableProvider>>> {
        for r in &self.0 {
            if let Some(t) = r.resolve(name).await? {
                return Ok(Some(t));
            }
        }
        Ok(None)
    }
}

/// A resolver that produces nothing, for a caller that registers its members directly.
///
/// The fixture tests do exactly that: they inject `MemTable`s under the same names the query binary
/// resolves from Delta, so they exercise the real views instead of a parallel registration path.
#[derive(Debug)]
pub struct Injected;

#[async_trait::async_trait]
impl Resolver for Injected {
    fn names(&self) -> Vec<String> {
        Vec::new()
    }

    async fn resolve(&self, _name: &str) -> DFResult<Option<Arc<dyn TableProvider>>> {
        Ok(None)
    }
}
