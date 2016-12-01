use vec_map::VecMap;

use ::backend::gl::*;
use ::backend::gl::types::*;
use ::backend::gl::bindings as glb;

pub struct Gbuffer {
    pub dimensions: (usize, usize),
    pub buffers: VecMap<GLTexture>,
    depth_stencil_buffer: GLRenderbuffer,
}

pub const COLOR_ATTACHMENTS: [GLenum; 32] = [
    glb::COLOR_ATTACHMENT0, glb::COLOR_ATTACHMENT1, glb::COLOR_ATTACHMENT2, glb::COLOR_ATTACHMENT3, glb::COLOR_ATTACHMENT4,
    glb::COLOR_ATTACHMENT5, glb::COLOR_ATTACHMENT6, glb::COLOR_ATTACHMENT7, glb::COLOR_ATTACHMENT8, glb::COLOR_ATTACHMENT9,
    glb::COLOR_ATTACHMENT10, glb::COLOR_ATTACHMENT11, glb::COLOR_ATTACHMENT12, glb::COLOR_ATTACHMENT13, glb::COLOR_ATTACHMENT14,
    glb::COLOR_ATTACHMENT15, glb::COLOR_ATTACHMENT16, glb::COLOR_ATTACHMENT17, glb::COLOR_ATTACHMENT18, glb::COLOR_ATTACHMENT19,
    glb::COLOR_ATTACHMENT20, glb::COLOR_ATTACHMENT21, glb::COLOR_ATTACHMENT22, glb::COLOR_ATTACHMENT23, glb::COLOR_ATTACHMENT24,
    glb::COLOR_ATTACHMENT25, glb::COLOR_ATTACHMENT26, glb::COLOR_ATTACHMENT27, glb::COLOR_ATTACHMENT28, glb::COLOR_ATTACHMENT29,
    glb::COLOR_ATTACHMENT30, glb::COLOR_ATTACHMENT31,
];

impl Gbuffer {
    pub fn new(width: usize, height: usize, mut framebuffer: &mut GLFramebuffer,
               components: &[(GLenum, GLenum)]) -> GLResult<Gbuffer> {
        try!(framebuffer.bind());

        let mut buffers = VecMap::with_capacity(components.len());
        let mut attachments = Vec::with_capacity(components.len());

        for (i, &(format, internal_format)) in components.iter().enumerate() {
            let mut buffer: GLTexture = try!(GLTexture::new(GLTextureKind::Texture2D));

            try!(buffer.load_empty(width, height, format, internal_format));
            try!(buffer.set_filter(GLTextureFilter::Nearest, None));

            let attachment = COLOR_ATTACHMENTS[i];

            unsafe {
                glb::FramebufferTexture2D(glb::FRAMEBUFFER, attachment, glb::TEXTURE_2D, buffer.raw(), 0);
            }

            check_errors!();

            buffers.insert(i, buffer);
            attachments.push(attachment);
        }

        let mut depth_stencil_buffer = try!(GLRenderbuffer::new());

        try!(depth_stencil_buffer.set_storage(width, height));

        try!(framebuffer.renderbuffer(&depth_stencil_buffer));

        unsafe {
            glb::DrawBuffers(attachments.len() as GLsizei, attachments.as_ptr() as *const _);
        }

        check_errors!();

        if framebuffer.is_complete()? {
            Ok(Gbuffer {
                dimensions: (width, height),
                buffers: buffers,
                depth_stencil_buffer: depth_stencil_buffer
            })
        } else {
            errln!("Incomplete framebuffer from Gbuffer creation");

            Err(GLError::IncompleteFramebuffer)
        }
    }

    #[inline]
    pub fn component(&self, component: usize) -> Option<&GLTexture> {
        self.buffers.get(component)
    }

    pub fn resize(&mut self, width: usize, height: usize) -> GLResult<()> {
        for (_, mut buffer) in self.buffers.iter_mut() {
            let format = buffer.format().unwrap();
            let internal_format = buffer.internal_format().unwrap();

            try!(buffer.load_empty(width, height, format, internal_format));
        }

        try!(self.depth_stencil_buffer.set_storage(width, height));

        self.dimensions = (width, height);

        Ok(())
    }
}