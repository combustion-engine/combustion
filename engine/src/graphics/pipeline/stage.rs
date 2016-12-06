use ::backend::gl::*;
use ::backend::gl::types::*;

use super::gbuffer::Gbuffer;

pub struct Stage {
    gbuffer: Option<Gbuffer>,
    framebuffer: GLFramebuffer
}

impl Stage {
    pub fn new(width: usize, height: usize, components: Option<&[(GLenum, GLenum)]>) -> GLResult<Stage> {
        if let Some(components) = components {
            let mut framebuffer = try!(GLFramebuffer::new());

            Ok(Stage {
                gbuffer: Some(Gbuffer::new(width, height, &mut framebuffer, components)?),
                framebuffer: framebuffer
            })
        } else {
            Ok(Stage {
                gbuffer: None,
                framebuffer: GLFramebuffer::default()
            })
        }
    }

    #[inline(always)]
    pub fn gbuffer(&self) -> Option<&Gbuffer> { self.gbuffer.as_ref() }

    pub fn set_filter(&mut self, filter: GLTextureFilter) -> GLResult<()> {
        if let Some(mut gbuffer) = self.gbuffer.as_mut() {
            try!(gbuffer.set_filter(filter));
        }

        Ok(())
    }

    pub fn resize(&mut self, width: usize, height: usize) -> GLResult<()> {
        if let Some(mut gbuffer) = self.gbuffer.as_mut() {
            try!(gbuffer.resize(width, height));
        }

        Ok(())
    }

    #[inline(always)]
    pub fn framebuffer(&self) -> &GLFramebuffer { &self.framebuffer }

    pub fn bind(&self) -> GLResult<()> {
        try!(self.framebuffer.bind());

        //if let Some(gbuffer) = self.gbuffer.as_ref() {
        //    try!(gbuffer.bind());
        //}

        Ok(())
    }
}