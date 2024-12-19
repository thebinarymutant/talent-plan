// Exercise: Serialize and deserialize 1000 data structures with serde (BSON).

// This one is slightly different. Where the previous exercises serialized and deserialized a single value to a buffer, in this one serialize 1000 different Move values to a single file, back-to-back, then deserialize them again. This time use the BSON format.

// Things to discover here are whether serde automatically maintains the correct file offsets (the "cursor") to deserialize multiple values in sequence, or if you need to parse your own "frames" around each value to define their size, and how to detect that there are no more values to parse at the end of the file.

// After you've succeeded at serializing and deserializing multiple values to a file, try it again with a Vec<u8>. Serializing and deserializing generally requires the destination implement the Write and Read traits. Does Vec<u8> implement either or both? What is the behavior of those implementations? You may need to wrap your buffer in wrapper types that implement these traits in order to get the correct behavior — the API docs for the traits list all their implementors in the standard library, and whatever you need will be in there somewhere.
use bson::{doc, to_bson, to_vec, Document};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};

// Run using command `cargo run --bin exercise3
//
// Bson deserialization -> Each call deserializes a document boundary.
// So we can have multiple documents stores in a file but will have to read through the whole file

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

pub fn read_next<R>(mut reader: R) -> Result<Move, Box<dyn Error>>
where
    R: Read,
{
    let doc = match bson::Document::from_reader(&mut reader) {
        Ok(result) => result,
        Err(err) => {
            return Err(Box::new(err));
        }
    };
    let result: Move = bson::de::from_document(doc)?;
    Ok(result)
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

fn main() -> Result<(), Box<dyn Error>> {
    let va: Vec<Move> = (0..=999)
        .map(|x| Move {
            direction: Direction::UP,
            distance: x,
        })
        .collect();

    let file_path = "move.bson";
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let mut writer = BufWriter::new(&file);

    va.iter().for_each(|v| {
        let serialized = to_bson(&v).unwrap();
        let doc = serialized.as_document().unwrap();
        doc.to_writer(&mut writer).unwrap();
        writer.flush().unwrap();
    });

    let file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut reader = BufReader::new(&file);

    let mut i = 0;
    while let Ok(the_move) = read_next(&mut reader) {
        println!("Move A = {:?}", va[i]);
        println!("Move B = {:?}", the_move);
        i += 1;
    }

    // 3.1 Serializing and deserializing multiple values to a Vec<u8>
    //
    let mut vec_buf: Vec<u8> = Vec::new();
    for i in 0..=999 {
        let serialized = to_bson(&va[i]).unwrap();
        let doc = serialized.as_document().unwrap();
        let v: Vec<u8> = to_vec(doc).unwrap();
        vec_buf = [vec_buf, v].concat();
    }

    // Convert to &[u8], which supports std::io::Read
    let mut u = &vec_buf[..];

    let mut i = 0;
    while let Ok(the_move) = read_next(&mut u) {
        println!("Move A = {:?}", va[i]);
        println!("Move B = {:?}", the_move);
        i += 1;
    }

    Ok(())
}
