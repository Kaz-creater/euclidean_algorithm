use std::io;
use rand::Rng;

fn gcd(a: u32, b: u32) -> u32{
    match b {
        0 => a,
        _ => gcd(b, a % b)
    }
}

fn main() {
    loop {   
        println!("Please input the number!");

        let b: u32 = rand::thread_rng().gen_range(1, 1001);

        let mut a = String::new();

            io::stdin()
                .read_line(&mut a)
                .expect("Failed to read line");

        let a: u32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        println!("a: {}", a); 

        println!("b: {}", b);

        println!("{}", gcd(a, b));
        break;
    }
}
