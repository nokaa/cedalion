mod parser;

use std::collections::HashMap;
use std::str;

use chomp::Input;

/// This function recevies our form data byte array as input
/// and returns a map of the form
/// ```text
/// { name: value }
/// ```
///
/// ### Panics
/// If `data` is not valid, `parse_form` will panic.
pub fn parse_form(data: &[u8]) -> HashMap<String, Vec<u8>> {
    // The map that will store form `names` and values
    let mut form_map: HashMap<String, Vec<u8>> = HashMap::new();

    // Move our u8 slice into an `Input` for use with chomp
    let data = Input::new(data);

    // Parse through form data and return `Vec<Form>`
    // TODO: Handle potential errors in a sane way
    let val = parser::form(data).unwrap();

    // Insert each `Form` into the map
    for form in &val {
        let name = str::from_utf8(form.name).unwrap().to_string();
        let value = parser::replace_special_characters(form.value);
        form_map.insert(name, value);
    }

    form_map
}
