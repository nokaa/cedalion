mod schema;
mod models;

pub use self::models::Paste;
use self::models::NewPaste;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use dotenv::dotenv;

use rand;
use rand::Rng;

use std::env;
use std::str;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn read_paste(paste_name: &[u8]) -> Option<Paste> {
    use self::schema::pastes::dsl::*;

    let connection = establish_connection();

    let paste_name = str::from_utf8(paste_name).unwrap();
    let my_paste = match pastes.filter(name.eq(paste_name))
        .first(&connection) {
            Ok(p) => Some(p),
            _ => None,
    };

    my_paste
}

pub fn write_paste(filetype: &[u8], paste: &[u8]) -> Paste {
    use self::schema::pastes;

    let connection = establish_connection();

    let mut name = gen_paste_name();
    let filetype = str::from_utf8(filetype).unwrap();
    name.push('.');
    name.push_str(filetype);
    let paste = str::from_utf8(paste).unwrap();

    let new_paste = NewPaste{
        name: &name[..],
        paste: paste,
    };

    diesel::insert(&new_paste).into(pastes::table)
        .get_result(&connection)
        .expect("Error saving paste")
}

fn gen_paste_name() -> String {
    let s: String = rand::thread_rng().gen_ascii_chars().take(10).collect();
    s
}
