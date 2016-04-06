/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

extern crate clap;
extern crate rand;
extern crate redis;
extern crate rotor;
extern crate rotor_http;
extern crate rotor_http_utils;

mod db;

use std::time::Duration;

use clap::App;
use rotor::{Scope, Time};
use rotor::mio::tcp::TcpListener;
use rotor_http::server::{RecvMode, Server, Head, Response, Fsm};
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

                let mut paste = match db::new_paste(filetype, paste) {
                    Ok(p) => p.into_bytes(),
                    Err(e) => {
                        let err = format!("Error creating paste: {}", e);
                        util::send_string(res, err.as_bytes());
                        return None;
                    }
                };
                paste.insert(0, b'/');

                if let Err(e) = util::redirect(res,
                                               b"You are being redirected",
                                               &paste[..],
                                               302) {
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
    let matches = App::new("cedalion")
        .version("0.1")
        .author("nokaa <nokaa@cock.li>")
        .about("A pastebin server")
        .args_from_usage(
            "-a, --addr=[ADDR] 'Sets the IP:PORT combination (default \"127.0.0.1:3000\")'")
        .get_matches();

    let addr = matches.value_of("ADDR").unwrap_or("127.0.0.1:3000");

    let event_loop = rotor::Loop::new(&rotor::Config::new()).unwrap();
    let mut loop_inst = event_loop.instantiate(Context { counter: 0 });
    let lst = TcpListener::bind(&addr.parse().unwrap()).unwrap();
    loop_inst.add_machine_with(|scope| Fsm::<PasteRoutes, _>::new(lst, (), scope))
        .unwrap();
    println!("Listening at {}", addr);
    loop_inst.run().unwrap();
}
