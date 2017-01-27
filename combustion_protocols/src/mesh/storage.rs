use ::error::ProtocolResult;
use ::utils;

use super::protocol;
use super::data;

/// Routine to load in a Mesh from a mesh Reader
fn load_mesh_from_reader(mesh_reader: &protocol::mesh::Reader) -> ProtocolResult<data::Mesh> {
    let vertices_reader = mesh_reader.get_vertices();

    let indices_option = try_throw!(mesh_reader.get_indices());

    let indices = match try_throw!(indices_option.which()) {
        utils::protocol::option::Some(indices) => {
            Some(try_throw!(indices).iter().collect())
        },
        _ => None,
    };

    let materials_raw = try_throw!(mesh_reader.get_materials());

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

                interleaved.push(data::Vertex {
                    position: position.get_point(),
                    normal: {
                        match try_throw!(normal.which()) {
                            utils::protocol::option::Some(normal) => {
                                Some(try_throw!(normal).get_vector())
                            },
                            _ => None,
                        }
                    },
                    uv: {
                        match try_throw!(uv.which()) {
                            utils::protocol::option::Some(uv) => {
                                let uv = try_throw!(uv);

                                Some(data::TexCoord {
                                    u: uv.get_u(),
                                    v: uv.get_v(),
                                })
                            },
                            _ => None
                        }
                    }
                })
            }

            data::MeshVertices::Interleaved(interleaved)
        },
        protocol::mesh::vertices::Discrete(vertices) => {
            let vertices = try_throw!(vertices);

            let raw_positions = try_throw!(vertices.get_positions());
            let raw_normals_option = try_throw!(vertices.get_normals());
            let raw_uvs_option = try_throw!(vertices.get_uvs());

            data::MeshVertices::Discrete(data::Vertices {
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
                                uvs.push(data::TexCoord {
                                    u: uv.get_u(),
                                    v: uv.get_v(),
                                })
                            }

                            Some(uvs)
                        },
                        _ => None,
                    }
                }
            })
        }
    };

    Ok(data::Mesh {
        vertices: vertices,
        indices: indices,
        materials: materials,
    })
}