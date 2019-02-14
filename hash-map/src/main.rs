use std::collections::HashMap;
use std::env;

fn read_args() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for arg in args {

        if let Some(c) = arg.chars().next() {
            match c {
                'w' | 'W' => println!("Hello {}", arg),
                _=>{}
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    read_args();

    let mut hm = HashMap::new();

    hm.insert(3, "hello");
    hm.insert(4, "world");

    let r = match hm.get(&3) {
        Some(v) => v,
        _=>"NOTHING"
    };

    let n = hm.get(&4).unwrap_or(&"NoString");

    println!("{} ==== {}", r, n);

}


