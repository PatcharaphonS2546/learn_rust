fn guessing_game() {
    let secret = rand::random_range(1..=100);
    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        match guess.cmp(&secret) {
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!");
            }
            std::cmp::Ordering::Less => {
                println!("Too small!");
            }
        }
    }
}

fn main() {
    println!("Guess the number! (1-100)");
    guessing_game();
}
