
use std::io;

mod actions;
use actions::get_action;

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

enum Directions {
    North,
    South,
    East,
    West,
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

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        &self.tiles[x as usize][y as usize]
    }
}

fn main() {
    println!("Welcome to The Island...");

    let mut player = Player {
        name: String::from("Player 1"),
        health: 100,
        x: 150,
        y: 150,
    };

    let map = GameMap::new();

    loop {
        
        let tile = map.get_tile(player.x, player.y);
        match tile {
            Tile::Land => println!("You are on land"),
            Tile::Water => println!("You are in water"),
        }

        println!("You are at {}, {}", player.x, player.y);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_lowercase();

        get_action(&input);
    }

}
