
use std::io;

fn main() {
    println!("Enter a string, I'll yell it back. Unless you tell me to quit. Then I will quit");


    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() == "quit" {
            println!("Quitting");
            break;
        } else {
            println!("You said {}", input);   
        }
    }

}
