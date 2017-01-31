use ::error::ProtocolResult;

use ::traits::Storage;

use ::math::data::Transform;

use ::mesh::data::Mesh;
use ::mesh::storage::MeshSaveArgs;

use super::protocol;
use super::data::{Node, Model};

#[derive(Debug, Clone, Copy)]
pub struct ModelSaveArgs {
    pub mesh_args: MeshSaveArgs,
}

impl Default for ModelSaveArgs {
    fn default() -> ModelSaveArgs {
        ModelSaveArgs { mesh_args: MeshSaveArgs::default() }
    }
}

impl<'a> Storage<'a> for Node {
    type Builder = protocol::node::Builder<'a>;
    type Reader = protocol::node::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = ();

    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<Self> {
        let raw_name = try_throw!(reader.get_name());
        let raw_meshes = try_throw!(reader.get_meshes());
        let raw_children = try_throw!(reader.get_children());
        let raw_transforms = try_throw!(reader.get_transforms());

        let mut children = Vec::with_capacity(raw_children.len() as usize);

        for child_reader in raw_children.iter() {
            children.push(Node::load_from_reader(child_reader)?);
        }

        let mut transforms = Vec::with_capacity(raw_transforms.len() as usize);

        for transform_reader in raw_transforms.iter() {
            transforms.push(Transform::load_from_reader(transform_reader)?)
        }

        let node = Node {
            name: raw_name.to_string(),
            meshes: raw_meshes.iter().collect(),
            children: children,
            transforms: transforms,
        };

        Ok(node)
    }

    fn save_to_builder_args(&self, mut builder: Self::Builder, _: ()) -> ProtocolResult<()> {
        {
            let mut children_list_builder = builder.borrow().init_children(self.children.len() as u32);

            for (i, child_node) in self.children.iter().enumerate() {
                let child_builder = children_list_builder.borrow().get(i as u32);

                try_rethrow!(child_node.save_to_builder(child_builder));
            }
        }

        {
            let mut mesh_list_builder = builder.borrow().init_meshes(self.meshes.len() as u32);

            for (i, mesh) in self.meshes.iter().enumerate() {
                mesh_list_builder.set(i as u32, *mesh);
            }
        }

        {
            let mut transform_list_builder = builder.borrow().init_transforms(self.transforms.len() as u32);

            for (i, transform) in self.transforms.iter().enumerate() {
                let transform_builder = transform_list_builder.borrow().get(i as u32);

                try_rethrow!(transform.save_to_builder(transform_builder));
            }
        }

        builder.set_name(self.name.as_str());

        Ok(())
    }
}

impl<'a> Storage<'a> for Model {
    type Builder = protocol::model::Builder<'a>;
    type Reader = protocol::model::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = ModelSaveArgs;

    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<Self> {
        let raw_root = try_throw!(reader.get_root());
        let raw_meshes = try_throw!(reader.get_meshes());
        let raw_materials = try_throw!(reader.get_materials());

        let mut meshes = Vec::with_capacity(raw_meshes.len() as usize);

        for mesh_reader in raw_meshes.iter() {
            meshes.push(Mesh::load_from_reader(mesh_reader)?);
        }

        let root = Node::load_from_reader(raw_root)?;

        let mut materials = Vec::with_capacity(raw_materials.len() as usize);

        for material in raw_materials.iter() {
            materials.push(try_throw!(material).into());
        }

        let model = Model {
            meshes: meshes,
            root: root,
            materials: materials,
        };

        Ok(model)
    }

    fn save_to_builder_args(&self, mut builder: Self::Builder, args: Self::SaveArgs) -> ProtocolResult<()> {
        {
            let mut mesh_list_builder = builder.borrow().init_meshes(self.meshes.len() as u32);

            for (i, mesh) in self.meshes.iter().enumerate() {
                let mesh_builder = mesh_list_builder.borrow().get(i as u32);

                try_rethrow!(mesh.save_to_builder_args(mesh_builder, args.mesh_args));
            }
        }

        { try_rethrow!(self.root.save_to_builder(builder.borrow().init_root())); }

        {
            let mut material_list_builder = builder.borrow().init_materials(self.materials.len() as u32);

            for (i, material) in self.materials.iter().enumerate() {
                material_list_builder.set(i as u32, material.as_str());
            }
        }

        Ok(())
    }
}