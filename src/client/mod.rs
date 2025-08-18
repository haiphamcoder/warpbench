use crate::{Config, Error, Result};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Method, Request as HyperRequest, StatusCode, Uri};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor,
};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, trace};

type HttpsClient = Client<HttpsConnector<HttpConnector>, Full<Bytes>>;

#[derive(Debug)]
pub struct HttpClient {
    client: HttpsClient,
    config: Config,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: hyper::HeaderMap,
    pub body: String,
    pub body_size: u64,
}

impl HttpClient {
    pub fn new(config: &Config) -> Result<Self> {
        let https = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let client = Client::builder(TokioExecutor::new())
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .build(https);

        Ok(Self {
            client,
            config: config.clone(),
        })
    }

    pub async fn execute_request(&self, _script_request: Option<()>) -> Result<HttpResponse> {
        let request = self.build_request()?;
        
        trace!("Executing request to {}", request.uri());
        
        let response = timeout(self.config.timeout, self.client.request(request))
            .await
            .map_err(|_| Error::Timeout)?
            .map_err(|e| Error::Http(e.to_string()))?;

        let status = response.status();
        let headers = response.headers().clone();

        // Read response body
        let body_bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| Error::Body(e.to_string()))?
            .to_bytes();

        let body_size = body_bytes.len() as u64;
        let body = String::from_utf8_lossy(&body_bytes).to_string();

        debug!("Response: {} bytes, status: {}", body_size, status);

        Ok(HttpResponse {
            status,
            headers,
            body,
            body_size,
        })
    }

    fn build_request(&self) -> Result<HyperRequest<Full<Bytes>>> {
        let uri = Uri::from_str(&self.config.url.to_string())
            .map_err(|e| Error::Http(format!("Invalid URI: {}", e)))?;

        let method = Method::from_str(&self.config.method)
            .map_err(|e| Error::Http(format!("Invalid method: {}", e)))?;

        let mut request_builder = HyperRequest::builder()
            .method(method)
            .uri(uri);

        // Add headers
        for (name, value) in &self.config.headers {
            request_builder = request_builder.header(name, value);
        }

        // Set body
        let body = if let Some(ref body_content) = self.config.body {
            Full::new(Bytes::from(body_content.clone()))
        } else {
            Full::new(Bytes::new())
        };

        request_builder
            .body(body)
            .map_err(|e| Error::Http(format!("Failed to build request: {}", e)))
    }
}

// Removed scripting::Response conversion as scripting is disabled

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[tokio::test]
    async fn test_http_client_creation() {
        let config = Config {
            url: Url::parse("https://httpbin.org/get").unwrap(),
            ..Default::default()
        };

        let client = HttpClient::new(&config);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_simple_get_request() {
        let config = Config {
            url: Url::parse("https://httpbin.org/get").unwrap(),
            timeout: Duration::from_secs(10),
            ..Default::default()
        };

        let client = HttpClient::new(&config).unwrap();
        let response = client.execute_request(None).await;
        
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.status.is_success());
        assert!(response.body_size > 0);
    }
}
