use std::time::Instant;
use std::{collections::HashMap, sync::Arc};

use petgraph::graph::NodeIndex;

use reearth_flow_action::{ActionContext, ActionDataframe, AsyncAction, SyncAction};
use reearth_flow_action_log::action_log;
use reearth_flow_action_log::span;
#[allow(unused_imports)]
use reearth_flow_action_universal::prelude::*;
use reearth_flow_common::serde as serde_utils;
use reearth_flow_state::State;
use reearth_flow_workflow::graph::NodeAction;
use reearth_flow_workflow::workflow::WorkflowParameter;

pub(crate) struct ActionRunner;

impl ActionRunner {
    pub(crate) async fn run(
        ctx: ActionContext,
        workflow_params: WorkflowParameter,
        action: NodeAction,
        ix: NodeIndex,
        dataframe_state: Arc<State>,
        input: Option<ActionDataframe>,
    ) -> crate::Result<(NodeIndex, ActionDataframe)> {
        let node_id = ctx.node_id;
        let node_name = ctx.node_name.clone();
        let start_logger = Arc::clone(&ctx.logger);
        let end_logger = Arc::clone(&ctx.logger);
        let span = span(
            ctx.root_span.clone(),
            action.to_string(),
            node_id.to_string(),
            node_name.clone(),
        );
        action_log!(
            parent: span,
            start_logger,
            "Start action = {:?}, name = {:?}",
            action,
            node_name,
        );
        let start = Instant::now();
        let mut params = vec![(
            "action".to_owned(),
            serde_json::Value::String(action.to_string()),
        )];
        match (&workflow_params.global, &workflow_params.node) {
            (Some(global_property), Some(node_property)) => {
                let mut global = serde_json::Value::from(global_property.clone());
                let with_value = serde_json::Value::Object(node_property.clone());
                serde_utils::merge_value(&mut global, with_value);
                params.push(("with".to_owned(), global));
            }
            (Some(global_property), None) => {
                let global = serde_json::Value::from(global_property.clone());
                params.push(("with".to_owned(), global));
            }
            (None, Some(node_property)) => {
                let with_value = serde_json::Value::Object(node_property.clone());
                params.push(("with".to_owned(), with_value));
            }
            _ => {}
        }
        let res = {
            let action_run: serde_json::Result<Box<dyn AsyncAction>> =
                serde_json::from_value(serde_json::Value::Object(
                    params
                        .clone()
                        .into_iter()
                        .collect::<serde_json::Map<_, _>>(),
                ));
            match action_run {
                Ok(action_run) => action_run
                    .run(ctx, input)
                    .await
                    .map_err(|e| crate::Error::action(e, action.to_string()))?,
                Err(e) if e.classify() == serde_json::error::Category::Data => {
                    let action_run: Box<dyn SyncAction> =
                        serde_json::from_value(serde_json::Value::Object(
                            params.into_iter().collect::<serde_json::Map<_, _>>(),
                        ))
                        .map_err(crate::Error::execution)?;
                    let result = tokio::task::spawn_blocking(move || action_run.run(ctx, input))
                        .await
                        .map_err(|e| {
                            crate::Error::action(
                                reearth_flow_action::error::Error::internal_runtime(format!(
                                    "{:?}",
                                    e
                                )),
                                action.to_string(),
                            )
                        })?;
                    result.map_err(|e| {
                        crate::Error::action(
                            reearth_flow_action::error::Error::internal_runtime(format!("{:?}", e)),
                            action.to_string(),
                        )
                    })?
                }
                Err(e) => {
                    return Err(crate::Error::execution(e));
                }
            }
        };
        dataframe_state
            .save(&convert_dataframe(&res), node_id.to_string().as_str())
            .await
            .map_err(crate::Error::execution)?;
        let duration = start.elapsed();
        action_log!(
            parent: span,
            end_logger,
            "Finish action = {:?}, name = {:?}, ports = {:?}, duration = {:?}",
            action,
            node_name,
            res.keys(),
            duration,
        );
        Ok((ix, res))
    }
}

fn convert_dataframe(dataframe: &ActionDataframe) -> HashMap<String, serde_json::Value> {
    dataframe
        .iter()
        .filter_map(|(k, v)| match v {
            Some(v) => {
                let value: serde_json::Value = v.clone().into();
                Some((k.clone().into_inner(), value))
            }
            None => None,
        })
        .collect::<HashMap<String, serde_json::Value>>()
}
