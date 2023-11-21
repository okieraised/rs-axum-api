//! Models which represent Elastic Common Schema (ECS) event and its fields
//!
//! The event follows [ECS Logging spec](https://github.com/elastic/ecs-logging/tree/master/spec).
//!
//! ## Example
//!
//! ```
//! use ecs_logger::ecs::{Event, LogOrigin, LogOriginFile, LogOriginRust};
//!
//! let event = Event {
//!     timestamp: chrono::Utc::now(),
//!     log_level: "ERROR",
//!     message: "Error!".to_string(),
//!     ecs_version: "1.12.1",
//!     log_origin: LogOrigin {
//!         file: LogOriginFile {
//!             line: Some(144),
//!             name: Some("server.rs"),
//!         },
//!         rust: LogOriginRust {
//!             target: "myApp",
//!             module_path: Some("my_app::server"),
//!             file_path: Some("src/server.rs"),
//!         },
//!     },
//! };
//!
//! println!("{}", serde_json::to_string(&event).unwrap());
//! ```

use std::borrow::BorrowMut;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::path::Path;
use crate::logging::extra_fields::merge_extra_fields;
use crate::logging::timestamp;

/// Represents Elastic Common Schema version.
const ECS_VERSION: &str = "1.12.1";

pub fn try_init() -> Result<(), log::SetLoggerError> {
    env_logger::builder().format(format).try_init()
}

pub fn format(buf: &mut impl std::io::Write, record: &log::Record) -> std::io::Result<()> {
    let event = Event::new(timestamp::get_timestamp(), record);

    let event_json_value =
        serde_json::to_value(event).expect("Event should be converted into JSON");
    let event_json_map = match event_json_value {
        serde_json::Value::Object(m) => m,
        _ => unreachable!("Event should be converted into a JSON object"),
    };

    let merged_json_map = merge_extra_fields(event_json_map);

    serde_json::to_writer(buf.borrow_mut(), &merged_json_map)?;
    writeln!(buf)?;

    Ok(())
}

/// Representation of an event compatible with ECS logging.
///
/// The event follows [ECS Logging spec](https://github.com/elastic/ecs-logging/tree/master/spec).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Event<'a> {
    /// Date and time when the message is logged.
    ///
    /// Mapped to `@timestamp` field.
    #[serde(rename = "@timestamp")]
    pub timestamp: DateTime<Utc>,

    /// The verbosity level of the message.
    ///
    /// Mapped to `log.level` field.
    #[serde(rename = "log.level")]
    pub log_level: &'static str,

    /// The message body.
    ///
    /// Mapped to `message` field.
    pub message: String,

    /// ECS version this event conforms to.
    ///
    /// Mapped to `ecs.version` field.
    #[serde(rename = "ecs.version")]
    pub ecs_version: &'static str,

    /// Information about the source code which logged the message.
    ///
    /// Mapped to `log.origin` field.
    #[serde(rename = "log.origin")]
    pub log_origin: LogOrigin<'a>,
}

/// Information about the source code which logged the message.
///
/// <https://www.elastic.co/guide/en/ecs/current/ecs-log.html>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogOrigin<'a> {
    /// Representation of the source code which logged the message.
    ///
    /// Mapped to `log.origin.file` field.
    pub file: LogOriginFile<'a>,

    /// Rust-specific information about the source code which logged the message.
    ///
    /// Mapped to `log.origin.rust` field.
    pub rust: LogOriginRust<'a>,
}

/// Representation of the source code which logged the message.
///
/// <https://www.elastic.co/guide/en/ecs/current/ecs-log.html>
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogOriginFile<'a> {
    /// The line number of the source code which logged the message.
    ///
    /// Mapped to `log.origin.file.line` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,

    /// The filename of the source code which logged the message.
    ///
    /// Mapped to `log.origin.file.name` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}

