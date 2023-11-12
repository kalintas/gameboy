#[allow(dead_code)]
pub fn clear_gl_errors() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

#[allow(dead_code)]
pub fn check_gl_errors() {
    loop {
        match unsafe { gl::GetError() } {
            gl::NO_ERROR => break,
            error => {
                panic!("[OpenGL Error]: {}", error);
            }
        }
    }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! gl_call {
    ($x: expr) => {{
        crate::renderer::utils::clear_gl_errors();

        let result = unsafe { $x };

        crate::renderer::utils::check_gl_errors();

        result
    }};
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! gl_call {
    ($x: expr) => {
        unsafe { $x }
    };
}
