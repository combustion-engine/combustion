//! Logging utils

use std::path::{Path, PathBuf};
use std::io;
use std::io::prelude::*;
use std::fs::{File, create_dir_all};

use slog;
use slog::DrainExt;
use slog_term;
use slog_stream;
use slog_scope::set_global_logger;

use chrono;

pub use slog_scope::logger;

pub const LOG_DATE_FORMAT: &'static str = "%Y%m%dT%H%M%SZ";

/// Create a new logger
pub fn new_logger<P: AsRef<Path>>(path: P) -> io::Result<slog::Logger> {
    let mut dir: PathBuf = path.as_ref().to_path_buf();

    try!(create_dir_all(dir.as_path()));

    dir.push(chrono::UTC::now().format(LOG_DATE_FORMAT).to_string());
    dir.set_extension("log");

    let log_file = try!(File::create(dir.as_path()));

    let term_drain = slog_term::streamer().plain().use_utc_timestamp().build();
    let file_drain = slog_stream::stream(log_file, EngineLogger);

    let logger = slog::Logger::root(slog::duplicate(term_drain, file_drain).fuse(), o!());

    Ok(logger)
}

/// Create a new logger and set it as the global logger
pub fn init_global_logger<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let logger = try!(new_logger(path));

    set_global_logger(logger);

    info!("Logger initialized");

    Ok(())
}

struct EngineLogger;

impl slog_stream::Format for EngineLogger {
    fn format(&self, io: &mut Write, info: &slog::Record, _: &slog::OwnedKeyValueList) -> io::Result<()> {
        let level = slog::LOG_LEVEL_SHORT_NAMES[info.level().as_usize()];

        if info.level().is_at_least(slog::Level::Warning) {
            writeln!(io, "{}: {} - {}\n\tat {}:{}:{}", chrono::UTC::now().to_rfc2822(), level, info.msg(),
                     info.file(), info.line(), info.column())
        } else {
            writeln!(io, "{}: {} - {}", chrono::UTC::now().to_rfc2822(), level, info.msg())
        }
    }
}
