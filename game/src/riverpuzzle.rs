use std::io;

//  defining fields for gamestate
struct GameState {
    left_bank: Vec<&'static str>,
    right_bank: Vec<&'static str>,
    boat: Vec<&'static str>,
    is_on_left_bank: bool,
    has_goat: bool,
    has_lion: bool,
    has_grass: bool,
    has_person: bool,
}

// implementing method for initializing game_state
impl GameState {
    fn new() -> Self {
        GameState {
            left_bank: vec!["goat", "grass", "lion", "person"],
            right_bank: Vec::new(),
            boat: Vec::new(),
            is_on_left_bank: true,
            has_goat: true,
            has_grass: true,
            has_lion: true,
            has_person: true,
        }
    }

    // moves object either from left bank to the boat or right bank to the object
    fn move_object(&mut self, object: &'static str) {
        // if we are on the left_bank
        if self.is_on_left_bank {
            // iterate throught the list of objects in left bank, remove the object being moved from the list
            //  and add it to the boat
            if let Some(index) = self.left_bank.iter().position(|&p| p == object) {
                self.left_bank.remove(index);
                self.boat.push(object);
            }
        }
        // if on the right bank ,iterate through the list of objects in right bank, remove the object being moved from the list
        //  and add it to the boat
        else if self.right_bank.contains(&object) {
            if let Some(index) = self.right_bank.iter().position(|&p| p == object) {
                self.right_bank.remove(index);
                self.boat.push(object);

                println!("THIS IS THE OBJECT THAT WE PUSHED TO THE BOAT{}", object);
            }
        }

        // ensures that the boat does not sink ; other words only two objects on the boat at most
        if self.boat.len() > 3 {
            println!("Only two object can be moved at a time.");
            return;
        }

        // Toggle the boolean state of the moved object
        if object == "person" {
            self.has_person = !self.has_person;
        } else if object == "grass" {
            self.has_grass = !self.has_grass;
        } else if object == "goat" {
            self.has_goat = !self.has_goat;
        } else if object == "lion" {
            self.has_lion = !self.has_lion;
        }
    }

    // moves the boat
    fn move_boat(&mut self) {
        // ensures we avoid disasters, cases where lion is left behind with goat or goat with grass on either side
        if self.left_bank.contains(&"goat")
            && self.left_bank.contains(&"lion")
            && self.left_bank.len() == 2
        {
            println!("Disaster! Lion ate Goat");
            self.restart_game();
        } else if self.left_bank.contains(&"goat")
            && self.left_bank.contains(&"grass")
            && self.left_bank.len() == 2
        {
            println!("Disaster! Goat ate grass");
            self.restart_game();
        } else if self.right_bank.contains(&"goat")
            && self.right_bank.contains(&"grass")
            && self.right_bank.len() == 2
        {
            println!("Disaster! Goat ate grass");
            self.restart_game();
        } else if self.right_bank.contains(&"goat")
            && self.right_bank.contains(&"lion")
            && self.right_bank.len() == 2
        {
            println!("Disaster! Lion ate goat");
            self.restart_game();
        }

        // the boat has to have at most 2 objects and one the objects has to be the person to move.
        //  moves the boat and updates the location of the boat
        if self.boat.len() <= 2 && self.boat.contains(&"person") {
            if !self.boat.is_empty() {
                if self.is_on_left_bank {
                    if self.has_person || self.has_goat || self.has_lion || self.has_grass {
                        self.is_on_left_bank = false;
                    } else if self.boat.len() < 3 {
                        self.is_on_left_bank = false;
                        println!("Moving the boat with the objects to the right bank ");
                    }
                }
                // updates the location of the boat to be on the left
                else {
                    self.is_on_left_bank = true;
                }
            }
        }
        // ensure the boat does not sink.
        else if self.boat.len() > 2 {
            println!("The boat Sunk! You can only carry two objects at once.");
            self.restart_game();
        }
        // ensures that one object in the boat is a person, you know to do the rowing
        else {
            println!("One object crossing the river has to be a person");
            self.restart_game();
        }
    }

    // allows one to unload whatever object they choose to on whatever side of the river
    fn unload_object(&mut self, object: &'static str) {
        if !self.boat.is_empty() && self.boat.len() <= 2 {
            // Inform the user about their current bank position
            if self.is_on_left_bank {
                println!("You are on the left bank.");
            } else {
                println!("You are on the right bank.");
            }

            // Remove the specified object from the boat and add it to the current bank
            if let Some(index) = self.boat.iter().position(|&p| p == object) {
                let removed_object = self.boat.remove(index);
                if self.is_on_left_bank {
                    println!("unloading {} to the left bank", removed_object);
                    self.left_bank.push(removed_object);
                } else {
                    println!("unloading {} to the right bank", removed_object);
                    self.right_bank.push(removed_object);
                }
                println!(
                    "Object '{}' unloaded from the boat and added to the bank.",
                    object
                );
            } else {
                println!("Object '{}' not found on the boat.", object);
            }
        } else {
            println!("The boat is empty or cannot accommodate more objects to unload.");
        }
    }
    // requirements to win the game
    fn is_goal_state(&self) -> bool {
        self.left_bank.len() == 0 && self.boat.len() == 0 && self.right_bank.len() == 4
    }
    // restarts the game
    fn restart_game(&mut self) {
        *self = GameState::new();
        println!("Restarting the game.");
    }
}

fn main() {
    let mut game_state = GameState::new();

    println!("Hello there! Welcome to the River Crossing Puzzle");

    loop {
        println!("Current State:");
        println!("Left bank {:?}", game_state.left_bank);
        println!("Right bank {:?}", game_state.right_bank);
        println!("Boat {:?}", game_state.boat);

        println!("Enter a command (Options: 'move goat, 'move person', 'move boat', 'move grass', 'unload goat', 'unload person', 'unload lion' 'unload grass':");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        match input.trim() {
            "move goat" => game_state.move_object("goat"),
            "move lion" => game_state.move_object("lion"),
            "move grass" => game_state.move_object("grass"),
            "move person" => game_state.move_object("person"),
            "move boat" => game_state.move_boat(),
            "unload goat" => game_state.unload_object("goat"),
            "unload lion" => game_state.unload_object("lion"),
            "unload grass" => game_state.unload_object("grass"),
            "unload person" => game_state.unload_object("person"),

            _ => println!("Invalid Command. Please try again"),
        }

        if game_state.is_goal_state() {
            println!("Congratulations! You solved the puzzle.");
            break;
        }
    }
}
