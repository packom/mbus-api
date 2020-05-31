use std::marker::PhantomData;
use futures::{Future, future, Stream, stream};
use hyper;
use hyper::{Request, Response, Error, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
use serde_json;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::io;
use url::form_urlencoded;
#[allow(unused_imports)]
use swagger;
use swagger::{ApiError, XSpanIdString, Has, RequestParser};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use swagger::context::ContextualPayload;
use serde_xml_rs;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

use crate::{Api,
     GetResponse,
     HatResponse,
     HatOffResponse,
     HatOnResponse,
     MbusApiResponse,
     ScanResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/mbus/api$",
            r"^/mbus/get/(?P<device>[^/?#]*)/(?P<baudrate>[^/?#]*)/(?P<address>[^/?#]*)$",
            r"^/mbus/hat$",
            r"^/mbus/hat/off$",
            r"^/mbus/hat/on$",
            r"^/mbus/scan/(?P<device>[^/?#]*)/(?P<baudrate>[^/?#]*)$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_MBUS_API: usize = 0;
    pub(crate) static ID_MBUS_GET_DEVICE_BAUDRATE_ADDRESS: usize = 1;
    lazy_static! {
        pub static ref REGEX_MBUS_GET_DEVICE_BAUDRATE_ADDRESS: regex::Regex =
            regex::Regex::new(r"^/mbus/get/(?P<device>[^/?#]*)/(?P<baudrate>[^/?#]*)/(?P<address>[^/?#]*)$")
                .expect("Unable to create regex for MBUS_GET_DEVICE_BAUDRATE_ADDRESS");
    }
    pub(crate) static ID_MBUS_HAT: usize = 2;
    pub(crate) static ID_MBUS_HAT_OFF: usize = 3;
    pub(crate) static ID_MBUS_HAT_ON: usize = 4;
    pub(crate) static ID_MBUS_SCAN_DEVICE_BAUDRATE: usize = 5;
    lazy_static! {
        pub static ref REGEX_MBUS_SCAN_DEVICE_BAUDRATE: regex::Regex =
            regex::Regex::new(r"^/mbus/scan/(?P<device>[^/?#]*)/(?P<baudrate>[^/?#]*)$")
                .expect("Unable to create regex for MBUS_SCAN_DEVICE_BAUDRATE");
    }
}

pub struct MakeService<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<'a, T, SC, RC> hyper::service::MakeService<&'a SC> for MakeService<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, RC>;
    type ResBody = Body;
    type Error = Error;
    type Service = Service<T, RC>;
    type Future = future::FutureResult<Self::Service, Self::MakeError>;
    type MakeError = Error;

    fn make_service(&mut self, _ctx: &'a SC) -> Self::Future {
        future::FutureResult::from(Ok(Service::new(
            self.api_impl.clone(),
        )))
    }
}

type ServiceFuture = Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;

fn method_not_allowed() -> ServiceFuture {
    Box::new(future::ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    ))
}

pub struct Service<T, RC> {
    api_impl: T,
    marker: PhantomData<RC>,
}

