/// Demonstrates basic operations on a vector of integers, including:
/// - Summing the elements
/// - Doubling each element and collecting the results into a new vector
/// - Mapping the doubled values into a vector of `Person` structs
/// - Printing the original vector, the doubled vector, and the `Person` vector
/// - Iterating over the `Person` vector to print each `Person`'s name and age
fn main() {
    // Create a vector of integers
    let v = vec![1, 2, 3, 4, 5];

    // Sum the elements of the vector
    let s: i32 = v.iter().sum();
    println!("Sum of elements: {}", s);

    // Print the original vector
    println!("Original vector: {:?}", v);

    // Double each element and collect into a new vector
    let v2: Vec<_> = v.iter().map(|x| x * 2).collect();
    println!("Doubled vector: {:?}", v2);

    // Map the doubled values into a vector of `Person` structs
    let foo = v2
        .iter()
        .map(|&x| Person {
            name: format!("name-{}", (x * 10).to_string()),
            age: x,
        })
        .collect::<Vec<Person>>();

    // Print the vector of `Person` structs
    println!("Person vector: {:?}", foo);

    // Iterate over the `Person` vector to print each `Person`'s name and age
    for p in foo {
        println!("name: {}, age: {}", p.name, p.age);
    }
}

/// Represents a person with a name and age
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
