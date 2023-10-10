use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Link error")]
    LinkError,
}
