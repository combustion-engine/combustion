use std::mem;
use std::slice;

use nalgebra::*;

use ::error::{ProtocolResult, ProtocolError};
use ::utils;

use ::traits::Storage;

use super::protocol;
use super::data::{Mesh, MeshVertices, TexCoord, Vertex, Vertices};

#[derive(Debug, Clone, Copy)]
pub struct MeshSaveArgs {
    pub raw: bool,
}

impl Default for MeshSaveArgs {
    fn default() -> MeshSaveArgs {
        MeshSaveArgs { raw: false }
    }
}

impl<'a> Storage<'a> for Mesh {
    type Builder = protocol::mesh::Builder<'a>;
    type Reader = protocol::mesh::Reader<'a>;

    type LoadArgs = ();
    type SaveArgs = MeshSaveArgs;

    /// Load in a `Mesh` from a mesh `Reader`
    ///
    /// This is expensive for non-raw meshes, but is safe. It basically has to iterate through every single number.
    ///
    /// This is cheap for raw meshes, but is unsafe, obviously. It basically just casts the pointers and copy the data directly.
    fn load_from_reader_args(reader: Self::Reader, _: ()) -> ProtocolResult<Self> {
        let vertices_reader = reader.get_vertices();

        let indices_option = try_throw!(reader.get_indices());

        let indices = match try_throw!(indices_option.which()) {
            utils::protocol::option::Some(indices) => {
                Some(try_throw!(indices).iter().collect())
            },
            _ => None,
        };

        let materials_raw = try_throw!(reader.get_materials());

        let mut materials = Vec::with_capacity(materials_raw.len() as usize);

        for material in materials_raw.iter() {
            materials.push(material);
        }

        let vertices = match try_throw!(vertices_reader.which()) {
            protocol::mesh::vertices::Interleaved(vertices) => {
                let vertices = try_throw!(vertices);

                let mut interleaved = Vec::with_capacity(vertices.len() as usize);

                for vertex in vertices.iter() {
                    let position = try_throw!(vertex.get_position());
                    let normal = try_throw!(vertex.get_normal());
                    let uv = try_throw!(vertex.get_uv());

                    interleaved.push(Vertex {
                        position: position.get_point(),
                        normal: normal.get_vector(),
                        uv: uv.get_texcoord(),
                    })
                }

                MeshVertices::Interleaved(interleaved)
            },
            protocol::mesh::vertices::Discrete(vertices) => {
                let vertices = try_throw!(vertices);

                let raw_positions = try_throw!(vertices.get_positions());
                let raw_normals_option = try_throw!(vertices.get_normals());
                let raw_uvs_option = try_throw!(vertices.get_uvs());

                MeshVertices::Discrete(Vertices {
                    positions: {
                        let mut positions = Vec::with_capacity(raw_positions.len() as usize);

                        for position in raw_positions.iter() {
                            positions.push(position.get_point());
                        }

                        positions
                    },
                    normals: {
                        match try_throw!(raw_normals_option.which()) {
                            utils::protocol::option::Some(raw_normals) => {
                                let raw_normals = try_throw!(raw_normals);

                                let mut normals = Vec::with_capacity(raw_normals.len() as usize);

                                for normal in raw_normals.iter() {
                                    normals.push(normal.get_vector());
                                }

                                Some(normals)
                            },
                            _ => None,
                        }
                    },
                    uvs: {
                        match try_throw!(raw_uvs_option.which()) {
                            utils::protocol::option::Some(raw_uvs) => {
                                let raw_uvs = try_throw!(raw_uvs);

                                let mut uvs = Vec::with_capacity(raw_uvs.len() as usize);

                                for uv in raw_uvs.iter() {
                                    uvs.push(uv.get_texcoord())
                                }

                                Some(uvs)
                            },
                            _ => None,
                        }
                    }
                })
            },
            protocol::mesh::vertices::InterleavedRaw(vertices_data) => {
                let vertices_data = try_throw!(vertices_data);

                let vertex_size = mem::size_of::<Vertex>();
                let vertices_data_len = vertices_data.len() as usize;

                // Check that this is probably even vertex data in the first place
                if vertices_data_len % vertex_size != 0 {
                    throw!(ProtocolError::InvalidLength);
                }

                let num_vertices = vertices_data_len / vertex_size;

                // Coerce to Vertex slice
                let vertices = unsafe { slice::from_raw_parts(vertices_data.as_ptr() as *const Vertex, num_vertices) };

                // Convert into Vec<Vertex>
                MeshVertices::Interleaved(vertices.into())
            },
            protocol::mesh::vertices::DiscreteRaw(vertices) => {
                let vertices = try_throw!(vertices);

                let positions_data = try_throw!(vertices.get_positions());
                let normals_data_option = try_throw!(vertices.get_normals());
                let uvs_data_option = try_throw!(vertices.get_uvs());

                MeshVertices::Discrete(Vertices {
                    positions: {
                        let point_size = mem::size_of::<Point3<f32>>();

                        if positions_data.len() % point_size != 0 {
                            throw!(ProtocolError::InvalidLength);
                        }

                        let num_positions = positions_data.len() / point_size;

                        let positions = unsafe { slice::from_raw_parts(positions_data.as_ptr() as *const Point3<f32>, num_positions) };

                        positions.into()
                    },
                    normals: {
                        match try_throw!(normals_data_option.which()) {
                            utils::protocol::option::Some(normals_data) => {
                                let normals_data = try_throw!(normals_data);

                                let normal_size = mem::size_of::<Vector3<f32>>();

                                if normals_data.len() % normal_size != 0 {
                                    throw!(ProtocolError::InvalidLength);
                                }

                                let num_normals = normals_data.len() / normal_size;

                                let normals = unsafe { slice::from_raw_parts(normals_data.as_ptr() as *const Vector3<f32>, num_normals) };

                                Some(normals.into())
                            },
                            _ => None,
                        }
                    },
                    uvs: {
                        match try_throw!(uvs_data_option.which()) {
                            utils::protocol::option::Some(uvs_data) => {
                                let uvs_data = try_throw!(uvs_data);

                                let uv_size = mem::size_of::<TexCoord>();

                                if uvs_data.len() % uv_size != 0 {
                                    throw!(ProtocolError::InvalidLength);
                                }

                                let num_uvs = uvs_data.len() / uv_size;

                                let uvs = unsafe { slice::from_raw_parts(uvs_data.as_ptr() as *const TexCoord, num_uvs) };

                                Some(uvs.into())
                            },
                            _ => None,
                        }
                    }
                })
            },
        };

        Ok(Mesh {
            vertices: vertices,
            indices: indices,
            materials: materials,
        })
    }

