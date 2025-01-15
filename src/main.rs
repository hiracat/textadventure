use std::{
    collections::HashMap,
    io::{self, stdin, Read, Write},
    thread::sleep,
    time::Duration,
};

fn main() {
    slow_type("HELLO, WELCOME TO THE DUNGEON OF DOOOM");
    let mut locations = init_locations();
    let mut game_state = GameState {
        items: vec![],
        current_room: LocationId::Starter,
    };

    loop {
        let current_room = game_state.current_room as usize;
        locations[current_room].print_description();
        loop {
            slow_type("what do you want to do");
            let mut input = String::new();
            stdin().read_line(&mut input);
            let input = input.trim_ascii();
            println!("{}", input);
            match locations[current_room].exits.get(input) {
                Some(x) => {
                    game_state.current_room = *x;
                    break;
                }
                None => slow_type("you cant do that"),
            }
        }
    }
}

fn slow_type(input: &str) {
    for char in input.chars() {
        print!("{}", char);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(50));
    }
    print!("\n");
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum LocationId {
    Starter,
    Room2,
    Hall,
    Unimplemented,
}

#[derive(Debug)]
enum Item {
    Bird,
}

#[derive(Debug)]
struct GameState {
    items: Vec<Item>,
    current_room: LocationId,
}

#[derive(Debug)]
struct Location {
    descriptions: Vec<String>, // descriptions, in decreasing degrees of complexity
    description_callback: fn(&mut Self),
    description_index: usize,

    exits: HashMap<&'static str, LocationId>,

    items: Option<Vec<Item>>,

    enter_callback: Option<fn(&mut Self)>,
    exit_callback: Option<fn(&mut Self)>,
}

impl Location {
    pub fn print_description(&mut self) {
        (self.description_callback)(self);
    }
    fn print_default(&mut self) {
        slow_type(&self.descriptions[self.description_index]);
        if self.descriptions.len() - 1 > self.description_index {
            self.description_index += 1;
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            descriptions: vec!["template room".to_string()],
            description_index: 0,
            description_callback: Location::print_default,
            exits: HashMap::new(),
            enter_callback: None,
            exit_callback: None,
            items: None,
        }
    }
}
fn init_locations() -> Vec<Location> {
    let mut locations = Vec::with_capacity(LocationId::Unimplemented as usize);

    let mut exits = HashMap::new();
    exits.insert("room2", LocationId::Room2);
    exits.insert("hall", LocationId::Hall);

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

    let mut exits = HashMap::new();
    exits.insert("starter", LocationId::Starter);
    locations.insert(
        LocationId::Room2 as usize,
        Location {
            descriptions: vec!["wow, this is room 2".to_string(), "room 2".to_string()],
            exits,
            ..Default::default()
        },
    );

    let mut exits = HashMap::new();
    exits.insert("starter", LocationId::Starter);

    locations.insert(
        LocationId::Hall as usize,
        Location {
            descriptions: vec!["wow, this is the hall".to_string(), "hall".to_string()],
            exits,
            ..Default::default()
        },
    );

    println!("{:#?}", locations);
    locations
}
