use std::collections::HashMap;

use once_cell::sync::Lazy;
use reearth_flow_geometry::validation::*;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::GeometryValue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

static SUCCESS_PORT: Lazy<Port> = Lazy::new(|| Port::new("success"));
static FAILED_PORT: Lazy<Port> = Lazy::new(|| Port::new("failed"));
static REJECTED_PORT: Lazy<Port> = Lazy::new(|| Port::new("rejected"));

#[derive(Debug, Clone, Default)]
pub struct GeometryValidatorFactory;

impl ProcessorFactory for GeometryValidatorFactory {
    fn name(&self) -> &str {
        "GeometryValidator"
    }

    fn description(&self) -> &str {
        "Validates the geometry of a feature"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(GeometryValidator))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![
            SUCCESS_PORT.clone(),
            FAILED_PORT.clone(),
            REJECTED_PORT.clone(),
        ]
    }
    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let processor: GeometryValidator = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::GeometryValidatorFactory(format!(
                    "Failed to serialize with: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::GeometryValidatorFactory(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::GeometryValidatorFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(processor))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub enum ValidationType {
    DuplicatePoints,
}

impl From<ValidationType> for reearth_flow_geometry::validation::ValidationType {
    fn from(validation_type: ValidationType) -> Self {
        match validation_type {
            ValidationType::DuplicatePoints => {
                reearth_flow_geometry::validation::ValidationType::DuplicatePoints
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeometryValidator {
    validation_type: ValidationType,
}

impl Processor for GeometryValidator {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        2
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let feature = &ctx.feature;
        let Some(geometry) = &feature.geometry else {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone()));
            return Ok(());
        };
        let port = match &geometry.value {
            GeometryValue::Null => REJECTED_PORT.clone(),
            GeometryValue::FlowGeometry2D(geometry) => {
                if geometry.validate(self.validation_type.clone().into()) {
                    SUCCESS_PORT.clone()
                } else {
                    FAILED_PORT.clone()
                }
            }
            GeometryValue::FlowGeometry3D(geometry) => {
                if geometry.validate(self.validation_type.clone().into()) {
                    SUCCESS_PORT.clone()
                } else {
                    FAILED_PORT.clone()
                }
            }
            GeometryValue::CityGmlGeometry(gml_geometry) => {
                if gml_geometry.features.iter().all(|feature| {
                    feature
                        .polygons
                        .iter()
                        .all(|polygon| polygon.validate(self.validation_type.clone().into()))
                }) {
                    SUCCESS_PORT.clone()
                } else {
                    FAILED_PORT.clone()
                }
            }
        };
        fw.send(ctx.new_with_feature_and_port(feature.clone(), port));
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
        "GeometryValidator"
    }
}
