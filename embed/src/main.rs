mod lib;
use lib::fibonacci;

fn main() {
    println!("Fibonacci generator");
    println!("{}", fibonacci(1));
    println!("{}", fibonacci(3));
    println!("{}", fibonacci(5));
}
