// Copyright 2018 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Different assorted utilities.

pub use self::types::{Height, Milliseconds, Round, ValidatorId};

pub mod config;
pub mod user_agent;
#[macro_use]
pub mod metrics;

use log::{Level, Record, SetLoggerError};
use env_logger::{Builder, Formatter};
use colored::*;
use chrono::{DateTime, Local};

use std::env;
use std::io::{self, Write};
use std::time::SystemTime;

use crypto::gen_keypair;

mod types;

/// Performs the logger initialization.
pub fn init_logger() -> Result<(), SetLoggerError> {
    let mut builder = Builder::new();
    builder.format(format_log_record);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.try_init()
}

fn has_colors() -> bool {
    use term::terminfo::TerminfoTerminal;
    use term::Terminal;
    use std::io;
    use atty;

    let out = io::stderr();
    match TerminfoTerminal::new(out) {
        Some(ref term) if atty::is(atty::Stream::Stderr) => term.supports_color(),
        _ => false,
    }
}

fn format_time(time: SystemTime) -> String {
    DateTime::<Local>::from(time).to_rfc2822()
}

fn format_log_record(buf: &mut Formatter, record: &Record) -> io::Result<()> {
    let time = format_time(SystemTime::now());

    let verbose_src_path = match env::var("RUST_VERBOSE_PATH") {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_) => false,
    };

    let module = record.module_path().unwrap_or("unknown_module");
    let source_path = if verbose_src_path {
        let file = record.file().unwrap_or("unknown_file");
        let line = record.line().unwrap_or(0);
        format!("{}:{}:{}", module, file, line)
    } else {
        module.to_string()
    };

    if has_colors() {
        let level = match record.level() {
            Level::Error => "ERROR".red(),
            Level::Warn => "WARN".yellow(),
            Level::Info => "INFO".green(),
            Level::Debug => "DEBUG".cyan(),
            Level::Trace => "TRACE".white(),
        };
        writeln!(
            buf,
            "{} {} {} {}",
            time.dimmed(),
            level,
            source_path.dimmed(),
            record.args()
        )
    } else {
        let level = match record.level() {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        };
        writeln!(buf, "{} {} {} {}", time, level, &source_path, record.args())
    }
}