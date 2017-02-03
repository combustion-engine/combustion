#![allow(missing_docs)]

use super::data;

include!(concat!(env!("OUT_DIR"), "/protocols/mesh_capnp.rs"));

impl<'a> tex_coord::Builder<'a> {
    pub fn set_texcoord(&mut self, uv: &data::TexCoord) {
        self.set_u(uv.u);
        self.set_v(uv.v);
    }
}

impl<'a> tex_coord::Reader<'a> {
    pub fn get_texcoord(&self) -> data::TexCoord {
        data::TexCoord {
            u: self.get_u(),
            v: self.get_v(),
        }
    }
}