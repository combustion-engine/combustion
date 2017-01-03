use std::os::raw::c_void;
use std::path::Path;
use std::thread;

use backtrace::{trace, resolve, SymbolName};

pub trait BacktraceFmt {
    /// Formats backtrace symbol components in some way
    fn format(count: u32,
              name: Option<SymbolName>,
              addr: Option<*mut c_void>,
              filename: Option<&Path>,
              lineno: Option<u32>) -> String;
}

/// Default backtrace formatter that tries to resemble rustc panic backtraces somewhat
pub struct DefaultBacktraceFmt;

impl BacktraceFmt for DefaultBacktraceFmt {
    fn format(count: u32,
              name: Option<SymbolName>,
              addr: Option<*mut c_void>,
              filename: Option<&Path>,
              lineno: Option<u32>) -> String {
        let name = name.and_then(|name| { name.as_str() }).unwrap_or("???");

        lazy_static! {
            static ref PADDING: String = format!("{:<24}", "");
        }

        let begin = format!("{:>4}: {:>18p} - {:<}\n{}", count, addr.unwrap_or(0x0 as *mut _), name, *PADDING);

        let end = if let Some(filename) = filename {
            format!("at {}:{}\n", filename.display(), lineno.unwrap_or(0))
        } else {
            format!("at <anonymous>:{}\n", lineno.unwrap_or(0))
        };

        begin + end.as_str()
    }
}

/// Generates a formatted backtrace (via `Fmt` type) from here, but expects `line` and `file` to be where it was called from.
///
/// The actual call to `format_trace` and `trace` are ignored.
#[inline(never)]
pub fn format_trace<Fmt: BacktraceFmt>(line: u32, file: &str) -> String {
    // Ignore `format_trace` and `backtrace::trace` call, both of which are marked as #[inline(never)],
    // so they will always show up.
    const IGNORE_COUNT: u32 = 2;

    let mut traces = format!("Stack backtrace for task \"<{}>\" at line {} of \"{}\":\n",
                             thread::current().name().unwrap_or("unnamed"), line, file);

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

/// Returns a string containing the formatted backtrace and a header message
///
/// Pass a custom `BacktraceFmt` type to the macro to use custom formatting
#[macro_export]
macro_rules! backtrace {
    () => {
        backtrace!($crate::bt::DefaultBacktraceFmt)
    };

    ($fmt:ty) => {
        $crate::bt::format_trace::<$fmt>(line!(), file!())
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bt() {
        println!("{}", backtrace!());
    }
}