pub(crate) const BIN_NAME: &str = env!("CARGO_BIN_NAME");
pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const LAST_COMMIT_ID: Option<&'static str> = option_env!("LAST_COMMIT_ID");
pub(crate) const LAST_COMMIT_ID_LONG: Option<&'static str> = option_env!("LAST_COMMIT_ID_LONG");
pub(crate) const LAST_COMMIT_DATE: Option<&'static str> = option_env!("LAST_COMMIT_DATE");
pub(crate) const BUILD_TIMESTAMP_UTC: Option<&'static str> = option_env!("BUILD_TIMESTAMP_UTC");
