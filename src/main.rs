use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    //parsing args
    match args.as_slice() {
        [port_arg, port, origin_arg, origin] if port_arg == "--port" && origin_arg == "--origin" => ,
        [clear_cache_arg] if clear_cache_arg == "--clear-cache" => ,
        _ => println!("Wrong arguments"),
    }
}
