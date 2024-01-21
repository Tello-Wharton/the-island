
use std::io;

mod actions;
use actions::get_action;

mod state;
use state::{Player, GameMap, Tile};


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

        if let Some((action, action_args)) = get_action(&input) {
            (action.take_action)(action_args, &mut player, &map);
        } else {
            println!("I don't understand {}", input);
        }
    }

}
