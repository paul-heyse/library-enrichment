// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! [`ListingSchemaProvider`]: [`SchemaProvider`] that scans ObjectStores for tables automatically

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::{SchemaProvider, TableProvider, TableProviderFactory};

use crate::Session;
use datafusion_common::{
    DFSchema, DataFusionError, HashMap, TableReference, internal_datafusion_err,
};
use datafusion_expr::CreateExternalTable;

use async_trait::async_trait;
use futures::TryStreamExt;
use object_store::ObjectStore;

/// A [`SchemaProvider`] that scans an [`ObjectStore`] to automatically discover tables
///
/// A subfolder relationship is assumed, i.e. given:
/// - authority = `s3://host.example.com:3000`
/// - path = `/data/tpch`
/// - factory = `DeltaTableFactory`
///
/// A table called "customer" will be registered for the folder:
/// `s3://host.example.com:3000/data/tpch/customer`
///
/// assuming it contains valid deltalake data, i.e:
/// - `s3://host.example.com:3000/data/tpch/customer/part-00000-xxxx.snappy.parquet`
/// - `s3://host.example.com:3000/data/tpch/customer/_delta_log/`
///
/// [`ObjectStore`]: object_store::ObjectStore
#[derive(Debug)]
pub struct ListingSchemaProvider {
    authority: String,
    path: object_store::path::Path,
    factory: Arc<dyn TableProviderFactory>,
    store: Arc<dyn ObjectStore>,
    tables: Arc<Mutex<HashMap<String, Arc<dyn TableProvider>>>>,
    format: String,
    max_objects: usize,
    max_tables: usize,
}

impl ListingSchemaProvider {
    /// Create a new `ListingSchemaProvider`
    ///
    /// Arguments:
    /// `authority`: The scheme (i.e. s3://) + host (i.e. example.com:3000)
    /// `path`: The root path that contains subfolders which represent tables
    /// `factory`: The `TableProviderFactory` to use to instantiate tables for each subfolder
    /// `store`: The `ObjectStore` containing the table data
    /// `format`: The `FileFormat` of the tables
    /// `has_header`: Indicates whether the created external table has the has_header flag enabled
    pub fn new(
        authority: String,
        path: object_store::path::Path,
        factory: Arc<dyn TableProviderFactory>,
        store: Arc<dyn ObjectStore>,
        format: String,
    ) -> Self {
        Self {
            authority,
            path,
            factory,
            store,
            tables: Arc::new(Mutex::new(HashMap::new())),
            format,
            max_objects: 100_000,
            max_tables: 10_000,
        }
    }

    /// Bound inventory before accumulating candidates or publishing any provider.
    pub fn with_discovery_limits(
        mut self,
        max_objects: usize,
        max_tables: usize,
    ) -> datafusion_common::Result<Self> {
        if max_objects == 0 || max_tables == 0 {
            return Err(DataFusionError::Configuration(
                "discovery limits must be positive".into(),
            ));
        }
        self.max_objects = max_objects;
        self.max_tables = max_tables;
        Ok(self)
    }

    /// Reload a complete inventory atomically. An incomplete listing, exceeded bound,
    /// ambiguous normalized name or failed factory leaves the previous inventory intact.
    pub async fn refresh(&self, state: &dyn Session) -> datafusion_common::Result<()> {
        let mut entries = self.store.list(Some(&self.path));
        let base = Path::new(self.path.as_ref());
        let mut candidates = BTreeMap::<String, String>::new();
        let mut examined = 0;
        while let Some(file) = entries.try_next().await? {
            if examined == self.max_objects {
                return Err(DataFusionError::ResourcesExhausted(
                    "listing object bound exceeded".into(),
                ));
            }
            examined += 1;
            let mut is_dir = false;
            let mut parent = Path::new(file.location.as_ref());
            while let Some(p) = parent.parent() {
                if p == base {
                    let candidate = TablePath {
                        is_dir,
                        path: parent,
                    };
                    let file_name = parent
                        .file_name()
                        .and_then(|n| n.to_str())
                        .ok_or_else(|| internal_datafusion_err!("Cannot parse file name!"))?;
                    let name = file_name
                        .split('.')
                        .next()
                        .filter(|name| !name.is_empty())
                        .ok_or_else(|| internal_datafusion_err!("Empty normalized table name"))?;
                    let location = candidate
                        .to_string()
                        .ok_or_else(|| internal_datafusion_err!("Cannot parse table location"))?;
                    if let Some(existing) = candidates.get(name) {
                        if existing != &location {
                            return Err(DataFusionError::Plan(format!(
                                "ambiguous discovered table name: {name}"
                            )));
                        }
                    } else {
                        if candidates.len() == self.max_tables {
                            return Err(DataFusionError::ResourcesExhausted(
                                "listing table bound exceeded".into(),
                            ));
                        }
                        candidates.insert(name.to_owned(), location);
                    }
                    break;
                }
                parent = p;
                is_dir = true;
            }
        }
        let mut prepared = HashMap::new();
        for (name, location) in candidates {
            let provider = self
                .factory
                .create(
                    state,
                    &CreateExternalTable::builder(
                        TableReference::bare(name.clone()),
                        format!("{}/{}", self.authority, location),
                        self.format.clone(),
                        Arc::new(DFSchema::empty()),
                    )
                    .build(),
                )
                .await?;
            prepared.insert(name, provider);
        }
        *self.tables.lock().expect("Can't lock tables") = prepared;
        Ok(())
    }
}

#[async_trait]
impl SchemaProvider for ListingSchemaProvider {
    fn table_names(&self) -> Vec<String> {
        self.tables
            .lock()
            .expect("Can't lock tables")
            .keys()
            .map(|it| it.to_string())
            .collect()
    }

    async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>, DataFusionError> {
        Ok(self
            .tables
            .lock()
            .expect("Can't lock tables")
            .get(name)
            .cloned())
    }

    fn register_table(
        &self,
        name: String,
        table: Arc<dyn TableProvider>,
    ) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>> {
        self.tables
            .lock()
            .expect("Can't lock tables")
            .insert(name, Arc::clone(&table));
        Ok(Some(table))
    }

    fn deregister_table(
        &self,
        name: &str,
    ) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>> {
        Ok(self.tables.lock().expect("Can't lock tables").remove(name))
    }

    fn table_exist(&self, name: &str) -> bool {
        self.tables
            .lock()
            .expect("Can't lock tables")
            .contains_key(name)
    }
}

/// Stores metadata along with a table's path.
/// Primarily whether the path is a directory or not.
#[derive(Eq, PartialEq, Hash, Debug)]
struct TablePath<'a> {
    path: &'a Path,
    is_dir: bool,
}

impl TablePath<'_> {
    /// Format the path with a '/' appended if its a directory.
    /// Clients (eg. object_store listing) can and will use the presence of trailing slash as a heuristic
    fn to_string(&self) -> Option<String> {
        self.path.to_str().map(|path| {
            if self.is_dir {
                format!("{path}/")
            } else {
                path.to_string()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_path_ends_with_slash_when_is_dir() {
        let table_path = TablePath {
            path: Path::new("/file"),
            is_dir: true,
        };
        assert!(table_path.to_string().expect("table path").ends_with('/'));
    }

    #[test]
    fn dir_table_path_str_does_not_end_with_slash_when_not_is_dir() {
        let table_path = TablePath {
            path: Path::new("/file"),
            is_dir: false,
        };
        assert!(!table_path.to_string().expect("table_path").ends_with('/'));
    }
}
