use std::fmt::Display;

use crate::state::{Player, GameMap, Tile};

pub struct Action {
    pub name: &'static str,
    pub prefix: &'static str,
    pub take_action: fn(action_args: &str, player: &mut Player, game_map: &GameMap),
}


impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

static ACTIONS: [Action; 1] = [Action {
    name: "Move",
    prefix: "move ",
    take_action: |action_args: &str, player: &mut Player, game_map: &GameMap| {

        let action_args = action_args.trim().to_lowercase();
        match action_args.as_str() {
            "north" => {
                if player.y < 299 {
                    player.y += 1;
                } else {
                    println!("You can't go any further north");
                }
            },
            "south" => {
                if player.y > 0 {
                    player.y -= 1;
                } else {
                    println!("You can't go any further south");
                }
            },
            "east" => {
                if player.x < 299 {
                    player.x += 1;
                } else {
                    println!("You can't go any further east");
                }
            },
            "west" => {
                if player.x > 0 {
                    player.x -= 1;
                } else {
                    println!("You can't go any further west");
                }
            },
            _ => println!("I don't understand {}", action_args),
        }
    },
}];

trait ActionsForString {
    fn parse_with_action(&self, action: &Action) -> Option<&str>;
}

impl ActionsForString for str {
    fn parse_with_action(&self, action: &Action) -> Option<&str> {
        self.strip_prefix(action.prefix)
    }
}

pub fn get_action(input: &str) -> Option<(&Action, &str)>{
    for action in ACTIONS.iter() {
        if let Some(action_args) = input.parse_with_action(&action) {
            println!("Action: {}, Remaining: {}", action, action_args);
            return Some((action, action_args));
        }
    }
    
    println!("I don't understand {}", input);
    None
}
