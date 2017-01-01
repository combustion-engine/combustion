use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use ecs::Entity;
use petgraph::graph::NodeIndex;
use super::Ix;

pub type SceneResult<T> = Result<T, SceneError>;

#[derive(Debug)]
pub enum SceneError {
    WouldCycle,
    MissingChild(Entity),
    InvalidNode,
    InvalidEdge,
    AlreadyExists(Entity, NodeIndex<Ix>),
}

impl Display for SceneError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for SceneError {
    fn description(&self) -> &str {
        match *self {
            SceneError::WouldCycle => "Relationship Would Cycle",
            SceneError::MissingChild(_) => "Missing Child",
            SceneError::InvalidNode => "Invalid Node",
            SceneError::InvalidEdge => "Invalid Edge",
            SceneError::AlreadyExists(..) => "Entity Already Exists",
        }
    }
}