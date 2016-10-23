/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 * */

extern crate clap;
extern crate rand;
extern crate redis;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tk_bufstream;
extern crate netbuf;
extern crate minihttp;

mod db;

use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use std::rc::Rc;

use clap::App;

use futures::{Async, Finished, finished, IntoFuture};
use tokio_core::io::Io;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_service::{Service, NewService};
use tk_bufstream::IoBuf;
use minihttp::{Request, Error, ResponseFn, ResponseWriter};

/*struct Context {
    counter: usize,
}

trait Counter {
    fn increment(&mut self);
    fn get(&self) -> usize;
}

impl Counter for Context {
    fn increment(&mut self) {
        self.counter += 1;
    }

    fn get(&self) -> usize {
        self.counter
    }
}

#[derive(Debug, Clone)]
enum PasteRoutes {
    New,
    GetPaste(String),
    MakePaste,
    GetNum,
    PageNotFound,
}

impl Server for PasteRoutes {
    type Seed = ();
    type Context = Context;

    fn headers_received(_seed: (),
                        head: Head,
                        _res: &mut Response,
                        scope: &mut Scope<Context>)
                        -> Option<(Self, RecvMode, Time)> {
        use self::PasteRoutes::*;
        scope.increment();
        Some((match head.path {
                  "/" => New,
                  "/new" => MakePaste,
                  "/num" => GetNum,
                  "/404" => PageNotFound,
                  p if p.starts_with('/') => GetPaste(p[1..].to_string()),
                  _ => PageNotFound,
              },
              RecvMode::Buffered(100_000),
              scope.now() + Duration::new(10, 0)))
    }

    fn request_received(self,
                        data: &[u8],
                        res: &mut Response,
                        scope: &mut Scope<Context>)
                        -> Option<Self> {
        use self::PasteRoutes::*;
        match self {
            New => {
                if let Err(e) = util::send_file(res, "views/new.html") {
                    println!("{}", e);
                }
            }
            MakePaste => {
                let form = forms::parse_form(data).unwrap();

                let filetype = form.get(&"filetype".to_string()).unwrap();
                let paste = form.get(&"paste".to_string()).unwrap();

                let mut paste = match db::new_paste(filetype, paste) {
                    Ok(p) => p.into_bytes(),
                    Err(e) => {
                        let err = format!("Error creating paste: {}", e);
                        util::send_string(res, err.as_bytes());
                        return None;
                    }
                };
                paste.insert(0, b'/');

                if let Err(e) = util::redirect(res, b"You are being redirected", &paste[..], 302) {
                    println!("{}", e);
                }
            }
            GetPaste(p) => {
                if let Ok(p) = db::read_paste(&p[..].as_bytes()) {
                    util::send_string_raw(res, p.as_bytes());
                } else {
                    if let Err(e) = util::error(res, b"404 - Page not found", 404) {
                        println!("{}", e);
                    }
                }
            }
            GetNum => {
                util::send_string(res,
                                  format!("This host has been visited {} times", scope.get())
                                      .as_bytes());
            }
            PageNotFound => {
                if let Err(e) = util::error(res, b"404 - Page not found", 404) {
                    println!("{}", e);
                }
            }
        }

        None
    }
}*/

#[derive(Clone)]
struct HelloWorld;

impl Service for HelloWorld {
    type Request = Request;
    type Response = ResponseFn<Finished<IoBuf<TcpStream>, Error>, TcpStream>;
    type Error = Error;
    type Future = Finished<Self::Response, Error>;

    fn call(&self, _req: minihttp::Request) -> Self::Future {
        // Note: rather than allocating a response object, we return
        // a lambda that pushes headers into `ResponseWriter` which
        // writes them directly into response buffer without allocating
        // intermediate structures
        finished(ResponseFn::new(move |mut res| {
            res.status(200, "OK");
            res.add_chunked().unwrap();
            if res.done_headers().unwrap() {
                res.write_body(b"Hello, world!");
            }
            res.done()
        }))
    }
    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

fn main() {
    let matches = App::new("cedalion")
        .version("0.1")
        .author("nokaa <nokaa@cock.li>")
        .about("A pastebin server")
        .args_from_usage("-a, --addr=[ADDR] 'Sets the IP:PORT combination (default \
                          \"127.0.0.1:3000\")'")
        .get_matches();

    let addr = matches.value_of("ADDR").unwrap_or("127.0.0.1:3000").parse().unwrap();
    let http: Http<_, _> = Http::new();
    http.run(addr);
}

type Response = ResponseFn<Finished<IoBuf<TcpStream>, Error>, TcpStream>;

#[derive(Clone)]
struct Http<E: Clone, S: Io + Clone> {
    routes: HashMap<String, Rc<Fn(minihttp::Request, ResponseWriter<S>) -> Finished<IoBuf<S>, E>>>,
}

impl<E: Clone, S: Io + Clone> Service for Http<E, S> {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Finished<Self::Response, Error>;

    fn call(&self, _req: minihttp::Request) -> Self::Future {
        // Note: rather than allocating a response object, we return
        // a lambda that pushes headers into `ResponseWriter` which
        // writes them directly into response buffer without allocating
        // intermediate structures
        finished(ResponseFn::new(move |mut res| {
            res.status(200, "OK");
            res.add_chunked().unwrap();
            if res.done_headers().unwrap() {
                res.write_body(b"Hello, world!");
            }
            res.done()
        }))
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

impl<E: 'static + Clone, S: 'static + Io + Clone> Http<E, S> {
    pub fn new() -> Http<E, S> {
        Http { routes: HashMap::new() }
    }

    // pub fn handle_func(key: String, )

    pub fn run(self, addr: SocketAddr) {
        let mut lp = Core::new().unwrap();
        minihttp::serve(&lp.handle(), addr, self);
        lp.run(futures::empty::<(), ()>()).unwrap()
    }
}
