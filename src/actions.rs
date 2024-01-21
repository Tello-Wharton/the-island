use std::fmt::Display;

struct Action {
    name: &'static str,
    prefix: &'static str,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

static ACTIONS: [Action; 1] = [
    Action { 
        name: "Move",
        prefix: "move " 
    }
];

trait ActionsForString {
    fn get_action_and_remaining(&self, prefix: &Action) -> Option<(&str)>;
}

impl ActionsForString for str {
    fn get_action_and_remaining(&self, action: &Action) -> Option<(&str)> {
        if let Some(remaining) = self.strip_prefix(action.prefix) {
            Some(remaining)
        } else {
            None
        }
    }
}

pub fn get_action(input: &str) {
    for action in ACTIONS.iter() {
        if let Some(remaining) = input.get_action_and_remaining(&action) {
            println!("Action: {}, Remaining: {}", action, remaining);
            return;
        }
    }

    if input == "quit" {
        println!("Quitting...");
    } else {
        println!("I don't understand {}", input);
    }
}
