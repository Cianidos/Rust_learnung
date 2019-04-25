use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut vectorrrr: Vec<u8> = Vec::new();
    
    for _ in 1..100 {
        let num: u8 = rand::thread_rng().gen_range(1, 255);
        vectorrrr.push(num);    
    }
    let sstr = String::from(
     "Long long apple juse gone down the street withe a Jisus");
    string_fay(sstr);
    vec_analizer(&mut vectorrrr);    
}

fn string_fay (sstr: String){
    let mask = String::from(
     "BCDFGHJKLMNPQRSTVWXYZbcdfghjklmnpqrstvwxz");
    let ssstr = sstr.split_whitespace();
    println!();
    for i in ssstr {
        let ch: char;
        let mut s = String::new();
        match i.chars().next() {
            Some(c) => ch = c,
            None => ch = '0',
        }
        if mask.contains(ch) {
            s = i[1..].to_string() + &"-fay".to_string();
        }
        if i.eq_ignore_ascii_case("apple") { 
            s = "apple-hey".to_string();
        }
        print!("{} ", s);
    }
    println!();
    println!();
}

fn vec_analizer(vect: &mut Vec<u8>) {
    let vector = {
        let veccopy = vect;
        veccopy.sort_unstable();
        veccopy
    };
    let mut sum: usize = 0;
    let mut map = HashMap::new();    
    let mut mode = 0;
    let mut c = 0;
    for i in vector.iter() {
        sum += *i as usize;
        let counter = map.entry(i).or_insert(0);
        *counter += 1;
        if *counter > c {
            mode = *i;
            c = *counter;
        }
    }
    let av1: usize = sum / vector.len() as usize;
    let av2: usize = ((*vector.first().expect("-1")
                     + *vector.last().expect("-1")) / 2) as usize;
    let midian: usize = vector[vector.len()/2] as usize;
    println!("The averege of rand vec is {}", av1);
    println!("The averege 2 of rand vec is {}", av2);
    println!("The midian of rand vec is {}", midian);
    println!("The mode of rand vec is {} with {} enterys", mode, c);
}