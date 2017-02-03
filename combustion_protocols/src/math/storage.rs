use ::error::ProtocolResult;

use ::traits::{Storage, StorageQuery};

use super::protocol;
use super::data::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformQuery {
    Translation,
    Rotation,
    Scale,
    Matrix,
}

impl StorageQuery for TransformQuery {
    type Arguments = ();
    type Result = TransformQuery;
}

impl<'a> Storage<'a> for Transform {
    type Builder = protocol::transform::Builder<'a>;
    type Reader = protocol::transform::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = ();
    type Query = TransformQuery;

    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<Transform> {
        Ok(match try_throw!(reader.get_transform().which()) {
            protocol::transform::transform::Translation(translation) => {
                Transform::Translation(try_throw!(translation).get_vector())
            },
            protocol::transform::transform::Rotation(rotation) => {
                Transform::Rotation(try_throw!(rotation).get_vector())
            },
            protocol::transform::transform::Scale(scale) => {
                Transform::Scale(try_throw!(scale).get_vector())
            },
            protocol::transform::transform::Matrix(matrix) => {
                Transform::Matrix(try_throw!(matrix).get_matrix())
            },
        })
    }

    fn save_to_builder_args(&self, builder: Self::Builder, _: ()) -> ProtocolResult<()> {
        let transform_builder = builder.init_transform();

        match *self {
            Transform::Translation(ref translation) => {
                transform_builder.init_translation().set_vector(translation);
            },
            Transform::Rotation(ref rotation) => {
                transform_builder.init_rotation().set_vector(rotation);
            },
            Transform::Scale(ref scale) => {
                transform_builder.init_scale().set_vector(scale);
            },
            Transform::Matrix(ref matrix) => {
                transform_builder.init_matrix().set_matrix(matrix);
            },
        }

        Ok(())
    }

    fn query_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<TransformQuery> {
        Ok(match try_throw!(reader.get_transform().which()) {
            protocol::transform::transform::Translation(_) => TransformQuery::Translation,
            protocol::transform::transform::Rotation(_) => TransformQuery::Rotation,
            protocol::transform::transform::Scale(_) => TransformQuery::Scale,
            protocol::transform::transform::Matrix(_) => TransformQuery::Matrix,
        })
    }
}