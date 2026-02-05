//! Logique musicale du projet : accords
//!
//! Ce module contient les structures et fonctions liées
//! aux accords.

/// Représente un accord.
///
/// Un accord est défini par une série d'intervalles
/// à partir d'une note racine et un type d'accord.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordType {
    Major,
    Minor,
}

/// Génère les notes d'un accord.
///
/// # Arguments
///
/// * `root` - Note racine en valeur MIDI (ex: 60 = C4)
/// * `chord` - Type d'accord (majeur ou mineur)
///
/// # Retour
///
/// Un vecteur contenant les notes MIDI de l'accord.
///
/// # Exemple
///
/// ```
/// use midi_pattern_generator::music::chord::{notes_in_chord, ChordType};
///
/// let notes = notes_in_chord(60, ChordType::Major);
/// assert_eq!(notes, vec![60, 64, 67]);
/// ```
pub fn notes_in_chord(root: i32, chord: ChordType) -> Vec<i32> {
    // Associe à intervals le vecteur representant les intervalles
    // pour l'accord 'chord' et la note 'root' voulue
    let intervals: Vec<i32> = match chord {
        ChordType::Major => vec![0,4,7],
        ChordType::Minor => vec![0,3,7],
    };
    intervals.iter().map(|i: &i32| root + i).collect()
}

#[cfg(test)]
mod tests {
    //! Tests unitaires pour la génération des accords.
    //!
    //! Ces tests vérifient que les fonctions du module `chord`
    //! produisent des résultats musicaux corrects pour des cas simples.
    use crate::music::chord::ChordType;
    use crate::music::chord::notes_in_chord;
    #[test]
    fn notes_in_chord_returns_correct_minor_chord() 
    {
        let root: i32 = 60; // C4 en midi
        let minor: ChordType = ChordType::Minor;
        assert_eq!(notes_in_chord(root, minor), [60, 63, 67]);
    }
    #[test]
    fn notes_in_chord_returns_correct_major_chord() 
    {
        let root: i32 = 60; // C4 en midi
        let major: ChordType = ChordType::Major;
        assert_eq!(notes_in_chord(root, major), [60, 64, 67]);
    }
}