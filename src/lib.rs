use url::Url;
use reqwest::header::HeaderMap;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct ObjectStorage {
    client: simplerusthttpsclient::HttpClient,
}

impl ObjectStorage {

    pub fn new(
        base_url: Url,
        tls_config: Option<simplerusthttpsclient::TlsConfig>,
        default_headers: Option<HeaderMap>,
    ) -> Self {
        Self {
            client: simplerusthttpsclient::HttpClient::new(base_url, tls_config, default_headers),
        }
    }
}
