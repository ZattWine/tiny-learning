// Struct - use to create a custom data types.

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Tuple struct
struct TColor(u8, u8, u8);

pub fn run() {
    // traditional
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // tuple
    let mut tc = TColor(255, 0, 0);
    tc.1 = 200;
    println!("TColor: {} {} {}", tc.0, tc.1, tc.2);

    // person
    let mut person = Person::new("Jhon", "Doe");
    println!("Person: {}", person.full_name());
    person.set_last_name("Smith");
    println!("Person: {}", person.full_name());
    println!("Person Tuple: {:?}", person.to_tuple());
}
