use std::fs::File;
use std::fs;
fn main() {

    fs::read_to_string("hello.txt");
    let i: u128;
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening file: {:?}", error)
        },
    };
    
    println!("Hello, world!");
}
