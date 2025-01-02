use learn_rust::generate_unique_string;

fn main() {
    let unique = generate_unique_string(10, "Hello World");
    let unique2 = generate_unique_string(14, "Hello World");
    let unique3 = generate_unique_string(12, "Hello World");
    println!("{}", unique);
    println!("{}", unique2);
    println!("{}", unique3);
}
