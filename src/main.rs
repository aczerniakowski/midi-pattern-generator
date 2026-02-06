//! MIDI Pattern Generator
//!
//! Point d’entrée du projet *MIDI Pattern Generator*.
//!
//! Rôle de `main.rs` :
//! - orchestrer le flux global
//! - appeler la logique musicale
//! - afficher des résultats de validation (debug)
//!
//! Toute la logique métier est isolée dans les modules dédiés.

mod music;

use music::scale::{notes_in_scale, ScaleType};
use music::chord::{notes_in_chord, ChordType};

fn main() {
    println!("~~ MIDI Pattern Generator ~~");
    println!("Final Project CS50");
    println!("Language : Rust");

    // Paramètres communs (temporaires, sans CLI)
    let root_note: i32 = 60; // C4 en MIDI

    // =========================
    // Étape 1 : Gammes
    // =========================
    let scale_minor = notes_in_scale(root_note, ScaleType::Minor);
    let scale_major = notes_in_scale(root_note, ScaleType::Major);

    println!(
        "Gamme {:?} à partir de {} : {:?}",
        ScaleType::Minor,
        root_note,
        scale_minor
    );
    println!(
        "Gamme {:?} à partir de {} : {:?}",
        ScaleType::Major,
        root_note,
        scale_major
    );

    // =========================
    // Étape 2 : Accords
    // =========================
    let chord_minor = notes_in_chord(root_note, ChordType::Minor);
    let chord_major = notes_in_chord(root_note, ChordType::Major);

    println!(
        "Accord {:?} à partir de {} : {:?}",
        ChordType::Minor,
        root_note,
        chord_minor
    );
    println!(
        "Accord {:?} à partir de {} : {:?}",
        ChordType::Major,
        root_note,
        chord_major
    );

    // Étape 3 (à venir) : Patterns
    // Étape 4 (à venir) : Export MIDI
}
