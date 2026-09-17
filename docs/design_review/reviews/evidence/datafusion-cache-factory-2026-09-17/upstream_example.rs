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

// Original example dataset runner omitted for isolated probes.

use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use datafusion::arrow::array::RecordBatch;
use datafusion::catalog::Session;
use datafusion::catalog::memory::MemorySourceConfig;
use datafusion::common::DFSchemaRef;
use datafusion::common::HashMap;
use datafusion::error::Result;
use datafusion::execution::context::QueryPlanner;
use datafusion::execution::session_state::CacheFactory;
use datafusion::execution::{SessionState, SessionStateBuilder};
use datafusion::logical_expr::physical_planning_context::PhysicalPlanningContext;
use datafusion::logical_expr::{
    Extension, LogicalPlan, UserDefinedLogicalNode, UserDefinedLogicalNodeCore,
};
use datafusion::physical_plan::{ExecutionPlan, collect_partitioned};
use datafusion::physical_planner::{DefaultPhysicalPlanner, ExtensionPlanner, PhysicalPlanner};
use datafusion::prelude::*;
#[derive(Debug)]
struct CustomCacheFactory {}

impl CacheFactory for CustomCacheFactory {
    fn create(&self, plan: LogicalPlan, _session_state: &SessionState) -> Result<LogicalPlan> {
        Ok(LogicalPlan::Extension(Extension {
            node: Arc::new(CacheNode { input: plan }),
        }))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Hash, Debug)]
struct CacheNode {
    input: LogicalPlan,
}

impl UserDefinedLogicalNodeCore for CacheNode {
    fn name(&self) -> &str {
        "CacheNode"
    }

    fn inputs(&self) -> Vec<&LogicalPlan> {
        vec![&self.input]
    }

    fn schema(&self) -> &DFSchemaRef {
        self.input.schema()
    }

    fn expressions(&self) -> Vec<Expr> {
        vec![]
    }

    fn fmt_for_explain(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CacheNode")
    }

    fn with_exprs_and_inputs(
        &self,
        _exprs: Vec<Expr>,
        mut inputs: Vec<LogicalPlan>,
    ) -> Result<Self> {
        assert_eq!(inputs.len(), 1, "input size must be one");
        Ok(Self {
            input: inputs.swap_remove(0),
        })
    }
}

struct CacheNodePlanner {
    cache_manager: Arc<RwLock<CacheManager>>,
}

#[async_trait]
impl ExtensionPlanner for CacheNodePlanner {
    async fn plan_extension(
        &self,
        _planner: &dyn PhysicalPlanner,
        node: &dyn UserDefinedLogicalNode,
        logical_inputs: &[&LogicalPlan],
        physical_inputs: &[Arc<dyn ExecutionPlan>],
        session_state: &dyn Session,
        _planning_ctx: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        if let Some(cache_node) = node.as_any().downcast_ref::<CacheNode>() {
            assert_eq!(logical_inputs.len(), 1, "Inconsistent number of inputs");
            assert_eq!(physical_inputs.len(), 1, "Inconsistent number of inputs");
            if self
                .cache_manager
                .read()
                .unwrap()
                .get(&cache_node.input)
                .is_none()
            {
                let ctx = session_state.task_ctx();
                println!("caching in memory");
                let batches = collect_partitioned(physical_inputs[0].clone(), ctx).await?;
                self.cache_manager
                    .write()
                    .unwrap()
                    .put(cache_node.input.clone(), batches);
            } else {
                println!("fetching directly from cache manager");
            }
            Ok(self
                .cache_manager
                .read()
                .unwrap()
                .get(&cache_node.input)
                .map(|batches| {
                    let exec: Arc<dyn ExecutionPlan> = MemorySourceConfig::try_new_exec(
                        batches,
                        physical_inputs[0].schema(),
                        None,
                    )
                    .unwrap();
                    exec
                }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Default)]
struct CacheNodeQueryPlanner {
    cache_manager: Arc<RwLock<CacheManager>>,
}

#[async_trait]
impl QueryPlanner for CacheNodeQueryPlanner {
    async fn create_physical_plan(
        &self,
        logical_plan: &LogicalPlan,
        session_state: &dyn Session,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        let physical_planner =
            DefaultPhysicalPlanner::with_extension_planners(vec![Arc::new(CacheNodePlanner {
                cache_manager: Arc::clone(&self.cache_manager),
            })]);
        physical_planner
            .create_physical_plan(logical_plan, session_state)
            .await
    }
}

// This naive implementation only includes put, but for real production use cases cache eviction and drop should also be implemented.
#[derive(Debug, Default)]
struct CacheManager {
    cache: HashMap<LogicalPlan, Vec<Vec<RecordBatch>>>,
}

impl CacheManager {
    pub fn put(&mut self, k: LogicalPlan, v: Vec<Vec<RecordBatch>>) {
        self.cache.insert(k, v);
    }

    pub fn get(&self, k: &LogicalPlan) -> Option<&Vec<Vec<RecordBatch>>> {
        self.cache.get(k)
    }
}
