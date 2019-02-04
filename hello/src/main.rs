

fn main() {
    let _b = highest(4,2,8);

    // created at compile time
    // available to set string
    let _s = format!("{} is highest", b);
    // macro, converted before run

    // println!("{} is highest", b);
    // println!("{} is other", other(5,9));
    // loopto10();
    array_loop();
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