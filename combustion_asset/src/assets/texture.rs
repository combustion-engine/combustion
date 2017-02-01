use std::io::prelude::*;

use image;

use protocols::texture::protocol;
use protocols::texture::data::{texture, format};

use ::error::AssetResult;
use ::traits::Asset;

impl Asset for texture::Texture {
    type LoadArgs = ();
    type SaveArgs = ();

    fn load<R: Read>(reader: R, _: ()) -> AssetResult<Self> {
        unimplemented!()
    }

    fn save<W: Write>(&self, writer: W, _: ()) -> AssetResult<()> {
        unimplemented!()
    }
}