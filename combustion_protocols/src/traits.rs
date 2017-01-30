use ::error::ProtocolResult;

pub trait Storage<'a> where Self: Sized {
    type Builder: 'a;
    type Reader: 'a;

    type LoadArgs: Copy + 'a;
    type SaveArgs: Copy + 'a;

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
}