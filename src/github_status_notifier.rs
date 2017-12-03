use std::error::Error;

extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use self::tokio_core::reactor::Core;
use self::futures::{Future, Stream};
use self::futures::future;

use self::hyper::Method;
use self::hyper::client::{Client, Request};
use self::hyper::header::{Authorization, Accept, UserAgent, qitem};
use self::hyper::mime::Mime;
use self::hyper_tls::HttpsConnector;

pub fn run() -> Result<String, Box<Error>> {
    let url = "https://api.github.com/user".parse::<hyper::Uri>().unwrap();
    let mut req = Request::new(Method::Get, url);
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
    let token = String::from("token {Your_Token_Here}");
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(
        UserAgent::new("github-status-notifier"),
    );
    req.headers_mut().set(Authorization(token));

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);
    let work = client.request(req).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        res.body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, hyper::Error>(v)
            })
            .and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, hyper::Error>(s)
            })
    });
    let user = event_loop.run(work).unwrap();
    println!(
        "We've made it outside the request! \
              We got back the following from our \
              request:\n"
    );
    println!("{}", user);
    Ok("ok".to_owned())
}
