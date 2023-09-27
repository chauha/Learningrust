struct Person
{ 
    name: String,
    age: usize,
}
#[derive(Debug)]
struct Book
{
    title: String,
    author: String,
    is_available: bool,
}

#[derive(Debug)]
struct Library
{
    books: Vec<Book>,
}

fn main() {
    println!("Hello, world!");
    let person1 = Person {
        name: String::from("Ashish"),
        age: 22
    };

    let book1 = Book {
        title: String::from("Learning Rust"),
        author: String::from("Steven Barns"),
        is_available: true

    };

    let home_library = Library {
        books: vec![book1],
    };

    println!("Books in Home {:?}", home_library);
}