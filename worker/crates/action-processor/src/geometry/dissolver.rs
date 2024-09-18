use std::collections::HashMap;
use std::time::Instant;

use itertools::Itertools;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use reearth_flow_action_log::action_log;
use reearth_flow_geometry::algorithm::bool_ops::BooleanOps;
use reearth_flow_geometry::types::geometry::Geometry2D;
use reearth_flow_geometry::types::multi_polygon::MultiPolygon2D;
use reearth_flow_geometry::types::polygon::Polygon2D;
use reearth_flow_runtime::node::REJECTED_PORT;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Attribute, Feature, GeometryValue};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::GeometryProcessorError;

pub static AREA_PORT: Lazy<Port> = Lazy::new(|| Port::new("area"));

#[derive(Debug, Clone, Default)]
pub struct GeometryDissolverFactory;

impl ProcessorFactory for GeometryDissolverFactory {
    fn name(&self) -> &str {
        "GeometryDissolver"
    }

    fn description(&self) -> &str {
        "Dissolve geometries"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(GeometryDissolverParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["Geometry"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![AREA_PORT.clone(), REJECTED_PORT.clone()]
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: GeometryDissolverParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                GeometryProcessorError::DissolverFactory(format!(
                    "Failed to serialize `with` parameter: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                GeometryProcessorError::DissolverFactory(format!(
                    "Failed to deserialize `with` parameter: {}",
                    e
                ))
            })?
        } else {
            return Err(GeometryProcessorError::DissolverFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        Ok(Box::new(GeometryDissolver {
            params,
            buffer: HashMap::new(),
        }))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GeometryDissolverParam {
    group_by: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone)]
pub struct GeometryDissolver {
    params: GeometryDissolverParam,
    buffer: HashMap<String, Vec<Feature>>,
}

impl Processor for GeometryDissolver {
    fn num_threads(&self) -> usize {
        2
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let feature = &ctx.feature;
        let start = Instant::now();
        let Some(geometry) = &feature.geometry else {
            fw.send(ctx.new_with_feature_and_port(ctx.feature.clone(), REJECTED_PORT.clone()));
            return Ok(());
        };
        match &geometry.value {
            GeometryValue::FlowGeometry2D(_) | GeometryValue::FlowGeometry3D(_) => {
                let key = if let Some(group_by) = &self.params.group_by {
                    group_by
                        .iter()
                        .map(|k| feature.get(&k).map(|v| v.to_string()).unwrap_or_default())
                        .collect::<Vec<_>>()
                        .join("\t")
                } else {
                    "_all".to_string()
                };
                if let Some(values) = self.buffer.get(&key) {
                    self.handle_geometry(feature, values, &ctx, fw);
                    {
                        if let Some(buffer) = self.buffer.get_mut(&key) {
                            buffer.push(feature.clone());
                        }
                    }
                } else {
                    self.buffer.insert(key, vec![feature.clone()]);
                    self.handle_geometry(feature, &[], &ctx, fw);
                }
            }
            _ => fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone())),
        }
        let span = ctx.info_span();
        action_log!(
            parent: span, ctx.logger.action_logger("dissolver"), "echo with feature = {:?}, duration = {:?}", feature.id, start.elapsed(),
        );
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
        "GeometryDissolver"
    }
}

impl GeometryDissolver {
    fn handle_geometry(
        &self,
        feature: &Feature,
        others: &[Feature],
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        let Some(geometry) = feature.geometry.as_ref() else {
            fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone()));
            return;
        };
        match &geometry.value {
            GeometryValue::FlowGeometry2D(geos) => {
                let others = others
                    .iter()
                    .filter_map(|f| {
                        f.geometry
                            .as_ref()
                            .and_then(|g| g.value.as_flow_geometry_2d().cloned())
                    })
                    .collect::<Vec<_>>();
                self.handle_2d_geometry(geos, &others, feature, ctx, fw);
            }
            _ => fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone())),
        }
    }

    fn handle_2d_geometry(
        &self,
        geos: &Geometry2D,
        others: &[Geometry2D],
        feature: &Feature,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        let mut target_polygons = others.iter().filter_map(|g| g.as_polygon()).collect_vec();
        target_polygons.extend({
            match others.len() {
                0..=1000 => others
                    .iter()
                    .filter_map(|g| {
                        g.as_multi_polygon()
                            .map(|multi_polygon| multi_polygon.iter().cloned().collect_vec())
                    })
                    .flatten()
                    .collect_vec(),
                _ => others
                    .par_iter()
                    .flat_map(|g| {
                        g.as_multi_polygon()
                            .map(|multi_polygon| multi_polygon.iter().cloned().collect_vec())
                    })
                    .flatten()
                    .collect::<Vec<_>>(),
            }
        });
        match geos {
            Geometry2D::MultiPolygon(mpolygons) => {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), AREA_PORT.clone()));
                if target_polygons.is_empty() {
                    return;
                }
                self.handle_2d_polygons(mpolygons, target_polygons, feature, ctx, fw);
            }
            Geometry2D::Polygon(polygon) => {
                fw.send(ctx.new_with_feature_and_port(feature.clone(), AREA_PORT.clone()));
                if target_polygons.is_empty() {
                    return;
                }
                self.handle_2d_polygons(
                    &MultiPolygon2D::new(vec![polygon.clone()]),
                    target_polygons,
                    feature,
                    ctx,
                    fw,
                );
            }
            _ => fw.send(ctx.new_with_feature_and_port(feature.clone(), REJECTED_PORT.clone())),
        }
    }

    fn handle_2d_polygons(
        &self,
        target: &MultiPolygon2D<f64>,
        others: Vec<Polygon2D<f64>>,
        feature: &Feature,
        ctx: &ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) {
        for polygon in target.iter() {
            others.iter().for_each(|other_polygon| {
                let multi_polygon = polygon.intersection(other_polygon);
                for polygon in multi_polygon.iter() {
                    let Some(geometry) = &feature.geometry else {
                        return;
                    };
                    let mut geometry = geometry.clone();
                    let mut feature = feature.clone();
                    feature.refresh_id();
                    geometry.value =
                        GeometryValue::FlowGeometry2D(Geometry2D::Polygon(polygon.clone()));
                    feature.geometry = Some(geometry);
                    fw.send(ctx.new_with_feature_and_port(feature, AREA_PORT.clone()));
                }
            });
        }
    }
}
