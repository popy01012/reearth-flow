use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    str::FromStr,
    sync::Arc,
};

use once_cell::sync::Lazy;
use reearth_flow_common::{
    uri::{Uri, PROTOCOL_SEPARATOR},
    xml::{self, XmlDocument, XmlRoNamespace},
};
use reearth_flow_runtime::{
    channels::ProcessorChannelForwarder,
    errors::BoxedError,
    event::EventHub,
    executor_operation::{ExecutorContext, NodeContext},
    node::{Port, Processor, ProcessorFactory, DEFAULT_PORT},
};
use reearth_flow_types::{Attribute, AttributeValue, Feature};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::errors::{Result, XmlProcessorError};

static SUCCESS_PORT: Lazy<Port> = Lazy::new(|| Port::new("success"));
static FAILED_PORT: Lazy<Port> = Lazy::new(|| Port::new("failed"));

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct ValidationResult {
    error_type: String,
    message: String,
    line: Option<i32>,
    col: Option<i32>,
}

impl ValidationResult {
    fn new(error_type: &str, message: &str) -> Self {
        ValidationResult {
            error_type: error_type.to_string(),
            message: message.to_string(),
            line: None,
            col: None,
        }
    }

    fn new_with_line_and_col(
        error_type: &str,
        message: &str,
        line: Option<i32>,
        col: Option<i32>,
    ) -> Self {
        ValidationResult {
            error_type: error_type.to_string(),
            message: message.to_string(),
            line,
            col,
        }
    }
}

impl From<ValidationResult> for HashMap<String, AttributeValue> {
    fn from(result: ValidationResult) -> Self {
        let mut map = HashMap::new();
        map.insert(
            "errorType".to_string(),
            AttributeValue::String(result.error_type),
        );
        map.insert(
            "message".to_string(),
            AttributeValue::String(result.message),
        );
        map
    }
}

#[derive(Debug, Clone, Default)]
pub struct XmlValidatorFactory;

impl ProcessorFactory for XmlValidatorFactory {
    fn name(&self) -> &str {
        "XMLValidator"
    }

    fn description(&self) -> &str {
        "Validates XML content"
    }

    fn parameter_schema(&self) -> Option<schemars::schema::RootSchema> {
        Some(schemars::schema_for!(XmlValidatorParam))
    }

    fn categories(&self) -> &[&'static str] {
        &["PLATEAU"]
    }

    fn get_input_ports(&self) -> Vec<Port> {
        vec![DEFAULT_PORT.clone()]
    }

    fn get_output_ports(&self) -> Vec<Port> {
        vec![SUCCESS_PORT.clone(), FAILED_PORT.clone()]
    }

