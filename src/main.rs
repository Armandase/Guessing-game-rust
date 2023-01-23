use std::io::stdin;
use rand::Rng;

fn main() {
    let mut s = String::new();
    let mut counter: i8 = 0;
    let mut input : u32 = 0;
    let value: u16 = rand::thread_rng().gen();

    println!("\n\x1b[94m🔥Choose a number between 0 and 65536.🔥\n\x1b[0m");
    while input != value as u32 {
        stdin().read_line(&mut s).expect("Error in standard in put");
        let content: u32 = match s.trim().parse() {
            Ok(num) => num,
            Err(_) => 65537,
        };
        input = content;
        if input > 65536{
            println!("\x1b[31m! Choose a value less than 65536 !\n\x1b[0m");
            s = String::new();
            continue ;
        }
        print!("\x1b[95mTry {}, number {}\x1b[0m", counter, s);
        if input > value as u32 {
            println!("\x1b[94mYour number is to high\n\x1b[0m")
        } else if input < value as u32 {
            println!("\x1b[96mYour number is to low\n\x1b[0m");
        }
        s = String::new();
        counter += 1;
    }
    println!("\x1b[92mYou win in {} tries 🤓\x1b[0m", counter);
}
