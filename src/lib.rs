#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use futures::Stream;
use std::io::Error;

#[deprecated(note = "Import swagger-rs directly")]
pub use swagger::{ApiError, ContextWrapper};
#[deprecated(note = "Import futures directly")]
pub use futures::Future;

pub const BASE_PATH: &'static str = "";
pub const API_VERSION: &'static str = "0.3.0";

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
pub trait Api<C> {
    fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        context: &C) -> Box<dyn Future<Item=GetResponse, Error=ApiError> + Send>;

    fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        maxframes: i32,
        context: &C) -> Box<dyn Future<Item=GetMultiResponse, Error=ApiError> + Send>;

    fn hat(
        &self,
        context: &C) -> Box<dyn Future<Item=HatResponse, Error=ApiError> + Send>;

    fn hat_off(
        &self,
        context: &C) -> Box<dyn Future<Item=HatOffResponse, Error=ApiError> + Send>;

    fn hat_on(
        &self,
        context: &C) -> Box<dyn Future<Item=HatOnResponse, Error=ApiError> + Send>;

    fn mbus_api(
        &self,
        context: &C) -> Box<dyn Future<Item=MbusApiResponse, Error=ApiError> + Send>;

    fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        context: &C) -> Box<dyn Future<Item=ScanResponse, Error=ApiError> + Send>;

}

/// API without a `Context`
pub trait ApiNoContext {
    fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        ) -> Box<dyn Future<Item=GetResponse, Error=ApiError> + Send>;

    fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        maxframes: i32,
        ) -> Box<dyn Future<Item=GetMultiResponse, Error=ApiError> + Send>;

    fn hat(
        &self,
        ) -> Box<dyn Future<Item=HatResponse, Error=ApiError> + Send>;

    fn hat_off(
        &self,
        ) -> Box<dyn Future<Item=HatOffResponse, Error=ApiError> + Send>;

    fn hat_on(
        &self,
        ) -> Box<dyn Future<Item=HatOnResponse, Error=ApiError> + Send>;

    fn mbus_api(
        &self,
        ) -> Box<dyn Future<Item=MbusApiResponse, Error=ApiError> + Send>;

    fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        ) -> Box<dyn Future<Item=ScanResponse, Error=ApiError> + Send>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {
    fn get(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        ) -> Box<dyn Future<Item=GetResponse, Error=ApiError> + Send>
    {
        self.api().get(device, baudrate, address, &self.context())
    }

    fn get_multi(
        &self,
        device: String,
        baudrate: models::Baudrate,
        address: i32,
        maxframes: i32,
        ) -> Box<dyn Future<Item=GetMultiResponse, Error=ApiError> + Send>
    {
        self.api().get_multi(device, baudrate, address, maxframes, &self.context())
    }

    fn hat(
        &self,
        ) -> Box<dyn Future<Item=HatResponse, Error=ApiError> + Send>
    {
        self.api().hat(&self.context())
    }

    fn hat_off(
        &self,
        ) -> Box<dyn Future<Item=HatOffResponse, Error=ApiError> + Send>
    {
        self.api().hat_off(&self.context())
    }

    fn hat_on(
        &self,
        ) -> Box<dyn Future<Item=HatOnResponse, Error=ApiError> + Send>
    {
        self.api().hat_on(&self.context())
    }

    fn mbus_api(
        &self,
        ) -> Box<dyn Future<Item=MbusApiResponse, Error=ApiError> + Send>
    {
        self.api().mbus_api(&self.context())
    }

    fn scan(
        &self,
        device: String,
        baudrate: models::Baudrate,
        ) -> Box<dyn Future<Item=ScanResponse, Error=ApiError> + Send>
    {
        self.api().scan(device, baudrate, &self.context())
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
