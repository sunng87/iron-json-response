extern crate iron;
extern crate params;
extern crate iron_json_response as ijr;
extern crate serde;
#[macro_use]
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use ijr::{JsonResponse, JsonResponseMiddleware};

mod data {
    use serde_json::value::Value;

    pub fn make_data() -> Value {
        json!({
            "team": "Jiangsu Suning",
            "players": [{
                "name": "Teixeira",
                "apps": 7,
                "goals": 2,
                "assists": 1
            }, {
                "name": "Ramires",
                "apps": 6,
                "goals": 2,
                "assists": 3
            }, {
                "name": "Wu Xi",
                "apps": 7,
                "goals": 1,
                "assists": 2
            }]
        })
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();

    let cb = req.get_ref::<Params>()
        .ok()
        .and_then(|p| p.find(&["cb"]))
        .and_then(|v| match v {
                      &Value::String(ref c) => Some(c.to_string()),
                      _ => None,
                  });

    let json_resp = JsonResponse::new(data, cb);
    resp.set_mut(json_resp).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(handler);
    chain.link_after(JsonResponseMiddleware);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
