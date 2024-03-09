#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error while running action: {0}")]
    InternalRuntime(String),

    #[error("InputError: {0}")]
    Input(String),

    #[allow(dead_code)]
    #[error("OutputError: {0}")]
    Output(String),

    #[error("InputError: {0}")]
    Validate(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[allow(dead_code)]
    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),
}

impl Error {
    pub fn internal_runtime<T: ToString>(message: T) -> Self {
        Self::InternalRuntime(message.to_string())
    }

    pub fn input<T: ToString>(message: T) -> Self {
        Self::Input(message.to_string())
    }

    #[allow(dead_code)]
    pub fn output<T: ToString>(message: T) -> Self {
        Self::Output(message.to_string())
    }

    pub fn validate<T: ToString>(message: T) -> Self {
        Self::Validate(message.to_string())
    }

    #[allow(dead_code)]
    pub fn io(source: std::io::Error) -> Self {
        Self::IO(source)
    }

    #[allow(dead_code)]
    pub fn unsupported_feature<T: ToString>(message: T) -> Self {
        Self::UnsupportedFeature(message.to_string())
    }
}

// implement Eq and PartialEq for Error so that we can compare errors in tests
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::InternalRuntime(a), Self::InternalRuntime(b)) => a == b,
            (Self::Input(a), Self::Input(b)) => a == b,
            (Self::Output(a), Self::Output(b)) => a == b,
            (Self::Validate(a), Self::Validate(b)) => a == b,
            (Self::IO(a), Self::IO(b)) => a.kind() == b.kind(),
            (Self::UnsupportedFeature(a), Self::UnsupportedFeature(b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(
            Error::internal_runtime("hello"),
            Error::internal_runtime("hello")
        );
        assert_eq!(
            Error::io(std::io::Error::new(std::io::ErrorKind::Other, "hello"),),
            Error::io(std::io::Error::new(std::io::ErrorKind::Other, "hello"))
        );
        assert_eq!(
            Error::unsupported_feature("hello"),
            Error::unsupported_feature("hello")
        );
        assert_eq!(Error::input("hello"), Error::input("hello"));
        assert_eq!(Error::output("hello"), Error::output("hello"));
    }

    #[test]
    fn test_ne() {
        assert_ne!(
            Error::internal_runtime("hello"),
            Error::internal_runtime("world")
        );
        assert_ne!(
            Error::io(std::io::Error::new(std::io::ErrorKind::Other, "hello"),),
            Error::io(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "world"
            ),)
        );
        assert_ne!(
            Error::unsupported_feature("hello"),
            Error::unsupported_feature("world")
        );
        assert_ne!(Error::input("hello"), Error::input("world"));
        assert_ne!(Error::output("hello"), Error::output("world"));
    }
}
