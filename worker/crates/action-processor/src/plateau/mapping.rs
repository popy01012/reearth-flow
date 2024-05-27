use std::collections::HashMap;

use once_cell::sync::Lazy;
use reearth_flow_runtime::node::{NodeKind, ProcessorFactory};

use super::{
    dictionaries_initiator::DictionariesInitiatorFactory,
    domain_of_definition_validator::DomainOfDefinitionValidatorFactory,
    udx_folder_extractor::UdxFolderExtractorFactory,
    xml_attribute_extractor::XmlAttributeExtractorFactory,
};

pub static ACTION_MAPPINGS: Lazy<HashMap<String, NodeKind>> = Lazy::new(|| {
    let factories: Vec<Box<dyn ProcessorFactory>> = vec![
        Box::<UdxFolderExtractorFactory>::default(),
        Box::<DomainOfDefinitionValidatorFactory>::default(),
        Box::<DictionariesInitiatorFactory>::default(),
        Box::<XmlAttributeExtractorFactory>::default(),
    ];
    factories
        .into_iter()
        .map(|f| (f.name().to_string(), NodeKind::Processor(f)))
        .collect::<HashMap<_, _>>()
});