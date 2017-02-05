//! The primary `Asset` trait and data structures

use std::io::prelude::*;
use std::path::Path;

use ::error::AssetResult;

/// Tells the asset save/load routines where the data is coming from
#[derive(Debug, Clone, Copy)]
pub enum AssetMedium<'a> {
    /// The asset is in a file at this path
    File(&'a Path),
    /// The asset is in memory, so no filepath can be given
    Memory,
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
    fn load<R: BufRead + Seek, T: AsMut<R>>(reader: T, medium: AssetMedium<'a>, args: Self::LoadArgs) -> AssetResult<Self>;

    /// Save the asset to the given writer
    fn save<W: Write, T: AsMut<W>>(&self, writer: T, medium: AssetMedium<'a>, args: Self::SaveArgs) -> AssetResult<()>;

    /// Query the asset type for something
    fn query(query: < Self::Query as AssetQuery >::Arguments) -> AssetResult<< Self::Query as AssetQuery >::Result>;
}