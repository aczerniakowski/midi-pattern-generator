//! Logique musicale du projet : patterns
//!
//! Ce module contient les structures et fonctions liées
//! aux patterns.
use crate::music::chord::ChordType;
/// Représente un évènement à partir d'un accord.
///
/// Un évènement est défini par une note root, un type d'accord,
/// un début d'évènement et une durée
#[derive(Debug, Clone, Copy, PartialEq)]
struct PatternEvent {
    root:i32,
    chord: ChordType,
    start: i32,
    duration: i32,
}
/// Représente un pattern à partir d'évènements et d'une taille.
///
/// Un pattern est défini par un suite d'évènements
struct Pattern{
    size: u8,
    sequence:Vec<PatternEvent>,
}
/// Génère un évènement.
///
/// # Arguments
///
/// * `root` - Note racine en valeur MIDI (ex: 60 = C4)
/// * `chord` - Type d'accord (majeur ou mineur)
/// * `start` - Début de l'évènement
/// * `duration` - Durée de l'évènement
/// # Retour
///
/// Une strcture contenant un évènement.
///
/// # Exemple
///
/// ```
/// use midi_pattern_generator::music::pattern::{create_event, PatternEvent};
///
/// let event = create_event(60, ChordType::Major, 0, 1);
/// assert_eq!(event, PatternEvent {root:60, chord:ChordType::Major, start:0, duration:1});
/// ```
fn create_event(root: i32, chord: ChordType, start: i32, duration: i32) -> PatternEvent {
    // Associe à intervals le vecteur representant les intervalles
    // pour l'accord 'chord' et la note 'root' voulue
    let pattern_tmp: PatternEvent = PatternEvent {root:root, chord:chord, start:start, duration:duration};
    return pattern_tmp;
}

#[cfg(test)]
mod tests {
    //! Tests unitaires pour la génération des patterns.
    //!
    //! Ces tests vérifient que les fonctions du module `chord`
    //! produisent des résultats musicaux corrects pour des cas simples.
    use crate::music::chord::ChordType;
    use crate::music::pattern::PatternEvent;
    use crate::music::pattern::create_event;
    #[test]
    fn create_event_returns_correct_event() 
    {
        let root: i32 = 60; // C4 en midi
        let minor: ChordType = ChordType::Minor;
        let start: i32 = 0;
        let duration: i32 = 1;
        assert_eq!(create_event(root, minor, start, duration), PatternEvent {root:60, chord:ChordType::Minor, start:0, duration:1});
    }
}