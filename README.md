# MIDI Pattern Generator

## Contexte & Objectif

Ce dépôt documente le projet *MIDI Pattern Generator*, ainsi que la **démarche d’apprentissage du langage Rust** associée.

Le projet est conçu par un **ingénieur logiciel embarqué (C, Python, VHDL)**, musicien (guitare, clavier, chant), avec pour objectifs principaux :

* découvrir **Rust à partir de zéro**,
* créer un **outil musical réellement utile** pour la composition et l’exploration musicale,
* structurer un projet logiciel propre, modulaire et testable.

Ce projet sert également de support pour la validation de la **certification CS50**.

---

## Description du projet

**MIDI Pattern Generator** est un outil **en ligne de commande écrit en Rust** permettant de générer automatiquement des **structures musicales** (gammes, accords, patterns) et, à terme, de les exporter sous forme de **fichiers MIDI standards**, importables dans n’importe quel DAW (Ableton, Logic, Reaper, etc.).

L’outil vise à aider les musiciens à :

* générer rapidement des idées musicales,
* comprendre la structure harmonique,
* explorer la composition assistée par algorithmes.

---

## Fonctionnalités prévues

* Génération de gammes musicales (majeures, mineures)
* Génération d’accords (majeurs, mineurs)
* Génération de patterns musicaux
* Choix de la tonalité
* Tempo configurable
* Nombre de mesures configurable
* Export MIDI standard (`.mid`)
* Interface CLI claire et documentée

---

## Utilisation (prévisionnelle)

```bash
cargo run -- --key C --scale minor --tempo 120 --bars 4
```

➡ Génère un fichier `output.mid` importable dans un DAW.

---

## Architecture logicielle

```text
midi-pattern-generator/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs          # Point d’entrée du programme
│   ├── music/
│   │   ├── mod.rs
│   │   ├── scale.rs     # Gammes musicales
│   │   └── chord.rs     # Accords
└── tests/
```

### Principes d’architecture

* `main.rs` orchestre le flux global du programme
* La logique musicale est isolée dans le module `music/`
* Chaque concept musical correspond à un module dédié
* L’architecture est volontairement **simple, modulaire et pédagogique**, sans sur‑ingénierie

---

## Objectifs pédagogiques Rust

Ce projet sert de **fil conducteur pour apprendre Rust**, en mettant l’accent sur :

* `struct`, `enum`, `match`
* Typage fort et sécurité à la compilation
* Organisation modulaire idiomatique
* Fonctions pures et testables
* Tests unitaires intégrés
* Documentation avec `rustdoc`
* Utilisation de l’outillage Rust (`cargo`, `cargo test`, `cargo doc`)

---

## Extensions possibles

* Patterns rythmiques avancés
* Enchaînements harmoniques complexes
* Génération multi‑pistes
* Swing et humanisation
* Export MIDI avancé

---

## Auteur

Projet réalisé par **Alexandre Czerniakowski**, ingénieur logiciel embarqué et musicien, dans le cadre de la certification **CS50**.

---

## État du projet

* ✔ Projet Rust initialisé et compilable
* ✔ Architecture modulaire définie (`music/scale`, `music/chord`, `music/pattern`)
* ✔ Génération de gammes majeures et mineures
* ✔ Génération d’accords majeurs et mineurs
* ✔ Modélisation événementielle des patterns (temps discret, durée explicite)
* ✔ Tests unitaires de base pour la logique musicale
* ✔ Constructeur idiomatique Rust pour `PatternEvent` avec validation et erreurs typées
* ✔ Tests unitaires couvrant les cas valides et invalides des événements
* 🔄 En cours : API du `Pattern` (gestion de la taille, ajout contrôlé d’événements)
* 🔜 Prochaine étape : règles de chevauchement et cohérence temporelle globale
