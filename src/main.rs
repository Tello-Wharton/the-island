
use std::io;

struct Player {
    name: String,
    health: i32,
    x: i32,
    y: i32,
}

enum Tile {
    Land,
    Water,
}

struct GameMap {
    tiles: Vec<Vec<Tile>>
}

// Add method to create game map with a 100x100 grid of land, surrounded by 100 tiles of water

impl GameMap {
    fn new() -> GameMap {
        let mut tiles = Vec::new();
        for x in 0..300 {
            let mut row = Vec::new();
            for y in 0..300 {
                
                if x < 100 || x > 200 || y < 100 || y > 200 {
                    row.push(Tile::Water);
                } else {
                    row.push(Tile::Land);
                }
            }
            tiles.push(row);
        }
        GameMap { tiles }
    }
}

fn main() {
    println!("Enter a string, I'll yell it back. Unless you tell me to quit. Then I will quit");

    let mut player = Player {
        name: String::from("Player 1"),
        health: 100,
        x: 150,
        y: 150,
    };

    let map = GameMap::new();

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
