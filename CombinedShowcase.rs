struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }

    fn increment_age(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let mut person = Person::new(String::from("Alice"), 30);

    // Immutable borrow to read the person’s data
    let person_ref = &person;
    person_ref.display(); // Prints: Name: Alice, Age: 30

    // Mutable borrow to modify the person’s age
    let person_mut_ref = &mut person;
    person_mut_ref.increment_age(); // Increment age by 1

    // After the mutable borrow is done, we can access the person again
    person.display(); // Prints: Name: Alice, Age: 31

    // Ownership transfer of the Person object to a new variable
    let person2 = person; // `person` is no longer valid after this
    // println!("{}", person.name); // Error: `person` is no longer valid

    person2.display(); // Prints: Name: Alice, Age: 31
}
