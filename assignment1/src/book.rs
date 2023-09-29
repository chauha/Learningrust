#[derive(Debug)]

pub struct Book {
    pub title: String,
    pub author: String,
    pub is_available: bool,
}

impl Book {
    pub fn new(title: String, author: String, is_available: bool) -> Book {
        Book {
            title,
            author,
            is_available,
        }
    }
}
