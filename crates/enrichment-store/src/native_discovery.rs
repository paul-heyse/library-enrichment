//! Native factory preparation and bounded, complete service-owned storage discovery.
//! Captured providers retain their exact Delta snapshot. Discovery cannot select publication.
use crate::native_delta::{
    CONTRACT_PROPERTY, CONTRACT_TABLE, DeltaStore, LoadedTable, StorageContract, contract_catalog,
    requires_plain_binary, verify_contract,
};
use async_trait::async_trait;
use datafusion::{
    catalog::{
        SchemaProvider, Session, TableProvider, TableProviderFactory,
        listing_schema::ListingSchemaProvider,
    },
    common::{DFSchema, TableReference},
    error::{DataFusionError, Result},
    execution::runtime_env::RuntimeEnv,
    logical_expr::CreateExternalTable,
    prelude::{SessionContext, col, lit},
};
use deltalake::{
    DeltaTable,
    delta_datafusion::{DeltaScanConfig, DeltaTableFactory, DeltaTableOpener},
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct Captured {
    table: LoadedTable,
    contract: StorageContract,
    runtime: Arc<RuntimeEnv>,
}

fn scan(
    session: &dyn Session,
    table: &DeltaTable,
    contract: &StorageContract,
) -> Result<DeltaScanConfig> {
    verify_contract(table, contract, session)?;
    table.update_datafusion_session(session).map_err(external)?;
    Ok(scan_config(session, contract))
}

pub(crate) fn scan_config(session: &dyn Session, contract: &StorageContract) -> DeltaScanConfig {
    let mut scan =
        DeltaScanConfig::new_from_session(session).with_schema(contract.semantic_schema());
    if contract
        .semantic_schema()
        .fields()
        .iter()
        .any(|f| requires_plain_binary(f.data_type()))
    {
        scan.schema_force_view_types = false;
    }
    scan
}

#[async_trait]
impl DeltaTableOpener for Captured {
    async fn open(
        &self,
        session: &dyn Session,
        command: &CreateExternalTable,
    ) -> Result<(DeltaTable, DeltaScanConfig)> {
        if !Arc::ptr_eq(session.runtime_env(), &self.runtime)
            || !command.options.is_empty()
            || command.locations != [self.table.log_store().root_url().to_string()]
        {
            return Err(invalid("factory command differs from owned capture"));
        }
        Ok((
            self.table.table_clone(),
            scan(session, &self.table, &self.contract)?,
        ))
    }
}

pub(crate) async fn captured_provider(
    context: &SessionContext,
    table: LoadedTable,
    contract: StorageContract,
) -> Result<Arc<dyn TableProvider>> {
    let memory = table.memory();
    let location = table.log_store().root_url().to_string();
    let factory = DeltaTableFactory::with_opener(Arc::new(Captured {
        table,
        contract,
        runtime: context.runtime_env(),
    }));
    let provider = factory
        .create(
            &context.state(),
            &CreateExternalTable::builder(
                TableReference::bare("captured"),
                location,
                "DELTATABLE",
                Arc::new(DFSchema::empty()),
            )
            .build(),
        )
        .await?;
    Ok(crate::leases::accounted_provider(provider, memory))
}

/// A complete immutable inventory for maintenance/preparation. It is never a head selector.
pub struct Discovered {
    pub tables: BTreeMap<String, Arc<dyn TableProvider>>,
    pub captures: Vec<enrichment_core::operation::StorageCapture>,
}

struct Discover {
    delta: DeltaStore,
    contracts: Option<LoadedTable>,
    captures: Mutex<
        BTreeMap<
            String,
            (
                enrichment_core::operation::StorageCapture,
                Arc<crate::snapshot_registry::SnapshotMemory>,
            ),
        >,
    >,
}
impl std::fmt::Debug for Discover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Discover")
            .field("root", &self.delta.root)
            .finish_non_exhaustive()
    }
}

#[async_trait]
impl DeltaTableOpener for Discover {
    async fn open(
        &self,
        session: &dyn Session,
        command: &CreateExternalTable,
    ) -> Result<(DeltaTable, DeltaScanConfig)> {
        let name = command.name.table();
        let location = self.delta.location(name)?;
        if !Arc::ptr_eq(session.runtime_env(), &self.delta.session().runtime_env())
            || !command.options.is_empty()
            || command.locations.len() != 1
            || url::Url::parse(&command.locations[0]).map_err(external)? != location
        {
            return Err(invalid("discovery command escaped the owned namespace"));
        }
        let table = self.delta.load(name, None).await?;
        let contract = if name == CONTRACT_TABLE {
            contract_catalog()?
        } else {
            let id = table
                .snapshot()
                .map_err(external)?
                .metadata()
                .configuration()
                .get(CONTRACT_PROPERTY)
                .ok_or_else(|| invalid("discovered table has no semantic contract"))?;
            let registry = self
                .contracts
                .as_ref()
                .ok_or_else(|| invalid("semantic registry absent"))?;
            self.delta
                .registered_contract(
                    registry,
                    &enrichment_core::identity::SchemaContractId::try_from(id.clone())
                        .map_err(external)?,
                )
                .await?
        };
        let config = scan(session, &table, &contract)?;
        let capture = enrichment_core::operation::StorageCapture {
            name: name.to_owned(),
            source: crate::native_delta::capture_version(name, &table, &contract)?,
        };
        if self
            .captures
            .lock()
            .map_err(|_| invalid("discovery capture poisoned"))?
            .insert(name.to_owned(), (capture, table.memory()))
            .is_some()
        {
            return Err(invalid("duplicate discovered table"));
        }
        Ok((table.table_clone(), config))
    }
}

