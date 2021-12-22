use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("error parsing param for plugin {0}")]
    ParseParam(String),
    #[error(r#"config field "{field:}" for plugin "{plugin:}" is not valid"#)]
    InvalidParam { plugin: String, field: &'static str },
    #[error(r#"cannot find descriptor "{descriptor:}" from plugin "{initiator:}", or "{descriptor:}" failed to load previously"#)]
    NoAccessPoint {
        initiator: String,
        descriptor: String,
    },
    #[error(r#"descriptor "{descriptor:}" cannot be used as a {r#type:} required by plugin "{initiator:?}""#)]
    BadAccessPointType {
        initiator: String,
        r#type: String,
        descriptor: String,
    },
    #[error(r#"cannot find plugin "{plugin:}" from plugin "{initiator:}""#)]
    NoPlugin { initiator: String, plugin: String },
    #[error(r#"don't know how to create plugin "{initiator:}" of type "{r#type:}" v{version:}"#)]
    NoPluginType {
        initiator: String,
        r#type: String,
        version: u16,
    },
    #[error("loading plugin {0} exceeds the depth limit")]
    RecursionLimitExceeded(String),
    #[error(r#"there are too many plugins of type "{r#type:}" when loading plugin "{plugin:}""#)]
    TooManyPlugin {
        plugin: String,
        r#type: &'static str,
    },
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("error in config")]
    Config(#[from] ConfigError),
    #[error(r#""{0}" cannot select a useable network interface"#)]
    NoUseableNetif(String),
    #[error(r#"plugin "{initiator:}" requires "{descriptor:}" to be fully loaded"#)]
    UnsatisfiedStrongReference {
        initiator: String,
        descriptor: String,
    },
}

pub type ConfigResult<T> = Result<T, ConfigError>;
pub type LoadResult<T> = Result<T, LoadError>;