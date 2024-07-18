use std::{collections::HashMap, sync::Arc};

use once_cell::sync::Lazy;
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Attribute, AttributeValue, Expr, Feature};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::AttributeProcessorError;

pub static COMPLETE_PORT: Lazy<Port> = Lazy::new(|| Port::new("complete"));

#[derive(Debug, Clone, Default)]
pub struct StatisticsCalculatorFactory;

impl ProcessorFactory for StatisticsCalculatorFactory {
    fn name(&self) -> &str {
        "StatisticsCalculator"
    }

    fn description(&self) -> &str {
        "Calculates statistics of features"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(StatisticsCalculatorParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["Attribute"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone(), COMPLETE_PORT.clone()]
    }

    fn build(
        &self,
        ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: StatisticsCalculatorParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                AttributeProcessorError::StatisticsCalculatorFactory(format!(
                    "Failed to serialize with: {}",
                    e
                ))
            })?;
            serde_json::from_value(value).map_err(|e| {
                AttributeProcessorError::StatisticsCalculatorFactory(format!(
                    "Failed to deserialize with: {}",
                    e
                ))
            })?
        } else {
            return Err(AttributeProcessorError::StatisticsCalculatorFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };
        let expr_engine = Arc::clone(&ctx.expr_engine);
        let mut calculations = Vec::<CompliledCalculation>::new();
        for calculation in &params.calculations {
            let expr = &calculation.expr;
            let template_ast = expr_engine.compile(expr.as_ref()).map_err(|e| {
                AttributeProcessorError::StatisticsCalculatorFactory(format!("{:?}", e))
            })?;
            calculations.push(CompliledCalculation {
                expr: template_ast,
                new_attribute: calculation.new_attribute.clone(),
            });
        }

        let process = StatisticsCalculator {
            aggregate_name: params.aggregate_name,
            aggregate_attribute: params.aggregate_attribute,
            calculations,
            aggregate_buffer: HashMap::new(),
        };
        Ok(Box::new(process))
    }
}

#[derive(Debug, Clone)]
pub struct StatisticsCalculator {
    aggregate_name: Attribute,
    aggregate_attribute: Option<Attribute>,
    calculations: Vec<CompliledCalculation>,
    aggregate_buffer: HashMap<Attribute, HashMap<String, i64>>,
}

#[derive(Debug, Clone)]
struct CompliledCalculation {
    new_attribute: Attribute,
    expr: rhai::AST,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatisticsCalculatorParam {
    aggregate_name: Attribute,
    aggregate_attribute: Option<Attribute>,
    calculations: Vec<Calculation>,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Calculation {
    new_attribute: Attribute,
    expr: Expr,
}

impl Processor for StatisticsCalculator {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        2
    }

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
        let aggregate = self
            .aggregate_attribute
            .clone()
            .map(|attr| {
                let Some(value) = feature.attributes.get(&attr) else {
                    return "undefined".to_string();
                };
                let AttributeValue::String(value) = value else {
                    return "undefined".to_string();
                };
                value.to_string()
            })
            .unwrap_or("all".to_string());

        for calculation in &self.calculations {
            let aggregate_buffer = self
                .aggregate_buffer
                .entry(calculation.new_attribute.clone())
                .or_default();
            let content = aggregate_buffer.entry(aggregate.clone()).or_default();
            let eval = scope.eval_ast::<i64>(&calculation.expr);
            match eval {
                Ok(eval) => {
                    *content += eval;
                }
                _ => {
                    continue;
                }
            }
        }
        fw.send(ctx.new_with_feature_and_port(feature.clone(), COMPLETE_PORT.clone()));
        Ok(())
    }

    fn finish(
        &self,
        ctx: NodeContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        let mut features = HashMap::<String, HashMap<Attribute, i64>>::new();
        for (new_attribute, value) in &self.aggregate_buffer {
            for (attr, count) in value {
                let current = features
                    .entry(attr.to_string())
                    .or_default()
                    .entry(new_attribute.clone())
                    .or_default();
                *current += count;
            }
        }
        for (attr, value) in features {
            let mut feature = Feature::new();
            feature.insert(self.aggregate_name.clone(), AttributeValue::String(attr));
            for (new_attribute, count) in value {
                feature.insert(
                    new_attribute.clone(),
                    AttributeValue::Number(serde_json::Number::from(count)),
                );
            }
            fw.send(ExecutorContext::new_with_node_context_feature_and_port(
                &ctx,
                feature,
                DEFAULT_PORT.clone(),
            ));
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "StatisticsCalculator"
    }
}
