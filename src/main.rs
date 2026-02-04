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
use music::chord::{notes_in_chord, ChordType};
fn main() {
    println!("~~ Midi Pattern Generator ~~");
    println!("Final Project CS50");
    println!("Language : Rust");
    
    // Etape 1 : Scale
    // Étape 1.1 : définition des paramètres d’exécution (temporaire, sans CLI)
    let root_note: i32 = 60; // C4 en midi
    let scale_minor_type: ScaleType = ScaleType::Minor; // Gamme mineure
    let scale_major_type: ScaleType = ScaleType::Major; // Gamme majeure

    // Étape 1.2 : génération de la logique musicale
    let scale_minor: Vec<i32> = notes_in_scale(root_note, scale_minor_type);
    let scale_major: Vec<i32> = notes_in_scale(root_note, scale_major_type);

    // Étape 1.3 : sortie utilisateur (debug / validation)
    println!("Gamme {:?} générée à partir de la note {:?}: {:?}", scale_minor_type, root_note, scale_minor);
    println!("Gamme {:?} générée à partir de la note {:?}: {:?}", scale_major_type, root_note, scale_major);
    
    // Etape 2 : Chord
    // Étape 1.1 : définition des paramètres d’exécution (temporaire, sans CLI)
    let root: i32 = 60; // C4 en midi
    let minor_type: ChordType = ChordType::Minor; // Gamme mineure
    let major_type: ChordType = ChordType::Major; // Gamme majeure

    // Étape 1.2 : génération de la logique musicale
    let chord_minor: Vec<i32> = notes_in_chord(root_note, minor_type);
    let chord_major: Vec<i32> = notes_in_chord(root_note, major_type);

    // Étape 1.3 : sortie utilisateur (debug / validation)
    println!("Accord {:?} générée à partir de la note {:?}: {:?}", minor_type, root, chord_minor);
    println!("Accord {:?} générée à partir de la note {:?}: {:?}", major_type, root, chord_major);
    }