/// Rust-specific information about the source code which logged the message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogOriginRust<'a> {
    /// The name of the log target.
    ///
    /// Mapped to `log.origin.rust.target` field.
    pub target: &'a str,

    /// The module path of the source code which logged the message.
    ///
    /// Mapped to `log.origin.rust.module_path` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_path: Option<&'a str>,

    /// The file path of the source code which logged the message.
    ///
    /// Mapped to `log.origin.rust.file_path` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<&'a str>,
}

impl<'a> Event<'a> {
    /// Creates ECS log event from a [`log::Record`].
    pub fn new(timestamp: DateTime<Utc>, record: &'a log::Record<'a>) -> Self {
        let file_path = record.file().map(Path::new);

        Event {
            timestamp,
            log_level: record.level().as_str(),
            message: record.args().to_string(),
            ecs_version: ECS_VERSION,
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: record.line(),
                    name: file_path
                        .and_then(|p| p.file_name())
                        .and_then(|os_str| os_str.to_str()),
                },
                rust: LogOriginRust {
                    target: record.target(),
                    module_path: record.module_path(),
                    file_path: record.file(),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_log_record() {
        let timestamp = DateTime::parse_from_rfc3339("2021-11-27T07:18:11.712009300Z")
            .unwrap()
            .with_timezone(&Utc);

        let record = log::Record::builder()
            .args(format_args!("Error!"))
            .level(log::Level::Error)
            .target("myApp")
            .file(Some("src/server.rs"))
            .line(Some(144))
            .module_path(Some("my_app::server"))
            .build();

        let event = Event::new(timestamp, &record);

        assert_eq!(
            event,
            Event {
                timestamp,
                log_level: "ERROR",
                message: "Error!".to_string(),
                ecs_version: "1.12.1",
                log_origin: LogOrigin {
                    file: LogOriginFile {
                        line: Some(144),
                        name: Some("server.rs")
                    },
                    rust: LogOriginRust {
                        target: "myApp",
                        module_path: Some("my_app::server"),
                        file_path: Some("src/server.rs")
                    }
                }
            }
        );
    }

    #[test]
    fn test_serialize() {
        let timestamp = DateTime::parse_from_rfc3339("2021-11-24T17:38:21.000098765Z")
            .unwrap()
            .with_timezone(&Utc);

        let event = Event {
            timestamp,
            log_level: "TRACE",
            message: "tracing msg".to_string(),
            ecs_version: "1.12.1",
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: Some(1234),
                    name: Some("file.rs"),
                },
                rust: LogOriginRust {
                    target: "myCustomTarget123",
                    module_path: Some("my_app::path::to::your::file"),
                    file_path: Some("src/path/to/your/file.rs"),
                },
            },
        };

        assert_eq!(
            serde_json::to_string(&event).expect("Failed to serialize ECS event"),
            r#"{"@timestamp":"2021-11-24T17:38:21.000098765Z","log.level":"TRACE","message":"tracing msg","ecs.version":"1.12.1","log.origin":{"file":{"line":1234,"name":"file.rs"},"rust":{"target":"myCustomTarget123","module_path":"my_app::path::to::your::file","file_path":"src/path/to/your/file.rs"}}}"#
        );
    }

    #[test]
    fn test_serialize_with_none() {
        let timestamp = DateTime::parse_from_rfc3339("2021-11-24T17:38:21.000098765Z")
            .unwrap()
            .with_timezone(&Utc);

        let event = Event {
            timestamp,
            log_level: "TRACE",
            message: "tracing msg".to_string(),
            ecs_version: "1.12.1",
            log_origin: LogOrigin {
                file: LogOriginFile {
                    line: None,
                    name: None,
                },
                rust: LogOriginRust {
                    target: "myCustomTarget123",
                    module_path: None,
                    file_path: None,
                },
            },
        };

        assert_eq!(
            serde_json::to_string(&event).expect("Failed to serialize ECS event"),
            r#"{"@timestamp":"2021-11-24T17:38:21.000098765Z","log.level":"TRACE","message":"tracing msg","ecs.version":"1.12.1","log.origin":{"file":{},"rust":{"target":"myCustomTarget123"}}}"#
        );
    }
}