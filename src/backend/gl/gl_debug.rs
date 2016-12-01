use super::bindings::types::*;
use super::bindings::*;

use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};
use enum_primitive::FromPrimitive;

use super::gl_error::*;

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
        format!("{}", match value {
            GLDebugType::Error => "Error",
            GLDebugType::DeprecatedBehavior => "Deprecated Behaviour",
            GLDebugType::UndefinedBehavior => "Undefined Behaviour",
            GLDebugType::Portability => "Portability",
            GLDebugType::Performance => "Performance",
            GLDebugType::Marker => "Marker",
            GLDebugType::PushGroup => "Push Group",
            GLDebugType::PopGroup => "Pop Group",
            GLDebugType::Other => "Other",
        }).into()
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
        format!("{}", match value {
            GLDebugSeverity::High => "High",
            GLDebugSeverity::Medium => "Medium",
            GLDebugSeverity::Low => "Low",
            GLDebugSeverity::Notification => "Notification",
        }).into()
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

    check_errors!();

    if flags as GLuint & CONTEXT_FLAG_DEBUG_BIT != 0 {
        unsafe {
            Enable(DEBUG_OUTPUT);

            check_errors!();

            try!(set_synchronous(synchronous));

            DebugMessageCallback(debug_callback, cb as *mut _);

            check_errors!();

            try!(set_filter(None, None, None));
        }

        Ok(())
    } else {
        Err(GLError::Unsupported)
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

    check_errors!();

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

    check_errors!();

    Ok(())
}

pub fn send_debug_message(id: GLuint,
                          ty: GLDebugType,
                          severity: GLDebugSeverity,
                          message: &'static str) -> GLResult<()> {
    let c_message = try!(CString::new(message));

    unsafe {
        DebugMessageInsert(DEBUG_SOURCE_APPLICATION, ty as GLenum, id, severity as GLenum, -1, c_message.as_ptr() as *const _);
    }

    check_errors!();

    Ok(())
}

extern "system" fn debug_callback(source: GLenum,
                                  ty: GLenum,
                                  id: GLuint,
                                  severity: GLenum,
                                  _: GLsizei, //length
                                  message: *const GLchar,
                                  callback_ptr: *mut c_void) {
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    }

    if callback_ptr.is_null() {
        errln!("Invalid callback supplied to OpenGL Debug Callback invocation function.");
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

    println!("{}", header);
    println!("{}", line);
    println!("Source:   {}", source.into_string());
    println!("Type:     {}", ty.into_string());
    let severity_line = format!("Severity: {}", severity.into_string());

    println!("{}", severity_line);

    //Reuse the header line if possible
    if severity_line.len() <= line.len() {
        println!("{}", &line[..severity_line.len()]);
    } else {
        let last_line = repeat("-").take(min(80, severity_line.len()) - line.len()).collect::<String>();

        println!("{}{}", line, last_line);
    }
}