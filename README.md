# MIDI Pattern Generator

![Static Badge][repo-badge] ![MIT licensed][license-badge] ![image][language-badge]

[repo-badge]: https://img.shields.io/badge/github-repo-blue?logo=github
[license-badge]: https://img.shields.io/badge/license-MIT-green.svg
[language-badge]: https://img.shields.io/badge/Rust-orange?&logo=rust&logoColor=#E57324

## Contexte & Objectif

Ce dépôt documente **l’intégralité du projet de fin de formation CS50** : *MIDI Pattern Generator*, ainsi que la **méthodologie d’apprentissage de Rust** associée.

Le projet est conçu par un **ingénieur logiciel embarqué (C, Python, VHDL)**, musicien (guitare, clavier, chant), souhaitant :

* découvrir **Rust à partir de zéro**,
* valider la **certification CS50**,
* créer un **outil musical réellement utile** pour la composition pour moi-même ou pour d'autres.

---

## Description du projet

**MIDI Pattern Generator** est un outil **en ligne de commande écrit en Rust** permettant de générer automatiquement des **patterns musicaux** (basse, accords, rythme) et de les exporter sous forme de **fichiers MIDI standards**, importables dans n’importe quel DAW (Ableton, Logic, Reaper, etc.).

L’outil vise à aider les musiciens à :

* générer rapidement des idées musicales,
* comprendre la structure harmonique et rythmique,
* explorer la composition.

---

## Fonctionnalités prévues

* Génération de patterns harmoniques simples
* Choix de la tonalité (ex : C, D#, F)
* Choix de la gamme (majeure, mineure, autres)
* Tempo configurable
* Nombre de mesures configurable
* Export MIDI standard (`.mid`)
* Interface CLI claire et documentée

---

## Utilisation (prévisionnelle)

```bash
cargo run -- --key C --scale minor --tempo 120 --bars 4
```

➡ Génère un fichier `output.mid` importable dans une *STAN* - Station Audio Numérique.

---

## Architecture logicielle

L'architecture logicielle n'est pas encore clairement définie mais globalement  elle resemblera à celle ci-dessous :
```text
midi-pattern-generator/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs          # Point d’entrée
│   ├── cli.rs           # Interface CLI (clap)
│   ├── music/
│   │   ├── mod.rs
│   │   ├── scale.rs     # Gammes
│   │   ├── chord.rs     # Accords
│   │   └── pattern.rs   # Patterns musicaux
│   ├── midi/
│   │   ├── mod.rs
│   │   └── writer.rs    # Écriture MIDI
│   └── utils.rs
└── tests/
    └── basic_patterns.rs
```
Une section documentation sera ajotuée et la section tests devra être approfondie.

### Rôles des modules

* **main.rs** : Pointd'entrée de l'application
* **cli.rs** : parsing et validation des arguments utilisateur
* **music/** : logique musicale (gammes, accords, patterns, etc.) => à définir en détails
* **midi/** : conversion des patterns en fichiers MIDI
* **tests/** : Tests divers

---

## Extensions possibles

* Ajotu de différents styles musicaux (jazz, pop, funk)
* Ajout de modes swing et humanisation
* Génération multi‑pistes
* Modes rythmiques avancés
* Interface graphique : à prévoir peut être rapidement en 

---

## Auteur

Projet réalisé dans le cadre du **CS50 – Final Project**
Auteur : *CZERNIAKOWSKI Alexandre*

---

## État du projet

* Architecture : définie
* Planning : validé
* Prochaine étape : **Semaine 1 – Jour 1 : création du projet Cargo**

---