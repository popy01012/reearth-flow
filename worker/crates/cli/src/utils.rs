use reearth_flow_runtime::node::NodeKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionSchema {
    pub name: String,
    pub r#type: String,
    pub description: String,
    pub parameter: serde_json::Value,
    pub builtin: bool,
    pub input_ports: Vec<String>,
    pub output_ports: Vec<String>,
    pub categories: Vec<String>,
}

impl ActionSchema {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        r#type: String,
        description: String,
        parameter: serde_json::Value,
        builtin: bool,
        input_ports: Vec<String>,
        output_ports: Vec<String>,
        categories: Vec<String>,
    ) -> Self {
        Self {
            name,
            r#type,
            description,
            parameter,
            builtin,
            input_ports,
            output_ports,
            categories,
        }
    }
}

pub fn create_action_schema(kind: &NodeKind, builtin: bool) -> ActionSchema {
    let (name, description, parameter, input_ports, output_ports, categories) = match kind {
        NodeKind::Source(factory) => (
            factory.name().to_string(),
            factory.description().to_string(),
            factory
                .parameter_schema()
                .map_or(serde_json::Value::Null, |schema| {
                    serde_json::from_str(serde_json::to_string(&schema).unwrap().as_str()).unwrap()
                }),
            vec![],
            factory
                .get_output_ports()
                .iter()
                .map(|p| p.to_string())
                .collect(),
            factory.categories().iter().map(|c| c.to_string()).collect(),
        ),
        NodeKind::Processor(factory) => (
            factory.name().to_string(),
            factory.description().to_string(),
            factory
                .parameter_schema()
                .map_or(serde_json::Value::Null, |schema| {
                    serde_json::from_str(serde_json::to_string(&schema).unwrap().as_str()).unwrap()
                }),
            factory
                .get_input_ports()
                .iter()
                .map(|p| p.to_string())
                .collect(),
            factory
                .get_output_ports()
                .iter()
                .map(|p| p.to_string())
                .collect(),
            factory.categories().iter().map(|c| c.to_string()).collect(),
        ),
        NodeKind::Sink(factory) => (
            factory.name().to_string(),
            factory.description().to_string(),
            factory
                .parameter_schema()
                .map_or(serde_json::Value::Null, |schema| {
                    serde_json::from_str(serde_json::to_string(&schema).unwrap().as_str()).unwrap()
                }),
            factory
                .get_input_ports()
                .iter()
                .map(|p| p.to_string())
                .collect(),
            vec![],
            factory.categories().iter().map(|c| c.to_string()).collect(),
        ),
    };

    ActionSchema::new(
        name,
        match kind {
            NodeKind::Source(_) => "source".to_string(),
            NodeKind::Processor(_) => "processor".to_string(),
            NodeKind::Sink(_) => "sink".to_string(),
        },
        description,
        parameter,
        builtin,
        input_ports,
        output_ports,
        categories,
    )
}
