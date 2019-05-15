//! High-level interface for web_sys HTTP requests.
//! # References
//! * [WASM bindgen fetch](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html)
//! * [JS Promises and Rust Futures](https://rustwasm.github.io/wasm-bindgen/reference/js-promises-and-rust-futures.html)
//! * [web_sys Request](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Request.html)
//! * [WASM bindgen Futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/)
//! * [web_sys Response](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Response.html)

use futures::Future;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys;
use std::{rc::Rc, cell::RefCell};
use gloo_timers::callback::Timeout;
use std::convert::identity;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

pub type DomException = web_sys::DomException;

// TODO once this is polished, publish as a standalone crate.
// @TODO: once await/async stabilized, refactor

// @TODO: examples - title, description
// @TODO make examples nicer
// @TODO: refactor this file
// @TODO: add references here at the top?
// @TODO documentation everywehere (vdom, dom-types, all examples, examples lib, fetch)

pub type ResponseResult<T> = Result<Response<T>, FailReason>;
pub type FetchResult<T> = Result<ResponseWithDataResult<T>, RequestError>;
pub type DataResult<T> = Result<T, DataError>;

#[derive(Debug, Clone)]
pub struct FetchObject<T> {
    pub request: Request,
    pub result: FetchResult<T>
}

impl<T> FetchObject<T> {
    pub fn response(self) -> ResponseResult<T> {
        match self.result {
            Ok(response) => {
                if response.status.code() >= 400 {
                    Err(FailReason::Status(response.status))
                } else {
                    match response.data {
                        Ok(data) => {
                            Ok(Response {
                                raw: response.raw,
                                status: response.status,
                                data,
                            })
                        }
                        Err(data_error) => {
                            Err(FailReason::DataError(data_error))
                        }
                    }
                }
            }
            Err(request_error) => Err(FailReason::RequestError(request_error))
        }
    }
}

#[derive(Debug, Clone)]
pub enum FailReason {
    RequestError(RequestError),
    Status(Status),
    DataError(DataError),
}

#[derive(Debug, Clone)]
pub enum RequestError {
    DomException(web_sys::DomException),
}

#[derive(Debug, Clone)]
pub enum DataError {
    DomException(web_sys::DomException),
    SerdeError(Rc<serde_json::Error>),
}

#[derive(Debug, Clone)]
pub struct RequestController {
    abort_controller: Rc<web_sys::AbortController>,
    timeout_handle: Rc<RefCell<Option<Timeout>>>,
}

impl RequestController {
    pub fn abort(&self) {
        // cancel timeout by dropping it
        self.timeout_handle.replace(None);
        self.abort_controller.abort();
    }
    pub fn disable_timeout(&self) -> Result<(), &'static str> {
        // cancel timeout by dropping it
        match self.timeout_handle.replace(None) {
            Some(_) => Ok(()),
            None => Err("disable_timeout: already disabled")
        }
    }
}

#[derive(Debug, Clone)]
pub enum Status {
    // 1xx
    Informational(u16, String),
    // 2xx
    Success(u16, String),
    // 3xx
    Redirection(u16, String),
    // 4xx
    ClientError(u16, String),
    // 5xx
    ServerError(u16, String),
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Status::Informational(code, _)
            | Status::Success(code, _)
            | Status::Redirection(code, _)
            | Status::ClientError(code, _)
            | Status::ServerError(code, _) => *code
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Status::Informational(_, text)
            | Status::Success(_, text)
            | Status::Redirection(_, text)
            | Status::ClientError(_, text)
            | Status::ServerError(_, text) => text
        }
    }
}

#[derive(Debug, Clone)]
pub struct Response<T> {
    pub raw: web_sys::Response,
    pub status: Status,
    pub data: T,
}

#[derive(Debug, Clone)]
pub struct ResponseWithDataResult<T> {
    pub raw: web_sys::Response,
    pub status: Status,
    pub data: Result<T, DataError>,
}

impl<T> ResponseWithDataResult<T> {
    fn map_data<U>(self, data_mapper: impl FnOnce(Result<T, DataError>) -> Result<U, DataError>) -> ResponseWithDataResult<U> {
        ResponseWithDataResult {
            raw: self.raw,
            status: self.status,
            data: data_mapper(self.data),
        }
    }
}

/// HTTP Method types
///
/// # References
/// * [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods)
#[derive(Debug, Clone, Copy)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Method {
    fn as_str(&self) -> &str {
        match *self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
        }
    }
}

/// Request is the entry point for all fetch requests. Its methods configure
/// the request, and and handle the response. Many of them return the original
/// struct, and are intended to be used chained together.
#[derive(Debug, Clone)]
pub struct Request {
    url: String,
    init: web_sys::RequestInit,
    headers: Option<web_sys::Headers>,
    timeout: Option<u32>,
    controller: Option<RequestController>,
}

impl Request {
    pub fn new(url: String) -> Self {
        Self {
            url,
            init: web_sys::RequestInit::new(),
            headers: None,
            timeout: None,
            controller: None,
        }
    }

    /// Set the HTTP method
    pub fn method(mut self, val: Method) -> Self {
        self.init.method(val.as_str());
        self
    }

