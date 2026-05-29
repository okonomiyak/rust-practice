fn main() {
    match std::fs::read_to_string("hello.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    //println!("Hello, world!");
}
