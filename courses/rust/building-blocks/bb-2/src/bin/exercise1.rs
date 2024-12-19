// This exercise and the next two will introduce basic serialization and deserialization with serde. serde serializes data quickly and is easy to use, while also being extensible and expressive.

// For your serializable data structure, imagine a flat game-playing surface covered in a grid of squares, like a chess board. Imagine you have a game character that every turn may move any number of squares in a single direction. Define a type, Move that represents a single move of that character.

// Derive the Debug trait so Move is easily printable with the {:?} format specifier.

// Write a main function that defines a variable, a, of type Move, serializes it with serde to a File, then deserializes it back again to a variable, b, also of type Move.

// Use JSON as the serialization format.

// Print a and b with println! and the {:?} format specifier to verify successful deserialization.

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Write};

// Run using command `cargo run --bin exercise1

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Move {
    pub direction: Direction,
    pub distance: u32,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::UP => write!(f, "up"),
            Direction::DOWN => write!(f, "down"),
            Direction::LEFT => write!(f, "left"),
            Direction::RIGHT => write!(f, "right"),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.direction, self.distance)
    }
}

fn main() {
    let a = Move {
        direction: Direction::UP,
        distance: 5,
    };

    let file_path = "move.json";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();

    let mut writer = BufWriter::new(&file);
    let serialized_move = serde_json::to_string(&a).unwrap();
    writer.write_all(serialized_move.as_bytes()).unwrap();

    println!("Serialized move: {}", serialized_move);

    // Close the file to free up resources
    drop(writer);

    let file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut reader = BufReader::new(&file);

    let mut buffer = String::new();

    reader.read_line(&mut buffer).unwrap();

    let b: Move = match from_str(&buffer) {
        Ok(move_deserialized) => move_deserialized,
        Err(err) => panic!("Error deserializing move: {:?}", err),
    };

    println!("Deserialized move: {:?}", b);
}