impl DeltaStore {
    /// One schema-registry decoder for discovery and native maintenance. The current
    /// registry is authority; decoded schemas must reproduce the complete contract id.
    pub(crate) async fn registered_contract(
        &self,
        registry: &LoadedTable,
        id: &enrichment_core::identity::SchemaContractId,
    ) -> Result<StorageContract> {
        let context = self.session();
        let provider = captured_provider(&context, registry.clone(), contract_catalog()?).await?;
        let output = self
            .runtime
            .execute(
                context
                    .read_table(provider)?
                    .filter(col("contract_id").eq(lit(id)))?
                    .select(vec![col("arrow_schema")])?
                    .limit(0, Some(2))?,
            )
            .await?;
        if output.rows != 1 {
            return Err(invalid("registered contract is missing or ambiguous"));
        }
        let column = arrow::compute::cast(
            output.batches[0].column(0),
            &arrow::datatypes::DataType::Binary,
        )?;
        let bytes = datafusion::common::cast::as_binary_array(&column)?.value(0);
        let contract = StorageContract::from_ipc(bytes)?;
        if contract.identity() != id {
            return Err(invalid(
                "registered schema identity or schema-only IPC framing mismatch",
            ));
        }
        Ok(contract)
    }

    pub(crate) async fn current_contract(&self, name: &str) -> Result<StorageContract> {
        if name == CONTRACT_TABLE {
            return contract_catalog();
        }
        let table = self.load(name, None).await?;
        let id = table
            .snapshot()
            .map_err(external)?
            .metadata()
            .configuration()
            .get(CONTRACT_PROPERTY)
            .ok_or_else(|| invalid("table has no registered semantic contract"))?;
        let registry = self.load(CONTRACT_TABLE, None).await?;
        self.registered_contract(
            &registry,
            &enrichment_core::identity::SchemaContractId::try_from(id.clone()).map_err(external)?,
        )
        .await
    }

    /// Inspect a complete bounded storage inventory, retaining exact provider captures.
    /// # Errors
    /// Unknown/incomplete contracts, bounded listing exhaustion and collisions refuse the inventory.
    pub async fn discover(&self, max_objects: usize, max_tables: usize) -> Result<Discovered> {
        let contracts = match self.load(CONTRACT_TABLE, None).await {
            Ok(table) => Some(table),
            Err(error) if crate::native_delta::missing_table(&error) => None,
            Err(error) => return Err(error),
        };
        let opener = Arc::new(Discover {
            delta: self.clone(),
            contracts,
            captures: Mutex::default(),
        });
        let context = self.session();
        let listing = ListingSchemaProvider::new(
            "file://".into(),
            object_store::path::Path::from_filesystem_path(&self.root)?,
            Arc::new(DeltaTableFactory::with_opener(opener.clone())),
            context.runtime_env().object_store(
                datafusion::execution::object_store::ObjectStoreUrl::local_filesystem(),
            )?,
            "DELTATABLE".into(),
        )
        .with_discovery_limits(max_objects, max_tables)?;
        listing.refresh(&context.state()).await?;
        let mut tables = BTreeMap::new();
        for name in listing.table_names() {
            let provider = listing
                .table(&name)
                .await?
                .ok_or_else(|| invalid("discovered table vanished"))?;
            let memory = opener
                .captures
                .lock()
                .map_err(|_| invalid("discovery capture poisoned"))?
                .get(&name)
                .ok_or_else(|| invalid("discovery charge absent"))?
                .1
                .clone();
            let provider = crate::leases::accounted_provider(provider, memory);
            tables.insert(name, context.read_table(provider)?.into_view());
        }
        let captures = opener
            .captures
            .lock()
            .map_err(|_| invalid("discovery capture poisoned"))?
            .values()
            .map(|(capture, _)| capture.clone())
            .collect();
        Ok(Discovered { tables, captures })
    }
}

fn invalid(message: &str) -> DataFusionError {
    DataFusionError::Plan(message.into())
}
fn external(error: impl std::error::Error + Send + Sync + 'static) -> DataFusionError {
    DataFusionError::External(Box::new(error))
}
