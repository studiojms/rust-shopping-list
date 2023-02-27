use std::{collections::BTreeMap, str::FromStr};

fn main() {
    println!("Hello, world!");
    let mut shopping_list = ShoppingList::new();
    shopping_list.add(Item::Banana, 10);
}

enum Command {
    Add { item: Item, quantity: u8 },
}

enum CommandError {
    InvalidCommand,
    EmptyCommand,
    Unknown(String),
    MissingItem,
    ItemParsingFailed,
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let command_part = parts.next();
        match command_part {
            Some(cmd) => match cmd {
                "add" => {
                    let item_part = parts.next().ok_or(CommandError::MissingItem)?;
                    let item = Item::from_str(item_part)?;
                    let quantity = 1;
                    Ok(Self::Add { item, quantity })
                }
                x => Err(CommandError::Unknown(x.to_string())),
            },
            None => Err(CommandError::EmptyCommand),
        }
    }
}

struct ItemError;

impl From<ItemError> for CommandError {
    fn from(_: ItemError) -> Self {
        CommandError::ItemParsingFailed
    }
}

impl FromStr for Item {
    type Err = ItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "banana" => Ok(Item::Banana),
            "coffee" => Ok(Item::Coffee),
            "red-pepper" => Ok(Item::Pepper(Color::Red)),
            "green-pepper" => Ok(Item::Pepper(Color::Green)),
            &_ => todo!(),
        }
    }
}

struct ShoppingList {
    list: BTreeMap<Item, u8>,
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum Item {
    Banana,
    Coffee,
    Pepper(Color),
}

#[derive(Ord, PartialOrd, PartialEq, Eq)]
enum Color {
    Red,
    Green,
}

impl ShoppingList {
    fn new() -> Self {
        Self {
            list: BTreeMap::new(),
        }
    }

    fn add(&mut self, item: Item, quantity: u8) {
        self.list
            .entry(item)
            .and_modify(|current_quantity| *current_quantity += quantity)
            .or_insert(quantity);
    }

    fn get(&self, item: Item) -> Option<u8> {
        let res = self.list.get(&item);
        match res {
            Some(value) => Some(*value),
            None => None,
        }
    }

    fn list(&self) -> Vec<(&Item, &u8)> {
        let mut vec = Vec::new();
        for x in self.list.iter() {
            vec.push(x)
        }
        vec
    }
}
