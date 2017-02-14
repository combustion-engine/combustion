//! Model asset implementation

use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::io::BufReader;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use protocols::traits::Storage;
use protocols::model::protocol;
use protocols::model::data::Model;
use protocols::model::storage;

use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery, AssetFileFormat};

use super::formats::ModelFileFormat;

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
    type LoadArgs = ();
    type SaveArgs = ModelAssetSaveArgs;

    type Query = ModelAssetQuery<'a>;

    fn query(query: ModelAssetQuery<'a>) -> AssetResult<bool> {
        Ok(match query {
            ModelAssetQuery::SupportedImportExtension(ext) => {
                match ModelFileFormat::from_extension(ext) {
                    Some(format) if format.can_import() => true,
                    _ => false
                }
            },
            ModelAssetQuery::SupportedExportExtension(ext) => {
                match ModelFileFormat::from_extension(ext) {
                    Some(format) if format.can_export() => true,
                    _ => false
                }
            },
            ModelAssetQuery::SupportedExtension(ext) => {
                match ModelFileFormat::from_extension(ext) {
                    Some(format) if format.can_import() && format.can_export() => true,
                    _ => false
                }
            },
        })
    }

    fn load(medium: AssetMedium<'a>, _: ()) -> AssetResult<ModelAsset> {
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
                    #[cfg(feature = "assimp")]
                    ModelFileFormat::Assimp => {
                        // Use custom IO for Assimp so it can use the virtual filesystem to interact with data
                        let mut io = ::assimp::io::CustomIO::callback(move |path| vfs.open(path));

                        let scene = try_rethrow!(::assimp::Scene::import_from(path, None, &mut io));

                        let model = try_rethrow!(super::external::assimp::scene_to_model(scene));

                        return Ok(ModelAsset(model));
                    },
                    ModelFileFormat::Standard(standard_format) => {
                        let reader = BufReader::new(try_throw!(vfs.open(path)));

                        return ::assets::standard::generic::load_standard_format(reader, standard_format);
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
                    ModelFileFormat::Standard(standard_format) => {
                        let writer = try_throw!(vfs.create_or_truncate(path));

                        return ::assets::standard::generic::save_standard_format(writer, standard_format, self, args.pretty);
                    },
                    #[cfg(feature = "assimp")]
                    ModelFileFormat::Assimp => throw!(AssetError::UnsupportedFormat),
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