extern crate iron;
extern crate params;
extern crate iron_json_response as ijr;
#[cfg(feature = "rustc_serialize_type")]
extern crate rustc_serialize;
#[cfg(feature = "serde_type")]
extern crate serde;
#[cfg(feature = "serde_type")]
extern crate serde_json;
#[macro_use]
extern crate maplit;

use iron::prelude::*;
use iron::{status};
use params::{Params, Value};
use ijr::{JsonResponse, JsonResponseMiddleware};

#[cfg(feature = "rustc_serialize_type")]
mod data {
    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;

    pub fn make_data () -> Json {
        let player: Vec<BTreeMap<String, Json>> = vec![
            btreemap! {
                "name".to_owned() => "Teixeira".to_owned().to_json(),
                "apps".to_owned() => 7u16.to_json(),
                "goals".to_owned() => 2u16.to_json(),
                "assists".to_owned() => 1u16.to_json()
            },
            btreemap! {
                "name".to_owned() => "Ramires".to_owned().to_json(),
                "apps".to_owned() => 6u16.to_json(),
                "goals".to_owned() => 2u16.to_json(),
                "assists".to_owned() => 3u16.to_json()
            },
            btreemap! {
                "name".to_owned() => "Wu Xi".to_owned().to_json(),
                "apps".to_owned() => 7u16.to_json(),
                "goals".to_owned() => 1u16.to_json(),
                "assists".to_owned() => 2u16.to_json()
            }
        ];
        let team = btreemap! {
            "team".to_owned() => "Jiangsu Suning".to_owned().to_json(),
            "players".to_owned() => player.to_json()
        };
        team.to_json()
    }
}

#[cfg(feature = "serde_type")]
mod data {
    use serde_json::value::{self, Value};
    use std::collections::BTreeMap;

    pub fn make_data () -> Value {
        let player: Vec<BTreeMap<String, Value>> = vec![
            btreemap! {
                "name".to_owned() => value::to_value("Teixeira"),
                "apps".to_owned() => value::to_value(&7u16),
                "goals".to_owned() => value::to_value(&2u16),
                "assists".to_owned() => value::to_value(&1u16)
            },
            btreemap! {
                "name".to_owned() => value::to_value("Ramires"),
                "apps".to_owned() => value::to_value(&6u16),
                "goals".to_owned() => value::to_value(&2u16),
                "assists".to_owned() => value::to_value(&3u16)
            },
            btreemap! {
                "name".to_owned() => value::to_value("Wu Xi"),
                "apps".to_owned() => value::to_value(&7u16),
                "goals".to_owned() => value::to_value(&1u16),
                "assists".to_owned() => value::to_value(&2u16)
            }
        ];
        let team = btreemap! {
            "team".to_owned() => value::to_value("Jiangsu Suning"),
            "players".to_owned() => value::to_value(&player)
        };
        value::to_value(&team)
    }
}

fn handler(req: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();

    let cb = req.get_ref::<Params>().ok()
        .and_then(|p| p.find(&["cb"]))
        .and_then(|v| match v {
            &Value::String(ref c) => Some(c.to_string()),
            _ => None
        });

    let json_resp = JsonResponse::new(data, cb);
    resp.set_mut(json_resp).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    let mut chain = Chain::new(handler);
    chain.link_after(JsonResponseMiddleware{});
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
