use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    io::{self, stdin, Write},
    thread::sleep,
    time::Duration,
    vec,
};

static mut DEBUG_MODE: bool = true;

fn debug_mode(mode: bool) {
    unsafe { DEBUG_MODE = mode }
}

fn debug_print(input: impl Debug) {
    unsafe {
        if DEBUG_MODE {
            dbg!(input);
        }
    }
}

fn main() {
    slow_type("HELLO, WELCOME TO THE DUNGEON OF DOOOM\n");
    let mut locations = init_locations();
    let mut game_state = GameState {
        inventory: HashSet::new(),
        current_room: LocationId::USB,
    };

    loop {
        let current_room = locations.get_mut(&game_state.current_room).unwrap();
        current_room.print_description();
        if let Some(callback) = current_room.enter_callback {
            callback(current_room);
        }
        loop {
            // First get the action of the match by getting value of key in hasmap
            let mut input = String::new();
            let _ = stdin().read_line(&mut input);
            let input = input.trim().to_lowercase();

            let mut action = current_room.custom_input_processing(&input, &mut game_state);
            debug_print("action?");
            debug_print(action.clone());

            if action == Action::None {
                action = match process_input(&input, &mut game_state, current_room) {
                    Some(x) => x.clone(),
                    None => {
                        slow_type("you cant do that\n");
                        continue; // Skip the rest and start the loop over
                    }
                };
            }
            debug_print("action?");
            debug_print(action.clone());

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
                Action::Debug(x) => {
                    debug_mode(x);
                }
                Action::None => unimplemented!(),
            }
        }
    }
}

