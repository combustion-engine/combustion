use std::io::prelude::*;
use std::path::Path;

use ::error::AssetResult;

pub enum AssetMedium<'a> {
    File(&'a Path),
    Memory,
}

pub trait Asset<'a> where Self: Sized {
    type LoadArgs: Clone + 'a;
    type SaveArgs: Clone + 'a;

    fn load<R: BufRead + Seek>(reader: R, medium: AssetMedium<'a>, args: Self::LoadArgs) -> AssetResult<Self>;
    fn save<W: Write>(&self, writer: W, medium: AssetMedium<'a>, args: Self::SaveArgs) -> AssetResult<()>;
}