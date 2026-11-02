trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;
    fn intro(&self) -> String {
        format!("My name is {} and I say {}!", self.name(), self.speak())
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn speak(&self) -> String {
        "woof".to_string()
    }
    fn intro(&self) -> String {
        format!(
            "My name is {} and I'm a dog. I say {}!",
            self.name(),
            self.speak()
        )
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn speak(&self) -> String {
        "meow".to_string()
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "Buddy".to_string(),
        }),
        Box::new(Cat {
            name: "Whiskers".to_string(),
        }),
    ];
    for animal in animals {
        println!("{}", animal.intro());
    }
}