    fn save_to_builder_args(&self, mut builder: Self::Builder, args: MeshSaveArgs) -> ProtocolResult<()> {
        {
            let mut indices_option_builder = builder.borrow().init_indices();

            if let Some(ref indices) = self.indices {
                let mut indices_builder = indices_option_builder.initn_some(indices.len() as u32);

                for (i, index) in indices.iter().enumerate() {
                    indices_builder.set(i as u32, *index);
                }
            } else {
                indices_option_builder.set_none(());
            }
        }

        {
            let mut materials_builder = builder.borrow().init_materials(self.materials.len() as u32);

            for (i, material) in self.materials.iter().enumerate() {
                materials_builder.set(i as u32, *material);
            }
        }

        {
            let mut vertices_builder = builder.borrow().init_vertices();

            match self.vertices {
                MeshVertices::Discrete(ref vertices) if args.raw == false => {
                    let mut discrete_vertices_builder = vertices_builder.init_discrete();

                    // build positions
                    {
                        let mut positions_list_builder = discrete_vertices_builder.borrow().init_positions(vertices.positions.len() as u32);

                        for (i, position) in vertices.positions.iter().enumerate() {
                            positions_list_builder.borrow().get(i as u32).set_point(position);
                        }
                    }

                    // build normals
                    {
                        let mut normals_list_option_builder = discrete_vertices_builder.borrow().init_normals();

                        if let Some(ref normals) = vertices.normals {
                            let mut normals_builder = normals_list_option_builder.initn_some(normals.len() as u32);

                            for (i, normal) in normals.iter().enumerate() {
                                normals_builder.borrow().get(i as u32).set_vector(normal);
                            }
                        } else {
                            normals_list_option_builder.set_none(());
                        }
                    }

                    // build uvs
                    {
                        let mut uvs_list_option_builder = discrete_vertices_builder.borrow().init_uvs();

                        if let Some(ref uvs) = vertices.uvs {
                            let mut uvs_builder = uvs_list_option_builder.initn_some(uvs.len() as u32);

                            for (i, uv) in uvs.iter().enumerate() {
                                uvs_builder.borrow().get(i as u32).set_texcoord(uv);
                            }
                        } else {
                            uvs_list_option_builder.set_none(());
                        }
                    }
                },
                MeshVertices::Interleaved(ref vertices) if args.raw == false => {
                    let mut interleaved_vertices_builder = vertices_builder.init_interleaved(vertices.len() as u32);

                    for (i, vertex) in vertices.iter().enumerate() {
                        let mut vertex_builder = interleaved_vertices_builder.borrow().get(i as u32);

                        { vertex_builder.borrow().init_position().set_point(&vertex.position); }

                        { vertex_builder.borrow().init_normal().set_vector(&vertex.normal); }

                        { vertex_builder.borrow().init_uv().set_texcoord(&vertex.uv); }
                    }
                },
                MeshVertices::Discrete(ref vertices) if args.raw == true => {
                    let mut discrete_raw_vertices_builder = vertices_builder.init_discrete_raw();

                    {
                        discrete_raw_vertices_builder.borrow().set_positions(unsafe {
                            slice::from_raw_parts(vertices.positions.as_ptr() as *const u8,
                                                  vertices.positions.len() * mem::size_of::<Point3<f32>>())
                        });
                    }

                    {
                        let mut normals_data_option_builder = discrete_raw_vertices_builder.borrow().init_normals();

                        if let Some(ref normals) = vertices.normals {
                            try_throw!(normals_data_option_builder.set_some(unsafe {
                                slice::from_raw_parts(normals.as_ptr() as *const u8,
                                                      normals.len() * mem::size_of::<Vector3<f32>>())
                            }));
                        } else {
                            normals_data_option_builder.set_none(());
                        }
                    }

                    {
                        let mut uvs_data_option_builder = discrete_raw_vertices_builder.borrow().init_uvs();

                        if let Some(ref uvs) = vertices.uvs {
                            try_throw!(uvs_data_option_builder.set_some(unsafe {
                                slice::from_raw_parts(uvs.as_ptr() as *const u8,
                                                      uvs.len() * mem::size_of::<TexCoord>())
                            }));
                        } else {
                            uvs_data_option_builder.set_none(());
                        }
                    }
                },
                MeshVertices::Interleaved(ref vertices) if args.raw == true => {
                    vertices_builder.set_interleaved_raw(unsafe {
                        slice::from_raw_parts(vertices.as_ptr() as *const u8, vertices.len() * mem::size_of::<Vertex>())
                    });
                },
                _ => unreachable!()
            }
        }

        Ok(())
    }
}