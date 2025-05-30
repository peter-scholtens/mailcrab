pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    AsyncIo(#[from] tokio::io::Error),
    #[error("{0}")]
    Tls(#[from] tokio_rustls::rustls::Error),
    #[error("error creating a self-signed key pair {0}")]
    SelfSigned(#[from] rcgen::Error),
    #[error("{0}")]
    Time(#[from] std::time::SystemTimeError),
    #[error("mail server error {0}")]
    Smtp(String),
    #[error("web server error {0}")]
    WebServer(String),
}
