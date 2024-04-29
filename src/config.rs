use std::{env, fmt};
use std::fmt::Formatter;

use reqwest::Url;

pub struct Config {
    user_agent: String,
    base_url: Url,
}

impl Config {
    /// Creates a new [`Config`].
    pub fn new() -> Self {
        let env_base_url = env::var("EINSTACK_BASE_URL")
            .map_or_else(
                |_| Url::parse("https://einstack.com/v1/"),
                |env_var| Url::parse(env_var.as_str()),
            )
            .expect("env variable `EINSTACK_BASE_URL` should be a valid URL");

        let env_user_agent = env::var("EINSTACK_USER_AGENT").unwrap_or_else(|_| {
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        });

        Self {
            user_agent: env_user_agent,
            base_url: env_base_url,
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       f.debug_struct("Client").finish_non_exhaustive()
    }
}

mod types {
    #[derive(Debug)]
    pub struct GlideError {}
}
