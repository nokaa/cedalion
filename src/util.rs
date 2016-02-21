use std::fs::File;
use std::io;
use std::io::{Read, Write};

use rotor_http::server::Response;

/// Sets headers for a HTTP 404 error.
/// `data` should be a file/message that
/// explains what happened to the user.
pub fn four_o_four(res: &mut Response, data: &[u8]) {
    res.status(404, "Not Found");
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}

/// Handles redirects. Redirects `res` to `location`
/// with code `code`. `data` should be a file/message that
/// explains what happened to the user.
//
// TODO(nokaa): We  want to allow an arbitrary code, with
// the proper message sent. Right now this redirect only works
// properly for a 302. This might be solved with two static
// arrays, the first containing codes and the second containing
// messages. We could then find the index of the code in the first,
// and use the corresponding message in the second.
pub fn redirect(res: &mut Response, data: &[u8], location: &[u8], code: u16) {
    res.status(code, "Found");
    res.add_header("Location", location).unwrap();
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}

/// Sends `data` to the client with status 200.
pub fn send_string(res: &mut Response, data: &[u8]) {
    res.status(200, "OK");
    // Add `Content-Type` header to ensure data is interpreted
    // as plaintext
    res.add_header("Content-Type",
                   "text/plain; charset=utf-8".as_bytes())
        .unwrap();
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}

/// Sends data read from `filename` to the client
/// with status 200.
///
/// ### Panics
/// Panics if `filename` cannot be read.
pub fn send_file(res: &mut Response, filename: &str) {
    let data = &read_file(filename)[..];

    res.status(200, "OK");
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}

/// Read file `filename` into a `Vec<u8>`.
///
/// ### Panics
/// `read_file` panics if `filename` cannot be opened.
///
/// Panics if `read_to_end` fails for `filename`.
pub fn read_file(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).ok()
        .expect(&format!("Unable to open file {}!", filename)[..]);
    let mut buf: Vec<u8> = vec![];
    f.read_to_end(&mut buf).unwrap();

    buf
}

#[allow(dead_code)]
/// Writes `data` to `filename`.
pub fn write_file(filename: &str, data: &[u8]) -> Result<(), io::Error> {
    let mut file = try!(File::create(filename));
    try!(file.write_all(data));
    Ok(())
}
