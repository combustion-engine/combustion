use super::bindings::types::*;
use super::bindings::*;

use std::sync::{Arc, RwLock};
use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use enum_primitive::FromPrimitive;

use super::error::*;

enum_from_primitive! {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum GLDebugSource {
        Api = DEBUG_SOURCE_API,
        WindowSystem = DEBUG_SOURCE_WINDOW_SYSTEM,
        ShaderCompiler = DEBUG_SOURCE_SHADER_COMPILER,
        ThirdParty = DEBUG_SOURCE_THIRD_PARTY,
        Application = DEBUG_SOURCE_APPLICATION,
        Other = DEBUG_SOURCE_OTHER,
    }
}

impl From<GLDebugSource> for String {
    fn from(value: GLDebugSource) -> String {
        match value {
            GLDebugSource::Api => "API",
            GLDebugSource::WindowSystem => "Window System",
            GLDebugSource::ShaderCompiler => "Shader Compiler",
            GLDebugSource::ThirdParty => "Third Party",
            GLDebugSource::Application => "Application",
            GLDebugSource::Other => "Other Source",
        }.into()
    }
}

impl GLDebugSource {
    #[inline(always)]
    pub fn into_string(self) -> String { self.into() }
}

enum_from_primitive! {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum GLDebugType {
        Error = DEBUG_TYPE_ERROR,
        DeprecatedBehavior = DEBUG_TYPE_DEPRECATED_BEHAVIOR,
        UndefinedBehavior = DEBUG_TYPE_UNDEFINED_BEHAVIOR,
        Portability = DEBUG_TYPE_PORTABILITY,
        Performance = DEBUG_TYPE_PERFORMANCE,
        Marker = DEBUG_TYPE_MARKER,
        PushGroup = DEBUG_TYPE_PUSH_GROUP,
        PopGroup = DEBUG_TYPE_POP_GROUP,
        Other = DEBUG_TYPE_OTHER,
    }
}

impl From<GLDebugType> for String {
    fn from(value: GLDebugType) -> String {
        match value {
            GLDebugType::Error => "Error",
            GLDebugType::DeprecatedBehavior => "Deprecated Behaviour",
            GLDebugType::UndefinedBehavior => "Undefined Behaviour",
            GLDebugType::Portability => "Portability",
            GLDebugType::Performance => "Performance",
            GLDebugType::Marker => "Marker",
            GLDebugType::PushGroup => "Push Group",
            GLDebugType::PopGroup => "Pop Group",
            GLDebugType::Other => "Other",
        }.into()
    }
}

impl GLDebugType {
    #[inline(always)]
    pub fn into_string(self) -> String { self.into() }
}

enum_from_primitive! {
    #[repr(u32)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum GLDebugSeverity {
        High = DEBUG_SEVERITY_HIGH,
        Medium = DEBUG_SEVERITY_MEDIUM,
        Low = DEBUG_SEVERITY_LOW,
        Notification = DEBUG_SEVERITY_NOTIFICATION,
    }
}

impl From<GLDebugSeverity> for String {
    fn from(value: GLDebugSeverity) -> String {
        match value {
            GLDebugSeverity::High => "High",
            GLDebugSeverity::Medium => "Medium",
            GLDebugSeverity::Low => "Low",
            GLDebugSeverity::Notification => "Notification",
        }.into()
    }
}

impl GLDebugSeverity {
    #[inline(always)]
    pub fn into_string(self) -> String { self.into() }
}

pub type GLDebugProc = fn(GLuint, GLDebugSource, GLDebugType, GLDebugSeverity, String);

