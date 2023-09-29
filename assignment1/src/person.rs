#[derive(Debug)]
pub struct Person {
    name: String,
    age: usize,
}

// create a method to print the name and age of the person in the format "name: <name>, age: <age>"
impl Person {
    pub fn new(name: String, age: usize) -> Person {
        Person { name, age }
    }

    pub fn display_person(person: &Person) {
        println!("name: {}, age: {}", person.name, person.age);
    }
}
