use std::fmt::Debug;

#[inline(never)]
#[cold]
fn unwrap_failed<E: Debug>(msg: &str, error: E) -> ! {
    panic!("{}: {:?}", msg, error)
}

#[inline(never)]
#[cold]
fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

pub trait ResultExt<T, E> {
    fn expect_logged(self, msg: &str) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where E: Debug {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!(msg);
                unwrap_failed(msg, e)
            },
        }
    }
}

pub trait OptionExt<T> {
    fn expect_logged(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!(msg);
                expect_failed(msg)
            },
        }
    }
}