    fn set_header(&mut self, name: &str, val: &str) {
        let headers = self
            .headers
            .get_or_insert_with(|| web_sys::Headers::new().expect("Cannot create headers!"));

        headers.set(name, val).expect("Cannot set Header!");
    }

    /// Add a single header. String multiple calls to this together to add multiple ones.
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers
    pub fn header(mut self, name: &str, val: &str) -> Self {
        self.set_header(name, val);
        self
    }

    pub fn body(mut self, val: &JsValue) -> Self {
        self.init.body(Some(val));
        self
    }

    fn get_json<A: Serialize>(val: &A) -> JsValue {
        let json = serde_json::to_string(val).expect("Error serializing JSON");
        JsValue::from_str(&json)
    }

    /// Serialize a Rust data structure as JSON; eg the payload in a POST request.
    pub fn body_json<A: Serialize>(self, val: &A) -> Self {
        self.body(&Self::get_json(val))
    }

    pub fn cache(mut self, val: web_sys::RequestCache) -> Self {
        self.init.cache(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/Request/credentials
    pub fn credentials(mut self, val: web_sys::RequestCredentials) -> Self {
        self.init.credentials(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/Security/Subresource_Integrity
    #[inline]
    pub fn integrity(mut self, val: &str) -> Self {
        self.init.integrity(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/Request/mode
    pub fn mode(mut self, val: web_sys::RequestMode) -> Self {
        self.init.mode(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Redirections
    pub fn redirect(mut self, val: web_sys::RequestRedirect) -> Self {
        self.init.redirect(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/API/Document/referrer
    pub fn referrer(mut self, val: &str) -> Self {
        self.init.referrer(val);
        self
    }

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referrer-Policy
    pub fn referrer_policy(mut self, val: web_sys::ReferrerPolicy) -> Self {
        self.init.referrer_policy(val);
        self
    }

    pub fn timeout(mut self, millis: u32) -> Self {
        self.lazy_controller();
        self.timeout = Some(millis);
        self
    }

    pub fn send_json<T: Serialize>(self, data: &T) -> Self {
        self
            .header("Content-Type", "application/json; charset=utf-8")
            .body_json(data)
    }

    fn lazy_controller(&mut self) -> RequestController {
        match &mut self.controller {
            Some(controller) => controller.clone(),
            None => {
                let abort_controller = Rc::new(
                    web_sys::AbortController::new().expect("fetch: create AbortController - failed")
                );
                self.init.signal(Some(&abort_controller.signal()));
                let request_controller = RequestController {
                    abort_controller: abort_controller.clone(),
                    timeout_handle: Rc::new(RefCell::new(None)),
                };
                self.controller = Some(request_controller.clone());
                request_controller
            }
        }
    }

    pub fn controller(mut self, controller_transferrer: impl FnOnce(RequestController)) -> Self {
        controller_transferrer(self.lazy_controller());
        self
    }

    fn prepare_request_for_send(&mut self) {
        if let Some(ref headers) = self.headers {
            self.init.headers(headers.as_ref());
        }
    }

    fn send_request(mut self) -> impl Future<Item=web_sys::Response, Error=JsValue> {
        self.prepare_request_for_send();

        let fetch_promise = web_sys::window()
            .expect("Cannot find window!")
            .fetch_with_str_and_init(&self.url, &self.init);

        if let Some(millis) = self.timeout {
            let controller = self.controller.clone()
                .expect("send_request: `controller` cannot be `None` if `timeout` is `Some`!");
            let abort_controller = controller.abort_controller.clone();

            *controller.timeout_handle.borrow_mut() = Some(
                Timeout::new(millis, move || abort_controller.abort())
            );
        };

        JsFuture::from(fetch_promise).map(|js_value| js_value.into())
    }

    fn create_status(response: &web_sys::Response) -> Status {
        match response.status() {
            code @ 100..=199 => Status::Informational(code, response.status_text()),
            code @ 200..=299 => Status::Success(code, response.status_text()),
            code @ 300..=399 => Status::Redirection(code, response.status_text()),
            code @ 400..=499 => Status::ClientError(code, response.status_text()),
            code @ 500..=599 => Status::ServerError(code, response.status_text()),
            code => panic!("create_status: invalid status code: {}", code)
        }
    }

    /// Use this if you want access to the web_sys::Request, eg for status code.
    pub fn fetch<U: 'static>(self, f: impl FnOnce(FetchObject<()>) -> U) -> impl Future<Item=U, Error=U> {
        let request_cloned = self.clone();
        futures::future::ok(())
            .and_then(move |_| {
                self.send_request()
            })
            .map(|raw_response: web_sys::Response| {
                ResponseWithDataResult {
                    status: Self::create_status(&raw_response),
                    raw: raw_response,
                    data: Ok(()),
                }
            })
            .map_err(|js_value_error| RequestError::DomException(js_value_error.into()))
            .then(|fetch_result|{
                Ok(f(FetchObject {
                    request: request_cloned,
                    result: fetch_result,
                }))
            })
    }

    // Use this for the response's text.
    /// https://developer.mozilla.org/en-US/docs/Web/API/Body/text
    pub fn fetch_string<U: 'static>(self, f: impl FnOnce(FetchObject<String>) -> U) -> impl Future<Item=U, Error=U> {
        let response_from_fetch_object: fn(Result<FetchObject<()>, FetchObject<()>>) -> Result<(FetchObject<()>, ResponseWithDataResult<()>), (FetchObject<()>, RequestError)> =
            |fetch_object_result| {
                let fetch_object = fetch_object_result.unwrap();
                let fetch_object_clone_1 = fetch_object.clone();
                let fetch_object_clone_2 = fetch_object.clone();
                fetch_object.result
                    .map(|response| (fetch_object_clone_1, response))
                    .map_err(|request_error| (fetch_object_clone_2, request_error))
            };

        let text_js_future_from_response: fn((FetchObject<()>, ResponseWithDataResult<()>)) -> Result<(FetchObject<()>, ResponseWithDataResult<()>, JsFuture), (FetchObject<()>, RequestError)> =
            |(fetch_object, response)| {
                let fetch_object_clone = fetch_object.clone();
                response.raw.text()
                    .map(|promise| (fetch_object, response, JsFuture::from(promise)))
                    .map_err(|js_value_error| (fetch_object_clone, RequestError::DomException(js_value_error.into())))
            };

        let resolve_js_future: fn((FetchObject<()>, ResponseWithDataResult<()>, JsFuture)) -> _ =
            |(fetch_object, response, js_future)| {
                let fetch_object_clone = fetch_object.clone();
                js_future
                    .map(|js_value| (fetch_object, response, js_value))
                    .map_err(|js_value_error| (fetch_object_clone, RequestError::DomException(js_value_error.into())))
            };

        let js_value_to_string: fn((FetchObject<()>, ResponseWithDataResult<()>, JsValue)) -> Result<(FetchObject<()>, ResponseWithDataResult<()>, Result<String, DataError>), (FetchObject<()>, RequestError)> =
            |(fetch_object, response, js_value)| {
                let text = js_value.as_string()
                    .expect("fetch_string: cannot convert js_value to string");
                Ok((fetch_object, response, Ok(text)))
            };

        self
            .fetch(identity)
            .then(response_from_fetch_object)
            .and_then(text_js_future_from_response)
            .and_then(resolve_js_future)
            .and_then(js_value_to_string)
            .map(|(fetch_object, response, value)| {
                (fetch_object, response.map_data(|_| value))
            })
            .map(|(fetch_object, response)|{
                FetchObject {
                    request: fetch_object.request,
                    result: Ok(response)
                }
            })
            .map_err(|(fetch_object, request_error)| {
                FetchObject {
                    request: fetch_object.request,
                    result: Err(request_error)
                }
            })
            .then(|fetch_object_result| {
                fetch_object_result
                    .or_else(|fetch_object| Ok(fetch_object))
                    .map(f)
            })
    }

    /// Use this to access the response's JSON:
    /// https://developer.mozilla.org/en-US/docs/Web/API/Body/json
    pub fn fetch_json<T: DeserializeOwned, U: 'static>(self, f: impl FnOnce(FetchObject<T>) -> U) -> impl Future<Item=U, Error=U> {
        let response_from_fetch_object: fn(Result<FetchObject<String>, FetchObject<String>>) -> Result<(FetchObject<String>, ResponseWithDataResult<String>), (FetchObject<String>, RequestError)> =
            |fetch_object_result| {
                let fetch_object = fetch_object_result.unwrap();
                let fetch_object_clone_1 = fetch_object.clone();
                let fetch_object_clone_2 = fetch_object.clone();
                fetch_object.result
                    .map(|response| (fetch_object_clone_1, response))
                    .map_err(|request_error| (fetch_object_clone_2, request_error))
            };

        let deserialize: fn((FetchObject<String>, ResponseWithDataResult<String>)) -> Result<(FetchObject<String>, ResponseWithDataResult<String>, Result<T, DataError>), (FetchObject<String>, RequestError)> =
            |(fetch_object, response)| {
                match response.data.clone() {
                    Ok(text) => {
                        match serde_json::from_str(&text) {
                            Ok(value) => Ok((fetch_object, response, Ok(value))),
                            Err(err) => Ok((fetch_object, response, Err(DataError::SerdeError(Rc::new(err)))))
                        }
                    }
                    Err(err) => Ok((fetch_object, response, Err(err)))
                }
            };

        self
            .fetch_string(identity)
            .then(response_from_fetch_object)
            .and_then(deserialize)
            .map(|(fetch_object, response, value)| {
                (fetch_object, response.map_data(|_| value))
            })
            .map(|(fetch_object, response)|{
                FetchObject {
                    request: fetch_object.request,
                    result: Ok(response)
                }
            })
            .map_err(|(fetch_object, request_error)| {
                FetchObject {
                    request: fetch_object.request,
                    result: Err(request_error)
                }
            })
            .then(|fetch_object_result| {
                fetch_object_result
                    .or_else(|fetch_object| Ok(fetch_object))
                    .map(f)
            })
    }
}