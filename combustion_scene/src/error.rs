use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use specs::Entity;

pub type SceneResult<T> = Result<T, SceneError>;

#[derive(Debug)]
pub enum SceneError {
    MissingChild(Entity),
    InvalidNode,
    InvalidEdge,
}

impl Display for SceneError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for SceneError {
    fn description(&self) -> &str {
        match *self {
            SceneError::MissingChild(_) => "Missing Child",
            SceneError::InvalidNode => "Invalid Node",
            SceneError::InvalidEdge => "Invalid Edge",
        }
    }
}