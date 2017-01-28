use ::error::ProtocolResult;

use super::protocol;
use super::data;

pub fn load_transform_from_reader(transform_reader: protocol::transform::Reader) -> ProtocolResult<data::Transform> {
    let transform_reader = transform_reader.get_transform();

    let transform = match try_throw!(transform_reader.which()) {
        protocol::transform::transform::Translation(translation) => {
            data::Transform::Translation(try_throw!(translation).get_vector())
        },
        protocol::transform::transform::Rotation(rotation) => {
            data::Transform::Rotation(try_throw!(rotation).get_vector())
        },
        protocol::transform::transform::Scale(scale) => {
            data::Transform::Scale(try_throw!(scale).get_vector())
        },
        protocol::transform::transform::Matrix(matrix) => {
            data::Transform::Matrix(try_throw!(matrix).get_matrix())
        },
    };

    Ok(transform)
}

pub fn save_transform_to_builder(transform_builder: protocol::transform::Builder, transform: &data::Transform) -> ProtocolResult<()> {
    let transform_builder = transform_builder.init_transform();

    match *transform {
        data::Transform::Translation(ref translation) => {
            transform_builder.init_translation().set_vector(translation);
        },
        data::Transform::Rotation(ref rotation) => {
            transform_builder.init_rotation().set_vector(rotation);
        },
        data::Transform::Scale(ref scale) => {
            transform_builder.init_scale().set_vector(scale);
        },
        data::Transform::Matrix(ref matrix) => {
            transform_builder.init_matrix().set_matrix(matrix);
        },
    }

    Ok(())
}