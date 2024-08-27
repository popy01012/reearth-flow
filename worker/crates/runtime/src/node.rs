use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::sync::Arc;

use once_cell::sync::Lazy;
use reearth_flow_types::Feature;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::Sender;

use crate::channels::ProcessorChannelForwarder;
use crate::errors::{BoxedError, DeserializationError};
use crate::event::EventHub;
use crate::executor_operation::{ExecutorContext, NodeContext};

pub static DEFAULT_PORT: Lazy<Port> = Lazy::new(|| Port::new("default"));
pub static REJECTED_PORT: Lazy<Port> = Lazy::new(|| Port::new("rejected"));
pub static ROUTING_PARAM_KEY: &str = "routingPort";
pub static REMAIN_PORT: Lazy<Port> = Lazy::new(|| Port::new("remain"));

pub(super) type GraphId = uuid::Uuid;

#[nutype::nutype(
    sanitize(trim),
    derive(
        Debug,
        Clone,
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
        AsRef,
        Serialize,
        Deserialize,
        Hash,
        Display,
        JsonSchema,
    )
)]
pub(super) struct EdgeId(String);

#[nutype::nutype(
    sanitize(trim),
    derive(
        Debug,
        Clone,
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
        AsRef,
        Serialize,
        Deserialize,
        Hash,
        Display,
        JsonSchema,
    )
)]
pub(super) struct NodeId(String);

#[nutype::nutype(
    sanitize(trim),
    derive(
        Debug,
        Clone,
        Eq,
        PartialEq,
        PartialOrd,
        Ord,
        AsRef,
        Serialize,
        Deserialize,
        Hash,
        Display,
        JsonSchema
    )
)]
pub struct Port(String);

#[derive(Debug, Clone)]
pub struct NodeType {
    pub handle: NodeHandle,
    pub name: String,
    pub action: String,
    pub kind: NodeKind,
    pub with: Option<HashMap<String, serde_json::Value>>,
}

