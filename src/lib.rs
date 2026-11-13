mod opts;
mod processor;

pub use opts::{Opts, SubCommand, GenPassOpts, CsvOpts};

pub use processor::process_csv;