impl<T, RC> Service<T, RC>
where
    T: Api<RC> + Clone + Send + 'static,
    RC: Has<XSpanIdString>  + 'static {
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> hyper::service::Service for Service<T, C>
where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + 'static + Send
{
    type ReqBody = ContextualPayload<Body, C>;
    type ResBody = Body;
    type Error = Error;
    type Future = ServiceFuture;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let api_impl = self.api_impl.clone();
        let (parts, body) = req.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());
        let mut context = body.context;
        let body = body.inner;

        match &method {

            // Get - POST /mbus/get/{device}/{baudrate}/{address}
            &hyper::Method::POST if path.matched(paths::ID_MBUS_GET_DEVICE_BAUDRATE_ADDRESS) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MBUS_GET_DEVICE_BAUDRATE_ADDRESS
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MBUS_GET_DEVICE_BAUDRATE_ADDRESS in set but failed match against \"{}\"", path, paths::REGEX_MBUS_GET_DEVICE_BAUDRATE_ADDRESS.as_str())
                    );

                let param_device = match percent_encoding::percent_decode(path_params["device"].as_bytes()).decode_utf8() {
                    Ok(param_device) => match param_device.parse::<String>() {
                        Ok(param_device) => param_device,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter device: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["device"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_baudrate = match percent_encoding::percent_decode(path_params["baudrate"].as_bytes()).decode_utf8() {
                    Ok(param_baudrate) => match param_baudrate.parse::<models::Baudrate>() {
                        Ok(param_baudrate) => param_baudrate,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter baudrate: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["baudrate"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_address = match percent_encoding::percent_decode(path_params["address"].as_bytes()).decode_utf8() {
                    Ok(param_address) => match param_address.parse::<i32>() {
                        Ok(param_address) => param_address,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.get(
                                            param_device,
                                            param_baudrate,
                                            param_address,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/xml")
                                                            .expect("Unable to create Content-Type header for GET_OK"));
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for GET_BAD_REQUEST"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for GET_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // Hat - GET /mbus/hat
            &hyper::Method::GET if path.matched(paths::ID_MBUS_HAT) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.hat(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                HatResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for HAT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                HatResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for HAT_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // HatOff - POST /mbus/hat/off
            &hyper::Method::POST if path.matched(paths::ID_MBUS_HAT_OFF) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.hat_off(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                HatOffResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                HatOffResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for HAT_OFF_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // HatOn - POST /mbus/hat/on
            &hyper::Method::POST if path.matched(paths::ID_MBUS_HAT_ON) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.hat_on(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                HatOnResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                HatOnResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for HAT_ON_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // MbusApi - GET /mbus/api
            &hyper::Method::GET if path.matched(paths::ID_MBUS_API) => {
                Box::new({
                        {{
                                Box::new(
                                    api_impl.mbus_api(
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                MbusApiResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/x-yaml")
                                                            .expect("Unable to create Content-Type header for MBUS_API_OK"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                MbusApiResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for MBUS_API_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            // Scan - POST /mbus/scan/{device}/{baudrate}
            &hyper::Method::POST if path.matched(paths::ID_MBUS_SCAN_DEVICE_BAUDRATE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_MBUS_SCAN_DEVICE_BAUDRATE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE MBUS_SCAN_DEVICE_BAUDRATE in set but failed match against \"{}\"", path, paths::REGEX_MBUS_SCAN_DEVICE_BAUDRATE.as_str())
                    );

                let param_device = match percent_encoding::percent_decode(path_params["device"].as_bytes()).decode_utf8() {
                    Ok(param_device) => match param_device.parse::<String>() {
                        Ok(param_device) => param_device,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter device: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["device"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                let param_baudrate = match percent_encoding::percent_decode(path_params["baudrate"].as_bytes()).decode_utf8() {
                    Ok(param_baudrate) => match param_baudrate.parse::<models::Baudrate>() {
                        Ok(param_baudrate) => param_baudrate,
                        Err(e) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter baudrate: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter"))),
                    },
                    Err(_) => return Box::new(future::ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["baudrate"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode")))
                };

                Box::new({
                        {{
                                Box::new(
                                    api_impl.scan(
                                            param_device,
                                            param_baudrate,
                                        &context
                                    ).then(move |result| {
                                        let mut response = Response::new(Body::empty());
                                        response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ScanResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/xml")
                                                            .expect("Unable to create Content-Type header for SCAN_OK"));
                                                    let body = serde_xml_rs::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ScanResponse::BadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for SCAN_BAD_REQUEST"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ScanResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("text/plain")
                                                            .expect("Unable to create Content-Type header for SCAN_NOT_FOUND"));
                                                    let body = body;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        future::ok(response)
                                    }
                                ))
                        }}
                }) as Self::Future
            },

            _ if path.matched(paths::ID_MBUS_API) => method_not_allowed(),
            _ if path.matched(paths::ID_MBUS_GET_DEVICE_BAUDRATE_ADDRESS) => method_not_allowed(),
            _ if path.matched(paths::ID_MBUS_HAT) => method_not_allowed(),
            _ if path.matched(paths::ID_MBUS_HAT_OFF) => method_not_allowed(),
            _ if path.matched(paths::ID_MBUS_HAT_ON) => method_not_allowed(),
            _ if path.matched(paths::ID_MBUS_SCAN_DEVICE_BAUDRATE) => method_not_allowed(),
            _ => Box::new(future::ok(
                Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response")
            )) as Self::Future
        }
    }
}

impl<T, C> Clone for Service<T, C> where T: Clone
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Result<&'static str, ()> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // Get - POST /mbus/get/{device}/{baudrate}/{address}
            &hyper::Method::POST if path.matched(paths::ID_MBUS_GET_DEVICE_BAUDRATE_ADDRESS) => Ok("Get"),
            // Hat - GET /mbus/hat
            &hyper::Method::GET if path.matched(paths::ID_MBUS_HAT) => Ok("Hat"),
            // HatOff - POST /mbus/hat/off
            &hyper::Method::POST if path.matched(paths::ID_MBUS_HAT_OFF) => Ok("HatOff"),
            // HatOn - POST /mbus/hat/on
            &hyper::Method::POST if path.matched(paths::ID_MBUS_HAT_ON) => Ok("HatOn"),
            // MbusApi - GET /mbus/api
            &hyper::Method::GET if path.matched(paths::ID_MBUS_API) => Ok("MbusApi"),
            // Scan - POST /mbus/scan/{device}/{baudrate}
            &hyper::Method::POST if path.matched(paths::ID_MBUS_SCAN_DEVICE_BAUDRATE) => Ok("Scan"),
            _ => Err(()),
        }
    }
}
