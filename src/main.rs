use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args {
        _ => println!("Wrong arguments"),
    }
}
