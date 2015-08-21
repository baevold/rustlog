//#![feature(phase)]  
//#[phase(plugin, link)]extern crate log;  
//#[phase(plugin, link)]extern crate time;  
extern crate log;
extern crate time;
/// import  
use log::{Log,LogRecord,LogLevel,set_logger, LogMetadata, SetLoggerError, LogLevelFilter};
struct CustomLogger;

/// Implements Logger trait for Custom Logger which support logging timestamp, file name and line number
/// in addition to log level, module path and message.
impl Log for CustomLogger {
	fn log(&self, record: &LogRecord) {
		println!("{}:{}:{}:{}:{} {}",
				time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(),
				record.level(),
				record.location().module_path(),
				record.location().file(),
				record.location().line(),
				record.args());
	}

	fn enabled(&self, metadata: &LogMetadata) -> bool {
		metadata.level() <= LogLevel::Info
	}
}

pub fn init() -> Result<(), SetLoggerError> {
	log::set_logger(|max_log_level| {
		max_log_level.set(LogLevelFilter::Info);
		Box::new(CustomLogger)
	})
}
