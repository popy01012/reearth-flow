use std::{collections::HashMap, sync::Arc};

use reearth_flow_geometry::types::geometry::Geometry as FlowGeometry;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Expr, Geometry, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

#[derive(Debug, Clone, Default)]
pub struct ExtruderFactory;

impl ProcessorFactory for ExtruderFactory {
    fn name(&self) -> &str {
        "Extruder"
    }

    fn description(&self) -> &str {
        "Extrudes a polygon by a distance"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(ExtruderParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn build(
        &self,
        ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: ExtruderParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::ExtruderFactory(format!("Failed to serialize with: {}", e))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::ExtruderFactory(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::ExtruderFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };

        let expr_engine = Arc::clone(&ctx.expr_engine);
        let expr = &params.distance;
        let template_ast = expr_engine
            .compile(expr.as_ref())
            .map_err(|e| GeometryProcessorError::ExtruderFactory(format!("{:?}", e)))?;
        let process = Extruder {
            distance: template_ast,
        };
        Ok(Box::new(process))
    }
}

#[derive(Debug, Clone)]
pub struct Extruder {
    distance: rhai::AST,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtruderParam {
    distance: Expr,
}

impl Processor for Extruder {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let expr_engine = Arc::clone(&ctx.expr_engine);
        let feature = &ctx.feature;
        let scope = expr_engine.new_scope();
        for (k, v) in &feature.attributes {
            scope.set(k.inner().as_str(), v.clone().into());
        }
        let Ok(height) = scope.eval_ast::<f64>(&self.distance) else {
            return Err(GeometryProcessorError::Extruder(
                "Failed to evaluate distance".to_string(),
            )
            .into());
        };
        let Some(geometry) = &feature.geometry else {
            return Err(GeometryProcessorError::Extruder("Missing geometry".to_string()).into());
        };
        let geometry = geometry.clone();
        let GeometryValue::FlowGeometry(flow_geometry) = &geometry.value else {
            return Err(GeometryProcessorError::Extruder("Invalid geometry".to_string()).into());
        };
        let FlowGeometry::Polygon(polygon) = flow_geometry else {
            return Err(GeometryProcessorError::Extruder("Invalid geometry".to_string()).into());
        };
        let solid = polygon.extrude(height);
        let geometry = Geometry {
            value: GeometryValue::FlowGeometry(FlowGeometry::Solid(solid)),
            ..geometry
        };
        let mut feature = feature.clone();
        feature.geometry = Some(geometry);
        fw.send(ctx.new_with_feature_and_port(feature, DEFAULT_PORT.clone()));
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
        "Extruder"
    }
}
