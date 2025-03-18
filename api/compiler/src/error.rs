use thiserror::Error;


pub type Result<T> = std::result::Result<T,Error>;

#[derive(Debug,Error)]
pub enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	SerdeYaml(#[from] serde_yaml::Error)
}