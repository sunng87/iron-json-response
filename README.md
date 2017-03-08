Iron Json Response
==================

[![Build Status](https://travis-ci.org/sunng87/iron-json-response.svg?branch=master)](https://travis-ci.org/sunng87/iron-json-response)
[![](http://meritbadge.herokuapp.com/iron-json-response)](https://crates.io/crates/iron-json-response)
[![](https://img.shields.io/crates/d/iron-json-response.svg)](https://crates.io/crates/iron-json-response)

Middleware for json or jsonp response with [Iron
framework](http://ironframework.io). Using
[serde](https://github.com/serde-rs/json) for data type and
serialization.

Usage
-----

### Setup

Middleware setup:

```rust
extern crate iron_json_response as ijr;
use ijr::{JsonResponseMiddleware, JsonResponse};

let mut chain = Chain::new(...);
chain.link_after(JsonResponseMiddleware{});
...
```

### Json

Send json data:

```rust
fn handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let data = ...

    resp.set_mut(JsonResponse::json(data)).set_mut(status::Ok);
    Ok(resp)
}
```

### Jsonp

Send json data via jsonp:

```rust
fn handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let data = ...

    resp.set_mut(JsonResponse::jsonp(data, "cb".to_owned())).set_mut(status::Ok);
    Ok(resp)
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
