fn main() {
    let mut x = 5;
    println!("The Value of x is: {}", x);
    x = 20;
    println!("The Value of x is: {}", x);
    let mut x = x + 6;
    println!("The Value of x is: {}", x);
    x = 79;
    println!("The Value of x is: {}", x);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The Value of guess is: {}", guess);

    
    let x: u8 = 255;
    let x = x - 10;
    println!("The Value of x is: {}", x);
    let x: f32 = 100.0;
    println!("The Value of x is: {}", x);
    let x: f64 = (x + 1.0).into();
    println!("The Value of x is: {}", x);
    let x: f32 = (x + 1.1);
}
