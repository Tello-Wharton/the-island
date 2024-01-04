
use std::io;

struct Player {
    name: String,
    health: i32
}

fn main() {
    println!("Enter a string, I'll yell it back. Unless you tell me to quit. Then I will quit");

    let mut player = Player {
        name: String::from("Player 1"),
        health: 100
    };

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().to_lowercase() == "quit" {
            println!("Quitting...");
            break;
        } else {
            println!("Player name is {}, player health is {}", player.name, player.health);
            println!("You said {}", input);   
        }
    }

}
