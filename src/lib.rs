#![feature(phase)]  
#[phase(plugin, link)]extern crate log;  
#[phase(plugin, link)]extern crate time;  
/// import  
use log::{Logger,LogRecord,LogLevel,LogLocation, set_logger};
use std::io::{ LineBufferedWriter, stdio, stderr};
/// Custom Logger
struct CustomLogger {
	handle: LineBufferedWriter<stdio::StdWriter>,
}
/// Implements Logger trait for Custom Logger which support logging timestamp, file name and line number
/// in addition to log level, module path and message.
impl Logger for CustomLogger {
	fn log(&mut self, record: &LogRecord) {
		match writeln!(&mut self.handle,
				"{}:{}:{}:{}:{} {}",
				time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(),
				record.level,
				record.module_path,
				record.file,
				record.line,
				record.args) {
		Err(e) => panic!("failed to log: {}", e),
		Ok(()) => {}
		}
	}
}
