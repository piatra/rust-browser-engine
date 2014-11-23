mod dom;
mod parser;

fn main() {
    //println!("Hello, world!")
    //println!("Hello {}", dom::text("Hi".to_string()))
    println!("{}", parser::parse("<p><h1><!-- test -->Hello, <span>world</span></h1></p>".to_string()));
}
