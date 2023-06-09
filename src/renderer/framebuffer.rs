use crate::gl_call;

/// A simple struct for copying a texture to the default frambuffer.
pub struct Framebuffer {
    id: u32,
    texture: u32,
}

impl Framebuffer {
    /// Createa new FrameBuffer object.
    pub fn new() -> Self {
        let mut id = 0;

        gl_call!(gl::GenFramebuffers(1, &mut id));
        gl_call!(gl::BindFramebuffer(gl::READ_FRAMEBUFFER, id));

        let mut texture = 0;

        gl_call!(gl::GenTextures(1, &mut texture));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, texture));

        let framebuffer = Self { id, texture };

        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR as _
        ));
        gl_call!(gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::LINEAR as _
        ));

        gl_call!(gl::FramebufferTexture2D(
            gl::READ_FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture,
            0
        ));

        framebuffer
    }

    /// Copies the given data to the framebuffer.
    pub fn update_buffer(
        &self,
        width: i32,
        height: i32,
        ptr: *const u8,
        internal_format: u32,
        format: u32,
    ) {
        gl_call!(gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.id));
        gl_call!(gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            internal_format as i32,
            width,
            height,
            0,
            format,
            gl::UNSIGNED_BYTE,
            ptr as _
        ));
    }

    /// Draws the framebuffer to the default framebuffer.
    /// OpenGL will give error if update_buffer is not called before this function.
    pub fn draw_buffer(
        &self,
        src0: (i32, i32),
        src1: (i32, i32),
        dest0: (i32, i32),
        dest1: (i32, i32),
    ) {
        gl_call!(gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.id));
        gl_call!(gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0));
        gl_call!(gl::BlitFramebuffer(
            src0.0,
            src0.1,
            src1.0,
            src1.1,
            dest0.0,
            dest0.1,
            dest1.0,
            dest1.1,
            gl::COLOR_BUFFER_BIT,
            gl::NEAREST
        ));
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        gl_call!(gl::DeleteFramebuffers(1, &self.id));
        gl_call!(gl::DeleteTextures(1, &self.texture));
    }
}
