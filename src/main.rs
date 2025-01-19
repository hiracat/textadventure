use std::{
    collections::HashSet,
    io::{self, stdin, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    slow_type("HELLO, WELCOME TO THE DUNGEON OF DOOOM\n");
    let mut locations = init_locations();
    let mut game_state = GameState {
        inventory: HashSet::new(),
        current_room: LocationId::Starter,
    };

    loop {
        let current_room = game_state.current_room.clone() as usize;
        if let Some(callback) = locations[current_room].enter_callback {
            callback(&mut locations[current_room]);
        }
        locations[current_room].print_description();
        loop {
            // First get the action of the match by getting value of key in hasmap
            let action = match process_input(&game_state, &locations[current_room]) {
                Some(x) => x.clone(), // Dereference and copy the LocationId
                None => {
                    slow_type("you cant do that\n");
                    continue; // Skip the rest and start the loop over
                }
            };

            match action {
                Action::Move(x) => {
                    if let Some(callback) = locations[current_room].exit_callback {
                        callback(&mut locations[current_room]);
                    }
                    game_state.current_room = x;
                    break;
                }
                Action::Pickup(x) => {
                    locations[current_room].items.remove(&x);
                }
                Action::Replace(x) => {
                    locations[current_room].items.insert(x);
                }
                Action::_Custom(x) => (x)(&mut locations[current_room], &mut game_state),
                Action::Exit => {
                    return;
                }
            }
        }
    }
}

fn process_input(game_state: &GameState, location: &Location) -> Option<Action> {
    let mut input = String::new();
    let mut actions: Vec<String> = vec![];
    let _ = stdin().read_line(&mut input);
    let input = input.trim_ascii().to_lowercase();
    if input == "quit" || input == "q" {
        return Some(Action::Exit);
    }

    for exit in location.exits.iter() {
        if input.contains(&exit.name()) {
            return Some(Action::Move(*exit));
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
    _Custom(fn(&mut Location, &mut GameState)),
    Pickup(Item),
    Replace(Item),
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
    id: LocationId,
    name: String,

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
            id: LocationId::Unimplimented,
            name: "empty room".to_string(),
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
    locations.insert(
        LocationId::Starter as usize,
        Location {
            descriptions: vec![
                "hello, this is the starter room".to_string(),
                "starter room".to_string(),
            ],
            exits,
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
