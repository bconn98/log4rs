//! Filters

use log::Record;
#[cfg(feature = "config_parsing")]
use serde::de;
#[cfg(feature = "config_parsing")]
use serde_value::Value;
#[cfg(feature = "config_parsing")]
use std::collections::BTreeMap;
use std::fmt;

#[cfg(feature = "config_parsing")]
use crate::config::Deserializable;

#[cfg(feature = "threshold_filter")]
pub mod threshold;

/// The trait implemented by log4rs filters.
///
/// Filters are associated with appenders and limit the log events that will be
/// sent to that appender.
pub trait Filter: fmt::Debug + Send + Sync + 'static {
    /// Filters a log event.
    fn filter(&self, record: &Record) -> Response;
}

#[cfg(feature = "config_parsing")]
impl Deserializable for dyn Filter {
    fn name() -> &'static str {
        "filter"
    }
}

#[derive(PartialEq, Debug)]
/// The response returned by a filter.
pub enum Response {
    /// Accept the log event.
    ///
    /// It will be immediately passed to the appender, bypassing any remaining
    /// filters.
    Accept,

    /// Take no action on the log event.
    ///
    /// It will continue on to remaining filters or pass on to the appender if
    /// there are none remaining.
    Neutral,

    /// Reject the log event.
    Reject,
}

/// Configuration for a filter.
#[cfg(feature = "config_parsing")]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct FilterConfig {
    /// The filter kind.
    pub kind: String,
    /// The filter configuration.
    pub config: Value,
}

#[cfg(feature = "config_parsing")]
impl<'de> de::Deserialize<'de> for FilterConfig {
    fn deserialize<D>(d: D) -> Result<FilterConfig, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut map = BTreeMap::<Value, Value>::deserialize(d)?;

        let kind = match map.remove(&Value::String("kind".to_owned())) {
            Some(kind) => kind.deserialize_into().map_err(|e| e.to_error())?,
            None => return Err(de::Error::missing_field("kind")),
        };

        Ok(FilterConfig {
            kind,
            config: Value::Map(map),
        })
    }
}

#[cfg(test)]
mod test {
    #[cfg(all(feature = "config_parsing", feature = "yaml_format"))]
    use super::*;

    #[test]
    #[cfg(all(feature = "config_parsing", feature = "yaml_format"))]
    fn test_cfg_deserializer() {
        // This point in the config should have already parsed out the filters portion of the config.
        let cfg_str = "
        filters:
        -  kind: threshold
           level: error
        ";
        let filter: Result<FilterConfig, serde_yaml::Error> = serde_yaml::from_str(cfg_str);
        assert!(filter.is_err());

        let cfg_str = "
          kind: threshold
          level: error
        ";

        let filter: Result<FilterConfig, serde_yaml::Error> = serde_yaml::from_str(cfg_str);
        assert!(filter.is_ok());
    }
}