    fn build(
        &self,
        _ctx: NodeContext,
        _event_hub: EventHub,
        _action: String,
        with: Option<HashMap<String, Value>>,
    ) -> Result<Box<dyn Processor>, BoxedError> {
        let params: XmlValidatorParam = if let Some(with) = with {
            let value: Value = serde_json::to_value(with).map_err(|e| {
                XmlProcessorError::ValidatorFactory(format!("Failed to serialize with: {}", e))
            })?;
            serde_json::from_value(value).map_err(|e| {
                XmlProcessorError::ValidatorFactory(format!("Failed to deserialize with: {}", e))
            })?
        } else {
            return Err(XmlProcessorError::ValidatorFactory(
                "Missing required parameter `with`".to_string(),
            )
            .into());
        };

        let process = XmlValidator {
            params,
            schema_store: Arc::new(parking_lot::RwLock::new(HashMap::new())),
        };
        Ok(Box::new(process))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
enum XmlInputType {
    File,
    Text,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
enum ValidationType {
    Syntax,
    SyntaxAndNamespace,
    SyntaxAndSchema,
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct XmlValidatorParam {
    attribute: Attribute,
    input_type: XmlInputType,
    validation_type: ValidationType,
}

#[derive(Clone)]
pub struct XmlValidator {
    params: XmlValidatorParam,
    schema_store: Arc<parking_lot::RwLock<HashMap<String, xml::XmlSchemaValidationContext>>>,
}

impl Debug for XmlValidator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("XmlValidator")
            .field("params", &self.params)
            .finish()
    }
}

impl Processor for XmlValidator {
    fn initialize(&mut self, _ctx: NodeContext) {}

    fn num_threads(&self) -> usize {
        20
    }

    fn process(
        &mut self,
        ctx: ExecutorContext,
        fw: &mut dyn ProcessorChannelForwarder,
    ) -> Result<(), BoxedError> {
        match self.params.validation_type {
            ValidationType::Syntax => {
                let feature = &ctx.feature;
                let xml_content = self.get_xml_content(&ctx, feature)?;
                let Ok(document) = xml::parse(xml_content) else {
                    let mut feature = feature.clone();
                    feature.attributes.insert(
                        Attribute::new("xmlError"),
                        AttributeValue::Array(vec![AttributeValue::Map(
                            ValidationResult::new("SyntaxError", "Invalid document structure")
                                .into(),
                        )]),
                    );
                    fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                    return Ok(());
                };
                let Ok(_) = xml::get_root_node(&document) else {
                    let mut feature = feature.clone();
                    feature.attributes.insert(
                        Attribute::new("xmlError"),
                        AttributeValue::Array(vec![AttributeValue::Map(
                            ValidationResult::new("SyntaxError", "Invalid document structure")
                                .into(),
                        )]),
                    );
                    fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                    return Ok(());
                };
                fw.send(ctx.new_with_feature_and_port(feature.clone(), SUCCESS_PORT.clone()));
            }
            ValidationType::SyntaxAndNamespace => {
                let feature = &ctx.feature;
                let xml_content = self.get_xml_content(&ctx, feature).unwrap();
                let document = match xml::parse(xml_content) {
                    Ok(doc) => doc,
                    Err(_) => {
                        let mut feature = feature.clone();
                        feature.attributes.insert(
                            Attribute::new("xmlError"),
                            AttributeValue::Array(vec![AttributeValue::Map(
                                ValidationResult::new("SyntaxError", "Invalid document structure")
                                    .into(),
                            )]),
                        );
                        fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                        return Ok(());
                    }
                };
                let root_node = match xml::get_root_readonly_node(&document) {
                    Ok(node) => node,
                    Err(_) => {
                        let mut feature = feature.clone();
                        feature.attributes.insert(
                            Attribute::new("xmlError"),
                            AttributeValue::Array(vec![AttributeValue::Map(
                                ValidationResult::new("SyntaxError", "Invalid document structure")
                                    .into(),
                            )]),
                        );
                        fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                        return Ok(());
                    }
                };
                let namespaces: Vec<XmlRoNamespace> = root_node
                    .get_namespace_declarations()
                    .into_iter()
                    .map(|ns| ns.into())
                    .collect::<Vec<_>>();
                let result = recursive_check_namespace(root_node, &namespaces);
                if result.is_empty() {
                    fw.send(ctx.new_with_feature_and_port(feature.clone(), SUCCESS_PORT.clone()));
                } else {
                    let mut feature = feature.clone();
                    feature.attributes.insert(
                        Attribute::new("xmlError"),
                        AttributeValue::Array(
                            result
                                .into_iter()
                                .map(|r| AttributeValue::Map(r.into()))
                                .collect::<Vec<_>>(),
                        ),
                    );
                    fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                }
            }
            ValidationType::SyntaxAndSchema => {
                let feature = &ctx.feature;
                let xml_content = self.get_xml_content(&ctx, feature)?;
                let Ok(document) = xml::parse(xml_content) else {
                    let mut feature = feature.clone();
                    feature.attributes.insert(
                        Attribute::new("xmlError"),
                        AttributeValue::Array(vec![AttributeValue::Map(
                            ValidationResult::new("SyntaxError", "Invalid document structure")
                                .into(),
                        )]),
                    );
                    fw.send(ctx.new_with_feature_and_port(feature.clone(), FAILED_PORT.clone()));
                    return Ok(());
                };
                if let Ok(result) = self.check_schema(feature, &ctx, &document) {
                    if result.is_empty() {
                        fw.send(
                            ctx.new_with_feature_and_port(feature.clone(), SUCCESS_PORT.clone()),
                        );
                    } else {
                        let mut feature = feature.clone();
                        feature.attributes.insert(
                            Attribute::new("xmlError"),
                            AttributeValue::Array(
                                result
                                    .into_iter()
                                    .map(|r| AttributeValue::Map(r.into()))
                                    .collect::<Vec<_>>(),
                            ),
                        );
                        fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                    }
                    return Ok(());
                } else {
                    let mut feature = feature.clone();
                    feature.attributes.insert(
                        Attribute::new("xmlError"),
                        AttributeValue::Array(vec![AttributeValue::Map(
                            ValidationResult::new("SyntaxError", "Invalid document structure")
                                .into(),
                        )]),
                    );
                    fw.send(ctx.new_with_feature_and_port(feature, FAILED_PORT.clone()));
                }
            }
        }
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
        "XMLValidator"
    }
}

impl XmlValidator {
    fn get_base_path(&self, feature: &Feature) -> String {
        match self.params.input_type {
            XmlInputType::File => feature
                .attributes
                .get(&self.params.attribute)
                .and_then(|v| {
                    if let AttributeValue::String(s) = v {
                        match Uri::from_str(s) {
                            Ok(uri) => {
                                if uri.is_dir() {
                                    Some(uri.to_string())
                                } else if let Some(parent) = uri.parent() {
                                    Some(parent.to_string())
                                } else {
                                    Some("".to_string())
                                }
                            }
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or("".to_string()),
            XmlInputType::Text => "".to_string(),
        }
    }

    fn get_xml_content(&self, ctx: &ExecutorContext, feature: &Feature) -> Result<String> {
        match self.params.input_type {
            XmlInputType::File => {
                let uri = feature
                    .attributes
                    .get(&self.params.attribute)
                    .ok_or(XmlProcessorError::Validator("Required Uri".to_string()))?;
                let uri = match uri {
                    AttributeValue::String(s) => Uri::from_str(s)
                        .map_err(|_| XmlProcessorError::Validator("Invalid URI".to_string()))?,
                    _ => {
                        return Err(XmlProcessorError::Validator(
                            "Invalid Attribute".to_string(),
                        ))
                    }
                };
                let storage = ctx
                    .storage_resolver
                    .resolve(&uri)
                    .map_err(|e| XmlProcessorError::Validator(format!("{:?}", e)))?;
                let content = storage
                    .get_sync(uri.path().as_path())
                    .map_err(|e| XmlProcessorError::Validator(format!("{:?}", e)))?;
                String::from_utf8(content.to_vec())
                    .map_err(|_| XmlProcessorError::Validator("Invalid UTF-8".to_string()))
            }
            XmlInputType::Text => {
                let content = feature
                    .attributes
                    .get(&self.params.attribute)
                    .ok_or(XmlProcessorError::Validator("No Attribute".to_string()))?;
                let content = match content {
                    AttributeValue::String(s) => s,
                    _ => {
                        return Err(XmlProcessorError::Validator(
                            "Invalid Attribute".to_string(),
                        ))
                    }
                };
                Ok(content.to_string())
            }
        }
    }

    fn check_schema(
        &self,
        feature: &Feature,
        _ctx: &ExecutorContext,
        document: &XmlDocument,
    ) -> Result<Vec<ValidationResult>> {
        let schema_locations = xml::parse_schema_locations(document)
            .map_err(|e| XmlProcessorError::Validator(format!("{:?}", e)))?;
        let target_locations = schema_locations
            .difference(&HashSet::from_iter(
                self.schema_store.read().keys().cloned(),
            ))
            .cloned()
            .collect::<Vec<_>>();
        if !target_locations.is_empty() {
            for location in target_locations {
                let target = if !location.contains(PROTOCOL_SEPARATOR) && !location.starts_with('/')
                {
                    format!("{}/{}", self.get_base_path(feature), location.clone())
                } else {
                    location.clone()
                };
                if target.is_empty() {
                    continue;
                }
                let schema_context = match xml::create_xml_schema_validation_context(target) {
                    Ok(ctx) => ctx,
                    Err(_) => {
                        continue;
                    }
                };
                self.schema_store.write().insert(location, schema_context);
            }
        }
        let mut store = self.schema_store.write();
        let mut result = Vec::new();
        for location in schema_locations {
            let location_store = store.get_mut(&location);
            let schema_context = match location_store {
                Some(ctx) => ctx,
                None => continue,
            };
            match xml::validate_document_by_schema_context(document, schema_context) {
                Ok(r) => {
                    r.iter().for_each(|v| {
                        let message = v.message.clone().unwrap_or_default();
                        result.push(ValidationResult::new_with_line_and_col(
                            "SchemaError",
                            &message,
                            v.line,
                            v.col,
                        ));
                    });
                }
                Err(e) => {
                    result.push(ValidationResult::new(
                        "SchemaError",
                        &format!("{:?}", e).to_string(),
                    ));
                }
            }
        }
        Ok(result)
    }
}

fn recursive_check_namespace(
    node: xml::XmlRoNode,
    namespaces: &Vec<XmlRoNamespace>,
) -> Vec<ValidationResult> {
    let mut result = Vec::new();
    match node.get_namespace() {
        Some(ns) => {
            if !namespaces.iter().any(|n| n.get_prefix() == ns.get_prefix()) {
                result.push(ValidationResult::new(
                    "NamespaceError",
                    format!("No namespace declaration for {}", ns.get_prefix()).as_str(),
                ));
            }
        }
        None => {
            let tag = xml::get_readonly_node_tag(&node);
            if tag.contains(':') {
                let prefix = tag.split(':').collect::<Vec<&str>>()[0];
                if !namespaces.iter().any(|n| n.get_prefix() == prefix) {
                    result.push(ValidationResult::new(
                        "NamespaceError",
                        format!("No namespace declaration for {}", prefix).as_str(),
                    ));
                }
            } else {
                result.push(ValidationResult::new(
                    "NamespaceError",
                    "No namespace declaration",
                ));
            }
        }
    };
    let child_node = node.get_child_nodes();
    let child_nodes = child_node
        .into_iter()
        .filter(|n| {
            if let Some(typ) = n.get_type() {
                typ == xml::XmlNodeType::ElementNode
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    for child in child_nodes {
        let child_result = recursive_check_namespace(child, namespaces);
        result.extend(child_result);
    }
    result
}
