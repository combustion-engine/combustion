use std::io::prelude::*;

use ::error::AssetResult;

pub trait Asset where Self: Sized {
    type LoadArgs: Clone;
    type SaveArgs: Clone;

    fn load<R: Read>(reader: R, args: Self::LoadArgs) -> AssetResult<Self>;
    fn save<W: Write>(&self, writer: W, args: Self::SaveArgs) -> AssetResult<()>;
}