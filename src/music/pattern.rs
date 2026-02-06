//! Logique musicale du projet : patterns
//!
//! Ce module contient les structures et fonctions liées
//! aux patterns.
use crate::music::chord::ChordType;

pub const MIDI_NOTE_MAX: i32 = 127;
pub const MIDI_NOTE_MIN: i32 = 0;

/// Représente un évènement à partir d'un accord.
///
/// Un évènement est défini par une note root, un type d'accord,
/// un début d'évènement et une durée
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PatternEvent {
    root:i32,
    chord: ChordType,
    start: i32,
    duration: i32,
}
/// Représente les erreurs lors de la création d'un évènement.
///
/// Plusieurs erreurs sont capturées : note invalide, début invalide
/// et durée invalide.
#[derive(Debug, PartialEq)]
pub enum PatternEventError{
    InvalidRoot,
    InvalidStart,
    InvalidDuration,
}
/// Génère un évènement d'un pattern.
///
/// # Arguments
///
/// * `root` - Note racine en valeur MIDI (ex: 60 = C4) 0 <= root <= 127
/// * `chord` - Type d'accord (majeur ou mineur)
/// * `start` - Début de l'évènement >= 0
/// * `duration` - Durée de l'évènement > 0
///
/// # Retour
///
/// L'évènement crée (PatternEvent) ou un message d'erreur (PatternEventError)
///

impl PatternEvent{
    pub fn new(root:i32, chord: ChordType, start: i32, duration: i32) -> Result<Self, PatternEventError>
    {
        if root > MIDI_NOTE_MAX || root < MIDI_NOTE_MIN
        {
            return Err(PatternEventError::InvalidRoot);
        }
        if start < 0
        {
            return Err(PatternEventError::InvalidStart);
        }
        if duration <= 0
        {
            return Err(PatternEventError::InvalidDuration);
        }
        Ok(Self {root, chord, start, duration})
    }
    // Getters si besoin
    pub fn root(&self) -> i32 { self.root }
    pub fn start(&self) -> i32 { self.start }
    pub fn duration(&self) -> i32 { self.duration }
}
/// Représente un pattern à partir d'évènements et d'une taille.
///
/// Un pattern est défini par un suite d'évènements
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern{
    size: u8,
    sequence:Vec<PatternEvent>,
}
/// Représente les erreurs lors de la création d'un évènement.
///
/// Plusieurs erreurs sont capturées : note invalide, début invalide
/// et durée invalide.
#[derive(Debug, PartialEq)]
pub enum PatternError{
    InvalidSize,
    InvalidEventSize,
    InvalidEventOverlap,
}
/// Génère un pattern à partir d'évènement d'un pattern.
///
/// # Arguments
///
/// * `size` - Taille maximal du Pattern
/// * `Vec<PatternEvent>` - Liste d'évènements
///
/// # Retour
///
/// Le pattern crée (Pattern) ou un message d'erreur (PatternError)
///
impl Pattern{
    pub fn new(size: u8) -> Result<Self, PatternError>
    {
        if size == 0 // u8 toujours >= 0
        {
            return Err(PatternError::InvalidSize);
        }
        Ok(Self{size, sequence: Vec::new()})
    }
    pub fn add_event(&mut self, event:PatternEvent)-> Result<(), PatternError>
    {
        // Vérification que l'évènement a ue durée convenable
        if event.start + event.duration > self.size as i32 {
            return Err(PatternError::InvalidEventSize);
        }

        // Vérification que les évènements ne se chevauchent pas
        for e in &self.sequence {
            if events_overlap(e, &event) {
                return Err(PatternError::InvalidEventOverlap);
            }
        }

        // Ajout de l'évènement
        self.sequence.push(event);
        Ok(())
    }
}

// Fonctions supplémentaires
fn events_overlap(a: &PatternEvent, b: &PatternEvent) -> bool {
    let a_end = a.start + a.duration;
    let b_end = b.start + b.duration;
    a.start < b_end && b.start < a_end
}
#[cfg(test)]
mod tests {
    //! Tests unitaires pour la génération d'évènements.
    //!
    //! Ces tests vérifient que les fonctions du module `pattern`
    //! produisent des résultats musicaux corrects pour des cas simples.
    use crate::music::chord::ChordType;
    use crate::music::pattern::Pattern;
    use crate::music::pattern::PatternError;
    use crate::music::pattern::PatternEvent;
    use crate::music::pattern::PatternEventError;
    use crate::music::pattern::MIDI_NOTE_MAX;
    #[test]
    fn new_ok_when_parameters_are_valid() 
    {
        let ev = PatternEvent::new(60, ChordType::Major, 0, 480)
        .expect("should be valid");

        assert_eq!(60, ev.root());
        assert_eq!(0, ev.start());
        assert_eq!(480, ev.duration());
    }
    #[test]
    fn new_nok_when_root_is_negative()
    {
        let ev = PatternEvent::new(-1,ChordType::Major, 0, 480);
        assert_eq!(Err(PatternEventError::InvalidRoot), ev);
    }
    #[test]
    fn new_nok_when_root_is_too_high()
    {
        let ev = PatternEvent::new(MIDI_NOTE_MAX + 1,ChordType::Major, 0, 480);
        assert_eq!(Err(PatternEventError::InvalidRoot), ev);
    }
    #[test]
    fn new_nok_when_start_is_negative()
    {
        let ev = PatternEvent::new(60,ChordType::Major, -1, 480);
        assert_eq!(Err(PatternEventError::InvalidStart), ev);
    }
    #[test]
    fn new_nok_when_duration_is_not_valid()
    {
        let ev_neg = PatternEvent::new(60,ChordType::Major, 0, -1);
        let ev_null = PatternEvent::new(60,ChordType::Major, 0, 0);
        assert_eq!(Err(PatternEventError::InvalidDuration), ev_neg);
        assert_eq!(Err(PatternEventError::InvalidDuration), ev_null);
    }
    #[test]
    fn add_event_ok_when_size_and_no_overlap() {
        let mut pattern = Pattern::new(16).unwrap();
        let ev = PatternEvent::new(0, ChordType::Major, 0, 4).unwrap();

        assert!(pattern.add_event(ev).is_ok());
        assert_eq!(1, pattern.sequence.len());
    }

    #[test]
    fn add_event_err_when_too_long() {
        let mut pattern = Pattern::new(16).unwrap();
        let ev = PatternEvent::new(0, ChordType::Major, 12, 8).unwrap(); // 12 + 8 = 20 > 16

        let res = pattern.add_event(ev);
        assert_eq!(Err(PatternError::InvalidEventSize), res);
    }
    #[test]
    fn add_event_err_when_overlap() {
        let mut pattern = Pattern::new(16).unwrap();
        let ev1 = PatternEvent::new(0, ChordType::Major, 0, 4).unwrap(); // 12 + 8 = 20 > 16
        let ev2 = PatternEvent::new(0, ChordType::Major, 2, 4).unwrap(); // 12 + 8 = 20 > 16

        pattern.add_event(ev1).unwrap();
        let res2 = pattern.add_event(ev2);
        assert_eq!(Err(PatternError::InvalidEventOverlap), res2);
    }
}
