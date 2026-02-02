//! Logique musicale du projet.
//!
//! Ce module contient les structures et fonctions liées
//! aux gammes, accords et patterns musicaux.

/// Représente une gamme musicale.
///
/// Une gamme est définie par une série d'intervalles
/// à partir d'une note racine.
#[derive(Debug, Clone, Copy)]
pub enum ScaleType {
    Major,
    Minor,
}

/// Génère les notes d'une gamme musicale.
///
/// # Arguments
///
/// * `root` - Note racine en valeur MIDI (ex: 60 = C4)
/// * `scale` - Type de gamme (majeure ou mineure)
///
/// # Retour
///
/// Un vecteur contenant les notes MIDI de la gamme.
///
/// # Exemple
///
/// ```
/// use midi_pattern_generator::music::scale::{notes_in_scale, ScaleType};
///
/// let notes = notes_in_scale(60, ScaleType::Major);
/// assert_eq!(notes, vec![60, 62, 64, 65, 67, 69, 71]);
/// ```
pub fn notes_in_scale(root: i32, scale: ScaleType) -> Vec<i32> {
    // Associe à intervals le vecteur representant les intervalles
    // pour la gamme 'scale' et la note 'root' voulue
    let intervals: Vec<i32> = match scale {
        ScaleType::Major => vec![0,2,4,5,7,9,11],
        ScaleType::Minor => vec![0,2,3,5,7,8,10],
    };
    intervals.iter().map(|i: &i32| root + i).collect()
}

#[cfg(test)]
mod tests {
    //! Tests unitaires pour la génération des gammes musicales.
    //!
    //! Ces tests vérifient que les fonctions du module `scale`
    //! produisent des résultats musicaux corrects pour des cas simples.
    use crate::music::scale::ScaleType;
    use crate::music::scale::notes_in_scale;
    #[test]
    fn notes_in_scale_returns_correct_minor_scale() 
    {
        let root_note: i32 = 60; // C4 en midi
        let minor_scale: ScaleType = ScaleType::Minor;
        assert_eq!(notes_in_scale(root_note, minor_scale), [60, 62, 63, 65, 67, 68, 70]);
    }
    #[test]
    fn notes_in_scale_returns_correct_major_scale() 
    {
        let root_note: i32 = 60; // C4 en midi
        let major_scale: ScaleType = ScaleType::Major;
        assert_eq!(notes_in_scale(root_note, major_scale), [60, 62, 64, 65, 67, 69, 71]);
    }
}