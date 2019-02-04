// enums stored in lowest bytes necessary
// min for actual type and contents

use std::collections::HashMap;
use std::env::args;

#[derive(Debug)]
pub struct Bed{
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32, String)
}

fn main() {
    use self::Room::*;
    // use here lets you 
    // let t = Bedroom(Bed{size:50, count:2});
    let t = Kitchen(5);
    println!("Hello from the {:?}", t);

    let v = match t {
        Kitchen(n) => n,
        _ => 0
    };

    // only working with one
    if let Kitchen(n) = t {
        println!("Its a kitchen with {} cupboards", n);
    }

    println!("v = {}", v);

    let mut hm = HashMap::new();
    hm.insert(3, "Hello");
    hm.insert(5, "world");
    // let r = match hm.get(&3) {
    //     Some(v) => v,
    //     _ => "Nothing",
    // };

    let r = hm.get(&3).unwrap();

    match "3".parse::<f32>() {
        Ok(v)=>println!("OK - {}", v),
        Err(e)=>println!("Error - {}",e)
    }

    println!("{}", r);
}

fn get_arg(n: usize)->Result<String,String>{
    for (i,a) in args().enumerate(){
        if (i == n) {
            return Ok(a)
        }        
    }
    Err("Not enough args".to_string());
}