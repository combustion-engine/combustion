use ::error::ProtocolResult;

use ::math::storage::{load_transform_from_reader, save_transform_to_builder};
use ::mesh::storage::{load_mesh_from_reader, save_mesh_to_builder};

use super::protocol;
use super::data;

pub fn load_node_from_reader(node_reader: protocol::node::Reader) -> ProtocolResult<data::Node> {
    let raw_name = try_throw!(node_reader.get_name());
    let raw_meshes = try_throw!(node_reader.get_meshes());
    let raw_children = try_throw!(node_reader.get_children());
    let raw_transforms = try_throw!(node_reader.get_transforms());

    let mut children = Vec::with_capacity(raw_children.len() as usize);

    for child in raw_children.iter() {
        children.push(try_rethrow!(load_node_from_reader(child)));
    }

    let mut transforms = Vec::with_capacity(raw_transforms.len() as usize);

    for transform in raw_transforms.iter() {
        transforms.push(try_rethrow!(load_transform_from_reader(transform)))
    }

    let node = data::Node {
        name: raw_name.to_string(),
        meshes: raw_meshes.iter().collect(),
        children: children,
        transforms: transforms,
    };

    Ok(node)
}

pub fn load_model_from_reader(model_reader: protocol::model::Reader) -> ProtocolResult<data::Model> {
    let raw_root = try_throw!(model_reader.get_root());
    let raw_meshes = try_throw!(model_reader.get_meshes());
    let raw_materials = try_throw!(model_reader.get_materials());

    let mut meshes = Vec::with_capacity(raw_meshes.len() as usize);

    for mesh in raw_meshes.iter() {
        meshes.push(try_rethrow!(load_mesh_from_reader(mesh)));
    }

    let root = try_rethrow!(load_node_from_reader(raw_root));

    let mut materials = Vec::with_capacity(raw_materials.len() as usize);

    for material in raw_materials.iter() {
        materials.push(try_throw!(material).into());
    }

    let model = data::Model {
        meshes: meshes,
        root: root,
        materials: materials,
    };

    Ok(model)
}

pub fn load_model_from_root_reader(root_model_reader: protocol::root_model::Reader) -> ProtocolResult<data::Model> {
    let model_reader = try_throw!(root_model_reader.get_model());

    load_model_from_reader(model_reader)
}

////////////////////////////////////

pub fn save_node_to_builder(mut node_builder: protocol::node::Builder, node: &data::Node) -> ProtocolResult<()> {
    {
        let mut children_list_builder = node_builder.borrow().init_children(node.children.len() as u32);

        for (i, child_node) in node.children.iter().enumerate() {
            let child_node_builder = children_list_builder.borrow().get(i as u32);

            try_rethrow!(save_node_to_builder(child_node_builder, child_node));
        }
    }

    {
        let mut mesh_list_builder = node_builder.borrow().init_meshes(node.meshes.len() as u32);

        for (i, mesh) in node.meshes.iter().enumerate() {
            mesh_list_builder.set(i as u32, *mesh);
        }
    }

    {
        let mut transform_list_builder = node_builder.borrow().init_transforms(node.transforms.len() as u32);

        for (i, transform) in node.transforms.iter().enumerate() {
            let transform_builder = transform_list_builder.borrow().get(i as u32);

            try_rethrow!(save_transform_to_builder(transform_builder, transform));
        }
    }

    node_builder.set_name(node.name.as_str());

    Ok(())
}

pub fn save_model_to_builder(mut model_builder: protocol::model::Builder, model: &data::Model, raw: bool) -> ProtocolResult<()> {
    {
        let mut mesh_list_builder = model_builder.borrow().init_meshes(model.meshes.len() as u32);

        for (i, mesh) in model.meshes.iter().enumerate() {
            let mesh_builder = mesh_list_builder.borrow().get(i as u32);

            try_rethrow!(save_mesh_to_builder(mesh_builder, mesh, raw));
        }
    }

    { try_rethrow!(save_node_to_builder(model_builder.borrow().init_root(), &model.root)); }

    {
        let mut material_list_builder = model_builder.borrow().init_materials(model.materials.len() as u32);

        for (i, material) in model.materials.iter().enumerate() {
            material_list_builder.set(i as u32, material.as_str());
        }
    }

    Ok(())
}

pub fn save_model_to_root_builder(root_model_builder: protocol::root_model::Builder, model: &data::Model, raw: bool) -> ProtocolResult<()> {
    let model_builder = root_model_builder.init_model();

    save_model_to_builder(model_builder, model, raw)
}