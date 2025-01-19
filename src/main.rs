use std::{
    collections::HashSet,
    io::{self, stdin, Write},
    thread::sleep,
    time::Duration,
    vec,
};

fn main() {
    slow_type("HELLO, WELCOME TO THE DUNGEON OF DOOOM\n");
    let mut locations = init_locations();
    let mut game_state = GameState {
        inventory: HashSet::new(),
        current_room: LocationId::CrystalCave,
    };

    loop {
        let current_room = &mut locations[game_state.current_room as usize];
        if let Some(callback) = current_room.enter_callback {
            callback(current_room);
        }
        current_room.print_description();
        loop {
            // First get the action of the match by getting value of key in hasmap
            let action = match process_input(&game_state, &current_room) {
                Some(x) => x.clone(),
                None => {
                    slow_type("you cant do that\n");
                    continue; // Skip the rest and start the loop over
                }
            };

            match action {
                Action::Move(x) => {
                    if let Some(callback) = current_room.exit_callback {
                        callback(current_room);
                    }
                    game_state.current_room = x;
                    break;
                }
                Action::Pickup(x) => {
                    current_room.items.remove(&x);
                    game_state.inventory.insert(x.clone());
                    slow_type(&format!("you pick up the {}\n", x.name));
                }
                Action::Replace(x) => {
                    game_state.inventory.remove(&x);
                    current_room.items.insert(x.clone());
                    slow_type(&format!("you put back the {}\n", x.name));
                }
                Action::Use(x) => {
                    if let Some(callback) = x.use_item {
                        (callback)(&mut game_state, current_room)
                    } else {
                        slow_type(&format!("you arent sure what to do with {}\n", x.name));
                    }
                }
                Action::Examine(x) => slow_type(&format!("{}\n", x.description)),
                Action::_Custom(x) => (x)(current_room, &mut game_state),
                Action::ShowInventory => {
                    slow_type("INVENTORY:\n");
                    for item in game_state.inventory.iter() {
                        slow_type(&item.name);
                        slow_type(", ");
                    }
                    slow_type("\n");
                }
                Action::Help => {
                    todo!()
                }
                Action::Exit => {
                    return;
                }
            }
        }
    }
}

fn process_input(game_state: &GameState, location: &Location) -> Option<Action> {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
    let input = input.trim_ascii().to_lowercase();

    // input must be an exact match
    let quit_input = ["quit", "q", "exit"];
    let help_input = ["help", "h", "?"];
    let inventory_input = ["inventory", "i", "inv"];

    // words will be searched through entire input string
    let moveto_words = ["go to", "go", "walk", "take", "go through", "go back"];
    let pickup_words = ["pick up", "take", "grab", "steal"];
    let ret_words = ["return", "replace", "put back"];
    let use_words = ["use"];
    let examine_words = ["examine", "look at", "inspect"];

    if quit_input.iter().any(|&word| word == input) {
        return Some(Action::Exit);
    }
    if help_input.iter().any(|&word| word == input) {
        return Some(Action::Help);
    }
    if inventory_input.iter().any(|&word| word == input) {
        return Some(Action::ShowInventory);
    }
    // move between rooms
    for exit in location.exits.iter() {
        if moveto_words.iter().any(|&word| input.contains(word))
            && exit.0.iter().any(|x| input.contains(x))
        {
            return Some(Action::Move(exit.1));
        }
    }
    // items
    for item in location.items.iter() {
        if pickup_words.iter().any(|&word| input.contains(word)) && input.contains(&item.name) {
            return Some(Action::Pickup(item.clone()));
        }
    }
    for item in game_state.inventory.iter() {
        if input.contains(&item.name) {
            if use_words.iter().any(|&word| input.contains(word)) && input.contains(&item.name) {
                return Some(Action::Use(item.clone()));
            }
            if ret_words.iter().any(|&word| input.contains(word)) && input.contains(&item.name) {
                return Some(Action::Replace(item.clone()));
            }
        }
    }
    for item in location.items.iter().chain(game_state.inventory.iter()) {
        if examine_words.iter().any(|&word| input.contains(word)) && input.contains(&item.name) {
            return Some(Action::Examine(item.clone()));
        }
    }
    None
}

