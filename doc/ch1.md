# Chapitre 1 — Prise en main

## Objectif du chapitre

Installer Rust, écrire un premier programme "Hello, world!" avec le compilateur `rustc`, puis découvrir Cargo — le système de compilation et gestionnaire de paquets — qui sera l'outil standard pour tous les projets Rust à venir.

## Concepts abordés

### 1. Installation de Rust via `rustup`

- **Définition** : `rustup` est un outil en ligne de commande qui installe et gère les versions du compilateur Rust.
- **Pourquoi il existe** : simplifie l'installation, les mises à jour et la gestion de multiples toolchains (stable, beta, nightly).
- **Quand l'utiliser** : à chaque fois qu'on installe ou met à jour Rust.
- **Erreurs fréquentes** : oublier d'ajouter Rust au `PATH` sous Windows ; ne pas avoir de linker (installer Xcode sur macOS ou `build-essential` sur Linux).
- **À retenir** : `rustup update` met à jour Rust ; `rustup self uninstall` le désinstalle ; `rustc --version` vérifie l'installation.

### 2. Compilation et exécution avec `rustc`

- **Définition** : `rustc` est le compilateur Rust, il transforme le code source `.rs` en binaire exécutable.
- **Pourquoi il existe** : Rust est un langage compilé à l'avance (ahead-of-time compilation), pas besoin d'interpréteur sur la machine cible.
- **Quand l'utiliser** : pour de très petits programmes ou scripts ; dans la pratique, on utilise plutôt Cargo.
- **Erreurs fréquentes** : oublier le `!` de `println!` (c'est une macro, pas une fonction) ; oublier le point-virgule `;`.
- **À retenir** : `rustc main.rs` produit un binaire `main` (ou `main.exe` sur Windows).

### 3. Structure d'un programme Rust

- **Définition** : tout programme Rust commence par une fonction `main()`.
- **Pourquoi il existe** : `main` est le point d'entrée obligatoire, premier code exécuté.
- **Quand l'utiliser** : dans tout binaire Rust.
- **Erreurs fréquentes** : mettre l'accolade ouvrante sur la ligne suivante (convention : sur la même ligne que `fn main()`).
- **À retenir** : le corps de la fonction est entre `{}` ; l'indentation fait 4 espaces.

### 4. Les macros (`println!`)

- **Définition** : une macro se reconnaît au `!` après son nom ; `println!` affiche du texte dans le terminal.
- **Pourquoi il existe** : les macros permettent de générer du code à la compilation (metaprogramming) ; `println!` gère la syntaxe de formatage sans risque d'erreur à l'exécution.
- **Quand l'utiliser** : `println!` pour afficher du texte ; les macros sont abordées en détail au chapitre 19.
- **Erreurs fréquentes** : écrire `println` sans `!` (appel de fonction inexistante).
- **À retenir** : le `!` distingue une macro d'une fonction.

### 5. Cargo

- **Définition** : Cargo est le système de compilation et de gestion de paquets de Rust.
- **Pourquoi il existe** : il automatise la compilation, le téléchargement des dépendances (crates), et standardise la structure des projets.
- **Quand l'utiliser** : pour tout projet Rust — c'est l'outil par défaut de l'écosystème.
- **Erreurs fréquentes** : lancer `cargo run` sans avoir fait `cargo build` au moins une fois (Cargo le fait automatiquement) ; confondre `cargo check` (vérification rapide) et `cargo build` (produit un binaire).
- **À retenir** : `cargo new` crée un projet ; `cargo build` compile ; `cargo run` compile + exécute ; `cargo check` vérifie la compilation sans produire de binaire.

### 6. Structure d'un projet Cargo

- **Définition** : un projet Cargo a un fichier `Cargo.toml` (configuration) et un dossier `src/` pour le code source ; `Cargo.lock` verrouille les versions des dépendances.
- **Pourquoi il existe** : convention claire et uniforme — le code dans `src/`, la config à la racine.
- **Quand l'utiliser** : dès la création d'un nouveau projet (`cargo new`).
- **Erreurs fréquentes** : placer le code source à la racine au lieu de `src/`.
- **À retenir** : `[package]` définit le nom, la version et l'édition ; `[dependencies]` liste les crates externes.

### 7. Profils de compilation

- **Définition** : deux profils existent : `dev` (défaut, rapide, sans optimisations) et `release` (`cargo build --release`, optimisé, plus lent à compiler).
- **Pourquoi il existe** : compromis entre vitesse de compilation et vitesse d'exécution.
- **Quand l'utiliser** : `dev` pendant le développement ; `release` pour livrer le programme.
- **À retenir** : le binaire `dev` est dans `target/debug/` ; le binaire `release` dans `target/release/`.

## Nouveaux mots-clés

- `fn` — déclaration d'une fonction
- `let` — (non utilisé directement au chapitre 1, mais le chapitre reste introductif ; aucun nouveau mot-clé formel n'est introduit)

Note : le chapitre 1 est un chapitre d'installation et de premier programme, il n'introduit pas encore de mots-clés du langage.

## Fonctions, méthodes et macros importantes

| Commande / Macro | Description |
|---|---|
| `println!("...")` | Macro qui affiche du texte suivi d'un saut de ligne |
| `rustc` | Compilateur Rust |
| `rustup` | Gestionnaire de versions de Rust |
| `cargo build` | Compile le projet |
| `cargo run` | Compile puis exécute le projet |
| `cargo check` | Vérifie la compilation sans produire de binaire |
| `cargo new` | Crée un nouveau projet Cargo |
| `cargo build --release` | Compile avec optimisations |
| `rustc --version` | Affiche la version du compilateur |
| `cargo --version` | Affiche la version de Cargo |
| `rustup doc` | Ouvre la documentation Rust locale dans le navigateur |

## Schéma mental

```
rustup (installer/gérer Rust)
  └─ rustc (compilateur) ──→ binaire
  └─ cargo (outil projet)
       ├─ cargo new      → crée src/main.rs + Cargo.toml
       ├─ cargo build    → target/debug/mon_projet
       ├─ cargo run      → build + exécution
       ├─ cargo check    → vérification rapide (pas de binaire)
       └─ cargo build --release → target/release/mon_projet (optimisé)
```

Le flux typique : on écrit le code dans `src/main.rs`, on édite `Cargo.toml` pour les dépendances, on lance `cargo check` fréquemment pendant l'écriture, `cargo run` pour tester, et `cargo build --release` pour livrer.

## Pièges classiques

1. **Oublier `!` dans `println!`** — `println` sans `!` est une fonction inexistante → erreur de compilation.
2. **Oublier le point-virgule `;`** — Rust considère une ligne sans `;` comme une expression retournée.
3. **Confondre `cargo check` et `cargo build`** — `cargo check` ne produit pas d'exécutable, seulement une vérification.
4. **Placer le code à la racine** — Cargo attend les fichiers sources dans `src/`.
5. **Oublier le linker** — sur macOS, lancer `xcode-select --install` ; sur Linux, installer `build-essential`.
6. **Croire que Rust a besoin d'un runtime** — non : le binaire compilé s'exécute sur toute machine sans Rust installé.
7. **Utiliser `rustc` pour des projets complexes** — `rustc` seul ne gère pas les dépendances ; utiliser Cargo.

## Résumé

1. Rust s'installe via `rustup` (un gestionnaire de toolchains).
2. `rustc` compile un fichier `.rs` en binaire exécutable.
3. Tout programme Rust commence par `fn main()`.
4. `println!` (avec `!`) est une macro, pas une fonction.
5. Cargo est l'outil standard : création (`cargo new`), compilation (`cargo build`), vérification rapide (`cargo check`), exécution (`cargo run`).
6. Les sources vont dans `src/`, la configuration dans `Cargo.toml`.
7. `--release` optimise le binaire pour la production (temps de compilation plus long).
8. `rustup doc` ouvre la documentation hors-ligne.
9. Le `!` distingue une macro d'une fonction classique.
10. Rust compile en avance (ahead-of-time) : pas de runtime requis sur la machine cible.
