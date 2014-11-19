mod dom;

fn main() {
    println!("Hello, world!")
    println!("Hello {}", dom::text("Hi".to_string()))
}
