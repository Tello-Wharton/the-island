
pub fn get_action(input: &str) {

    if let Some(remaining) = input.strip_prefix("move ") {
        println!("Time to move {}", remaining);
    } 
    else if input == "quit" {
        println!("Quitting...");
    } 
    else {
        println!("I don't understand {}", input);
    }
}