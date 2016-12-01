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
#[cfg(debug)]
macro_rules! debug_print {
    ($($arg:tt)*) => (print!($($arg)*))
}

#[macro_export]
#[cfg(debug)]
macro_rules! debug_println {
    ($($arg:tt)*) => (println!($($arg)*))
}

#[macro_export]
#[cfg(debug)]
macro_rules! debug_err {
    ($($arg:tt)*) => (err!($($arg)*))
}

#[macro_export]
#[cfg(debug)]
macro_rules! debug_errln {
    ($($arg:tt)*) => (errln!($($arg)*))
}

#[macro_export]
#[cfg(not(debug))]
macro_rules! debug_print {
    ($($arg:tt)*) => {}
}

#[macro_export]
#[cfg(not(debug))]
macro_rules! debug_println {
    ($($arg:tt)*) => ()
}

#[macro_export]
#[cfg(not(debug))]
macro_rules! debug_err {
    ($($arg:tt)*) => {}
}

#[macro_export]
#[cfg(not(debug))]
macro_rules! debug_errln {
    ($($arg:tt)*) => ()
}