use crate::book::Book;

#[derive(Debug)]
pub struct Library<'a> {
    books: Vec<&'a mut Book>,
}

impl<'a> Library<'a> {
    pub fn new(books: Vec<&'a mut Book>) -> Library<'a> {
        Library { books }
    }

    pub fn checkout_book(&mut self, book_name: String) -> bool {
        for b in &mut self.books {
            if b.title == book_name {
                b.is_available = false;
                println!("Book {} checked out", book_name);
                return true;
            }
        }
        return false;
    }

    pub fn return_book(&mut self, book_name: String) -> bool {
        for book in &mut self.books {
            if book.title == book_name {
                book.is_available = true;
                println!("Book {} returned", book_name);
                return true;
            }
        }
        return false;
    }
}
