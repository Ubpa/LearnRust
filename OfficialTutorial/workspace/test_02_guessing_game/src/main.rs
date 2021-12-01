use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("@ubpa 2021/12/01 22:53");

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    let mut cnt : u32 = 0;

    loop {
        println!("Please input your guess");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        cnt = cnt + 1;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You should input a number!");
                continue;
            }
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }

    println!("cnt: {}", cnt);
}
