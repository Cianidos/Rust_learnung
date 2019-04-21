use std::io;

fn main() {
    
    println!("==============IF=============");
    let number = 8;

    if number % 4 == 0 {
        println!("num is divisible by 4");
    } else if number % 3 == 0 {
        println!("num is divisible by 3");
    } else if number % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The number value is: {}", number);
    println!("===========LOOPS===========");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The reault is {}", result);
    println!();

    for i in (1..10).rev() {
        println!("{}!", i);
    }    
    fib(100);
    evclid2();
    println!("Hello, world!");
}

fn evclid() {
    let mut sss = String::new();
    io::stdin().read_line(&mut sss)
        .expect("Failed to read line");
    let mut n1: u32 = sss.trim().parse().expect("000");

    let mut sss = String::new();
    io::stdin().read_line(&mut sss)
        .expect("Failed to read line");
    let mut n2: u32 = sss.trim().parse().expect("000");

    while n1 != 0 && n2 != 0 {
        if n1 > n2 { n1 %= n2
        } else { n2 %= n1 }
    }
    if n1 > n2 { println!("{}", n1);
    } else { println!("{}", n2); }
}

fn evclid2() {
    let mut n: (usize, usize) = input_two_int();
    while n.0 != 0 && n.1 != 0 {
        if n.0 > n.1 { n.0 %= n.1;
        } else { n.1 %= n.0; }
    }
    if n.0 > n.1 { println!("{}", n.0);
    } else { println!("{}", n.1); }    
}

fn input_two_int() -> (usize, usize) {
    let mut sss = String::new();
    io::stdin().read_line(&mut sss)
        .expect("Fail to read line");
    
    let mut n1 = String::new();
    let mut n2 = String::new();

    let bytes = sss.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            n1 = sss[..i].to_string();
            n2 = sss[i..].to_string();
        }
    }
    let n1 = n1.trim().parse().expect("000");
    let n2 = n2.trim().parse().expect("000");
    (n1, n2)
}

fn fib(n: u32) {
    let mut f: u128 = 1;
    let mut s: u128 = 1;
    for i in (1..n) {
        let t = f;
        f = s;
        s = s + t;
    }
    println!("{}",f);
}