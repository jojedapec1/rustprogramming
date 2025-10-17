enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        println!("Available fruits in store:");
        for f in &self.fruit {
            match f {
                Fruit::Apple(_) => println!(" - Apple"),
                Fruit::Banana(_) => println!(" - Banana"),
                Fruit::Tomato(_) => println!(" - Tomato"),
            }
        }
        println!();
    }

    fn tell_me_joke(&self, fruit: &Fruit) {
        match fruit {
            Fruit::Apple(msg) => println!("🍎 Apple joke: {}", msg),
            Fruit::Banana(msg) => println!("🍌 Banana joke: {}", msg),
            Fruit::Tomato(msg) => println!("🍅 Tomato joke: {}", msg),
        }
    }
}

fn main() {
    let a = "Why did the apple stop in the middle of the road? It ran out of juice!".to_string();
    let b = "What do bananas say when they pick up the phone? Yellow!".to_string();
    let t = "Why did the tomato blush? Because it saw the salad dressing!".to_string();

    let fruits = vec![
        Fruit::Apple(a),
        Fruit::Banana(b),
        Fruit::Tomato(t),
    ];

    let grocery_store = Inventory { fruit: fruits };

    grocery_store.available_fruits();

    for f in &grocery_store.fruit {
        grocery_store.tell_me_joke(f);
    }
}
