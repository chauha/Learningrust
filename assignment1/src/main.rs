use crate::book::Book;
use crate::library::Library;
use crate::person::Person;

mod book;
mod library;
mod person;

fn main() {
    let person1 = Person::new(String::from("John"), 30);

    Person::display_person(&person1);

    let mut book1 = Book::new(
        String::from("Learning Rust"),
        String::from("Steven Barns"),
        true,
    );

    let mut book2 = Book::new(
        String::from("Designing Data Intensive Applications"),
        String::from("Martin Kleppmann"),
        true,
    );

    let mut home_library = Library::new(vec![&mut book1, &mut book2]);

    Library::checkout_book(&mut home_library, String::from("Learning Rust"));

    Library::return_book(&mut home_library, String::from("Learning Rust"));

    println!("Books in Home {:?}", home_library);
}
