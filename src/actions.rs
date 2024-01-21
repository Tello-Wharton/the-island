
trait ActionsForString {

    fn get_action_and_remaining<'a, 'b>(&'a self, prefix: &'b str) -> Option<(&'b str, &'a str)>;

}

impl ActionsForString for str {

    fn get_action_and_remaining<'a, 'b>(&'a self, prefix: &'b str) -> Option<(&'b str, &'a str)> {
        if let Some(remaining) = self.strip_prefix(prefix) {
            Some((prefix, remaining))
        } else {
            None
        }
    }

}

pub fn get_action(input: &str) {

    if let Some((prefix, remaining)) = input.get_action_and_remaining("move ") {
        println!("Prefix: {}, Remaining: {}", prefix, remaining);
    } 
    else if input == "quit" {
        println!("Quitting...");
    } 
    else {
        println!("I don't understand {}", input);
    }
}