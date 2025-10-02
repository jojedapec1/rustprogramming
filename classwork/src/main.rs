// Define a struct called Student with name and major
struct Student {
    name: String,
    major: String,
}

impl Student {
    // constructor-like static method: new()
    fn new(name: &str, major: &str) -> Self {
        Self {
            name: name.to_string(),
            major: major.to_string(),
        }
    }

    // set_major: updates the major
    fn set_major(&mut self, new_major: &str) {
        self.major = new_major.to_string();
    }

    // get_major: returns the current major
    fn get_major(&self) -> &str {
        &self.major
    }
}

fn main() {
    // Create a Student
    let mut s = Student::new("Alice", "Computer Engineering");

    // Use get_major
    println!("Major: {}", s.get_major());

    // Change major
    s.set_major("Electrical Engineering");

    // Verify update
    println!("Updated major: {}", s.get_major());
}
