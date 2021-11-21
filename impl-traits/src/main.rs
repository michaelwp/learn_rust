struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("name: {}, age: {}", self.name, self.age);
    }
}

fn main() {

    let person = Person{name: String::from("michael"), age: 38};

    println!("{}", person.to_string());
}
