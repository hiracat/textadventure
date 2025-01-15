use std::{
    collections::{hash_map, HashMap},
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

#[derive(Eq, Hash, PartialEq)]
enum LocationId {
    Starter,
    Room2,
    Unimplemented,
}

enum Item {
    Bird,
}

struct GameState {
    items: Vec<Item>,
    current_room: LocationId,
}

struct Location<'a> {
    descriptions: Vec<String>, // descriptions, in decreasing degrees of complexity
    description_callback: fn(&mut Self),
    description_index: usize,

    exits: HashMap<&'a str, LocationId>,

    items: Option<Vec<Item>>,

    enter_callback: Option<fn(&mut Self)>,
    exit_callback: Option<fn(&mut Self)>,
}

impl Location<'_> {
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

impl Default for Location<'_> {
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

fn main() {
    let mut locations = Vec::with_capacity(LocationId::Unimplemented as usize);
    let mut exits = HashMap::new();
    exits.insert("room2", LocationId::Room2);
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
            ..Default::default()
        },
    );

    locations[LocationId::Starter as usize].print_description();
    locations[LocationId::Starter as usize].print_description();
    locations[LocationId::Starter as usize].print_description();

    locations[LocationId::Room2 as usize].print_description();
    locations[LocationId::Room2 as usize].print_description();
    locations[LocationId::Room2 as usize].print_description();

    slow_type("HELLO, WELCOME TO THE DUNGEON OF DOOOM");
}

fn slow_type(input: &str) {
    for char in input.chars() {
        print!("{}", char);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(50));
    }
    print!("\n");
}
