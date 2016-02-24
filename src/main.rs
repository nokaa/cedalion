#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate chomp;
extern crate rand;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_http_utils;

mod database;
//mod forms;
//mod util;

//use std::str;
use std::time::Duration;

use rotor::{Scope, Time};
use rotor::mio::tcp::TcpListener;
use rotor_http::server::{RecvMode, Server, Head, Response};
use rotor_http::ServerFsm;
use rotor_http_utils::{forms, util};

struct Context {
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

impl rotor_http::server::Context for Context {
    // Default impl is okay
    fn byte_timeout(&self) -> Duration {
        Duration::new(1000, 0)
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
    type Context = Context;
    
    fn headers_received(head: Head,
                        _res: &mut Response,
                        scope: &mut Scope<Context>)
        -> Option<(Self, RecvMode, Time)>
    {
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
        -> Option<Self>
    {
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

                let paste = database::write_paste(filetype, paste).name;
                let mut location: Vec<u8> = vec![b'/'];
                for c in paste[..].as_bytes() {
                    location.push(*c);
                }

                if let Err(e) = util::redirect(res,
                                               b"You are being redirected",
                                               &location[..],
                                               302) {
                    println!("{}", e);
                }
            }
            GetPaste(p) => {
                let paste = database::read_paste(&p[..].as_bytes());
                match paste {
                    Some(p) => util::send_string_raw(res, &p.paste[..].as_bytes()),
                    None => { 
                        if let Err(e) = util::error(res,
                                                    b"404 - Page not found",
                                                    404) {
                            println!("{}", e);
                        }
                    }
                }
            }
            GetNum => {
                util::send_string(res,
                                  format!("This host has been visited {} times", scope.get())
                                  .as_bytes());
            }
            PageNotFound => {
                if let Err(e) = util::error(res,
                                            b"404 - Page not found",
                                            404) {
                    println!("{}", e);
                }
            }
        }

        None
    }

    fn request_chunk(self,
                     _chunk: &[u8],
                     _response: &mut Response,
                     _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        unreachable!();
    }

    /// End of request body, only for Progressive requests
    fn request_end(self,
                   _response: &mut Response,
                   _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        unreachable!();
    }

    fn timeout(self,
               _response: &mut Response,
               _scope: &mut Scope<Context>)
        -> Option<(Self, Time)>
    {
        unimplemented!();
    }

    fn wakeup(self,
              _response: &mut Response,
              _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        unimplemented!();
    }
}

fn main() {
    let event_loop = rotor::Loop::new(&rotor::Config::new()).unwrap();
    let mut loop_inst = event_loop.instantiate(Context { counter: 0 });
    let lst = TcpListener::bind(&"127.0.0.1:3000".parse().unwrap()).unwrap();
    loop_inst.add_machine_with(|scope| ServerFsm::<PasteRoutes, _>::new(lst, scope))
        .unwrap();
    loop_inst.run().unwrap();
    println!("Listening at 127.0.0.1:3000");
}
