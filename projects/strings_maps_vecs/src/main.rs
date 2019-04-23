use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut vector: Vec<u8> = Vec::new();
    
    for _ in 1..100 {
        let num: u8 = rand::thread_rng().gen_range(1, 255);
        vector.push(num);    
    }
    let mut sum: usize = 0;
    
    for i in vector.iter() {
        sum += *i as usize ;
    }

    vector.sort_unstable();
    let av1: usize = sum / vector.len() as usize;
    let av2: usize = ((vector.first().expect("-1")
                     + vector.last().expect("-1")) / 2) as usize;
    let midian: usize = *vector.get(vector.len()/2).expect("-1") as usize;

    println!("The averege of rand vec is {}\n", av1);
    print!("The averege 2 of rand vec is {}\n", av2);
    println!("The midian of rand vec is {}\n", midian);
    

    for val in vector.iter() {
        //println!("{}", val);
    }
    
}