use super::schema::pastes;

#[derive(Queryable)]
pub struct Paste {
    pub id: i32,
    pub name: String,
    pub paste: String,
}

#[insertable_into(pastes)]
pub struct NewPaste<'a> {
    pub name: &'a str,
    pub paste: &'a str,
}
