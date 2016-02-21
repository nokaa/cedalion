#[macro_use]
extern crate chomp;
extern crate rotor;
extern crate rotor_http;

mod forms;
mod util;

use std::str;
use std::time::Duration;

use rotor::{Scope, Time};
use rotor_http::server::{RecvMode, Server, Head, Response};
use rotor_http::ServerFsm;
use rotor::mio::tcp::TcpListener;

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
            "/new" => {
                //println!("{:?}", head);
                MakePaste
            }
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
                util::send_file(res, "views/new.html");
            }
            MakePaste => {
                let form = forms::parse_form(data);

                let filename = str::from_utf8(form.get(&"filetype".to_string()).unwrap())
                    .ok().expect("Unable to convert filetype to str!");
                let paste = form.get(&"paste".to_string()).unwrap();

                util::write_file(filename, paste).unwrap();

                util::redirect(res,
                               b"You are being redirected",
                               b"/",
                               302);
            }
            GetPaste(_) => {
                util::send_file(res, "views/view.html");
            }
            GetNum => {
                util::send_string(res,
                                  format!("This host has been visited {} times", scope.get())
                                  .as_bytes());
            }
            PageNotFound => {
                util::four_o_four(res, b"404 - Page not found");
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