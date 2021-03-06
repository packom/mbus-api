//! Main library entry point for mbus_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use openssl::ssl::SslAcceptorBuilder;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use mbus_api::models;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        mbus_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set cerificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = Arc::new(ssl.build());
            let mut tcp_listener = TcpListener::bind(&addr).await.unwrap();
            let mut incoming = tcp_listener.incoming();

            while let (Some(tcp), rest) = incoming.into_future().await {
                if let Ok(tcp) = tcp {
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);
                    let tls_acceptor = Arc::clone(&tls_acceptor);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::accept(&*tls_acceptor, tcp).await.map_err(|_| ())?;

                        let service = service.await.map_err(|_| ())?;

                        Http::new().serve_connection(tls, service).await.map_err(|_| ())
                    });
                }

                incoming = rest;
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use mbus_api::{
    Api,
    GetResponse,
    GetMultiResponse,
    HatResponse,
    HatOffResponse,
    HatOnResponse,
    MbusApiResponse,
    ScanResponse,
};
use mbus_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        context: &C) -> Result<GetResponse, ApiError>
    {
        let context = context.clone();
        info!("get(\"{}\", {:?}, \"{}\") - X-Span-ID: {:?}", device, baudrate, address, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        maxframes: i32,
        context: &C) -> Result<GetMultiResponse, ApiError>
    {
        let context = context.clone();
        info!("get_multi(\"{}\", {:?}, \"{}\", {}) - X-Span-ID: {:?}", device, baudrate, address, maxframes, context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn hat(
        &self,
        context: &C) -> Result<HatResponse, ApiError>
    {
        let context = context.clone();
        info!("hat() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn hat_off(
        &self,
        context: &C) -> Result<HatOffResponse, ApiError>
    {
        let context = context.clone();
        info!("hat_off() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn hat_on(
        &self,
        context: &C) -> Result<HatOnResponse, ApiError>
    {
        let context = context.clone();
        info!("hat_on() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn mbus_api(
        &self,
        context: &C) -> Result<MbusApiResponse, ApiError>
    {
        let context = context.clone();
        info!("mbus_api() - X-Span-ID: {:?}", context.get().0.clone());
        Err("Generic failuare".into())
    }

    async fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        context: &C) -> Result<ScanResponse, ApiError>
    {
        let context = context.clone();
        info!("scan(\"{}\", {:?}) - X-Span-ID: {:?}", device, baudrate, context.get().0.clone());
        Err("Generic failuare".into())
    }

}
