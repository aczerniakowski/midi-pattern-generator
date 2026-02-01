mod music;

use music::scale::{notes_in_scale, ScaleType};
fn main() {
    println!("~~ Midi Pattern Generator ~~");
    println!("Final Project CS50");
    println!("Language : Rust");
    let scale: Vec<i32> = notes_in_scale(60, ScaleType::Minor);
    println!("Gamme générée : {:?}", scale);
    }
