//! Provides extensions to the `Result` and `Option` types that add extra logging and OS message box alerts

use std::fmt::Debug;

use tinyfiledialogs::*;

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

/// Extensions to the `Result` type that add extra logging or OS message box alerts
pub trait ResultExt<T, E> {
    /// Logs failed `expect` with `error!`
    fn expect_logged(self, msg: &str) -> T;
    /// Logs failed `expect` with `error!` and spawns an OS message box dialog with the given message
    fn expect_logged_box(self, msg: &str) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E> where E: Debug {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                error!("{}\n\n\nDetails:\n\n{:?}", msg, e);
                unwrap_failed(msg, e)
            },
        }
    }

    #[inline]
    fn expect_logged_box(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                let formatted = format!("{}\n\n\nDetails:\n\n{:?}", msg, e);
                error!("{}", formatted);
                message_box(MessageBox::Ok, "Combustion Error", formatted.as_str(), Some(Icon::Error), None);
                unwrap_failed(msg, e)
            },
        }
    }
}

/// Extensions to the `Option` type that add extra logging or OS message box alerts
pub trait OptionExt<T> {
    /// Logs failed `expect` with `error!`
    fn expect_logged(self, msg: &str) -> T;
    /// Logs failed `expect` with `error!` and spawns an OS message box dialog with the given message
    fn expect_logged_box(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    #[inline]
    fn expect_logged(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!("{}", msg);
                expect_failed(msg)
            },
        }
    }

    #[inline]
    fn expect_logged_box(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                error!("{}", msg);
                let _ = message_box(MessageBox::Ok, "Combustion Error", msg, Some(Icon::Error), Some(BoxButton::OkYes));
                expect_failed(msg)
            },
        }
    }
}