fn slow_type(input: &str) {
    for char in input.chars() {
        print!("{}", char);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(20));
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Action {
    Move(LocationId),
    Pickup(Item),
    Replace(Item),
    Use(Item),
    Examine(Item),
    _Custom(fn(&mut Location, &mut GameState)),
    ShowInventory,
    Help,
    Exit,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Item {
    name: String,
    description: String,
    examined: bool,
    use_item: Option<fn(&mut GameState, &mut Location)>,
}
impl Default for Item {
    fn default() -> Self {
        Item {
            name: "rock".to_string(),
            description: "its a rock, not much interesting".to_string(),
            examined: false,
            use_item: None,
        }
    }
}

#[derive(Debug)]
struct GameState {
    inventory: HashSet<Item>,
    current_room: LocationId,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum LocationId {
    CrystalCave,
    Room2,
    Hall,
    Unimplimented,
}
//impl LocationId {
//    fn name(&self) -> String {
//        match self {
//            LocationId::CrystalCave => "crystal cave",
//            LocationId::Room2 => "room 2",
//            LocationId::Hall => "hall",
//            LocationId::Unimplimented => unimplemented!(),
//        }
//        .to_string()
//    }
//}

#[derive(Debug)]
struct Location {
    descriptions: Vec<String>, // descriptions, in decreasing degrees of complexity
    print_description_callback: fn(&mut Self),
    description_detail_index: usize,
    exits: Vec<(Vec<&'static str>, LocationId)>,
    items: HashSet<Item>,

    enter_callback: Option<fn(&mut Self)>,
    exit_callback: Option<fn(&mut Self)>,
}

impl Location {
    pub fn print_description(&mut self) {
        (self.print_description_callback)(self);
    }

    fn print_default(&mut self) {
        slow_type(&self.descriptions[self.description_detail_index]);
        slow_type("\n");
        if self.descriptions.len() - 1 > self.description_detail_index {
            self.description_detail_index += 1;
        }
    }
    fn entered_hall(&mut self) {
        println!("entered hall wheeee");
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            descriptions: vec!["an empty room".to_string()],
            description_detail_index: 0,
            print_description_callback: Location::print_default,
            exits: Vec::new(),
            items: HashSet::new(),
            enter_callback: None,
            exit_callback: None,
        }
    }
}
fn init_locations() -> Vec<Location> {
    let mut locations = Vec::with_capacity(LocationId::Unimplimented as usize);

    locations.insert(
        LocationId::CrystalCave as usize,
        Location {
            descriptions: vec![
                "You are in a cavern made of crystal walls and a glass ceiling. Before you, a pair of iron-reinforced double doors stand gilded, and to the side, a small hidden door awaits. On the floor there is a motionless bird".to_string(),
                "You return to the shimmering crystal cavern.".to_string(),
            ],
            exits: vec![
                (vec!["side door", "hidden"], LocationId::Room2),
                (vec!["iron", "gilded", "double doors"], LocationId::Hall),
            ],
            items: {
                let mut items = HashSet::new();
                items.insert(Item {
                    name: "bird".to_string(),
                    description: "A dead bird with a broken wing, likely trapped under the glass ceiling.".to_string(),
                    ..Default::default()
                });
                items
            },
            ..Default::default()
        },
    );

    locations.insert(
        LocationId::Room2 as usize,
        Location {
            descriptions: vec![
                "You are in Room 2, a smaller, simpler space connected to the cavern.".to_string(),
                "This is Room 2.".to_string(),
            ],
            exits: vec![(vec!["start", "crystal cavern"], LocationId::CrystalCave)],
            items: HashSet::new(),
            ..Default::default()
        },
    );

    locations.insert(
        LocationId::Hall as usize,
        Location {
            descriptions: vec![
                "You are in a grand hall with polished floors and towering columns, connected to the crystal cavern.".to_string(),
                "This is the Hall.".to_string(),
            ],
            exits: vec![(vec!["starter", "crystal cavern"], LocationId::CrystalCave)],
            items: HashSet::new(),
            enter_callback: Some(Location::entered_hall),
            ..Default::default()
        },
    );

    locations
}
