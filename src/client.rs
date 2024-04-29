use crate::config::Config;

/// A minimal [EinStack](https://einstack.ai/) client.
///
/// #### Example
///
/// ```rust,no_run
/// use glide_rs::Client;
///
/// let client = Client::new();
/// ```
#[derive(Clone)]
pub struct Client {}

impl Client {
    /// Creates a new [`Client`].
    pub fn new() -> Self {
        let config = Config::new();
        Self {}
    }
}
