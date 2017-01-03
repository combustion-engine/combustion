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