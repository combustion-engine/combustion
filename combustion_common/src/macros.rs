#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {{
        use std::io::Write;
        write!(&mut ::std::io::stderr(), $($arg)*).unwrap()
    }};
}

#[macro_export]
macro_rules! errln {
    ($fmt:expr) => (err!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (err!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! error_panic {
    ($($arg:tt)*) => {{
        error!($($arg)*);
        panic!($($arg)*);
    }}
}

/*
#[macro_export]
macro_rules! backtrace {
    () => {{
        let mut result = String::new();

        $crate::bt::trace(|frame| {
            let mut symbols = Vec::new();

            resolve(frame.ip(), |symbol| {
                symbols.push(BacktraceSymbol {
                    name: symbol.name().map(|m| m.as_bytes().to_vec()),
                    addr: symbol.addr().map(|a| a as usize),
                    filename: symbol.filename().map(|m| m.to_path_buf()),
                    lineno: symbol.lineno(),
                });
            });

            frames.push(BacktraceFrame {
                ip: frame.ip() as usize,
                symbol_address: frame.symbol_address() as usize,
                symbols: symbols,
            });

            true
        });

        result
    }}
}*/