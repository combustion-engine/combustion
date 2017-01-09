//! Backtrace utilities

use std::os::raw::c_void;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::path::Path;
use std::thread;
use std::mem;

use bt::*;

/// Trait to define formats for backtraces
pub trait BacktraceFmt {
    /// Formats backtrace symbol components in some way
    fn format(count: u32, name: Option<SymbolName>, addr: Option<*mut c_void>, filename: Option<&Path>, lineno: Option<u32>) -> String;
}

/// Default backtrace formatter that tries to resemble rustc panic backtraces somewhat
pub struct DefaultBacktraceFmt;

impl BacktraceFmt for DefaultBacktraceFmt {
    fn format(count: u32, name: Option<SymbolName>, addr: Option<*mut c_void>, filename: Option<&Path>, lineno: Option<u32>) -> String {
        let ptr_width = mem::size_of::<usize>() * 2 + 2;

        let name = name.and_then(|name| { name.as_str() }).unwrap_or("<unknown>");

        let begin = format!("{:>4}: {:>4$p} - {:<}\n{:<5$}", count, addr.unwrap_or(0x0 as *mut _), name, "", ptr_width, ptr_width + 6);

        let end = if let Some(filename) = filename {
            if let Some(lineno) = lineno {
                format!("at {}:{}\n", filename.display(), lineno)
            } else {
                format!("at {}\n", filename.display())
            }
        } else if let Some(lineno) = lineno {
            format!("at <anonymous>:{}\n", lineno)
        } else {
            format!("at <anonymous>\n")
        };

        begin + end.as_str()
    }
}

/// Generates a formatted backtrace (via `Fmt` type) from here, but expects `line` and `file` to be where it was called from.
///
/// The actual call to `format_trace` and `trace` are ignored.
#[inline(never)]
pub fn format_trace<Fmt: BacktraceFmt>(header: bool, line: u32, file: &str) -> String {
    // Ignore `format_trace` and `backtrace::trace` calls, both of which are marked as #[inline(never)],
    // so they will always show up.
    const IGNORE_COUNT: u32 = 2;

    let mut traces = if header {
        format!("Stack backtrace for task \"<{}>\" at line {} of \"{}\":\n",
                thread::current().name().unwrap_or("unnamed"), line, file)
    } else {
        String::new()
    };

    let mut count = 0;

    trace(|frame| {
        if count < IGNORE_COUNT {
            count += 1;
        } else {
            let before = count;

            resolve(frame.ip(), |symbol| {
                traces += Fmt::format(count - IGNORE_COUNT,
                                      symbol.name(),
                                      symbol.addr(),
                                      symbol.filename(),
                                      symbol.lineno()).as_str();

                count += 1;
            });

            // These will be equal if `resolve_cb` was not invoked
            if count == before {
                // If `symbol_address` doesn't work, oh well.
                resolve(frame.symbol_address(), |symbol| {
                    traces += Fmt::format(count - IGNORE_COUNT,
                                          symbol.name(),
                                          symbol.addr(),
                                          symbol.filename(),
                                          symbol.lineno()).as_str();

                    count += 1;
                });
            }
        }

        // Always continue
        true
    });

    traces
}

/// Backtrace that also contains the exact line and file in which it originated from.
///
/// Usually created in a macro using the `line!()` and `file!()` macros
pub struct LineBacktrace {
    backtrace: Backtrace,
    line: u32,
    file: &'static str,
}

impl Debug for LineBacktrace {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "LineBacktrace {{\n    line: {},\n    file: {},\n    backtrace:\n{}}}", self.line, self.file, self.format::<DefaultBacktraceFmt>(false, false))
    }
}

impl LineBacktrace {
    /// Create a new `LineBacktrace` if you know the file and file
    pub fn new(line: u32, file: &'static str) -> LineBacktrace {
        LineBacktrace {
            backtrace: Backtrace::new(),
            line: line,
            file: file,
        }
    }

    pub fn format<Fmt: BacktraceFmt>(&self, header: bool, reverse: bool) -> String {
        // Ignore `backtrace::trace` call
        const IGNORE_COUNT: u32 = 1;

        let mut traces = if header {
            format!("Stack backtrace for task \"<{}>\" at line {} of \"{}\":\n",
                    thread::current().name().unwrap_or("unnamed"), self.line, self.file)
        } else {
            String::new()
        };

        let mut count = 0;

        if reverse {
            let mut symbols = Vec::new();

            for frame in self.backtrace.frames() {
                for symbol in frame.symbols() {
                    symbols.push(symbol);
                }
            }

            for symbol in symbols.iter().rev() {
                if count >= IGNORE_COUNT {
                    if let Some(name) = symbol.name() {
                        if let Some(name_str) = name.as_str() {
                            // Checks for `Backtrace::new` AND `ThinBacktrace::new`
                            if name_str.contains("Backtrace::new") {
                                // Ignore and don't increment `count`
                                continue;
                            }
                        }
                    }

                    traces += Fmt::format(count - IGNORE_COUNT,
                                          symbol.name(),
                                          symbol.addr(),
                                          symbol.filename(),
                                          symbol.lineno()).as_str();
                }

                count += 1;
            }
        } else {
            for frame in self.backtrace.frames() {
                for symbol in frame.symbols() {
                    if count >= IGNORE_COUNT {
                        if let Some(name) = symbol.name() {
                            if let Some(name_str) = name.as_str() {
                                // Checks for `Backtrace::new` AND `ThinBacktrace::new`
                                if name_str.contains("Backtrace::new") {
                                    // Ignore and don't increment `count`
                                    continue;
                                }
                            }
                        }

                        traces += Fmt::format(count - IGNORE_COUNT,
                                              symbol.name(),
                                              symbol.addr(),
                                              symbol.filename(),
                                              symbol.lineno()).as_str();
                    }

                    count += 1;
                }
            }
        }

        traces
    }
}

impl From<Backtrace> for LineBacktrace {
    fn from(backtrace: Backtrace) -> LineBacktrace {
        LineBacktrace { line: line!(), file: file!(), backtrace: backtrace }
    }
}

/// Returns a string containing the formatted backtrace and a header message
///
/// Pass a custom `BacktraceFmt` type to the macro to use custom formatting
#[macro_export]
macro_rules! backtrace {
    () => {
        backtrace!($crate::backtrace::DefaultBacktraceFmt)
    };

    ($fmt:ty) => {
        $crate::backtrace::format_trace::<$fmt>(true, line!(), file!())
    };
}

/// Variation of `backtrace!` that doesn't include the header line
#[macro_export]
macro_rules! backtrace_noheader {
    () => {
        backtrace_noheader!($crate::backtrace::DefaultBacktraceFmt)
    };

    ($fmt:ty) => {
        $crate::backtrace::format_trace::<$fmt>(false, line!(), file!())
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_backtrace() {
        println!("{}", backtrace!());
    }
}