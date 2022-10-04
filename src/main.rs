// todo:
// turn off "helper"!

#![allow(dead_code)]
#![allow(non_snake_case)]

#[derive(Debug)]
enum Chroma {
    A,
}

#[derive(Debug)]
struct Note {
    chroma: Chroma,
    octave: u8, // todo: Octave type limited to 0-8?
}

fn main() {
    // println!("Hello, world!");

    let A4 = Note { chroma: Chroma::A, octave: 4 };
    println!("{A4:?}")
}