pub fn enable_debug(cb: GLDebugProc, synchronous: bool) -> GLResult<()> {
    let mut flags: GLint = 0;

    unsafe {
        GetIntegerv(CONTEXT_FLAGS, &mut flags as *mut GLint);
    }

    check_gl_errors!();

    if flags as GLuint & CONTEXT_FLAG_DEBUG_BIT != 0 {
        unsafe {
            Enable(DEBUG_OUTPUT);

            check_gl_errors!();

            try_rethrow!(set_synchronous(synchronous));

            DebugMessageCallback(debug_callback, cb as *mut _);

            check_gl_errors!();

            try_rethrow!(set_filter(None, None, None));
        }

        Ok(())
    } else {
        throw!(GLError::Unsupported)
    }
}

pub fn set_filter(source: Option<GLDebugSource>, ty: Option<GLDebugType>, severity: Option<GLDebugSeverity>) -> GLResult<()> {
    macro_rules! filter_or_dontcare {
        ($filter:ident) => {
            match $filter {
                Some(value) => value as GLenum,
                None => DONT_CARE
            }
        }
    }

    unsafe {
        DebugMessageControl(filter_or_dontcare!(source),
                            filter_or_dontcare!(ty),
                            filter_or_dontcare!(severity),
                            0, ptr::null(), TRUE);
    }

    check_gl_errors!();

    Ok(())
}

pub fn set_synchronous(value: bool) -> GLResult<()> {
    unsafe {
        if value {
            Enable(DEBUG_OUTPUT_SYNCHRONOUS);
        } else {
            Disable(DEBUG_OUTPUT_SYNCHRONOUS);
        }
    }

    check_gl_errors!();

    Ok(())
}

pub fn send_debug_message(id: GLuint,
                          ty: GLDebugType,
                          severity: GLDebugSeverity,
                          message: &'static str) -> GLResult<()> {
    let c_message = try_throw!(CString::new(message));

    unsafe {
        DebugMessageInsert(DEBUG_SOURCE_APPLICATION, ty as GLenum, id, severity as GLenum, -1, c_message.as_ptr() as *const _);
    }

    check_gl_errors!();

    Ok(())
}
lazy_static! {
    pub static ref DEBUG_IGNORED: Arc<RwLock<Vec<GLuint>>> = {
        //Default ignores
        Arc::new(RwLock::new(vec![131169, 131185, 131218, 131204]))
    };
}

extern "system" fn debug_callback(source: GLenum,
                                  ty: GLenum,
                                  id: GLuint,
                                  severity: GLenum,
                                  _: GLsizei, //length
                                  message: *const GLchar,
                                  callback_ptr: *mut c_void) {
    if DEBUG_IGNORED.read().unwrap().contains(&id) {
        return;
    }

    if callback_ptr.is_null() {
        error!("Invalid callback supplied to OpenGL Debug Callback invocation function.");
    } else {
        unsafe {
            let callback: GLDebugProc = mem::transmute(callback_ptr);

            callback(id,
                     GLDebugSource::from_u32(source as u32).expect("Cannot get GLDebugSource from u32 primitive"),
                     GLDebugType::from_u32(ty as u32).expect("Cannot get GLDebugType from u32 primitive"),
                     GLDebugSeverity::from_u32(severity as u32).expect("Cannot get GLDebugSeverity from u32 primitive"),
                     CStr::from_ptr(message).to_string_lossy().into());
        }
    }
}

#[inline(never)]
pub fn default_debug_callback(id: GLuint, source: GLDebugSource, ty: GLDebugType, severity: GLDebugSeverity, message: String) {
    use std::iter::repeat;
    use std::cmp::min;

    let header = format!("Debug message: ({}): {}", id, message);
    let line = repeat("-").take(min(80, header.len())).collect::<String>();

    info!("{}", header);
    info!("{}", line);
    info!("Source:   {}", source.into_string());
    info!("Type:     {}", ty.into_string());

    let severity_line = format!("Severity: {}", severity.into_string());

    info!("{}", severity_line);

    //Reuse the header line if possible
    if severity_line.len() <= line.len() {
        info!("{}", &line[..severity_line.len()]);
    } else {
        let last_line = repeat("-").take(min(80, severity_line.len()) - line.len()).collect::<String>();

        info!("{}{}", line, last_line);
    }
}