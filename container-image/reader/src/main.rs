use std::fs;

fn main() {
    let contents = fs::read_to_string("/foo.txt")
        .expect("Could not read /foo.txt");

    println!("{}", contents);
}
