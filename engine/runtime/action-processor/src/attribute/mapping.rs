use std::collections::HashMap;

use once_cell::sync::Lazy;
use reearth_flow_runtime::node::{NodeKind, ProcessorFactory};

use super::{
    aggregator::AttributeAggregatorFactory, bulk_renamer::BulkAttributeRenamerFactory,
    duplicate_filter::AttributeDuplicateFilterFactory,
    file_path_info_extractor::AttributeFilePathInfoExtractorFactory,
    manager::AttributeManagerFactory, mapper::AttributeMapperFactory,
    statistics_calculator::StatisticsCalculatorFactory,
};

pub static ACTION_FACTORY_MAPPINGS: Lazy<HashMap<String, NodeKind>> = Lazy::new(|| {
    let factories: Vec<Box<dyn ProcessorFactory>> = vec![
        Box::<AttributeMapperFactory>::default(),
        Box::<AttributeManagerFactory>::default(),
        Box::<AttributeAggregatorFactory>::default(),
        Box::<AttributeDuplicateFilterFactory>::default(),
        Box::<AttributeFilePathInfoExtractorFactory>::default(),
        Box::<BulkAttributeRenamerFactory>::default(),
        Box::<StatisticsCalculatorFactory>::default(),
    ];
    factories
        .into_iter()
        .map(|f| (f.name().to_string(), NodeKind::Processor(f)))
        .collect::<HashMap<_, _>>()
});