impl NodeType {
    pub fn new(
        id: NodeId,
        name: String,
        action: String,
        kind: NodeKind,
        with: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            handle: NodeHandle::new(id),
            name,
            action,
            kind,
            with,
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Source(Box<dyn SourceFactory>),
    Processor(Box<dyn ProcessorFactory>),
    Sink(Box<dyn SinkFactory>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeHandle {
    pub id: NodeId,
}

impl NodeHandle {
    pub fn new(id: NodeId) -> Self {
        Self { id }
    }
}

impl Display for NodeHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.id.to_string().as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
/// All possible kinds of `IngestionMessage`.
pub enum IngestionMessage {
    OperationEvent { feature: Feature },
}

pub trait SourceClone {
    fn clone_box(&self) -> Box<dyn Source>;
}

impl<T> SourceClone for T
where
    T: 'static + Source + Clone,
{
    fn clone_box(&self) -> Box<dyn Source> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Source> {
    fn clone(&self) -> Box<dyn Source> {
        self.clone_box()
    }
}

pub trait ProcessorClone {
    fn clone_box(&self) -> Box<dyn Processor>;
}

impl<T> ProcessorClone for T
where
    T: 'static + Processor + Clone,
{
    fn clone_box(&self) -> Box<dyn Processor> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Processor> {
    fn clone(&self) -> Box<dyn Processor> {
        self.clone_box()
    }
}

pub trait SinkClone {
    fn clone_box(&self) -> Box<dyn Sink>;
}

impl<T> SinkClone for T
where
    T: 'static + Sink + Clone,
{
    fn clone_box(&self) -> Box<dyn Sink> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Sink> {
    fn clone(&self) -> Box<dyn Sink> {
        self.clone_box()
    }
}

pub trait SourceFactory: Send + Sync + Debug + SourceFactoryClone {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema>;

    fn categories(&self) -> &[&'static str] {
        &[]
    }

    fn get_output_ports(&self) -> Vec<Port>;
    fn build(
        &self,
        ctx: NodeContext,
        event_hub: EventHub,
        action: String,
        with: Option<HashMap<String, Value>>,
        state: Option<Vec<u8>>,
    ) -> Result<Box<dyn Source>, BoxedError>;
}

pub trait SourceFactoryClone {
    fn clone_box(&self) -> Box<dyn SourceFactory>;
}

impl<T> SourceFactoryClone for T
where
    T: 'static + SourceFactory + Clone,
{
    fn clone_box(&self) -> Box<dyn SourceFactory> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SourceFactory> {
    fn clone(&self) -> Box<dyn SourceFactory> {
        self.clone_box()
    }
}

#[async_trait::async_trait]
pub trait Source: Send + Sync + Debug + SourceClone {
    async fn initialize(&self, ctx: NodeContext);
    fn name(&self) -> &str;
    async fn serialize_state(&self) -> Result<Vec<u8>, BoxedError>;

    async fn start(
        &mut self,
        ctx: NodeContext,
        sender: Sender<(Port, IngestionMessage)>,
    ) -> Result<(), BoxedError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceState {
    /// This source hasn't been ingested.
    NotStarted,
    /// This source has some data ingested, and it can't be restarted.
    NonRestartable,
}

pub type SourceStates = HashMap<NodeHandle, SourceState>;

pub trait ProcessorFactory: Send + Sync + Debug + ProcessorFactoryClone {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema>;

    fn categories(&self) -> &[&'static str] {
        &[]
    }

    fn get_input_ports(&self) -> Vec<Port>;
    fn get_output_ports(&self) -> Vec<Port>;
    fn build(
        &self,
        ctx: NodeContext,
        event_hub: EventHub,
        action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError>;
}

pub trait ProcessorFactoryClone {
    fn clone_box(&self) -> Box<dyn ProcessorFactory>;
}

impl<T> ProcessorFactoryClone for T
where
    T: 'static + ProcessorFactory + Clone,
{
    fn clone_box(&self) -> Box<dyn ProcessorFactory> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ProcessorFactory> {
    fn clone(&self) -> Box<dyn ProcessorFactory> {
        self.clone_box()
    }
}

pub trait Processor: Send + Sync + Debug + ProcessorClone {
    fn initialize(&mut self, ctx: NodeContext);
    fn num_threads(&self) -> usize {
        1
    }
    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError>;
    fn finish(
        &self,
        ctx: NodeContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError>;

    fn name(&self) -> &str;
}

pub trait SinkFactory: Send + Sync + Debug + SinkFactoryClone {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema>;

    fn categories(&self) -> &[&'static str] {
        &[]
    }
    fn get_input_ports(&self) -> Vec<Port>;
    fn prepare(&self) -> Result<(), BoxedError>;
    fn build(
        &self,
        ctx: NodeContext,
        event_hub: EventHub,
        action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Sink>, BoxedError>;
}

pub trait SinkFactoryClone {
    fn clone_box(&self) -> Box<dyn SinkFactory>;
}

impl<T> SinkFactoryClone for T
where
    T: 'static + SinkFactory + Clone,
{
    fn clone_box(&self) -> Box<dyn SinkFactory> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SinkFactory> {
    fn clone(&self) -> Box<dyn SinkFactory> {
        self.clone_box()
    }
}

pub trait Sink: Send + Debug + SinkClone {
    fn initialize(&self, ctx: NodeContext);
    fn name(&self) -> &str;
    fn process(&mut self, ctx: ExecutorContext) -> Result<(), BoxedError>;

    fn finish(&self, ctx: NodeContext) -> Result<(), BoxedError>;
    fn set_source_state(&mut self, _source_state: &[u8]) -> Result<(), BoxedError> {
        Ok(())
    }

    fn get_source_state(&mut self) -> Result<Option<Vec<u8>>, BoxedError> {
        Ok(None)
    }

    fn preferred_batch_size(&self) -> Option<u64> {
        None
    }

    fn max_batch_duration_ms(&self) -> Option<u64> {
        None
    }

    fn flush_batch(&mut self) -> Result<(), BoxedError> {
        Ok(())
    }

    fn supports_batching(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct RouterFactory;

impl ProcessorFactory for RouterFactory {
    fn name(&self) -> &str {
        "Router"
    }

    fn description(&self) -> &str {
        "Action for last port forwarding for sub-workflows."
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(Router))
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![]
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let process: Router = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(DeserializationError::Json)?;
            serde_json::from_value(value).map_err(DeserializationError::Json)?
        } else {
            return Err(DeserializationError::EmptyInput.into());
        };
        Ok(Box::new(process))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Router {
    routing_port: String,
}

impl Processor for Router {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let feature = ctx.feature;
        fw.send(ExecutorContext::new(
            feature,
            Port::new(&self.routing_port),
            Arc::clone(&ctx.expr_engine),
            Arc::clone(&ctx.storage_resolver),
            Arc::clone(&ctx.logger),
            Arc::clone(&ctx.kv_store),
        ));
        Ok(())
    }

    fn finish(
        &self,
        _ctx: NodeContext,
        _fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "Router"
    }
}
