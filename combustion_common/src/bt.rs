use backtrace::resolve;
use std::os::raw::c_void;

pub use backtrace::trace;
/*
pub fn collect_frame(ip: *mut c_void) -> String {
    let mut result = String::new();

    resolve(ip, |symbol| {

    });

    result
}*/