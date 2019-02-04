

fn main() {
    let _b = highest(4,2,8);

    // created at compile time
    // available to set string
    let _s = format!("{} is highest", _b);
    // macro, converted before run

    // println!("{} is highest", b);
    // println!("{} is other", other(5,9));
    // loopto10();
    // array_loop();
    // strings();
    println!("l len: {}",count_l("hello 中国"));
}

fn highest(a:i32, b:u32, c:i8) -> i32 {
    let mut res = a;
    if b as i32 > res {
        res = b as i32;
    }
    if c as i32 > res {
        res = c as i32;
    }
    return res;
}

fn other(a:i32, b:i32)->i32 {
    let mut c = a + b;
    c = c % 4;
    c = c / 2;
    c += 1;
    return c;
}

fn loopto10(){
    let mut n = 0;

    for m in 0..10 {
        println!("hello for {}", m);
    }

    loop {
        n +=1;
        println!("Hello {}", n);
        if n >= 10 {
            return;
        }
    }
}

fn array_loop() {
    // let mut v = Vec::new();
    let v = vec![3,7,9,1];
    for n in v {
        if n % 2 == 0 {
            continue;
        }
        println!("{}", n);
    }
}

fn strings() {
    // &str is pointer to slice of string
    // whereas string is vector owns the array
    // which means it has to copy around that string
    // &str is smaller as a pointer much lighter in memory

    let s = String::from("Hello 中国");

    println!("S Len = {}", s.len());
    // len == length of bytes


    // need to know will take more than 8 chars
    for c in s.chars() {
        println!("{}",c);
    } 

    for c in s.bytes() {
        println!("{}",c);
    } 
}

fn count_l(s:&str) ->i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res +=1;
        }
    }
    res
}