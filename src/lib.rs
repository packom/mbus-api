#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.3.5";

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetResponse {
    /// OK
    OK
    (String)
    ,
    /// Bad request
    BadRequest
    (String)
    ,
    /// Not found (or M-Bus HTTPD is unauthorized to access it, or to change baud rate to that specified, etc)
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum GetMultiResponse {
    /// OK
    OK
    (String)
    ,
    /// Bad request
    BadRequest
    (String)
    ,
    /// Not found (or M-Bus HTTPD is unauthorized to access it, or to change baud rate to that specified, etc)
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum HatResponse {
    /// OK
    OK
    (models::Hat)
    ,
    /// Not found
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum HatOffResponse {
    /// OK
    OK
    ,
    /// Not found
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum HatOnResponse {
    /// OK
    OK
    ,
    /// Not found
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum MbusApiResponse {
    /// OK
    OK
    (String)
    ,
    /// Not found
    NotFound
    (String)
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum ScanResponse {
    /// OK
    OK
    (String)
    ,
    /// Bad request
    BadRequest
    (String)
    ,
    /// Not found (e.g. device not found, or M-Bus HTTPD is unauthorized to access it, or to change baud rate to that specified, device not responding etc)
    NotFound
    (String)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        context: &C) -> Result<GetResponse, ApiError>;

    async fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        maxframes: i32,
        context: &C) -> Result<GetMultiResponse, ApiError>;

    async fn hat(
        &self,
        context: &C) -> Result<HatResponse, ApiError>;

    async fn hat_off(
        &self,
        context: &C) -> Result<HatOffResponse, ApiError>;

    async fn hat_on(
        &self,
        context: &C) -> Result<HatOnResponse, ApiError>;

    async fn mbus_api(
        &self,
        context: &C) -> Result<MbusApiResponse, ApiError>;

    async fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        context: &C) -> Result<ScanResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        ) -> Result<GetResponse, ApiError>;

    async fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        maxframes: i32,
        ) -> Result<GetMultiResponse, ApiError>;

    async fn hat(
        &self,
        ) -> Result<HatResponse, ApiError>;

    async fn hat_off(
        &self,
        ) -> Result<HatOffResponse, ApiError>;

    async fn hat_on(
        &self,
        ) -> Result<HatOnResponse, ApiError>;

    async fn mbus_api(
        &self,
        ) -> Result<MbusApiResponse, ApiError>;

    async fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        ) -> Result<ScanResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        ) -> Result<GetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get(device, baudrate, address, &context).await
    }

    async fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: String,
        maxframes: i32,
        ) -> Result<GetMultiResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_multi(device, baudrate, address, maxframes, &context).await
    }

    async fn hat(
        &self,
        ) -> Result<HatResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().hat(&context).await
    }

    async fn hat_off(
        &self,
        ) -> Result<HatOffResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().hat_off(&context).await
    }

    async fn hat_on(
        &self,
        ) -> Result<HatOnResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().hat_on(&context).await
    }

    async fn mbus_api(
        &self,
        ) -> Result<MbusApiResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().mbus_api(&context).await
    }

    async fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        ) -> Result<ScanResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().scan(device, baudrate, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
