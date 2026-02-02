//! MIDI Pattern Generator
//!
//! Ce programme est le point d’entrée du projet *MIDI Pattern Generator*.
//! Il orchestre les différentes étapes du traitement :
//!
//! 1. Lecture des paramètres utilisateur (CLI)
//! 2. Génération de la logique musicale (gammes, accords, patterns)
//! 3. Export du résultat sous forme de fichier MIDI
//!
//! La logique métier est volontairement séparée dans des modules dédiés
//! afin de garder `main.rs` simple, lisible et centré sur le flux global.
mod music;

use music::scale::{notes_in_scale, ScaleType};
fn main() {
    println!("~~ Midi Pattern Generator ~~");
    println!("Final Project CS50");
    println!("Language : Rust");
    let scale: Vec<i32> = notes_in_scale(60, ScaleType::Minor);
    println!("Gamme générée : {:?}", scale);
    }
