#[macro_use]
extern crate mime;
extern crate iron;
extern crate plugin;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[cfg(feature = "rustc_serialize_type")]
extern crate rustc_serialize;

use iron::prelude::*;
use iron::{status};
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use plugin::Plugin as PluginFor;
use iron::headers::ContentType;

#[cfg(feature = "rustc_serialize_type")]
use rustc_serialize::json::{ToJson, Json, self};
#[cfg(feature = "serde_type")]
use serde::ser::Serialize as ToJson;
#[cfg(feature = "serde_type")]
use serde_json::value::{self, Value as Json};

#[derive(Clone)]
pub struct JsonResponse {
    value: Json,
    callback: Option<String>
}

#[cfg(feature = "rustc_serialize_type")]
impl JsonResponse {
    pub fn new<T: ToJson>(value: T, callback: Option<String>) -> JsonResponse {
        JsonResponse {
            value: value.to_json(),
            callback: callback
        }
    }
}

#[cfg(feature = "serde_type")]
impl JsonResponse {
    pub fn new<T: ToJson>(value: T, callback: Option<String>) -> JsonResponse {
        JsonResponse {
            value: value::to_value(&value),
            callback: callback
        }
    }
}


pub struct JsonResponseMiddleware {}

impl typemap::Key for JsonResponseMiddleware {
    type Value = JsonResponse;
}

impl Modifier<Response> for JsonResponse {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<JsonResponseMiddleware>(self);
    }
}

impl PluginFor<Response> for JsonResponseMiddleware {
    type Error = ();

    fn eval(resp: &mut Response) -> Result<JsonResponse, ()> {
        resp.extensions.get::<JsonResponseMiddleware>()
            .ok_or(()).map(|t| t.clone())
    }
}

#[cfg(feature = "rustc_serialize_type")]
impl AfterMiddleware for JsonResponseMiddleware {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        if let Some(j) = resp.extensions.get::<JsonResponseMiddleware>().as_ref() {
            if let Ok(json_string) = json::encode(&j.value) {
                match j.callback {
                    Some(ref cb) => {
                        if resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType(mime!(Text/Javascript; Charset=Utf8)))
                        }
                        let mut jsonp = String::new();
                        jsonp.push_str(cb);
                        jsonp.push('(');
                        jsonp.push(')');
                        resp.set_mut(jsonp);
                    },
                    None => {
                        if resp.headers.has::<ContentType>() {
                            resp.headers.set(ContentType::json());
                        }
                        resp.set_mut(json_string);
                    }
                }
            }
        }
        Ok(resp)
    }
}
