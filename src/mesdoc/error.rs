use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
	#[error("Invalid selector:'{context}'<{reason}>")]
	InvalidSelector { context: String, reason: String },
	#[error("Call method '{method}' with {error}")]
	MethodOnInvalidSelector { method: String, error: String },
	#[error("Call method '{method}' cause an error: {message}")]
	InvalidTraitMethodCall { method: String, message: String },
}