fn process_input(
    input: &str,
    game_state: &mut GameState,
    location: &mut Location,
) -> Option<Action> {
    // input must be an exact match
    let quit_input = ["quit", "q", "exit"];
    let help_input = ["help", "h", "?"];
    let inventory_input = ["inventory", "i", "inv"];
    let debug_input = ["debug"];

    // words will be searched through entire input string
    let moveto_words = ["go to", "go", "walk", "take", "go through", "go back"];
    let pickup_words = ["pick up", "take", "grab", "steal"];
    let ret_words = ["return", "replace", "put back"];
    let use_words = ["use", "open"];
    let examine_words = ["examine", "look at", "inspect"];

    if contains(input, debug_input.into()) {
        if contains(input, vec!["off"]) {
            return Some(Action::Debug(false));
        } else {
            return Some(Action::Debug(true));
        }
    }

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

fn contains(input: &str, contains: Vec<&str>) -> bool {
    contains.iter().any(|&word| input.contains(word))
}

fn slow_type(input: &str) {
    for char in input.chars() {
        print!("{}", char);
        let _ = io::stdout().flush();
        sleep(Duration::from_millis(1));
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
    Debug(bool),
    Exit,
    None,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Item {
    name: &'static str,
    description: &'static str,
    examined: bool,
    use_item: Option<fn(&mut GameState, &mut Location)>,
}
impl Default for Item {
    fn default() -> Self {
        Item {
            name: "rock",
            description: "its a rock, not much interesting",
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

#[derive(Debug)]
struct Location {
    descriptions: Vec<&'static str>, // descriptions, in decreasing degrees of complexity
    description_detail_index: usize,
    exits: Vec<(Vec<&'static str>, LocationId)>,
    items: HashSet<Item>,

    enter_callback: Option<fn(&mut Self)>,
    exit_callback: Option<fn(&mut Self)>,
    process_custom_input_callback: Option<fn(&str) -> Action>,
}

impl Location {
    pub fn print_description(&mut self) {
        slow_type(&self.descriptions[self.description_detail_index]);
        slow_type("\n");
        if self.descriptions.len() - 1 > self.description_detail_index {
            self.description_detail_index += 1;
        }
    }

    pub fn custom_input_processing(&mut self, input: &str, game_state: &mut GameState) -> Action {
        if let Some(callback) = self.process_custom_input_callback {
            debug_print("running custom input callback");
            return (callback)(input);
        }
        debug_print("no input callback");
        Action::None
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            descriptions: vec!["an empty room"],
            description_detail_index: 0,
            exits: Vec::new(),
            items: HashSet::new(),
            enter_callback: None,
            exit_callback: None,
            process_custom_input_callback: None,
        }
    }
}

fn kick_boulder(location: &mut Location, game_state: &mut GameState) {
    location.exits.push((
        vec![
            "boulder",
            "hidden",
            "sealed",
            "covered",
            "blocked",
            "passageway",
            "dark",
        ],
        LocationId::Downloads,
    ));
    slow_type("you push on the boulder, and it slowly moves away to reveal a dark passageway\n");
}

fn input_kick_boulder(input: &str) -> Action {
    let push_words = ["kick", "shove", "push", "move"];
    if contains(&input, push_words.to_vec()) {
        debug_print("in input_kick_boudler with a custom action");
        return Action::_Custom(kick_boulder);
    }
    debug_print("in input_kcik boudler with no custom action");
    Action::None
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
enum LocationId {
    USB,
    FileExplorer,
    Desktop,
    Downloads,
    Music,
    Unimplimented,
}

fn show_photo(game_state: &mut GameState, location: &mut Location) {
    slow_type(
        r#"
                       _
                       | \
                       | |
                       | |
  |\                   | |
 /, ~\                / /
X     `-.....-------./ /
 ~-. ~  ~              |
    \             /    |
     \  /_     ___\   /
     | /\ ~~~~~   \ |
     | | \        || |
     | |\ \       || )
    (_/ (_/      ((_/
"#,
    )
}

fn cont_beginning(input: &str) -> Action {
    if input == "continue" {
        Action::Move(LocationId::FileExplorer)
    } else {
        Action::None
    }
}

fn init_locations() -> HashMap<LocationId, Location> {
    let mut locations = HashMap::with_capacity(LocationId::Unimplimented as usize);

    locations.insert(
        LocationId::USB,
        Location {
            descriptions: vec![
                "You wake up, you have another job to do. You hear the familiar bling of windows automatically mounting your host, its time to work now. All you have to now is \x1b[5mcontinue.\x1b[5m",
                "Back to the safety of the usb, you will go back to file explorer when you catch your breath",
            ],
            exits: vec![
                (vec!["file explorer"], LocationId::FileExplorer),
            ],
            process_custom_input_callback: Some(cont_beginning),
            ..Default::default()
        },
    );

    locations.insert(
        LocationId::FileExplorer,
        Location {
            descriptions: vec![
                "File explorer Springs to life. You always though it was a particularly useless program to be stuck in, but unfortunately its the default, so now you need a way to escape. The only folders that you can see are desktop, downloads, and music",
                "Back to an overview of file explorer",
            ],
            exits: vec![
                (vec!["desktop"], LocationId::Desktop),
                (vec!["downloads"], LocationId::Downloads),
                (vec!["dusic"], LocationId::Music),
                (vec!["home", "usb"], LocationId::USB),
            ],
            items: HashSet::new(),
            ..Default::default()
        },
    );

    locations.insert(
        LocationId::Desktop,
        Location {
            descriptions: vec!["You Look around their desktop, nothing you are after, maybe some good photos, but she probably has backups, oh, and a symlink to chrome too, that is very useful", "The Desktop"],
            exits: vec![(vec!["start", "crystal cavern"], LocationId::USB)],
            items: {
                let mut items = HashSet::new();
                items.insert(Item {
                    name: "cat photo",
                    description: "A photo, what could it be?",
                    use_item: Some(show_photo),
                        ..Default::default()

                });
                items
            },
            ..Default::default()
        },
    );

    locations.insert(
        LocationId::Downloads,
        Location {
            descriptions: vec![
                "you walk down the dark passageway, and enter a small alcove, with some water dripping down the wall",
                "dark passageway room",
            ],
            exits: vec![(vec!["small room", "passageway", "back"], LocationId::FileExplorer)],
            items: HashSet::new(),
            ..Default::default()
        },
    );
    locations.insert(
        LocationId::Music,
        Location {
            descriptions: vec![
                "you walk down the dark passageway, and enter a small alcove, with some water dripping down the wall",
                "dark passageway room",
            ],
            exits: vec![(vec!["small room", "passageway", "back"], LocationId::FileExplorer)],
            items: HashSet::new(),
            ..Default::default()
        },
    );

    locations
}
