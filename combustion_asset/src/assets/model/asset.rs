//! Model asset implementation

use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::sync::{Arc, RwLock};
use std::io::BufReader;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use protocols::traits::Storage;
use protocols::model::protocol;
use protocols::model::data::Model;
use protocols::model::storage;

use assimp::{self, Scene};

use ::cache::AssetHashMapCache;
use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

use super::formats::ModelFileFormat;

/// Cache object for Assimp scenes
pub type AssimpSceneCache<'a> = AssetHashMapCache<'a, String, Scene<'a>>;

/// Model Asset queries
#[derive(Debug, Clone, Copy)]
pub enum ModelAssetQuery<'a> {
    /// Check if a file extension for a model file is supported for importing
    SupportedImportExtension(&'a str),
    /// Check if a file extension for a model file is supported for exporting
    SupportedExportExtension(&'a str),
    /// Check if a file extension for a model file is supported for both importing and exporting
    SupportedExtension(&'a str),
}

impl<'a> AssetQuery for ModelAssetQuery<'a> {
    type Arguments = ModelAssetQuery<'a>;
    type Result = bool;
}

/// Arguments for loading models
#[derive(Default, Clone)]
pub struct ModelAssetLoadArgs<'a> {
    /// Assimp scene cache
    pub scene_cache: Arc<RwLock<AssimpSceneCache<'a>>>,
}

unsafe impl<'a> Send for ModelAssetLoadArgs<'a> {}

unsafe impl<'a> Sync for ModelAssetLoadArgs<'a> {}

/// Arguments for model save routines
#[derive(Debug, Default, Clone)]
pub struct ModelAssetSaveArgs {
    /// Arguments for the storage routines
    pub storage_args: storage::ModelSaveArgs,
    /// For serialization formats that support "pretty-printing", pretty-print the data
    pub pretty: bool,
}

/// Model Asset
#[derive(Serialize, Deserialize)]
pub struct ModelAsset(Model);

impl<'a> Asset<'a> for ModelAsset {
    type LoadArgs = ModelAssetLoadArgs<'a>;
    type SaveArgs = ModelAssetSaveArgs;

    type Query = ModelAssetQuery<'a>;

    fn query(query: ModelAssetQuery<'a>) -> AssetResult<bool> {
        match query {
            ModelAssetQuery::SupportedImportExtension(ext) => {
                Ok(if let Some(format) = ModelFileFormat::from_extension(ext) {
                    format.can_import()
                } else { false })
            },
            ModelAssetQuery::SupportedExportExtension(ext) |
            ModelAssetQuery::SupportedExtension(ext) => {
                Ok(if let Some(format) = ModelFileFormat::from_extension(ext) {
                    format.can_export() && format.can_import()
                } else { false })
            },
        }
    }

    fn load(medium: AssetMedium<'a>, _ /*TODO*/: ModelAssetLoadArgs<'a>) -> AssetResult<ModelAsset> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                let format = match ModelFileFormat::from_extension(ext.as_str()) {
                    Some(format) if format.can_import() => format,
                    _ => throw!(AssetError::UnsupportedFormat),
                };

                match format {
                    ModelFileFormat::Native => {
                        let mut reader = BufReader::new(try_throw!(vfs.open(path)));

                        let message_reader = try_throw!(serialize_packed::read_message(&mut reader, ReaderOptions {
                            traversal_limit_in_words: u64::max_value(),
                            nesting_limit: 1024,
                        }));

                        let model_reader = try_throw!(message_reader.get_root::<protocol::model::Reader>());

                        let model = try_rethrow!(Model::load_from_reader(model_reader));

                        return Ok(ModelAsset(model));
                    },
                    ModelFileFormat::Assimp => {
                        // Use custom IO for Assimp so it can use the virtual filesystem to interact with data
                        let mut io = assimp::io::CustomIO::callback(move |path| vfs.open(path));

                        let scene = try_rethrow!(assimp::Scene::import_from(path, None, &mut io));

                        let model = try_rethrow!(::external::assimp::scene_to_model(scene));

                        return Ok(ModelAsset(model));
                    },

                    #[cfg(feature = "bincode")]
                    ModelFileFormat::Bincode => {
                        use bincode::{deserialize_from, SizeLimit};

                        let mut reader = BufReader::new(try_throw!(vfs.open(path)));

                        return Ok(ModelAsset(try_throw!(deserialize_from(&mut reader, SizeLimit::Infinite))));
                    },
                    #[cfg(feature = "json")]
                    ModelFileFormat::Json => {
                        use json::from_reader;

                        let reader = BufReader::new(try_throw!(vfs.open(path)));

                        return Ok(ModelAsset(try_throw!(from_reader(reader))));
                    },
                    #[cfg(feature = "yaml")]
                    ModelFileFormat::Yaml => {
                        use yaml::from_reader;

                        let reader = BufReader::new(try_throw!(vfs.open(path)));

                        return Ok(ModelAsset(try_throw!(from_reader(reader))));
                    },
                    #[cfg(feature = "cbor")]
                    ModelFileFormat::Cbor => {
                        use cbor::from_reader;

                        let reader = BufReader::new(try_throw!(vfs.open(path)));

                        return Ok(ModelAsset(try_throw!(from_reader(reader))));
                    },
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }

    fn save(&self, medium: AssetMedium<'a>, args: ModelAssetSaveArgs) -> AssetResult<()> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                let format = match ModelFileFormat::from_extension(ext.as_str()) {
                    Some(format) if format.can_export() => format,
                    _ => throw!(AssetError::UnsupportedFormat),
                };

                match format {
                    ModelFileFormat::Native => {
                        let mut writer = try_throw!(vfs.create_or_truncate(path));

                        let mut message = ::capnp::message::Builder::new_default();

                        {
                            let model_builder = message.init_root::<protocol::model::Builder>();

                            try_rethrow!(self.0.save_to_builder_args(model_builder, args.storage_args));
                        }

                        try_throw!(serialize_packed::write_message(&mut writer, &message));

                        return Ok(());
                    },
                    #[cfg(feature = "bincode")]
                    ModelFileFormat::Bincode => {
                        use bincode::{serialize_into, SizeLimit};

                        let mut writer = try_throw!(vfs.create_or_truncate(path));

                        try_throw!(serialize_into(&mut writer, &**self, SizeLimit::Infinite));

                        return Ok(());
                    },
                    #[cfg(feature = "json")]
                    ModelFileFormat::Json => {
                        use json::{to_writer, to_writer_pretty};

                        let mut writer = try_throw!(vfs.create_or_truncate(path));

                        if args.pretty {
                            try_throw!(to_writer_pretty(&mut writer, &**self));
                        } else {
                            try_throw!(to_writer(&mut writer, &**self));
                        }

                        return Ok(());
                    },
                    #[cfg(feature = "yaml")]
                    ModelFileFormat::Yaml => {
                        use yaml::to_writer;

                        let mut writer = try_throw!(vfs.create_or_truncate(path));

                        try_throw!(to_writer(&mut writer, &**self));

                        return Ok(());
                    },
                    #[cfg(feature = "cbor")]
                    ModelFileFormat::Cbor => {
                        use cbor::ser::to_writer_packed_sd;

                        let mut writer = try_throw!(vfs.create_or_truncate(path));

                        try_throw!(to_writer_packed_sd(&mut writer, &**self));

                        return Ok(());
                    },
                    _ => throw!(AssetError::UnsupportedFormat),
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }
}

impl Deref for ModelAsset {
    type Target = Model;

    fn deref(&self) -> &Model {
        &self.0
    }
}

impl DerefMut for ModelAsset {
    fn deref_mut(&mut self) -> &mut Model {
        &mut self.0
    }
}