#[macro_use]
extern crate mime;
extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::{AfterMiddleware, typemap};
use iron::modifier::Modifier;
use iron::headers::ContentType;

use serde::ser::Serialize as ToJson;
use serde_json::value::{self, Value as Json};

#[derive(Clone)]
pub struct JsonResponse {
    value: Json,
    callback: Option<String>,
}

impl JsonResponse {
    pub fn new<T: ToJson>(value: T, callback: Option<String>) -> JsonResponse {
        JsonResponse {
            value: value::to_value(&value).unwrap_or(Json::Null),
            callback: callback,
        }
    }


    pub fn json<T: ToJson>(value: T) -> JsonResponse {
        JsonResponse::new(value, None)
    }

    pub fn jsonp<T: ToJson>(value: T, cb_name: String) -> JsonResponse {
        JsonResponse::new(value, Some(cb_name))
    }
}


pub struct JsonResponseMiddleware;

impl JsonResponseMiddleware {
    pub fn new() -> Self {
        JsonResponseMiddleware {}
    }
}

impl typemap::Key for JsonResponseMiddleware {
    type Value = JsonResponse;
}

impl Modifier<Response> for JsonResponse {
    fn modify(self, resp: &mut Response) {
        resp.extensions.insert::<JsonResponseMiddleware>(self);
    }
}

impl AfterMiddleware for JsonResponseMiddleware {
    fn after(&self, _: &mut Request, r: Response) -> IronResult<Response> {
        let mut resp = r;
        let header_body =
            resp.extensions
                .remove::<JsonResponseMiddleware>()
                .and_then(|j| {
                    if let Ok(json_string) = serde_json::to_string(&j.value) {
                        match j.callback {
                            Some(ref cb) => {
                                let mut jsonp = String::new();
                                jsonp.push_str(cb);
                                jsonp.push('(');
                                jsonp.push_str(&json_string);
                                jsonp.push(')');
                                Some((ContentType(mime!(Text/Javascript; Charset=Utf8)), jsonp))
                            }
                            None => Some((ContentType::json(), json_string)),
                        }
                    } else {
                        None
                    }
                });

        if let Some((content_type, body)) = header_body {
            if !resp.headers.has::<ContentType>() {
                resp.headers.set(content_type);
            }
            resp.set_mut(body);
        }
        Ok(resp)
    }

    fn catch(&self, req: &mut Request, mut err: IronError) -> IronResult<Response> {
        err.response = try!(self.after(req, err.response));
        Err(err)
    }
}
