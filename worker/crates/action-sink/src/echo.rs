use reearth_flow_action_log::action_log;
use serde_json::Value;
use std::collections::HashMap;

use reearth_flow_runtime::errors::BoxedError;
use reearth_flow_runtime::event::EventHub;
use reearth_flow_runtime::executor_operation::{ExecutorContext, NodeContext};
use reearth_flow_runtime::node::{Port, Sink, SinkFactory, DEFAULT_PORT};

#[derive(Debug, Clone, Default)]
pub struct EchoSinkFactory;

impl SinkFactory for EchoSinkFactory {
    fn name(&self) -> &str {
        "Echo"
    }

    fn description(&self) -> &str {
        "Echo features"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        None
    }

    fn categories(&self) -> &[&'static str] {
        &["Debug"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn prepare(&self) -> Result<(), BoxedError> {
        Ok(())
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        _with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Sink>, BoxedError> {
        Ok(Box::new(Echo))
    }
}

#[derive(Debug, Clone)]
pub struct Echo;

impl Sink for Echo {
    fn initialize(&self, _ctx: NodeContext) {}

    fn name(&self) -> &str {
        "Echo"
    }

    fn process(&mut self, ctx: ExecutorContext) -> Result<(), BoxedError> {
        let span = ctx.info_span();
        let feature: serde_json::Value = ctx.feature.clone().into();
        action_log!(
            parent: span, ctx.logger.action_logger("echo"), "echo with feature = {:?}", serde_json::to_string(&feature).unwrap_or_default(),
        );
        Ok(())
    }
    fn finish(&self, _ctx: NodeContext) -> Result<(), BoxedError> {
        Ok(())
    }
}
