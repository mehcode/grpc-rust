use client::Client;
use std::sync::Arc;
use ClientConf;
use Result as grpc_Result;

/// Trait implemented by `XxxClient` structs for `Xxx` trait.
pub trait ClientStub: Sized {
    /// Create a client stub using given `Client` object.
    fn with_client(grpc_client: Arc<Client>) -> Self;
}

pub trait ClientStubExt: Sized {
    /// Create a plain (non-encrypted) client connected to specified host and port
    fn new_plain(host: &str, port: u16, conf: ClientConf) -> grpc_Result<Self>;

    /// Create a client connected to specified host and port using TLS.
    fn new_tls<C: ::tls_api::TlsConnector>(
        host: &str,
        port: u16,
        conf: ClientConf,
    ) -> grpc_Result<Self>;

    /// Create a client connected to specified unix socket.
    fn new_plain_unix(addr: &str, conf: ClientConf) -> grpc_Result<Self>;
}

impl<C: ClientStub> ClientStubExt for C {
    fn new_plain(host: &str, port: u16, conf: ClientConf) -> grpc_Result<Self> {
        Client::new_plain(host, port, conf).map(|c| Self::with_client(Arc::new(c)))
    }

    fn new_tls<T: ::tls_api::TlsConnector>(
        host: &str,
        port: u16,
        conf: ClientConf,
    ) -> grpc_Result<Self> {
        Client::new_tls::<T>(host, port, conf).map(|c| Self::with_client(Arc::new(c)))
    }

    #[cfg(unix)]
    fn new_plain_unix(addr: &str, conf: ClientConf) -> grpc_Result<Self> {
        Client::new_plain_unix(addr, conf).map(|c| Self::with_client(Arc::new(c)))
    }

    #[cfg(not(unix))]
    fn new_plain_unix(addr: &str, conf: ClientConf) -> grpc_Result<Self> {
        Err(::Error::Other("new_plain_unix unsupported"))
    }
}
