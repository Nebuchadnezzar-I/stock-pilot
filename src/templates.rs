use serde::Serialize;

#[derive(Serialize)]
pub struct PageLogin<'a> {
    pub title: &'a str,
}
