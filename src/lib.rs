//! A logger that prints all messages in browser's console.

extern crate log;
#[macro_use]
extern crate stdweb;

use log::{
    Log,
    Level,
    Metadata,
    Record,
    SetLoggerError,
};

mod console {
    pub(super) fn trace(message: &str) {
        js! { @(no_return) console.log(@{message}); }
    }

    pub(super) fn debug(message: &str) {
        js! { @(no_return) console.debug(@{message}); }
    }

    pub(super) fn info(message: &str) {
        js! { @(no_return) console.info(@{message}); }
    }

    pub(super) fn warn(message: &str) {
        js! { @(no_return) console.warn(@{message}); }
    }

    pub(super) fn error(message: &str) {
        js! { @(no_return) console.error(@{message}); }
    }
}

static LOGGER: WebLogger = WebLogger;

struct WebLogger;

impl Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO Check the args of a location
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let msg = format!("{}:{} -- {}",
                record.level(),
                record.target(),
                record.args());
            match metadata.level() {
                Level::Trace => console::trace(&msg),
                Level::Debug => console::debug(&msg),
                Level::Info => console::info(&msg),
                Level::Warn => console::warn(&msg),
                Level::Error => console::error(&msg),
            }
        }
    }

    fn flush(&self) {
    }
}

pub fn try_init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    Ok(())
}

pub fn init() {
    try_init().expect("web_logger::init should not be called after logger initialized");
}
