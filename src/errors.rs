#[derive(Debug)]
pub enum BundleError {
    NotFound(String),
    InvalidFormat(String),
    IoError(std::io::Error),
    NotLoaded
}

#[derive(Debug)]
pub struct BundleValidationResult {
    pub is_valid: bool,
    pub message: String,
    pub warnings: Vec<String>,
}

#[derive(Debug)]
pub enum BundleValidationError {
    MissingField(String),
    InvalidFormat(String),
    ConfigLoadError(String)
}
