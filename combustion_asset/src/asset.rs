//! The primary `Asset` trait and data structures

use std::path::Path;
use std::sync::Arc;

use common::streams::BoxedStream;

use ::error::AssetResult;
use ::vfs::BoxedVFS;

/// Tells the asset save/load routines where the data is coming from
#[derive(Debug, Clone)]
pub enum AssetMedium<'a> {
    /// Some file-like stream with a specific path on the given virtual filesystem
    File(&'a Path, Arc<BoxedVFS>),
    /// An in-memory data stream
    Memory(Arc<BoxedStream>),
}

/// Defines a query to an asset,
/// allowing calling components to check if a certain asset can fulfil some requirement.
///
/// Usually the pattern is to implement this for either the argument or result types
pub trait AssetQuery: Clone {
    /// Argument type for the query
    type Arguments: Clone;
    /// Result type of the query
    type Result: Clone;
}

impl AssetQuery for () {
    type Arguments = ();
    type Result = ();
}

/// Defines an asset which can be loaded and saved from/to a data stream
pub trait Asset<'a> where Self: Sized {
    /// Argument type to pass to the load routine
    type LoadArgs: Clone + 'a;
    /// Argument type to pass to the save routine
    type SaveArgs: Clone + 'a;

    /// Query type
    type Query: AssetQuery + 'a;

    /// Load the asset from the given reader
    fn load(medium: AssetMedium<'a>, args: Self::LoadArgs) -> AssetResult<Self>;

    /// Save the asset to the given writer
    fn save(&self, medium: AssetMedium<'a>, args: Self::SaveArgs) -> AssetResult<()>;

    /// Query the asset type for something
    fn query(query: < Self::Query as AssetQuery >::Arguments) -> AssetResult<< Self::Query as AssetQuery >::Result>;
}