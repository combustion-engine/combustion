use std::os::raw::c_void;
use std::path::Path;

pub use backtrace::{trace, resolve, SymbolName, Backtrace};

pub trait BacktraceFmt {
    fn format(count: u32,
              name: Option<SymbolName>,
              addr: Option<*mut c_void>,
              filename: Option<&Path>,
              lineno: Option<u32>) -> String;
}

pub struct DefaultBacktraceFmt;

impl BacktraceFmt for DefaultBacktraceFmt {
    fn format(count: u32,
              name: Option<SymbolName>,
              addr: Option<*mut c_void>,
              _filename: Option<&Path>,
              _lineno: Option<u32>) -> String {
        let name = name.and_then(|name| { name.as_str() }).unwrap_or("???");

        format!("{:>5}: {:>20p} - {:<}\n", count, addr.unwrap_or(0x0 as *mut _), name)
    }
}

pub fn format_trace<Fmt: BacktraceFmt>(start: String) -> String {
    let mut traces = start;

    let mut count = 1;

    trace(|frame| {
        resolve(frame.ip(), |symbol| {
            traces += Fmt::format(count,
                                  symbol.name(),
                                  symbol.addr(),
                                  symbol.filename(),
                                  symbol.lineno()).as_str();
        });

        count += 1;

        true
    });

    traces
}

/// Returns a string containing the backtrace and a header message
#[macro_export]
macro_rules! backtrace {
    () => {
        backtrace!($crate::bt::DefaultBacktraceFmt)
    };

    ($fmt:ty) => {
        $crate::bt::format_trace::<$fmt>(
            format!("Stack backtrace starting at Line {} in \"{}\":\n", line!(), file!())
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_bt() {
        println!("{}", backtrace!());
    }
}