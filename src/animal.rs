use std::any::{Any, TypeId};

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

struct Bird {
    name: String,
}

trait Animal {
    /// This method returns a reference to the object as a trait object of type `Any`.
    fn as_any(&self) -> &dyn Any;
}

impl Animal for Dog {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Animal for Cat {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Animal for Bird {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

enum FlowType {
    IfLet,
    Match,
}

// The `animal` parameter is a trait object of type `&dyn Animal`.
// The `as_any()` method is used to obtain a reference to the animal as a trait object of type `Any`,
// which allows dynamic type checking and downcasting to specific types (e.g., `Dog`, `Cat`, `Bird`).
fn describe_animal(animal: &dyn Animal, flow_type: FlowType) {
    match flow_type {
        FlowType::IfLet => {
            if let Some(d) = animal.as_any().downcast_ref::<Dog>() {
                println!("dog: {}", d.name);
            } else if let Some(c) = animal.as_any().downcast_ref::<Cat>() {
                println!("cat: {}", c.name);
            } else if let Some(b) = animal.as_any().downcast_ref::<Bird>() {
                println!("bird: {}", b.name);
            }
        }
        FlowType::Match => match animal.as_any().type_id() {
            id if id == TypeId::of::<Dog>() => {
                let dog = animal.as_any().downcast_ref::<Dog>().unwrap();
                println!("This is a dog named {}", dog.name);
            }
            id if id == TypeId::of::<Cat>() => {
                let cat = animal.as_any().downcast_ref::<Cat>().unwrap();
                println!("This is a cat named {}", cat.name);
            }
            id if id == TypeId::of::<Bird>() => {
                let bird = animal.as_any().downcast_ref::<Bird>().unwrap();
                println!("This is a bird named {}", bird.name);
            }
            _ => {
                println!("Unknown animal type.");
            }
        },
    }
}

/// This function takes a reference to a trait object of type `Any` (a dynamically-typed value),
/// and checks its type at runtime. If the type matches `Dog`, `Cat`, or `Bird`, it prints the
/// corresponding message with the animal's name. Otherwise, it prints "Unknown animal type".
///
/// animal can be any type that implements the `Any` trait.
fn describe_animal_from_any(animal: &dyn std::any::Any) {
    match animal {
        a if a.is::<Dog>() => println!(
            "This is a dog named {}",
            a.downcast_ref::<Dog>().unwrap().name
        ),
        a if a.is::<Cat>() => println!(
            "This is a cat named {}",
            a.downcast_ref::<Cat>().unwrap().name
        ),
        a if a.is::<Bird>() => println!(
            "This is a bird named {}",
            a.downcast_ref::<Bird>().unwrap().name
        ),
        _ => {
            println!("Unknown animal type.");
        }
    }
}

pub fn demo_animal() {
    let dog = Dog {
        name: String::from("Dog1"),
    };
    let cat = Cat {
        name: String::from("Cat1"),
    };
    let bird = Bird {
        name: String::from("Bird1"),
    };

    describe_animal_from_any(&dog);
    describe_animal_from_any(&cat);
    describe_animal_from_any(&bird);

    // pay attention we can pass &1 , because i32 implements the Any trait
    describe_animal_from_any(&1);

    // won't compile
    // describe_animal(&1, Which::LetIf);

    describe_animal(&dog, FlowType::IfLet);
    describe_animal(&cat, FlowType::Match);
    describe_animal(&bird, FlowType::Match);
}
