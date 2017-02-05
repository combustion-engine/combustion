//! Model asset

use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use protocols::traits::Storage;
use protocols::model::{protocol, EXTENSION};
use protocols::model::data::{Model, Node};
use protocols::model::storage::ModelSaveArgs;

use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

/// Model Asset
pub struct ModelAsset(Model);

impl<'a> Asset<'a> for ModelAsset {
    type LoadArgs = ();
    type SaveArgs = ();

    type Query = ();

    fn query(_: ()) -> AssetResult<()> {
        unimplemented!()
    }

    fn load<R: BufRead + Seek>(reader: R, medium: AssetMedium<'a>, _: ()) -> AssetResult<ModelAsset> {
        unimplemented!()
    }

    fn save<W: Write>(&self, writer: W, medium: AssetMedium<'a>, _: ()) -> AssetResult<()> {
        unimplemented!()
    }
}