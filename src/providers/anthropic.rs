use std::fmt;
use std::sync::Arc;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Anthropic(Arc<Config>);

impl fmt::Debug for Anthropic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}
