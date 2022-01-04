struct PersonObject {
    name: String,
    last_name: String,
    age: u32,
    email: String,
}

impl PersonObject {
    fn display_data(&self) {
        println!(
            "Name: {}, Last Name: {}, Age: {}, Email: {}",
            self.name, self.last_name, self.age, self.email
        )
    }
}

impl PersonObject {
    fn over_eighteen(age: &u32) -> bool {
        if age > &18 {
            true
        } else {
            false
        }
    }
}

fn main() {
    let new_player = PersonObject {
        name: String::from("Bokr"),
        last_name: String::from("Loha"),
        age: 12,
        email: String::from("bokr@gmail.com"),
    };

    new_player.display_data();

    let new_player2 = PersonObject {
        name: String::from("Bokr"),
        last_name: String::from("Loha"),
        ..new_player
    };

    println!(
        "Player is: {}",
        PersonObject::over_eighteen(&new_player.age)
    )
}
