use ::error::ProtocolResult;

/// Trait that allows for storage mediums to be queried for information,
/// usually before any expensive operations are performed.
pub trait StorageQuery: Clone {
    /// Any arguments that can be passed to the query function
    type Arguments: Clone;
    /// Result of the query
    type Result: Clone;
}

/// Default implementation for storage mediums that are not able to be queried.
impl StorageQuery for () {
    type Arguments = ();
    type Result = ();
}

pub trait Storage<'a> where Self: Sized {
    /// Protocol builder type
    type Builder: 'a;
    /// Protocol reader type
    type Reader: 'a;

    /// Any arguments that can be passed to the loading routines
    type LoadArgs: Clone + 'a;
    /// Any arguments that can be passed to the saving routines
    type SaveArgs: Clone + 'a;
    /// Query types for storage queries
    type Query: StorageQuery + 'a;

    /// Load Storage from associated reader with the given arguments
    fn load_from_reader_args(reader: Self::Reader, args: Self::LoadArgs) -> ProtocolResult<Self>;

    /// Load Storage from associated reader with default arguments
    fn load_from_reader(reader: Self::Reader) -> ProtocolResult<Self> where Self::LoadArgs: Default {
        Self::load_from_reader_args(reader, Default::default())
    }

    /// Save Storage to associated builder with the given arguments
    fn save_to_builder_args(&self, builder: Self::Builder, args: Self::SaveArgs) -> ProtocolResult<()>;

    /// Save Storage to associated builder with default arguments
    fn save_to_builder(&self, builder: Self::Builder) -> ProtocolResult<()> where Self::SaveArgs: Default {
        self.save_to_builder_args(builder, Default::default())
    }

    /// Query the storage medium for information
    ///
    /// # Panics
    ///
    /// Panics with `unimplemented!()` if called on storage mediums without query support
    fn query_reader_args(reader: Self::Reader, args: <Self::Query as StorageQuery>::Arguments) -> ProtocolResult<<Self::Query as StorageQuery>::Result>;

    /// Queries the storage medium with default arguments
    ///
    /// # Panics
    ///
    /// Panics with `unimplemented!()` if called on storage mediums without query support
    fn query_reader(reader: Self::Reader) -> ProtocolResult<<Self::Query as StorageQuery>::Result> where <Self::Query as StorageQuery>::Arguments: Default {
        Self::query_reader_args(reader, Default::default())
    }
}