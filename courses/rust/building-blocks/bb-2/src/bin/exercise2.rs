// Exercise: Serialize and deserialize a data structure to a buffer with serde (RON).

// Do the same as above, except this time, instead of serializing to a File, serialize to a Vec<u8> buffer, and after that try using RON instead of JSON as the format. Are there any differences in serialization to a Vec instead of a File? What about in using the RON crate vs the JSON crate?

// Convert the Vec<u8> to String with str::from_utf8, unwrapping the result, then print that serialized string representation to see what Move looks like serialized to RON.
use core::str;
use ron;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::BufWriter;

// Run using command `cargo run --bin exercise2

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

    let mut ron_vec_buff: Vec<u8> = vec![];
    let writer = BufWriter::new(&mut ron_vec_buff);
    ron::ser::to_writer(writer, &a).unwrap();

    let ron_deserialized_string: &str = str::from_utf8(&ron_vec_buff).unwrap();

    println!(
        "Vec contents: {:?} {:?}",
        ron_deserialized_string, ron_vec_buff
    );
}
