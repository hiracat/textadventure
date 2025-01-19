use std::{
    collections::HashSet,
    fmt::format,
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
        current_room: LocationId::Starter,
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
                Some(x) => x.clone(), // Dereference and copy the LocationId
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
                    game_state.inventory.insert(x);
                }
                Action::Replace(x) => {
                    game_state.inventory.remove(&x);
                    current_room.items.insert(x);
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
                Action::GetHelp => {
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
    // quit
    if input == "quit" || input == "q" {
        return Some(Action::Exit);
    }
    if input == "help" {
        return Some(Action::GetHelp);
    }
    if input == "inventory" {
        return Some(Action::ShowInventory);
    }
    // move between rooms
    for exit in location.exits.iter() {
        if input.contains(&exit.name())
            && (input.contains("take")
                || input.contains("go to")
                || input.contains("move to")
                || input.contains("walk"))
        {
            return Some(Action::Move(*exit));
        }
    }
    // items
    for item in location.items.iter() {
        if input.contains(&item.name) {
            if input.contains("pick up")
                || input.contains("take")
                || input.contains("steal")
                || input.contains("grab")
            {
                return Some(Action::Pickup(item.clone()));
            }
        }
    }
    for item in game_state.inventory.iter() {
        if input.contains(&item.name) {
            if input.contains("use") {
                return Some(Action::Use(item.clone()));
            } else if input.contains("put back")
                || input.contains("replace")
                || input.contains("return")
            {
                return Some(Action::Replace(item.clone()));
            }
        }
    }
    for item in location.items.iter().chain(game_state.inventory.iter()) {
        if input.contains("examine") || input.contains("look at") {
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
    GetHelp,
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
    Starter,
    Room2,
    Hall,
    Unimplimented,
}
impl LocationId {
    fn name(&self) -> String {
        match self {
            LocationId::Starter => "starter room",
            LocationId::Room2 => "room 2",
            LocationId::Hall => "hall",
            LocationId::Unimplimented => unimplemented!(),
        }
        .to_string()
    }
}

#[derive(Debug)]
struct Location {
    descriptions: Vec<String>, // descriptions, in decreasing degrees of complexity
    print_description_callback: fn(&mut Self),
    description_detail_index: usize,
    exits: HashSet<LocationId>,
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
            exits: HashSet::new(),
            items: HashSet::new(),
            enter_callback: None,
            exit_callback: None,
        }
    }
}
fn init_locations() -> Vec<Location> {
    let mut locations = Vec::with_capacity(LocationId::Unimplimented as usize);

    let mut exits = HashSet::new();
    exits.insert(LocationId::Room2);
    exits.insert(LocationId::Hall);
    let mut items = HashSet::new();
    items.insert(Item {
        name: "bird".to_string(),
        description: "it is a dead bird with a broken wing, probably trapped by the glass cieling and couldent escape".to_string(),
        ..Default::default()
    });
    locations.insert(
        LocationId::Starter as usize,
        Location {
            descriptions: vec![
                "hello, this is the starter room".to_string(),
                "starter room".to_string(),
            ],
            exits,
            items,
            ..Default::default()
        },
    );

    let mut exits = HashSet::new();
    exits.insert(LocationId::Starter);
    locations.insert(
        LocationId::Room2 as usize,
        Location {
            descriptions: vec!["wow, this is room 2".to_string(), "room 2".to_string()],
            exits,
            ..Default::default()
        },
    );

    let mut exits = HashSet::new();
    exits.insert(LocationId::Starter);
    locations.insert(
        LocationId::Hall as usize,
        Location {
            descriptions: vec!["wow, this is the hall".to_string(), "hall".to_string()],
            exits,
            enter_callback: Some(Location::entered_hall),
            ..Default::default()
        },
    );

    // println!("{:#?}", locations);
    locations
}
