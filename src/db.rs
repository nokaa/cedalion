/* Copyright (C)  2016 nokaa <nokaa@cock.li>
 * This software is licensed under the terms of the
 * GNU Affero General Public License. You should have
 * received a copy of this license with this software.
 * The license may also be found at https://gnu.org/licenses/agpl.txt
 */

use rand::{self, Rng};
use redis::{self, Commands};

use std::str;

/// Create a new entry in redis
pub fn new_paste(filetype: &[u8], paste: &[u8]) -> redis::RedisResult<String> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());

    let mut name = gen_paste_name();
    let filetype = str::from_utf8(filetype).unwrap();
    name.push('.');
    name.push_str(filetype);

    let paste = str::from_utf8(paste).unwrap();

    let _: () = try!(con.set(name.clone(), paste));
    Ok(name)
}

/// Read key `paste_name` from redis and return
pub fn read_paste(paste_name: &[u8]) -> redis::RedisResult<String> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());

    //let key = String::from(str::from_utf8(paste_name).unwrap());
    let key = str::from_utf8(paste_name).unwrap();
    con.get(key)
}

/// Generate a unique id of length 10 from the set of ascii characters
fn gen_paste_name() -> String {
    let s: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
    s
}
