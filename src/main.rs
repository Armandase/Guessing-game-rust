use std::io::stdin;
use rand::Rng;

fn main() {
    let mut s = String::new();
    let mut counter: i8 = 0;
    let mut input : u32 = 0;
    let value: u16 = rand::thread_rng().gen();

    while input != value as u32 {
        stdin().read_line(&mut s).expect("Error in standard in put");
        println!("\x1b[95mTry {}, number choose : {}\x1b[0m", counter, s);
        input =  s.trim().parse().expect("Error in conversion");
        if input > value as u32 {
            println!("\x1b[94mYour number is to upper\x1b[0m")
        } else if input < value as u32 {
            println!("\x1b[96mYour number is to lower\x1b[0m");
        }
        s = String::new();
        counter += 1;
    }
    println!("\x1b[94mYou win in {} tries\x1b[0m", counter);
}
