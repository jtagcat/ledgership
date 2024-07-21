use serde::Deserialize;
use std::{env, error::Error, fs, io};
use thiserror::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Endpoint {
    SandboxEU,
    LiveEU,
    LiveUK,
}

impl Endpoint {
    pub fn as_string(&self) -> &str {
        match self {
            Endpoint::SandboxEU => "https://connect.prelive.lhv.eu/",
            Endpoint::LiveEU => "https://connect.lhv.eu/",
            Endpoint::LiveUK => "https://connect.lhv.com/",
        }
    }

    pub fn with(&self, path: &str) -> String {
        return self.as_string().to_owned() + path.strip_prefix("/").unwrap_or(path);
    }
}

impl Default for Endpoint {
    fn default() -> Self {
        Endpoint::SandboxEU
    }
}

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub endpoint: Endpoint,
}

#[derive(Error, Debug)]
pub enum ClientFromEnvError {
    #[error("Reading environment {0}")]
    ReadEnv(String, env::VarError),
    #[error("Reading certificate {0}")]
    ReadCert(String, io::Error),
    #[error("Parsing root certificate")]
    ParseRoot(reqwest::Error),
    #[error("Parsing client certificate")]
    ParseClient(reqwest::Error),
    #[error("Creating HTTP client with certificates")]
    ClientFromCert(ClientFromCertError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ClientFromCertError {
    BuildReqwestClient(reqwest::Error),
    FirstHeartbeat(Box<dyn Error>),
}

impl Client {
    // Pem-encoded CONNECT_CERT_CHAIN, CONNECT_CERT_PRIVATE, and
    // CONNECT_CERT_ROOT (likely https://cacerts.digicert.com/DigiCertHighAssuranceEVRootCA.crt.pem)
    pub async fn from_env(endpoint: Endpoint) -> Result<Client, ClientFromEnvError> {
        fn from_var(key: &str) -> Result<Vec<u8>, ClientFromEnvError> {
            let cert_path =
                env::var(key).map_err(|e| ClientFromEnvError::ReadEnv(key.to_owned(), e))?;

            Ok(fs::read(cert_path.clone())
                .map_err(|e| ClientFromEnvError::ReadCert(cert_path, e))?)
        }

        let root = reqwest::Certificate::from_pem(&from_var("CONNECT_CERT_ROOT")?)
            .map_err(|e| ClientFromEnvError::ParseRoot(e))?;

        let identity = reqwest::Identity::from_pkcs8_pem(
            &from_var("CONNECT_CERT_CHAIN")?,
            &from_var("CONNECT_CERT_PRIVATE")?,
        )
        .map_err(|e| ClientFromEnvError::ParseClient(e))?;

        Ok(Client::from_cert(endpoint, root, identity)
            .await
            .map_err(|e| ClientFromEnvError::ClientFromCert(e))?)
    }

    pub async fn from_cert(
        endpoint: Endpoint,
        root_cert: reqwest::Certificate, // https://cacerts.digicert.com/DigiCertHighAssuranceEVRootCA.crt.pem
        identity: reqwest::Identity,
    ) -> Result<Client, ClientFromCertError> {
        let client = reqwest::Client::builder()
            .tls_built_in_root_certs(false)
            .add_root_certificate(root_cert)
            .identity(identity)
            // .proxy(reqwest::Proxy::https("http://127.0.0.1:8080").unwrap()) // DEBUG:
            .build()
            .map_err(|e| ClientFromCertError::BuildReqwestClient(e))?;

        Client::new(endpoint, client)
            .await
            .map_err(|e| ClientFromCertError::FirstHeartbeat(e))
    }

    pub async fn new(
        endpoint: Endpoint,
        client: reqwest::Client,
    ) -> Result<Client, Box<dyn Error>> {
        let client = Client { client, endpoint };

        _ = client.heartbeat().await?;

        Ok(client)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct HeartbeatResponse {
    #[serde(rename = "TimeStamp")]
    timestamp: String,
}

impl Client {
    pub async fn heartbeat(&self) -> Result<chrono::DateTime<chrono::FixedOffset>, Box<dyn Error>> {
        let body = self
            .client
            .get(self.endpoint.with("/heartbeat"))
            .send()
            .await?
            .text()
            .await?;

        let response: HeartbeatResponse = serde_xml_rs::from_str(&body)?;

        Ok(chrono::DateTime::parse_from_rfc3339(&response.timestamp)?)
    }
}
