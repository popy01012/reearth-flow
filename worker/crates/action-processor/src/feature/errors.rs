use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub(super) enum FeatureProcessorError {
    #[error("Feature Merger Factory error: {0}")]
    MergerFactory(String),
    #[error("Feature Merger error: {0}")]
    Merger(String),
    #[error("Feature Sorter Factory error: {0}")]
    SorterFactory(String),
    #[error("Feature Sorter error: {0}")]
    Sorter(String),
    #[error("Feature Filter Factory error: {0}")]
    FilterFactory(String),
    #[error("Feature Filter error: {0}")]
    Filter(String),
    #[error("Feature Transformer Factory error: {0}")]
    TransformerFactory(String),
    #[error("Feature Transformer error: {0}")]
    Transformer(String),
    #[error("Feature Counter Factory error: {0}")]
    CounterFactory(String),
    #[error("Feature Counter error: {0}")]
    Counter(String),
}

#[allow(dead_code)]
pub(super) type Result<T, E = FeatureProcessorError> = std::result::Result<T, E>;